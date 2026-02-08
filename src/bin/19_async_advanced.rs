use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let (tx, rx) = mpsc::sync_channel::<u32>(2);

    let producer = thread::spawn(move || {
        for i in 0..6 {
            let t0 = Instant::now();
            tx.send(i).expect("send failed");
            let waited = t0.elapsed();
            println!("produce {i}, backpressure_wait_ms={}", waited.as_millis());
        }
    });

    let consumer = thread::spawn(move || {
        while let Ok(v) = rx.recv() {
            println!("consume {v}");
            thread::sleep(Duration::from_millis(120));
        }
    });

    producer.join().expect("producer panic");
    consumer.join().expect("consumer panic");
}
