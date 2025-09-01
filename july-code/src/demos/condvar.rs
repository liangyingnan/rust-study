use std::sync::{Arc, Condvar, Mutex};
use std::thread;

pub fn run() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    let worker = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        {
            // 模拟工作：更现实的示例可在此执行计算/IO
        }
        let mut ready = lock.lock().unwrap();
        *ready = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut ready = lock.lock().unwrap();
    while !*ready {
        ready = cvar.wait(ready).unwrap();
    }
    worker.join().unwrap();
    println!("[Condvar] 条件满足，继续执行");
}


