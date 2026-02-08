use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::thread;
use std::time::{Duration, Instant};

struct DelayTicks {
    remaining: u8,
    cancelled: Arc<AtomicBool>,
}

impl Future for DelayTicks {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.cancelled.load(Ordering::Relaxed) {
            return Poll::Ready("cancelled");
        }

        if self.remaining == 0 {
            Poll::Ready("done")
        } else {
            self.remaining -= 1;
            Poll::Pending
        }
    }
}

fn block_on<F: Future>(mut fut: F) -> F::Output {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);

    let raw = RawWaker::new(std::ptr::null(), &VTABLE);
    // SAFETY: no-op waker is sufficient for this demo poll loop.
    let waker = unsafe { Waker::from_raw(raw) };
    let mut cx = Context::from_waker(&waker);

    // SAFETY: fut is pinned and never moved after this point.
    let mut fut = unsafe { Pin::new_unchecked(&mut fut) };

    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => thread::sleep(Duration::from_millis(20)),
        }
    }
}

fn main() {
    // 背压与取消（sync_channel 模拟）
    let (tx, rx) = mpsc::sync_channel::<u32>(2);
    let cancelled = Arc::new(AtomicBool::new(false));

    let producer_cancel = Arc::clone(&cancelled);
    let producer = thread::spawn(move || {
        for i in 0..10 {
            if producer_cancel.load(Ordering::Relaxed) {
                println!("producer sees cancellation, stop");
                break;
            }

            let t0 = Instant::now();
            if tx.send(i).is_err() {
                break;
            }
            let waited = t0.elapsed();
            println!("produce {i}, backpressure_wait_ms={}", waited.as_millis());
        }
    });

    let consumer_cancel = Arc::clone(&cancelled);
    let consumer = thread::spawn(move || {
        let start = Instant::now();
        loop {
            match rx.recv_timeout(Duration::from_millis(80)) {
                Ok(v) => {
                    println!("consume {v}");
                    thread::sleep(Duration::from_millis(120));
                    if start.elapsed() > Duration::from_millis(500) {
                        consumer_cancel.store(true, Ordering::Relaxed);
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    if consumer_cancel.load(Ordering::Relaxed) {
                        println!("consumer timeout + cancelled, stop");
                        break;
                    }
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            }
        }
    });

    producer.join().expect("producer panic");
    consumer.join().expect("consumer panic");

    // Pin/Future 语义最小演示。
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let cancel_flag_setter = Arc::clone(&cancel_flag);
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        cancel_flag_setter.store(true, Ordering::Relaxed);
    });

    let status = block_on(DelayTicks {
        remaining: 10,
        cancelled: cancel_flag,
    });
    println!("manual future status={status}");
}
