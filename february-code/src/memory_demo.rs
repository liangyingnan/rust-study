use std::collections::HashMap;
use crate::cache::Cache;
use crate::text::TextContext;

// 演示所有权相关概念
pub fn run_ownership_demo(cache_collection: &mut HashMap<String, Cache>) {
    println!("=== 所有权演示 ===");
    
    // 所有权转移示例
    if let Some((key, cache)) = cache_collection.remove_entry("安全特性") {
        println!("从集合中移除缓存 '{}'", key);
        
        // 所有权转移到函数并返回
        let processed_cache = process_and_return(cache);
        
        // 借用新处理的缓存进行分析
        let analysis = TextContext::new(processed_cache.get_data());
        println!("处理后的缓存内容分析:");
        println!("单词数: {}", analysis.count_words());
        println!("最长单词: '{}'", analysis.longest_word());
        
        // 所有权再次转移回集合
        cache_collection.insert(key, processed_cache);
    }
}

// 演示借用规则
pub fn run_borrowing_demo(cache_collection: &mut HashMap<String, Cache>) {
    println!("\n=== 借用规则演示 ===");
    
    if let Some(cache) = cache_collection.get_mut("内存管理") {
        // 可变借用示例
        cache.update_data(String::from("借用生命周期和所有权是 Rust 的核心概念"));
        
        // 创建引用数据的多个分析上下文 (共享不可变借用)
        let analysis1 = TextContext::new(cache.get_data());
        let analysis2 = TextContext::new(cache.get_data());
        
        // 同时使用多个不可变引用
        println!("多重分析演示：");
        println!("分析1 - 单词数: {}", analysis1.count_words());
        println!("分析2 - 最长单词: '{}'", analysis2.longest_word());
        
        // 生命周期应用示例
        demonstrate_lifetime_concepts(&analysis1, &analysis2);
    }
}

// 演示所有权转移并返回所有权的函数
fn process_and_return(mut cache: Cache) -> Cache {
    // 在函数内部获取缓存的可变引用并修改数据
    cache.update_data(String::from("已处理的数据"));
    // 返回所有权
    cache
}

// 展示生命周期概念的函数
fn demonstrate_lifetime_concepts(analysis1: &TextContext, analysis2: &TextContext) {
    println!("\n=== 生命周期概念演示 ===");
    
    // 使用第一个分析进行单词查找
    match analysis1.find_word("生命周期") {
        Some(pos) => println!("'生命周期' 在位置: {}", pos + 1),
        None => println!("未找到单词"),
    }
    
    // 在同一作用域中同时使用两个引用
    println!("同时使用两个分析实例:");
    println!("分析1中的单词数: {}", analysis1.count_words());
    println!("分析2中的最长单词: '{}'", analysis2.longest_word());
}