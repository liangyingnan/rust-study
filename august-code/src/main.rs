use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
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
use utils::time::time_utils_example;
use utils::config::config_utils_example;
use utils::logging::logging_utils_example;

// 导入测试模块
use tests::performance::performance_test_example;
use tests::error_handling::error_handling_test_example;
use tests::concurrency::concurrency_test_example;
use tests::integration::integration_test_example;

#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    url: String,
    status: u16,
    response_time_ms: u64,
    content_length: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct HttpBinResponse {
    url: String,
    headers: serde_json::Value,
    origin: String,
}

/// 异步获取单个 URL 的数据
async fn fetch_url(client: &Client, url: &str) -> Result<ApiResponse> {
    let start = Instant::now();
    
    let response = client
        .get(url)
        .timeout(Duration::from_secs(10))
        .send()
        .await?;
    
    let status = response.status().as_u16();
    let content_length = response.content_length().map(|len| len as usize);
    let response_time = start.elapsed().as_millis() as u64;
    
    // 读取响应体（可选）
    let _body = response.text().await?;
    
    Ok(ApiResponse {
        url: url.to_string(),
        status,
        response_time_ms: response_time,
        content_length,
    })
}

/// 并发获取多个 URL 的数据
async fn fetch_multiple_urls(urls: Vec<String>) -> Result<Vec<ApiResponse>> {
    let client = Client::new();
    let mut handles = Vec::new();
    
    // 为每个 URL 创建异步任务
    for url in urls {
        let client_clone = client.clone();
        let handle = tokio::spawn(async move {
            fetch_url(&client_clone, &url).await
        });
        handles.push(handle);
    }
    
    // 等待所有任务完成
    let mut results = Vec::new();
    for handle in handles {
        match handle.await? {
            Ok(response) => results.push(response),
            Err(e) => eprintln!("请求失败: {}", e),
        }
    }
    
    Ok(results)
}

/// 使用 join! 宏并发执行多个异步操作
async fn concurrent_operations() -> Result<()> {
    println!("\n=== 使用 join! 宏并发执行 ===");
    
    let start = Instant::now();
    
    let client1 = Client::new();
    let client2 = Client::new();
    let client3 = Client::new();
    let (result1, result2, result3) = tokio::join!(
        fetch_url(&client1, "https://httpbin.org/delay/1"),
        fetch_url(&client2, "https://httpbin.org/delay/2"),
        fetch_url(&client3, "https://httpbin.org/delay/1")
    );
    
    let total_time = start.elapsed();
    
    println!("总执行时间: {:?}", total_time);
    println!("结果1: {:?}", result1);
    println!("结果2: {:?}", result2);
    println!("结果3: {:?}", result3);
    
    Ok(())
}

/// 异步流处理示例
async fn stream_processing() -> Result<()> {
    println!("\n=== 异步流处理示例 ===");
    
    use tokio_stream::{self as stream, StreamExt};
    
    let numbers = stream::iter(1..=5);
    let results: Vec<i32> = numbers
        .then(|n| async move {
            // 模拟异步处理
            tokio::time::sleep(Duration::from_millis(100)).await;
            n * n
        })
        .collect()
        .await;
    
    println!("流处理结果: {:?}", results);
    Ok(())
}

/// 异步错误处理示例
async fn error_handling_example() -> Result<()> {
    println!("\n=== 异步错误处理示例 ===");
    
    let client = Client::new();
    
    // 处理超时错误
    let result = tokio::time::timeout(
        Duration::from_secs(2),
        client.get("https://httpbin.org/delay/3").send()
    ).await;
    
    match result {
        Ok(Ok(response)) => println!("请求成功: {}", response.status()),
        Ok(Err(e)) => println!("请求失败: {}", e),
        Err(_) => println!("请求超时"),
    }
    
    // 处理网络错误
    let invalid_url = "https://invalid-url-that-does-not-exist.com";
    match fetch_url(&client, invalid_url).await {
        Ok(response) => println!("意外成功: {:?}", response),
        Err(e) => println!("预期的错误: {}", e),
    }
    
    Ok(())
}

/// 异步文件操作示例
async fn async_file_operations() -> Result<()> {
    println!("\n=== 异步文件操作示例 ===");
    
    use tokio::fs;
    
    // 异步写入文件
    let content = "Hello, Async World!\n这是异步文件操作示例。";
    fs::write("async_output.txt", content).await?;
    println!("文件写入完成");
    
    // 异步读取文件
    let read_content = fs::read_to_string("async_output.txt").await?;
    println!("读取内容: {}", read_content);
    
    // 异步文件信息
    let metadata = fs::metadata("async_output.txt").await?;
    println!("文件大小: {} 字节", metadata.len());
    
    Ok(())
}

/// 异步任务池示例
async fn task_pool_example() -> Result<()> {
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

/// 异步 HTTP 客户端高级用法
async fn advanced_http_client() -> Result<()> {
    println!("\n=== 高级 HTTP 客户端用法 ===");
    
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("Rust Async Client/1.0")
        .build()?;
    
    // 发送 JSON 数据
    let json_data = serde_json::json!({
        "name": "Rust Async",
        "version": "1.0"
    });
    
    let response = client
        .post("https://httpbin.org/post")
        .json(&json_data)
        .send()
        .await?;
    
    let status = response.status();
    println!("POST 请求状态: {}", status);
    
    // 解析 JSON 响应
    if let Ok(json_response) = response.json::<HttpBinResponse>().await {
        println!("响应 URL: {}", json_response.url);
        println!("来源 IP: {}", json_response.origin);
    }
    
    Ok(())
}

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
    error_handling_test_example().await?;
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
