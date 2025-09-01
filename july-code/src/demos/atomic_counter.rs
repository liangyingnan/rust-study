use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

pub fn run() {
    let threads = 4usize;
    let adds_per_thread = 50_000usize;
    let expected = threads * adds_per_thread;

    let counter = Arc::new(AtomicUsize::new(0));
    let mut hs = Vec::new();
    for _ in 0..threads {
        let c = Arc::clone(&counter);
        hs.push(thread::spawn(move || {
            for _ in 0..adds_per_thread {
                c.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }
    for h in hs { h.join().unwrap(); }

    let total = counter.load(Ordering::Relaxed);
    println!("[Atomic] 计数结果: {total} (期望: {expected})");
}


