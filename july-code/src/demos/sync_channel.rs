use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
    let (tx, rx) = mpsc::sync_channel::<u32>(2); // 有界容量=2

    let producer = thread::spawn(move || {
        for i in 0..6u32 {
            // 当缓冲满时，这里会阻塞，体现背压
            tx.send(i).expect("send failed");
        }
        // 发送端在离开作用域时自动 drop，接收端将收到关闭事件
    });

    let consumer = thread::spawn(move || {
        // 模拟较慢消费者
        while let Ok(v) = rx.recv() {
            println!("[SyncChannel] 收到 {v}");
            thread::sleep(Duration::from_millis(20));
        }
        println!("[SyncChannel] 发送端已关闭");
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}


