//! 离线异步编程示例
//! 
//! 包含不依赖网络连接的异步编程示例：
//! - 基础异步操作
//! - 文件系统操作
//! - 本地数据处理
//! - 模拟网络操作

use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::Instant;

/// 离线版本的异步编程示例
pub async fn offline_async_examples() -> Result<()> {
    println!("\n=== 离线异步编程示例 ===");
    
    // 1. 基本异步操作
    basic_async_operations().await?;
    
    // 2. 文件系统操作
    filesystem_operations().await?;
    
    // 3. 本地数据处理
    local_data_processing().await?;
    
    // 4. 模拟网络操作
    simulated_network_operations().await?;
    
    // 5. 异步资源管理
    async_resource_management().await?;
    
    Ok(())
}

/// 基本异步操作
async fn basic_async_operations() -> Result<()> {
    println!("\n--- 基本异步操作 ---");
    
    // 异步函数调用
    let result = async_function(42).await;
    println!("异步函数结果: {}", result);
    
    // 并发执行
    let start = Instant::now();
    let (result1, result2, result3) = tokio::join!(
        async_task("任务A", 500),
        async_task("任务B", 300),
        async_task("任务C", 700)
    );
    let elapsed = start.elapsed();
    
    println!("并发执行结果: {}, {}, {}", result1, result2, result3);
    println!("并发执行耗时: {:?}", elapsed);
    
    // 异步循环
    let mut sum = 0;
    for i in 1..=10 {
        sum += async_calculation(i).await;
    }
    println!("异步循环结果: {}", sum);
    
    Ok(())
}

/// 文件系统操作
async fn filesystem_operations() -> Result<()> {
    println!("\n--- 文件系统操作 ---");
    
    use tokio::fs;
    
    // 创建测试目录
    fs::create_dir_all("test_data").await?;
    println!("创建测试目录: test_data");
    
    // 异步写入多个文件
    let mut handles = Vec::new();
    for i in 1..=5 {
        let handle = tokio::spawn(async move {
            let filename = format!("test_data/file_{}.txt", i);
            let content = format!("这是文件 {} 的内容\n时间: {}", 
                                i, chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"));
            
            fs::write(&filename, content).await?;
            println!("写入文件: {}", filename);
            Ok::<(), anyhow::Error>(())
        });
        handles.push(handle);
    }
    
    // 等待所有文件写入完成
    for handle in handles {
        handle.await??;
    }
    
    // 异步读取文件
    let mut entries = fs::read_dir("test_data").await?;
    let mut file_count = 0;
    while let Some(entry) = entries.next_entry().await? {
        if entry.file_type().await?.is_file() {
            file_count += 1;
            let path = entry.path();
            let content = fs::read_to_string(&path).await?;
            println!("读取文件: {} ({} 字节)", path.display(), content.len());
        }
    }
    
    println!("总共处理了 {} 个文件", file_count);
    
    // 清理测试文件
    fs::remove_dir_all("test_data").await?;
    println!("清理测试目录");
    
    Ok(())
}

/// 本地数据处理
async fn local_data_processing() -> Result<()> {
    println!("\n--- 本地数据处理 ---");
    
    // 生成测试数据
    let data = (1..=1000).collect::<Vec<_>>();
    println!("生成测试数据: {} 个数字", data.len());
    
    // 异步数据过滤
    let filtered_data = async_filter_data(data.clone()).await?;
    println!("过滤后数据: {} 个数字", filtered_data.len());
    
    // 异步数据转换
    let transformed_data = async_transform_data(filtered_data).await?;
    println!("转换后数据: {} 个数字", transformed_data.len());
    
    // 异步数据聚合
    let aggregated = async_aggregate_data(transformed_data).await?;
    println!("聚合结果: {:?}", aggregated);
    
    Ok(())
}

/// 模拟网络操作
async fn simulated_network_operations() -> Result<()> {
    println!("\n--- 模拟网络操作 ---");
    
    // 模拟HTTP请求
    let urls = vec![
        "http://localhost:8080/api/users",
        "http://localhost:8080/api/products",
        "http://localhost:8080/api/orders",
    ];
    
    let mut handles = Vec::new();
    for url in urls {
        let handle = tokio::spawn(async move {
            simulate_http_request(url).await
        });
        handles.push(handle);
    }
    
    let mut results = Vec::new();
    for handle in handles {
        if let Ok(result) = handle.await {
            results.push(result);
        }
    }
    
    println!("模拟网络请求结果:");
    for result in results {
        println!("  {}: 状态={}, 耗时={:?}", result.url, result.status, result.duration);
    }
    
    // 模拟数据库操作
    let db_operations = vec![
        "SELECT * FROM users WHERE active = true",
        "INSERT INTO logs (message) VALUES ('Test log')",
        "UPDATE products SET price = price * 1.1",
    ];
    
    for sql in db_operations {
        let result = simulate_database_operation(sql).await?;
        println!("数据库操作: {} -> 影响行数: {}", sql, result);
    }
    
    Ok(())
}

/// 异步资源管理
async fn async_resource_management() -> Result<()> {
    println!("\n--- 异步资源管理 ---");
    
    // 创建资源池
    let resource_pool = Arc::new(Mutex::new(Vec::new()));
    
    // 初始化资源
    for i in 1..=5 {
        let resource = Resource {
            id: i,
            name: format!("资源{}", i),
            created_at: Instant::now(),
        };
        let mut pool = resource_pool.lock().await;
        pool.push(resource);
    }
    
    println!("创建了 5 个资源");
    
    // 异步使用资源
    let mut handles = Vec::new();
    for i in 0..10 {
        let pool = Arc::clone(&resource_pool);
        let handle = tokio::spawn(async move {
            use_resource(pool, i).await
        });
        handles.push(handle);
    }
    
    // 等待所有任务完成
    for handle in handles {
        handle.await?;
    }
    
    // 检查资源状态
    let pool = resource_pool.lock().await;
    println!("剩余资源数量: {}", pool.len());
    
    Ok(())
}

/// 异步函数
async fn async_function(input: i32) -> i32 {
    tokio::time::sleep(Duration::from_millis(100)).await;
    input * 2
}

/// 异步任务
async fn async_task(name: &str, delay_ms: u64) -> String {
    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    format!("{} 完成", name)
}

/// 异步计算
async fn async_calculation(n: i32) -> i32 {
    tokio::time::sleep(Duration::from_millis(50)).await;
    n * n
}

/// 异步数据过滤
async fn async_filter_data(data: Vec<i32>) -> Result<Vec<i32>> {
    let mut handles = Vec::new();
    
    for chunk in data.chunks(100) {
        let chunk = chunk.to_vec();
        let handle = tokio::spawn(async move {
            let mut results = Vec::new();
            for item in chunk {
                tokio::time::sleep(Duration::from_millis(1)).await;
                if item % 2 == 0 {
                    results.push(item);
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
    
    Ok(all_results)
}

/// 异步数据转换
async fn async_transform_data(data: Vec<i32>) -> Result<Vec<i32>> {
    let mut handles = Vec::new();
    
    for chunk in data.chunks(100) {
        let chunk = chunk.to_vec();
        let handle = tokio::spawn(async move {
            let mut results = Vec::new();
            for item in chunk {
                tokio::time::sleep(Duration::from_millis(1)).await;
                results.push(item * item);
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
    
    Ok(all_results)
}

/// 异步数据聚合
async fn async_aggregate_data(data: Vec<i32>) -> Result<DataAggregate> {
    let mut handles = Vec::new();
    
    for chunk in data.chunks(100) {
        let chunk = chunk.to_vec();
        let handle = tokio::spawn(async move {
            let mut sum = 0;
            let mut count = 0;
            let mut min = i32::MAX;
            let mut max = i32::MIN;
            
            for item in chunk {
                tokio::time::sleep(Duration::from_millis(1)).await;
                sum += item;
                count += 1;
                min = min.min(item);
                max = max.max(item);
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
    
    Ok(DataAggregate {
        sum: total_sum,
        count: total_count,
        min: global_min,
        max: global_max,
        average: if total_count > 0 {
            total_sum as f64 / total_count as f64
        } else {
            0.0
        },
    })
}

/// 模拟HTTP请求
async fn simulate_http_request(url: &str) -> HttpResult {
    let delay = Duration::from_millis(100 + (url.len() as u64 * 10));
    tokio::time::sleep(delay).await;
    
    HttpResult {
        url: url.to_string(),
        status: 200,
        duration: delay,
    }
}

/// 模拟数据库操作
async fn simulate_database_operation(sql: &str) -> Result<u64> {
    let delay = Duration::from_millis(50 + (sql.len() as u64 * 2));
    tokio::time::sleep(delay).await;
    
    // 模拟影响的行数
    let affected_rows = if sql.contains("SELECT") {
        0
    } else if sql.contains("INSERT") {
        1
    } else if sql.contains("UPDATE") {
        5
    } else {
        0
    };
    
    Ok(affected_rows)
}

/// 使用资源
async fn use_resource(pool: Arc<Mutex<Vec<Resource>>>, task_id: usize) -> Result<()> {
    // 获取资源
    let resource = {
        let mut pool = pool.lock().await;
        pool.pop()
    };
    
    if let Some(mut resource) = resource {
        println!("任务 {} 使用资源: {}", task_id, resource.name);
        
        // 模拟使用资源
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        // 归还资源
        let mut pool = pool.lock().await;
        pool.push(resource);
    } else {
        println!("任务 {} 没有可用资源", task_id);
    }
    
    Ok(())
}

/// 资源结构
#[derive(Debug)]
struct Resource {
    id: usize,
    name: String,
    created_at: Instant,
}

/// HTTP结果
#[derive(Debug)]
struct HttpResult {
    url: String,
    status: u16,
    duration: Duration,
}

/// 数据聚合结果
#[derive(Debug)]
struct DataAggregate {
    sum: i32,
    count: usize,
    min: i32,
    max: i32,
    average: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_async_function() {
        let result = async_function(21).await;
        assert_eq!(result, 42);
    }
    
    #[tokio::test]
    async fn test_async_task() {
        let result = async_task("测试", 10).await;
        assert_eq!(result, "测试 完成");
    }
    
    #[tokio::test]
    async fn test_async_calculation() {
        let result = async_calculation(5).await;
        assert_eq!(result, 25);
    }
}
