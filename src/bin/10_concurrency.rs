use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

fn run_with_mutex(workers: usize, loops: u64) -> u64 {
    let counter = Arc::new(Mutex::new(0_u64));
    let mut handles = Vec::new();

    for _ in 0..workers {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..loops {
                let mut n = counter.lock().expect("lock poisoned");
                *n += 1;
            }
        }));
    }

    for h in handles {
        h.join().expect("thread panicked");
    }

    *counter.lock().expect("lock poisoned")
}

fn run_with_atomic(workers: usize, loops: u64) -> u64 {
    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::new();

    for _ in 0..workers {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..loops {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for h in handles {
        h.join().expect("thread panicked");
    }

    counter.load(Ordering::Relaxed)
}

fn main() {
    let workers = 4;
    let loops = 10_000;

    let m = run_with_mutex(workers, loops);
    let a = run_with_atomic(workers, loops);

    println!("mutex_counter={m}, atomic_counter={a}");
}
