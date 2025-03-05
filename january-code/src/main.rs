fn main() {
    for i in 0..10 {
        println!("fib({}) = {}", i, fibonacci(i));
    }
}



// 普通实现斐波那契数列
fn fibonacci(n: u32) -> u64 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }
    a
}

// 递归实现（性能警告：n>40会明显变慢）
fn fib_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_recursive(n-1) + fib_recursive(n-2)
    }
}

// 迭代器实现（函数式风格）
fn fib_iterative(n: u32) -> u64 {
    (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
}


// 性能测试
#[test]
fn test_fib() {
    assert_eq!(fib_iterative(50), 12586269025);
    assert_eq!(fib_recursive(50), 12586269025);
    assert_eq!(fib_iterative(10), 55);
    assert_eq!(fib_iterative(10), 55);
}