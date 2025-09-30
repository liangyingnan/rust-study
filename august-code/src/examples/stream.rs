//! 异步流处理示例
//! 
//! 包含各种异步流处理的示例：
//! - 基本流处理
//! - 批处理
//! - 流转换
//! - 并发流处理

use anyhow::Result;
use std::time::Duration;
use tokio::time::Instant;

/// 简化的异步流处理示例
pub async fn simple_stream_example() -> Result<()> {
    println!("\n=== 简化异步流处理示例 ===");
    
    // 使用基本的并发处理而不是复杂的流
    let numbers = (1..=10).collect::<Vec<_>>();
    let mut handles = Vec::new();
    
    for chunk in numbers.chunks(3) {
        let chunk = chunk.to_vec();
        let handle = tokio::spawn(async move {
            let mut results = Vec::new();
            for n in chunk {
                tokio::time::sleep(Duration::from_millis(50)).await;
                results.push(n * n);
            }
            results
        });
        handles.push(handle);
    }
    
    let mut all_results = Vec::new();
    for handle in handles {
        if let Ok(chunk_results) = handle.await {
            all_results.extend(chunk_results);
        }
    }
    
    all_results.sort();
    println!("流处理结果: {:?}", all_results);
    Ok(())
}

/// 异步流转换示例
pub async fn stream_transform_example() -> Result<()> {
    println!("\n=== 异步流转换示例 ===");
    
    let numbers = (1..=20).collect::<Vec<_>>();
    let mut handles = Vec::new();
    
    // 将数字分组并异步处理
    for chunk in numbers.chunks(5) {
        let chunk = chunk.to_vec();
        let handle = tokio::spawn(async move {
            let mut results = Vec::new();
            for n in chunk {
                // 模拟异步转换
                tokio::time::sleep(Duration::from_millis(20)).await;
                
                // 应用多种转换
                let transformed = (n as i32)
                    .saturating_mul(2)  // 乘以2
                    .saturating_add(10) // 加10
                    .pow(2); // 平方
                
                results.push(transformed);
            }
            results
        });
        handles.push(handle);
    }
    
    let mut all_results = Vec::new();
    for handle in handles {
        if let Ok(chunk_results) = handle.await {
            all_results.extend(chunk_results);
        }
    }
    
    all_results.sort();
    println!("转换结果（前10个）: {:?}", &all_results[..10]);
    println!("处理了 {} 个数字", all_results.len());
    
    Ok(())
}

/// 异步流过滤示例
pub async fn stream_filter_example() -> Result<()> {
    println!("\n=== 异步流过滤示例 ===");
    
    let numbers = (1..=100).collect::<Vec<_>>();
    let mut handles = Vec::new();
    
    // 分组处理并过滤
    for chunk in numbers.chunks(10) {
        let chunk = chunk.to_vec();
        let handle = tokio::spawn(async move {
            let mut results = Vec::new();
            for n in chunk {
                // 模拟异步处理
                tokio::time::sleep(Duration::from_millis(5)).await;
                
                // 只保留偶数
                if n % 2 == 0 {
                    results.push(n);
                }
            }
            results
        });
        handles.push(handle);
    }
    
    let mut all_results = Vec::new();
    for handle in handles {
        if let Ok(chunk_results) = handle.await {
            all_results.extend(chunk_results);
        }
    }
    
    all_results.sort();
    println!("过滤结果（前10个）: {:?}", &all_results[..10]);
    println!("找到 {} 个偶数", all_results.len());
    
    Ok(())
}

/// 异步流聚合示例
pub async fn stream_aggregate_example() -> Result<()> {
    println!("\n=== 异步流聚合示例 ===");
    
    let numbers = (1..=1000).collect::<Vec<_>>();
    let mut handles = Vec::new();
    
    // 分组处理并计算统计信息
    for chunk in numbers.chunks(100) {
        let chunk = chunk.to_vec();
        let handle = tokio::spawn(async move {
            let mut sum = 0;
            let mut count = 0;
            let mut min = i32::MAX;
            let mut max = i32::MIN;
            
            for n in chunk {
                // 模拟异步处理
                tokio::time::sleep(Duration::from_millis(1)).await;
                
                sum += n;
                count += 1;
                min = min.min(n);
                max = max.max(n);
            }
            
            (sum, count, min, max)
        });
        handles.push(handle);
    }
    
    let mut total_sum = 0;
    let mut total_count = 0;
    let mut global_min = i32::MAX;
    let mut global_max = i32::MIN;
    
    for handle in handles {
        if let Ok((sum, count, min, max)) = handle.await {
            total_sum += sum;
            total_count += count;
            global_min = global_min.min(min);
            global_max = global_max.max(max);
        }
    }
    
    let average = if total_count > 0 {
        total_sum as f64 / total_count as f64
    } else {
        0.0
    };
    
    println!("统计结果:");
    println!("  总数: {}", total_count);
    println!("  总和: {}", total_sum);
    println!("  平均值: {:.2}", average);
    println!("  最小值: {}", global_min);
    println!("  最大值: {}", global_max);
    
    Ok(())
}

/// 异步流合并示例
pub async fn stream_merge_example() -> Result<()> {
    println!("\n=== 异步流合并示例 ===");
    
    // 创建多个数据源
    let source1 = (1..=10).collect::<Vec<_>>();
    let source2 = (11..=20).collect::<Vec<_>>();
    let source3 = (21..=30).collect::<Vec<_>>();
    
    let mut handles = Vec::new();
    
    // 处理每个数据源
    for (i, source) in [source1, source2, source3].iter().enumerate() {
        let source = source.clone();
        let handle = tokio::spawn(async move {
            let mut results = Vec::new();
            for n in source {
                // 模拟异步处理
                tokio::time::sleep(Duration::from_millis(10)).await;
                results.push(format!("源{}: {}", i + 1, n * n));
            }
            results
        });
        handles.push(handle);
    }
    
    // 合并所有结果
    let mut all_results = Vec::new();
    for handle in handles {
        if let Ok(chunk_results) = handle.await {
            all_results.extend(chunk_results);
        }
    }
    
    all_results.sort();
    println!("合并结果（前10个）: {:?}", &all_results[..10]);
    println!("总共处理了 {} 个元素", all_results.len());
    
    Ok(())
}

/// 异步流错误处理示例
pub async fn stream_error_handling_example() -> Result<()> {
    println!("\n=== 异步流错误处理示例 ===");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut handles = Vec::new();
    
    for chunk in numbers.chunks(2) {
        let chunk = chunk.to_vec();
        let handle = tokio::spawn(async move {
            let mut results = Vec::new();
            for n in chunk {
                // 模拟可能失败的操作
                tokio::time::sleep(Duration::from_millis(50)).await;
                
                if n % 3 == 0 {
                    // 模拟错误
                    return Err(anyhow::anyhow!("处理数字 {} 时发生错误", n));
                }
                
                results.push(n * n);
            }
            Ok(results)
        });
        handles.push(handle);
    }
    
    let mut success_count = 0;
    let mut error_count = 0;
    let mut all_results = Vec::new();
    
    for handle in handles {
        match handle.await {
            Ok(Ok(results)) => {
                success_count += 1;
                all_results.extend(results);
            }
            Ok(Err(e)) => {
                error_count += 1;
                println!("处理失败: {}", e);
            }
            Err(e) => {
                error_count += 1;
                println!("任务失败: {}", e);
            }
        }
    }
    
    all_results.sort();
    println!("成功处理: {} 个批次", success_count);
    println!("失败处理: {} 个批次", error_count);
    println!("处理结果: {:?}", all_results);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_simple_stream_example() {
        let result = simple_stream_example().await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_stream_transform_example() {
        let result = stream_transform_example().await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_stream_filter_example() {
        let result = stream_filter_example().await;
        assert!(result.is_ok());
    }
}
