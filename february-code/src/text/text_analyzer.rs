use std::collections::HashMap;
use crate::cache::Cache;
use crate::text::TextContext;

// 分析所有缓存数据
pub fn analyze_all_caches(caches: &HashMap<String, Cache>) -> usize {
    println!("\n=== 文本分析演示 ===");
    println!("所有缓存的分析：");
    let mut total_words = 0;
    
    for (key, cache) in caches {
        // 创建临时的文本分析上下文
        let context = TextContext::new(cache.get_data());
        let count = context.count_words();
        println!("缓存 '{}' 包含 {} 个单词", key, count);
        total_words += count;
    }
    
    println!("所有缓存总共包含 {} 个单词", total_words);
    total_words
}

// 对特定文本进行高级分析
pub fn perform_advanced_analysis(text: &str) {
    let context = TextContext::new(text);
    
    println!("\n=== 高级文本分析 ===");
    println!("文本: \"{}\"", text);
    println!("单词数: {}", context.count_words());
    println!("最长单词: '{}'", context.longest_word());
    
    // 统计平均单词长度
    let words = text.split_whitespace().collect::<Vec<&str>>();
    if !words.is_empty() {
        let total_length: usize = words.iter().map(|w| w.len()).sum();
        let avg_length = total_length as f64 / words.len() as f64;
        println!("平均单词长度: {:.2}", avg_length);
    }
}