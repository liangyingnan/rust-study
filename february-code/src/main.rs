mod cache;
mod text;
mod memory_demo;
mod text_analyzer;

use std::collections::HashMap;
use cache::Cache;

fn main() {
    // 创建缓存集合用于演示
    let mut cache_collection = initialize_caches();
    
    // 演示内存管理功能
    memory_demo::run_ownership_demo(&mut cache_collection);
    
    // 演示文本分析功能
    text_analyzer::analyze_all_caches(&cache_collection);
    
    // 演示借用规则
    memory_demo::run_borrowing_demo(&mut cache_collection);
    
    // 展示最终结果
    print_final_state(&cache_collection);
}

// 初始化缓存集合
fn initialize_caches() -> HashMap<String, Cache> {
    let mut caches = HashMap::new();
    
    // 创建并添加第一个缓存
    let data1 = String::from("Rust 保证内存安全无数据竞争");
    let cache1 = Cache::new(data1);
    caches.insert(String::from("安全特性"), cache1);
    
    // 创建并添加第二个缓存
    let data2 = String::from("所有权系统管理内存无需垃圾回收");
    let cache2 = Cache::new(data2);
    caches.insert(String::from("内存管理"), cache2);
    
    caches
}

// 打印所有缓存的最终状态
fn print_final_state(caches: &HashMap<String, Cache>) {
    println!("\n最终缓存内容:");
    for (key, cache) in caches {
        println!("缓存 '{}': \"{}\"", key, cache.get_data());
    }
}