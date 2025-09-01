mod demos;
use std::env;

fn main() {
    println!("=== Rust 并发示例 ===");
    let arg = env::args().nth(1).unwrap_or_else(|| "all".to_string());
    match arg.as_str() {
        "all" => {
            demos::mutex_counter::run();
            demos::channels::run();
            demos::rwlock_map::run();
            demos::atomic_counter::run();
            demos::condvar::run();
            demos::sync_channel::run();
            demos::scoped_threads::run();
        }
        "mutex" => demos::mutex_counter::run(),
        "channels" => demos::channels::run(),
        "rwlock" => demos::rwlock_map::run(),
        "atomic" => demos::atomic_counter::run(),
        "condvar" => demos::condvar::run(),
        "sync" => demos::sync_channel::run(),
        "scoped" => demos::scoped_threads::run(),
        other => {
            eprintln!(
                "未知示例: {}\n用法: cargo run -- <all|mutex|channels|rwlock|atomic|condvar|sync|scoped>",
                other
            );
        }
    }
}


