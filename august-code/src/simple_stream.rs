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
