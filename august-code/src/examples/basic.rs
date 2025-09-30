//! 基础异步编程示例
//! 
//! 包含异步编程的基础示例，不依赖网络连接：
//! - 基本异步函数
//! - 并发执行
//! - 异步定时器
//! - 异步互斥锁
//! - 异步文件操作

use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::Instant;

/// 简单的异步示例，不依赖网络
pub async fn simple_async_examples() -> Result<()> {
    println!("\n=== 简单异步示例（无网络依赖）===");
    
    // 1. 基本异步函数
    println!("1. 基本异步函数");
    let result = basic_async_function().await;
    println!("结果: {}", result);
    
    // 2. 并发执行
    println!("\n2. 并发执行");
    let start = Instant::now();
    let (result1, result2, result3) = tokio::join!(
        async_task("任务1", 1000),
        async_task("任务2", 1500),
        async_task("任务3", 800)
    );
    let total_time = start.elapsed();
    
    println!("任务1结果: {}", result1);
    println!("任务2结果: {}", result2);
    println!("任务3结果: {}", result3);
    println!("总耗时: {:?}", total_time);
    
    // 3. 异步流处理
    println!("\n3. 异步流处理");
    stream_processing_example().await?;
    
    // 4. 异步错误处理
    println!("\n4. 异步错误处理");
    error_handling_example().await?;
    
    // 5. 异步文件操作
    println!("\n5. 异步文件操作");
    file_operations_example().await?;
    
    Ok(())
}

/// 基本异步函数
async fn basic_async_function() -> i32 {
    tokio::time::sleep(Duration::from_millis(500)).await;
    42
}

/// 异步任务
async fn async_task(name: &str, delay_ms: u64) -> String {
    println!("开始执行: {}", name);
    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    println!("完成执行: {}", name);
    format!("{} 完成", name)
}

/// 异步流处理示例
async fn stream_processing_example() -> Result<()> {
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

/// 异步错误处理示例
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
    
    // 模拟错误恢复
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

/// 模拟操作
async fn simulate_operation(attempt: i32) -> Result<String> {
    tokio::time::sleep(Duration::from_millis(50)).await;
    
    if attempt < 3 {
        Err(anyhow::anyhow!("模拟失败"))
    } else {
        Ok("操作成功".to_string())
    }
}

/// 异步文件操作示例
async fn file_operations_example() -> Result<()> {
    use tokio::fs;
    
    // 异步写入文件
    let content = format!(
        "Hello, Async World!\n这是异步文件操作示例。\n时间: {}\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
    );
    
    fs::write("async_basic_output.txt", content).await?;
    println!("文件写入完成");
    
    // 异步读取文件
    let read_content = fs::read_to_string("async_basic_output.txt").await?;
    println!("读取内容:\n{}", read_content);
    
    // 异步文件信息
    let metadata = fs::metadata("async_basic_output.txt").await?;
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

/// 异步任务池示例
pub async fn task_pool_example() -> Result<()> {
    println!("\n=== 异步任务池示例 ===");
    
    let mut handles = Vec::new();
    
    // 创建多个异步任务
    for i in 1..=5 {
        let handle = tokio::spawn(async move {
            let duration = Duration::from_millis(i * 100);
            tokio::time::sleep(duration).await;
            println!("任务 {} 完成", i);
            i * i
        });
        handles.push(handle);
    }
    
    // 等待所有任务完成并收集结果
    let mut results = Vec::new();
    for handle in handles {
        if let Ok(result) = handle.await {
            results.push(result);
        }
    }
    
    println!("所有任务结果: {:?}", results);
    Ok(())
}

/// 异步定时器示例
pub async fn timer_example() -> Result<()> {
    println!("\n=== 异步定时器示例 ===");
    
    // 一次性定时器
    println!("设置一次性定时器（2秒后触发）");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("一次性定时器触发！");
    
    // 周期性定时器
    println!("设置周期性定时器（每1秒触发一次，共3次）");
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    
    for i in 1..=3 {
        interval.tick().await;
        println!("周期性定时器触发 #{}", i);
    }
    
    Ok(())
}

/// 异步互斥锁示例
pub async fn mutex_example() -> Result<()> {
    println!("\n=== 异步互斥锁示例 ===");
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    
    // 创建多个任务并发修改计数器
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            for _ in 0..10 {
                let mut count = counter.lock().await;
                *count += 1;
                println!("任务 {} 增加计数到 {}", i, *count);
                // 模拟一些工作
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        });
        handles.push(handle);
    }
    
    // 等待所有任务完成
    for handle in handles {
        handle.await?;
    }
    
    let final_count = *counter.lock().await;
    println!("最终计数: {}", final_count);
    
    Ok(())
}

/// 异步资源管理示例
pub async fn resource_management_example() -> Result<()> {
    println!("\n=== 异步资源管理示例 ===");
    
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_basic_async_function() {
        let result = basic_async_function().await;
        assert_eq!(result, 42);
    }
    
    #[tokio::test]
    async fn test_async_task() {
        let result = async_task("测试任务", 10).await;
        assert_eq!(result, "测试任务 完成");
    }
    
    #[tokio::test]
    async fn test_simulate_operation() {
        // 测试失败情况
        let result = simulate_operation(1).await;
        assert!(result.is_err());
        
        // 测试成功情况
        let result = simulate_operation(3).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "操作成功");
    }
}
