use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
    let (tx, rx) = mpsc::channel::<String>();

    let producer_count = 5usize;
    let messages_per_producer = 3usize;
    let mut handles = Vec::new();

    for i in 0..producer_count {
        let tx_i = tx.clone();
        let handle = thread::spawn(move || {
            for j in 0..messages_per_producer {
                let msg = format!("worker-{i} -> message-{j}");
                tx_i.send(msg).expect("send failed");
                thread::sleep(Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }

    drop(tx);

    let mut received = Vec::new();
    for msg in rx {
        received.push(msg);
    }

    for h in handles {
        h.join().expect("producer panicked");
    }

    println!("[Channel] 共收到 {} 条消息", received.len());
    for msg in received.iter().take(5) {
        println!("  [Channel] 收到: {msg}");
    }
}


