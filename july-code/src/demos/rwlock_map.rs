use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

pub fn run() {
    let shared_map: Arc<RwLock<HashMap<String, usize>>> = Arc::new(RwLock::new(HashMap::new()));

    let map_for_writer = Arc::clone(&shared_map);
    let writer = thread::spawn(move || {
        for i in 0..10usize {
            {
                let mut w = map_for_writer.write().expect("rwlock poisoned");
                w.insert(format!("key-{i}"), i);
            }
            thread::sleep(Duration::from_millis(5));
        }
    });

    let mut readers = Vec::new();
    for r_id in 0..3usize {
        let map_for_reader = Arc::clone(&shared_map);
        let reader = thread::spawn(move || {
            for _ in 0..5 {
                let len_now = {
                    let r = map_for_reader.read().expect("rwlock poisoned");
                    r.len()
                };
                println!("[RwLock][reader-{r_id}] 当前可见键数: {len_now}");
                thread::sleep(Duration::from_millis(12));
            }
        });
        readers.push(reader);
    }

    writer.join().expect("writer panicked");
    for r in readers {
        r.join().expect("reader panicked");
    }

    let final_len = shared_map.read().unwrap().len();
    println!("[RwLock] 最终键数: {final_len}");
}


