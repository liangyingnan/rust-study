//! 模块化版本的异步编程示例程序
//! 
//! 这个文件展示了如何使用模块化的结构来组织异步编程代码

use anyhow::Result;
use std::time::Duration;
use tokio::time::Instant;

// 模块声明
mod core;
mod examples;
mod utils;
mod tests;

// 导入核心模块
use core::http_client::AsyncHttpClient;
use core::database::database_operations_example;
use core::web_server::{AsyncWebServer, TaskScheduler, RateLimiter};

// 导入示例模块
use examples::basic::{simple_async_examples, timer_example, mutex_example};
use examples::stream::{simple_stream_example, stream_transform_example};
use examples::batch::{simple_batch_example, dynamic_batch_example};
use examples::offline::offline_async_examples;

// 导入工具模块
use utils::error::error_handling_example;
use utils::time::time_utils_example;
use utils::config::config_utils_example;
use utils::logging::logging_utils_example;

// 导入测试模块
use tests::performance::performance_test_example;
use tests::error_handling::error_handling_test_example;
use tests::concurrency::concurrency_test_example;
use tests::integration::integration_test_example;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Rust 异步编程示例程序（模块化版本）");
    println!("=====================================");
    
    // 1. 基础异步示例
    println!("\n=== 基础异步示例 ===");
    simple_async_examples().await?;
    
    // 2. 流处理示例
    println!("\n=== 流处理示例 ===");
    simple_stream_example().await?;
    stream_transform_example().await?;
    
    // 3. 批处理示例
    println!("\n=== 批处理示例 ===");
    simple_batch_example().await?;
    dynamic_batch_example().await?;
    
    // 4. 定时器和互斥锁示例
    println!("\n=== 定时器和互斥锁示例 ===");
    timer_example().await?;
    mutex_example().await?;
    
    // 5. 工具模块示例
    println!("\n=== 工具模块示例 ===");
    time_utils_example().await?;
    error_handling_example().await?;
    config_utils_example().await?;
    logging_utils_example().await?;
    
    // 6. 核心模块示例
    println!("\n=== 核心模块示例 ===");
    
    // HTTP客户端示例
    let http_client = AsyncHttpClient::new();
    let urls = vec![
        "https://httpbin.org/get".to_string(),
        "https://httpbin.org/status/200".to_string(),
        "https://httpbin.org/user-agent".to_string(),
    ];
    
    let start = Instant::now();
    let results = http_client.fetch_multiple_urls(urls).await?;
    let total_time = start.elapsed();
    
    println!("HTTP客户端并发请求完成，总耗时: {:?}", total_time);
    for result in results {
        println!("URL: {}, 状态: {}, 响应时间: {}ms, 内容长度: {:?}",
                result.url, result.status, result.response_time_ms, result.content_length);
    }
    
    // Web服务器示例
    let web_server = AsyncWebServer::new();
    let test_urls = vec![
        "https://httpbin.org/get",
        "https://httpbin.org/user-agent",
        "https://httpbin.org/headers",
    ];
    
    let start = Instant::now();
    let results = web_server.process_multiple_requests(test_urls).await?;
    let server_time = start.elapsed();
    
    println!("Web服务器处理完成，耗时: {:?}", server_time);
    for (i, result) in results.iter().enumerate() {
        println!("结果 {}: {} 字符", i + 1, result.len());
    }
    
    // 数据库操作示例
    database_operations_example().await?;
    
    // 限流器示例
    println!("\n=== 限流器示例 ===");
    let rate_limiter = RateLimiter::new(3, Duration::from_secs(1));
    
    for i in 1..=5 {
        if rate_limiter.allow_request().await {
            println!("请求 {} 被允许", i);
        } else {
            println!("请求 {} 被限制", i);
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    
    // 任务调度器示例
    println!("\n=== 任务调度器示例 ===");
    let scheduler = TaskScheduler::new();
    
    scheduler.add_periodic_task(
        "清理任务",
        Duration::from_secs(1),
        || {
            println!("执行清理任务...");
        },
    ).await;
    
    // 运行调度器 3 秒
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // 7. 测试模块示例
    println!("\n=== 测试模块示例 ===");
    performance_test_example().await?;
    error_handling_test_example().await?;
    concurrency_test_example().await?;
    integration_test_example().await?;
    
    // 8. 离线示例
    println!("\n=== 离线示例 ===");
    offline_async_examples().await?;
    
    println!("\n所有异步操作完成！");
    Ok(())
}
