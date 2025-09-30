//! 异步批处理示例
//! 
//! 包含各种异步批处理的示例：
//! - 基本批处理
//! - 动态批处理
//! - 批处理优化
//! - 批处理监控

use anyhow::Result;
use std::time::Duration;
use tokio::time::Instant;

/// 简化的异步批处理示例
pub async fn simple_batch_example() -> Result<()> {
    println!("\n=== 简化异步批处理示例 ===");
    
    let items = (1..=20).collect::<Vec<_>>();
    let batch_size = 5;
    
    println!("开始批处理 {} 个项目，批次大小: {}", items.len(), batch_size);
    
    let start = Instant::now();
    let mut handles = Vec::new();
    
    for chunk in items.chunks(batch_size) {
        let chunk = chunk.to_vec();
        let handle = tokio::spawn(async move {
            let mut results = Vec::new();
            for item in chunk {
                tokio::time::sleep(Duration::from_millis(100)).await;
                results.push(item * item);
            }
            results
        });
        handles.push(handle);
    }
    
    let mut all_results = Vec::new();
    for handle in handles {
        if let Ok(batch_results) = handle.await {
            all_results.extend(batch_results);
        }
    }
    
    let total_time = start.elapsed();
    println!("批处理完成，耗时: {:?}", total_time);
    println!("处理结果（前10个）: {:?}", &all_results[..10]);
    
    Ok(())
}

/// 动态批处理示例
pub async fn dynamic_batch_example() -> Result<()> {
    println!("\n=== 动态批处理示例 ===");
    
    let items = (1..=50).collect::<Vec<_>>();
    let mut batch_size = 3;
    let mut total_processed = 0;
    
    println!("开始动态批处理 {} 个项目", items.len());
    
    let start = Instant::now();
    let mut current_batch = Vec::new();
    let mut handles = Vec::new();
    
    for item in items {
        current_batch.push(item);
        
        // 当批次达到指定大小时处理
        if current_batch.len() >= batch_size {
            let batch = current_batch.clone();
            let handle = tokio::spawn(async move {
                let mut results = Vec::new();
                for item in batch {
                    // 模拟处理时间
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    results.push(item * 2);
                }
                results
            });
            handles.push(handle);
            
            current_batch.clear();
            total_processed += batch_size;
            
            // 动态调整批次大小
            if total_processed % 15 == 0 {
                batch_size = (batch_size + 1).min(10);
                println!("调整批次大小为: {}", batch_size);
            }
        }
    }
    
    // 处理剩余的批次
    if !current_batch.is_empty() {
        let batch = current_batch.clone();
        let handle = tokio::spawn(async move {
            let mut results = Vec::new();
            for item in batch {
                tokio::time::sleep(Duration::from_millis(50)).await;
                results.push(item * 2);
            }
            results
        });
        handles.push(handle);
    }
    
    let mut all_results = Vec::new();
    let batch_count = handles.len();
    for handle in handles {
        if let Ok(batch_results) = handle.await {
            all_results.extend(batch_results);
        }
    }
    
    let total_time = start.elapsed();
    println!("动态批处理完成，耗时: {:?}", total_time);
    println!("处理了 {} 个批次", batch_count);
    println!("处理结果（前10个）: {:?}", &all_results[..10]);
    
    Ok(())
}

/// 批处理优化示例
pub async fn optimized_batch_example() -> Result<()> {
    println!("\n=== 批处理优化示例 ===");
    
    let items = (1..=100).collect::<Vec<_>>();
    let optimal_batch_size = find_optimal_batch_size().await;
    
    println!("开始优化批处理 {} 个项目，最优批次大小: {}", items.len(), optimal_batch_size);
    
    let start = Instant::now();
    let mut handles = Vec::new();
    
    for chunk in items.chunks(optimal_batch_size) {
        let chunk = chunk.to_vec();
        let handle = tokio::spawn(async move {
            let mut results = Vec::new();
            let mut batch_start = Instant::now();
            
            for item in chunk {
                // 模拟处理时间
                tokio::time::sleep(Duration::from_millis(20)).await;
                results.push(item * item);
            }
            
            let batch_time = batch_start.elapsed();
            println!("批次处理完成，耗时: {:?}", batch_time);
            results
        });
        handles.push(handle);
    }
    
    let mut all_results = Vec::new();
    let batch_count = handles.len();
    for handle in handles {
        if let Ok(batch_results) = handle.await {
            all_results.extend(batch_results);
        }
    }
    
    let total_time = start.elapsed();
    println!("优化批处理完成，总耗时: {:?}", total_time);
    println!("处理了 {} 个批次", batch_count);
    println!("平均每批次耗时: {:?}", total_time / batch_count as u32);
    
    Ok(())
}

/// 批处理监控示例
pub async fn monitored_batch_example() -> Result<()> {
    println!("\n=== 批处理监控示例 ===");
    
    let items = (1..=200).collect::<Vec<_>>();
    let batch_size = 10;
    
    println!("开始监控批处理 {} 个项目，批次大小: {}", items.len(), batch_size);
    
    let start = Instant::now();
    let mut handles = Vec::new();
    let mut batch_stats = Vec::new();
    
    for (batch_idx, chunk) in items.chunks(batch_size).enumerate() {
        let chunk = chunk.to_vec();
        let batch_idx = batch_idx + 1;
        
        let handle = tokio::spawn(async move {
            let batch_start = Instant::now();
            let mut results = Vec::new();
            let mut success_count = 0;
            let mut error_count = 0;
            
            for item in chunk {
                // 模拟处理时间
                tokio::time::sleep(Duration::from_millis(30)).await;
                
                // 模拟偶尔的错误
                if item % 13 == 0 {
                    error_count += 1;
                    println!("批次 {} 处理项目 {} 时发生错误", batch_idx, item);
                } else {
                    results.push(item * item);
                    success_count += 1;
                }
            }
            
            let batch_time = batch_start.elapsed();
            let stats = BatchStats {
                batch_idx,
                success_count,
                error_count,
                processing_time: batch_time,
                result_count: results.len(),
            };
            
            (results, stats)
        });
        handles.push(handle);
    }
    
    let mut all_results = Vec::new();
    let mut total_success = 0;
    let mut total_errors = 0;
    
    for handle in handles {
        if let Ok((batch_results, stats)) = handle.await {
            all_results.extend(batch_results);
            total_success += stats.success_count;
            total_errors += stats.error_count;
            batch_stats.push(stats);
        }
    }
    
    let total_time = start.elapsed();
    
    // 打印统计信息
    println!("\n批处理统计信息:");
    println!("  总耗时: {:?}", total_time);
    println!("  总批次: {}", batch_stats.len());
    println!("  成功处理: {} 个项目", total_success);
    println!("  处理失败: {} 个项目", total_errors);
    println!("  成功率: {:.2}%", (total_success as f64 / (total_success + total_errors) as f64) * 100.0);
    
    // 打印每个批次的统计
    for stats in &batch_stats {
        println!("  批次 {}: 成功={}, 失败={}, 耗时={:?}", 
                stats.batch_idx, stats.success_count, stats.error_count, stats.processing_time);
    }
    
    Ok(())
}

/// 批次统计信息
#[derive(Debug)]
struct BatchStats {
    batch_idx: usize,
    success_count: usize,
    error_count: usize,
    processing_time: Duration,
    result_count: usize,
}

/// 寻找最优批次大小
async fn find_optimal_batch_size() -> usize {
    let test_sizes = vec![5, 10, 15, 20];
    let mut best_size = 5;
    let mut best_time = Duration::from_secs(1000);
    
    for size in test_sizes {
        let test_items = (1..=30).collect::<Vec<_>>();
        let start = Instant::now();
        
        let mut handles = Vec::new();
        for chunk in test_items.chunks(size) {
            let chunk = chunk.to_vec();
            let handle = tokio::spawn(async move {
                for item in chunk {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                    let _ = item * item;
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let _ = handle.await;
        }
        
        let elapsed = start.elapsed();
        if elapsed < best_time {
            best_time = elapsed;
            best_size = size;
        }
        
        println!("批次大小 {} 耗时: {:?}", size, elapsed);
    }
    
    best_size
}

/// 批处理管道示例
pub async fn batch_pipeline_example() -> Result<()> {
    println!("\n=== 批处理管道示例 ===");
    
    let items = (1..=60).collect::<Vec<_>>();
    let batch_size = 6;
    
    println!("开始批处理管道 {} 个项目，批次大小: {}", items.len(), batch_size);
    
    let start = Instant::now();
    
    // 第一阶段：数据预处理
    let preprocessed = preprocess_batch(items, batch_size).await?;
    
    // 第二阶段：数据转换
    let transformed = transform_batch(preprocessed, batch_size).await?;
    
    // 第三阶段：数据后处理
    let final_results = postprocess_batch(transformed, batch_size).await?;
    
    let total_time = start.elapsed();
    println!("批处理管道完成，总耗时: {:?}", total_time);
    println!("最终结果（前10个）: {:?}", &final_results[..10]);
    
    Ok(())
}

/// 预处理批次
async fn preprocess_batch(items: Vec<i32>, batch_size: usize) -> Result<Vec<i32>> {
    println!("  阶段1: 数据预处理");
    let mut handles = Vec::new();
    
    for chunk in items.chunks(batch_size) {
        let chunk = chunk.to_vec();
        let handle = tokio::spawn(async move {
            let mut results = Vec::new();
            for item in chunk {
                tokio::time::sleep(Duration::from_millis(20)).await;
                results.push(item * 2); // 预处理：乘以2
            }
            results
        });
        handles.push(handle);
    }
    
    let mut all_results = Vec::new();
    for handle in handles {
        if let Ok(batch_results) = handle.await {
            all_results.extend(batch_results);
        }
    }
    
    Ok(all_results)
}

/// 转换批次
async fn transform_batch(items: Vec<i32>, batch_size: usize) -> Result<Vec<i32>> {
    println!("  阶段2: 数据转换");
    let mut handles = Vec::new();
    
    for chunk in items.chunks(batch_size) {
        let chunk = chunk.to_vec();
        let handle = tokio::spawn(async move {
            let mut results = Vec::new();
            for item in chunk {
                tokio::time::sleep(Duration::from_millis(30)).await;
                results.push(item * item); // 转换：平方
            }
            results
        });
        handles.push(handle);
    }
    
    let mut all_results = Vec::new();
    for handle in handles {
        if let Ok(batch_results) = handle.await {
            all_results.extend(batch_results);
        }
    }
    
    Ok(all_results)
}

/// 后处理批次
async fn postprocess_batch(items: Vec<i32>, batch_size: usize) -> Result<Vec<i32>> {
    println!("  阶段3: 数据后处理");
    let mut handles = Vec::new();
    
    for chunk in items.chunks(batch_size) {
        let chunk = chunk.to_vec();
        let handle = tokio::spawn(async move {
            let mut results = Vec::new();
            for item in chunk {
                tokio::time::sleep(Duration::from_millis(25)).await;
                results.push(item + 100); // 后处理：加100
            }
            results
        });
        handles.push(handle);
    }
    
    let mut all_results = Vec::new();
    for handle in handles {
        if let Ok(batch_results) = handle.await {
            all_results.extend(batch_results);
        }
    }
    
    Ok(all_results)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_simple_batch_example() {
        let result = simple_batch_example().await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_dynamic_batch_example() {
        let result = dynamic_batch_example().await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_find_optimal_batch_size() {
        let size = find_optimal_batch_size().await;
        assert!(size > 0);
    }
}
