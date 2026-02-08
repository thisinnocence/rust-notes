use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0_u64));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        let h = thread::spawn(move || {
            for _ in 0..10_000 {
                let mut n = counter.lock().expect("lock poisoned");
                *n += 1;
            }
        });
        handles.push(h);
    }

    for h in handles {
        h.join().expect("thread panicked");
    }

    let result = *counter.lock().expect("lock poisoned");
    println!("counter={result}");
}
