use anyhow::Result;
use std::time::Duration;
use tokio::time::Instant;
use tokio_stream::StreamExt;

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

async fn basic_async_function() -> i32 {
    tokio::time::sleep(Duration::from_millis(500)).await;
    42
}

async fn async_task(name: &str, delay_ms: u64) -> String {
    println!("开始执行: {}", name);
    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    println!("完成执行: {}", name);
    format!("{} 完成", name)
}

async fn stream_processing_example() -> Result<()> {
    use tokio_stream::{self as stream, StreamExt};
    
    let numbers = stream::iter(1..=10);
    let results: Vec<i32> = numbers
        .map(|n| async move {
            tokio::time::sleep(Duration::from_millis(50)).await;
            n * n
        })
        .buffered(3) // 并发处理3个
        .collect()
        .await;
    
    println!("流处理结果: {:?}", results);
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
    let content = "Hello, Async World!\n这是异步文件操作示例。\n时间: ".to_string() + 
        &chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    fs::write("async_simple_output.txt", content).await?;
    println!("文件写入完成");
    
    // 异步读取文件
    let read_content = fs::read_to_string("async_simple_output.txt").await?;
    println!("读取内容:\n{}", read_content);
    
    // 异步文件信息
    let metadata = fs::metadata("async_simple_output.txt").await?;
    println!("文件大小: {} 字节", metadata.len());
    
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
    
    use std::sync::Arc;
    use tokio::sync::Mutex;
    
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
