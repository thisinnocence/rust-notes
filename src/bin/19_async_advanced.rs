use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
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
}
