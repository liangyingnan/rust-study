use anyhow::Result;
use std::time::Duration;
use tokio::time::Instant;

mod simple_example;

use simple_example::{simple_async_examples, task_pool_example, timer_example, mutex_example};

/// 离线版本的异步编程示例
/// 不依赖网络连接，适合在没有网络的环境中运行
#[tokio::main]
async fn main() -> Result<()> {
    println!("Rust 异步编程示例程序（离线版本）");
    println!("=====================================");
    println!("注意：此版本不包含网络相关功能，适合离线环境");
    println!();
    
    // 1. 基本异步示例
    simple_async_examples().await?;
    
    // 2. 任务池示例
    task_pool_example().await?;
    
    // 3. 定时器示例
    timer_example().await?;
    
    // 4. 互斥锁示例
    mutex_example().await?;
    
    // 5. 异步流处理示例
    println!("\n=== 异步流处理示例 ===");
    stream_processing_example().await?;
    
    // 6. 异步错误处理示例
    println!("\n=== 异步错误处理示例 ===");
    error_handling_example().await?;
    
    // 7. 异步文件操作示例
    println!("\n=== 异步文件操作示例 ===");
    file_operations_example().await?;
    
    // 8. 异步批处理示例
    println!("\n=== 异步批处理示例 ===");
    batch_processing_example().await?;
    
    // 9. 异步资源管理示例
    println!("\n=== 异步资源管理示例 ===");
    resource_management_example().await?;
    
    println!("\n所有离线异步操作完成！");
    println!("如需体验网络功能，请运行 'cargo run' 或使用网络版本");
    
    Ok(())
}

async fn stream_processing_example() -> Result<()> {
    use tokio_stream::{self as stream, StreamExt};
    
    let numbers = stream::iter(1..=20);
    let results: Vec<i32> = numbers
        .map(|n| async move {
            // 模拟异步处理
            tokio::time::sleep(Duration::from_millis(50)).await;
            n * n
        })
        .buffered(5) // 并发处理5个
        .collect()
        .await;
    
    println!("流处理结果（前10个）: {:?}", &results[..10]);
    println!("处理了 {} 个数字", results.len());
    Ok(())
}

async fn error_handling_example() -> Result<()> {
    // 模拟可能失败的操作
    let result = tokio::time::timeout(
        Duration::from_millis(100),
        async {
            tokio::time::sleep(Duration::from_millis(200)).await;
            "操作完成"
        }
    ).await;
    
    match result {
        Ok(value) => println!("操作成功: {}", value),
        Err(_) => println!("操作超时"),
    }
    
    // 模拟重试机制
    for i in 1..=3 {
        match simulate_operation(i).await {
            Ok(result) => {
                println!("操作 {} 成功: {}", i, result);
                break;
            }
            Err(e) => {
                println!("操作 {} 失败: {}，重试中...", i, e);
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }
    
    Ok(())
}

async fn simulate_operation(attempt: i32) -> Result<String> {
    tokio::time::sleep(Duration::from_millis(50)).await;
    
    if attempt < 3 {
        Err(anyhow::anyhow!("模拟失败"))
    } else {
        Ok("操作成功".to_string())
    }
}

async fn file_operations_example() -> Result<()> {
    use tokio::fs;
    
    // 异步写入文件
    let content = format!(
        "Hello, Async World!\n这是异步文件操作示例。\n时间: {}\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
    );
    
    fs::write("async_offline_output.txt", content).await?;
    println!("文件写入完成");
    
    // 异步读取文件
    let read_content = fs::read_to_string("async_offline_output.txt").await?;
    println!("读取内容:\n{}", read_content);
    
    // 异步文件信息
    let metadata = fs::metadata("async_offline_output.txt").await?;
    println!("文件大小: {} 字节", metadata.len());
    
    // 异步目录操作
    let mut entries = fs::read_dir(".").await?;
    let mut file_count = 0;
    while let Some(entry) = entries.next_entry().await? {
        if entry.file_type().await?.is_file() {
            file_count += 1;
        }
    }
    println!("当前目录文件数量: {}", file_count);
    
    Ok(())
}

async fn batch_processing_example() -> Result<()> {
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
                // 模拟处理
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

async fn resource_management_example() -> Result<()> {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    
    struct Resource {
        id: String,
        data: String,
        created_at: Instant,
    }
    
    impl Drop for Resource {
        fn drop(&mut self) {
            println!("资源 {} 被释放（存活时间: {:?}）", 
                    self.id, self.created_at.elapsed());
        }
    }
    
    let resources = Arc::new(Mutex::new(Vec::new()));
    
    // 创建资源
    for i in 1..=5 {
        let resource = Resource {
            id: format!("resource_{}", i),
            data: format!("data_{}", i),
            created_at: Instant::now(),
        };
        
        let mut resources = resources.lock().await;
        resources.push(resource);
    }
    
    println!("创建了 5 个资源");
    
    // 异步处理资源
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let resources = Arc::clone(&resources);
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(200)).await;
                
                let mut resources = resources.lock().await;
                if let Some(resource) = resources.pop() {
                    println!("处理资源: {} (数据: {})", resource.id, resource.data);
                }
            })
        })
        .collect();
    
    // 等待所有处理完成
    for handle in handles {
        handle.await?;
    }
    
    println!("所有资源处理完成");
    Ok(())
}
