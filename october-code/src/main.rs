//! 性能优化示例项目 - 主程序
//! 
//! 运行此程序可以查看优化前后的性能对比

use performance_optimization_demo::{optimized, unoptimized};
use rand::Rng;
use std::time::Instant;

fn generate_test_data(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| rng.gen_range(-1000..=1000))
        .collect()
}

fn main() {
    println!("性能优化示例项目");
    println!("==================\n");

    let data = generate_test_data(100000);
    let iterations = 100;

    // 测试1: 计算平均值
    println!("测试1: 计算平均值 (数据量: {}, 迭代次数: {})", data.len(), iterations);
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = unoptimized::calculate_average(&data);
    }
    let unopt_time = start.elapsed();
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = optimized::calculate_average(&data);
    }
    let opt_time = start.elapsed();
    
    println!("  未优化版本: {:?}", unopt_time);
    println!("  优化版本:   {:?}", opt_time);
    println!("  性能提升:   {:.2}x\n", unopt_time.as_secs_f64() / opt_time.as_secs_f64());

    // 测试2: 查找最频繁数字
    println!("测试2: 查找最频繁数字 (数据量: {}, 迭代次数: {})", data.len(), iterations);
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = unoptimized::find_most_frequent(&data);
    }
    let unopt_time = start.elapsed();
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = optimized::find_most_frequent(&data);
    }
    let opt_time = start.elapsed();
    
    println!("  未优化版本: {:?}", unopt_time);
    println!("  优化版本:   {:?}", opt_time);
    println!("  性能提升:   {:.2}x\n", unopt_time.as_secs_f64() / opt_time.as_secs_f64());

    // 测试3: 过滤和转换
    println!("测试3: 过滤和转换 (数据量: {}, 迭代次数: {})", data.len(), iterations);
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = unoptimized::filter_and_transform(&data);
    }
    let unopt_time = start.elapsed();
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = optimized::filter_and_transform(&data);
    }
    let opt_time = start.elapsed();
    
    println!("  未优化版本: {:?}", unopt_time);
    println!("  优化版本:   {:?}", opt_time);
    println!("  性能提升:   {:.2}x\n", unopt_time.as_secs_f64() / opt_time.as_secs_f64());

    // 测试4: 处理字符串
    let small_data: Vec<i32> = (0..10000).collect();
    println!("测试4: 处理字符串 (数据量: {}, 迭代次数: {})", small_data.len(), iterations);
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = unoptimized::process_strings(&small_data);
    }
    let unopt_time = start.elapsed();
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = optimized::process_strings(&small_data);
    }
    let opt_time = start.elapsed();
    
    println!("  未优化版本: {:?}", unopt_time);
    println!("  优化版本:   {:?}", opt_time);
    println!("  性能提升:   {:.2}x\n", unopt_time.as_secs_f64() / opt_time.as_secs_f64());

    println!("提示: 运行 'cargo bench' 进行更详细的基准测试");
}

