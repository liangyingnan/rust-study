//! 性能测试模块
//! 
//! 包含各种异步性能测试：
//! - 并发性能测试
//! - 内存使用测试
//! - 延迟测试
//! - 吞吐量测试

use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::Instant;

/// 性能测试结果
#[derive(Debug, Clone)]
pub struct PerformanceResult {
    pub operation: String,
    pub total_time: Duration,
    pub operations_count: u64,
    pub operations_per_second: f64,
    pub average_latency: Duration,
    pub min_latency: Duration,
    pub max_latency: Duration,
    pub memory_usage: u64,
}

/// 性能测试器
pub struct PerformanceTester {
    results: Arc<Mutex<Vec<PerformanceResult>>>,
}

impl PerformanceTester {
    /// 创建新的性能测试器
    pub fn new() -> Self {
        Self {
            results: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// 运行并发性能测试
    pub async fn run_concurrency_test(
        &self,
        operation: &str,
        concurrency: usize,
        operations_per_task: u64,
    ) -> Result<PerformanceResult> {
        println!("运行并发性能测试: {} (并发数: {}, 每任务操作数: {})", 
                operation, concurrency, operations_per_task);
        
        let start = Instant::now();
        let mut handles = Vec::new();
        
        // 创建并发任务
        for i in 0..concurrency {
            let handle = tokio::spawn(async move {
                let mut latencies = Vec::new();
                
                for j in 0..operations_per_task {
                    let op_start = Instant::now();
                    
                    // 模拟异步操作
                    tokio::time::sleep(Duration::from_millis(1)).await;
                    
                    let latency = op_start.elapsed();
                    latencies.push(latency);
                }
                
                latencies
            });
            handles.push(handle);
        }
        
        // 等待所有任务完成
        let mut all_latencies = Vec::new();
        for handle in handles {
            if let Ok(latencies) = handle.await {
                all_latencies.extend(latencies);
            }
        }
        
        let total_time = start.elapsed();
        let total_operations = all_latencies.len() as u64;
        
        // 计算统计信息
        let min_latency = all_latencies.iter().min().copied().unwrap_or_default();
        let max_latency = all_latencies.iter().max().copied().unwrap_or_default();
        let total_latency: Duration = all_latencies.iter().sum();
        let average_latency = if !all_latencies.is_empty() {
            total_latency / all_latencies.len() as u32
        } else {
            Duration::from_secs(0)
        };
        
        let operations_per_second = if total_time.as_secs_f64() > 0.0 {
            total_operations as f64 / total_time.as_secs_f64()
        } else {
            0.0
        };
        
        let result = PerformanceResult {
            operation: operation.to_string(),
            total_time,
            operations_count: total_operations,
            operations_per_second,
            average_latency,
            min_latency,
            max_latency,
            memory_usage: self.get_memory_usage(),
        };
        
        // 保存结果
        {
            let mut results = self.results.lock().await;
            results.push(result.clone());
        }
        
        self.print_result(&result);
        Ok(result)
    }
    
    /// 运行延迟测试
    pub async fn run_latency_test(
        &self,
        operation: &str,
        iterations: u64,
    ) -> Result<PerformanceResult> {
        println!("运行延迟测试: {} (迭代次数: {})", operation, iterations);
        
        let start = Instant::now();
        let mut latencies = Vec::new();
        
        for _ in 0..iterations {
            let op_start = Instant::now();
            
            // 模拟异步操作
            tokio::time::sleep(Duration::from_millis(1)).await;
            
            let latency = op_start.elapsed();
            latencies.push(latency);
        }
        
        let total_time = start.elapsed();
        
        // 计算统计信息
        let min_latency = latencies.iter().min().copied().unwrap_or_default();
        let max_latency = latencies.iter().max().copied().unwrap_or_default();
        let total_latency: Duration = latencies.iter().sum();
        let average_latency = if !latencies.is_empty() {
            total_latency / latencies.len() as u32
        } else {
            Duration::from_secs(0)
        };
        
        let operations_per_second = if total_time.as_secs_f64() > 0.0 {
            iterations as f64 / total_time.as_secs_f64()
        } else {
            0.0
        };
        
        let result = PerformanceResult {
            operation: operation.to_string(),
            total_time,
            operations_count: iterations,
            operations_per_second,
            average_latency,
            min_latency,
            max_latency,
            memory_usage: self.get_memory_usage(),
        };
        
        // 保存结果
        {
            let mut results = self.results.lock().await;
            results.push(result.clone());
        }
        
        self.print_result(&result);
        Ok(result)
    }
    
    /// 运行吞吐量测试
    pub async fn run_throughput_test(
        &self,
        operation: &str,
        duration: Duration,
    ) -> Result<PerformanceResult> {
        println!("运行吞吐量测试: {} (持续时间: {:?})", operation, duration);
        
        let start = Instant::now();
        let mut operations_count = 0u64;
        let mut latencies = Vec::new();
        
        while start.elapsed() < duration {
            let op_start = Instant::now();
            
            // 模拟异步操作
            tokio::time::sleep(Duration::from_millis(1)).await;
            
            let latency = op_start.elapsed();
            latencies.push(latency);
            operations_count += 1;
        }
        
        let total_time = start.elapsed();
        
        // 计算统计信息
        let min_latency = latencies.iter().min().copied().unwrap_or_default();
        let max_latency = latencies.iter().max().copied().unwrap_or_default();
        let total_latency: Duration = latencies.iter().sum();
        let average_latency = if !latencies.is_empty() {
            total_latency / latencies.len() as u32
        } else {
            Duration::from_secs(0)
        };
        
        let operations_per_second = if total_time.as_secs_f64() > 0.0 {
            operations_count as f64 / total_time.as_secs_f64()
        } else {
            0.0
        };
        
        let result = PerformanceResult {
            operation: operation.to_string(),
            total_time,
            operations_count,
            operations_per_second,
            average_latency,
            min_latency,
            max_latency,
            memory_usage: self.get_memory_usage(),
        };
        
        // 保存结果
        {
            let mut results = self.results.lock().await;
            results.push(result.clone());
        }
        
        self.print_result(&result);
        Ok(result)
    }
    
    /// 运行内存使用测试
    pub async fn run_memory_test(
        &self,
        operation: &str,
        iterations: u64,
    ) -> Result<PerformanceResult> {
        println!("运行内存使用测试: {} (迭代次数: {})", operation, iterations);
        
        let start = Instant::now();
        let mut data = Vec::new();
        
        for i in 0..iterations {
            // 分配内存
            let chunk = vec![0u8; 1024]; // 1KB
            data.push(chunk);
            
            // 模拟异步操作
            tokio::time::sleep(Duration::from_millis(1)).await;
            
            if i % 100 == 0 {
                println!("已分配 {} KB 内存", (i + 1) * 1024 / 1024);
            }
        }
        
        let total_time = start.elapsed();
        let memory_usage = self.get_memory_usage();
        
        let result = PerformanceResult {
            operation: operation.to_string(),
            total_time,
            operations_count: iterations,
            operations_per_second: iterations as f64 / total_time.as_secs_f64(),
            average_latency: Duration::from_millis(1),
            min_latency: Duration::from_millis(1),
            max_latency: Duration::from_millis(1),
            memory_usage,
        };
        
        // 清理内存
        drop(data);
        
        // 保存结果
        {
            let mut results = self.results.lock().await;
            results.push(result.clone());
        }
        
        self.print_result(&result);
        Ok(result)
    }
    
    /// 获取内存使用量（简化版本）
    fn get_memory_usage(&self) -> u64 {
        // 这里应该使用更精确的内存测量方法
        // 为了示例，我们返回一个模拟值
        std::process::id() as u64 * 1024
    }
    
    /// 打印测试结果
    fn print_result(&self, result: &PerformanceResult) {
        println!("测试结果: {}", result.operation);
        println!("  总时间: {:?}", result.total_time);
        println!("  操作次数: {}", result.operations_count);
        println!("  每秒操作数: {:.2}", result.operations_per_second);
        println!("  平均延迟: {:?}", result.average_latency);
        println!("  最小延迟: {:?}", result.min_latency);
        println!("  最大延迟: {:?}", result.max_latency);
        println!("  内存使用: {} KB", result.memory_usage / 1024);
        println!();
    }
    
    /// 获取所有测试结果
    pub async fn get_all_results(&self) -> Vec<PerformanceResult> {
        let results = self.results.lock().await;
        results.clone()
    }
    
    /// 打印性能报告
    pub async fn print_performance_report(&self) {
        let results = self.get_all_results().await;
        
        if results.is_empty() {
            println!("没有性能测试结果");
            return;
        }
        
        println!("\n=== 性能测试报告 ===");
        
        for result in &results {
            self.print_result(result);
        }
        
        // 计算总体统计
        let total_operations: u64 = results.iter().map(|r| r.operations_count).sum();
        let total_time: Duration = results.iter().map(|r| r.total_time).sum();
        let avg_ops_per_second: f64 = results.iter().map(|r| r.operations_per_second).sum::<f64>() / results.len() as f64;
        
        println!("总体统计:");
        println!("  总操作数: {}", total_operations);
        println!("  总时间: {:?}", total_time);
        println!("  平均每秒操作数: {:.2}", avg_ops_per_second);
    }
}

/// 性能测试示例
pub async fn performance_test_example() -> Result<()> {
    println!("\n=== 性能测试示例 ===");
    
    let tester = PerformanceTester::new();
    
    // 并发性能测试
    tester.run_concurrency_test("并发操作", 10, 100).await?;
    
    // 延迟测试
    tester.run_latency_test("延迟测试", 1000).await?;
    
    // 吞吐量测试
    tester.run_throughput_test("吞吐量测试", Duration::from_secs(2)).await?;
    
    // 内存使用测试
    tester.run_memory_test("内存测试", 1000).await?;
    
    // 打印性能报告
    tester.print_performance_report().await;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_performance_tester_creation() {
        let tester = PerformanceTester::new();
        let results = tester.get_all_results().await;
        assert!(results.is_empty());
    }
    
    #[tokio::test]
    async fn test_concurrency_test() {
        let tester = PerformanceTester::new();
        let result = tester.run_concurrency_test("测试", 2, 10).await;
        assert!(result.is_ok());
        
        let results = tester.get_all_results().await;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].operation, "测试");
    }
    
    #[tokio::test]
    async fn test_latency_test() {
        let tester = PerformanceTester::new();
        let result = tester.run_latency_test("延迟测试", 10).await;
        assert!(result.is_ok());
        
        let results = tester.get_all_results().await;
        assert_eq!(results.len(), 1);
        assert!(results[0].operations_count > 0);
    }
}
