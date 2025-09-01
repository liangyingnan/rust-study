use std::thread;

pub fn run() {
    let data = String::from("hello scoped");
    let mut acc = 0usize;

    thread::scope(|s| {
        s.spawn(|| {
            // 只读借用外部数据
            println!("[Scoped] data = {}", data);
        });
        s.spawn(|| {
            // 修改外部可变变量需配合同步原语；此处仅演示作用域内可访问同一栈帧变量
            // 为避免数据竞争，这里不做可变借用写入，仅演示生命周期安全
            println!("[Scoped] acc (只读访问) = {}", acc);
        });
    });
    println!("[Scoped] 作用域结束，子线程已完成");
}


