use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    let num_threads: usize = 10;
    let increments_per_thread: usize = 10_000;
    let expected_total: usize = num_threads * increments_per_thread;

    let shared_counter = Arc::new(Mutex::new(0usize));
    let mut handles = Vec::new();

    for _ in 0..num_threads {
        let counter = Arc::clone(&shared_counter);
        let handle = thread::spawn(move || {
            let local_add = increments_per_thread;
            let mut guard = counter.lock().expect("mutex poisoned");
            *guard += local_add;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().expect("counter thread panicked");
    }

    let result = *shared_counter.lock().unwrap();
    println!("[Mutex] 计数结果: {result} (期望: {expected_total})");
}


