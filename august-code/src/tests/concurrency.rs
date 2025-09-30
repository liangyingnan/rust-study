//! 并发测试模块
//! 
//! 包含各种并发测试：
//! - 并发安全性测试
//! - 死锁检测测试
//! - 竞态条件测试
//! - 并发性能测试

use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock, Semaphore};
use tokio::time::{sleep, timeout};

/// 并发测试器
pub struct ConcurrencyTester;

impl ConcurrencyTester {
    /// 测试互斥锁并发安全性
    pub async fn test_mutex_safety() -> Result<()> {
        println!("\n--- 测试互斥锁并发安全性 ---");
        
        let counter = Arc::new(Mutex::new(0));
        let mut handles = Vec::new();
        
        // 创建多个并发任务
        for i in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = tokio::spawn(async move {
                for _ in 0..100 {
                    let mut count = counter.lock().await;
                    *count += 1;
                    // 模拟一些工作
                    sleep(Duration::from_millis(1)).await;
                }
                println!("任务 {} 完成", i);
            });
            handles.push(handle);
        }
        
        // 等待所有任务完成
        for handle in handles {
            handle.await?;
        }
        
        let final_count = *counter.lock().await;
        println!("最终计数: {} (期望: 1000)", final_count);
        
        assert_eq!(final_count, 1000);
        Ok(())
    }
    
    /// 测试读写锁并发安全性
    pub async fn test_rwlock_safety() -> Result<()> {
        println!("\n--- 测试读写锁并发安全性 ---");
        
        let data = Arc::new(RwLock::new(Vec::new()));
        let mut handles = Vec::new();
        
        // 创建写入任务
        for i in 0..5 {
            let data = Arc::clone(&data);
            let handle = tokio::spawn(async move {
                for j in 0..20 {
                    let mut writer = data.write().await;
                    writer.push(format!("写入任务{}: 数据{}", i, j));
                    sleep(Duration::from_millis(1)).await;
                }
                println!("写入任务 {} 完成", i);
            });
            handles.push(handle);
        }
        
        // 创建读取任务
        for i in 0..10 {
            let data = Arc::clone(&data);
            let handle = tokio::spawn(async move {
                for _ in 0..10 {
                    let reader = data.read().await;
                    let count = reader.len();
                    sleep(Duration::from_millis(1)).await;
                    if count % 50 == 0 {
                        println!("读取任务{}: 当前数据量 {}", i, count);
                    }
                }
                println!("读取任务 {} 完成", i);
            });
            handles.push(handle);
        }
        
        // 等待所有任务完成
        for handle in handles {
            handle.await?;
        }
        
        let final_data = data.read().await;
        println!("最终数据量: {}", final_data.len());
        
        Ok(())
    }
    
    /// 测试信号量并发控制
    pub async fn test_semaphore_control() -> Result<()> {
        println!("\n--- 测试信号量并发控制 ---");
        
        let semaphore = Arc::new(Semaphore::new(3)); // 最多3个并发
        let mut handles = Vec::new();
        
        // 创建多个任务
        for i in 0..10 {
            let semaphore = Arc::clone(&semaphore);
            let handle = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                println!("任务 {} 获得许可", i);
                
                // 模拟工作
                sleep(Duration::from_millis(100)).await;
                
                println!("任务 {} 释放许可", i);
            });
            handles.push(handle);
        }
        
        // 等待所有任务完成
        for handle in handles {
            handle.await?;
        }
        
        println!("所有任务完成");
        Ok(())
    }
    
    /// 测试死锁检测
    pub async fn test_deadlock_detection() -> Result<()> {
        println!("\n--- 测试死锁检测 ---");
        
        let resource1 = Arc::new(Mutex::new(0));
        let resource2 = Arc::new(Mutex::new(0));
        let mut handles = Vec::new();
        
        // 创建可能导致死锁的任务
        for i in 0..5 {
            let r1 = Arc::clone(&resource1);
            let r2 = Arc::clone(&resource2);
            let handle = tokio::spawn(async move {
                if i % 2 == 0 {
                    // 先获取r1，再获取r2
                    let _lock1 = r1.lock().await;
                    sleep(Duration::from_millis(10)).await;
                    let _lock2 = r2.lock().await;
                    println!("任务 {} 按顺序获取锁", i);
                } else {
                    // 先获取r2，再获取r1
                    let _lock2 = r2.lock().await;
                    sleep(Duration::from_millis(10)).await;
                    let _lock1 = r1.lock().await;
                    println!("任务 {} 按顺序获取锁", i);
                }
            });
            handles.push(handle);
        }
        
        // 设置超时以防止死锁
        let result = timeout(Duration::from_secs(5), async {
            for handle in handles {
                handle.await?;
            }
            Ok::<(), anyhow::Error>(())
        }).await;
        
        match result {
            Ok(Ok(())) => println!("所有任务完成，没有死锁"),
            Ok(Err(e)) => println!("任务执行错误: {}", e),
            Err(_) => println!("检测到可能的死锁或超时"),
        }
        
        Ok(())
    }
    
    /// 测试竞态条件
    pub async fn test_race_condition() -> Result<()> {
        println!("\n--- 测试竞态条件 ---");
        
        let counter = Arc::new(Mutex::new(0));
        let mut handles = Vec::new();
        
        // 创建多个并发任务，模拟竞态条件
        for i in 0..20 {
            let counter = Arc::clone(&counter);
            let handle = tokio::spawn(async move {
                // 读取当前值
                let current = {
                    let count = counter.lock().await;
                    *count
                };
                
                // 模拟一些处理时间
                sleep(Duration::from_millis(1)).await;
                
                // 写入新值
                {
                    let mut count = counter.lock().await;
                    *count = current + 1;
                }
                
                println!("任务 {} 完成", i);
            });
            handles.push(handle);
        }
        
        // 等待所有任务完成
        for handle in handles {
            handle.await?;
        }
        
        let final_count = *counter.lock().await;
        println!("最终计数: {} (期望: 20)", final_count);
        
        // 注意：由于竞态条件，实际结果可能小于期望值
        if final_count < 20 {
            println!("检测到竞态条件！实际值小于期望值");
        }
        
        Ok(())
    }
    
    /// 测试原子操作
    pub async fn test_atomic_operations() -> Result<()> {
        println!("\n--- 测试原子操作 ---");
        
        use std::sync::atomic::{AtomicU64, Ordering};
        
        let counter = Arc::new(AtomicU64::new(0));
        let mut handles = Vec::new();
        
        // 创建多个并发任务
        for i in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = tokio::spawn(async move {
                for _ in 0..100 {
                    counter.fetch_add(1, Ordering::SeqCst);
                    sleep(Duration::from_millis(1)).await;
                }
                println!("原子操作任务 {} 完成", i);
            });
            handles.push(handle);
        }
        
        // 等待所有任务完成
        for handle in handles {
            handle.await?;
        }
        
        let final_count = counter.load(Ordering::SeqCst);
        println!("最终计数: {} (期望: 1000)", final_count);
        
        assert_eq!(final_count, 1000);
        Ok(())
    }
    
    /// 测试并发性能
    pub async fn test_concurrency_performance() -> Result<()> {
        println!("\n--- 测试并发性能 ---");
        
        let start = std::time::Instant::now();
        
        // 测试顺序执行
        let sequential_start = std::time::Instant::now();
        for i in 0..100 {
            sleep(Duration::from_millis(1)).await;
            let _ = i * i;
        }
        let sequential_time = sequential_start.elapsed();
        
        // 测试并发执行
        let concurrent_start = std::time::Instant::now();
        let mut handles = Vec::new();
        
        for i in 0..100 {
            let handle = tokio::spawn(async move {
                sleep(Duration::from_millis(1)).await;
                i * i
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.await?;
        }
        let concurrent_time = concurrent_start.elapsed();
        
        let total_time = start.elapsed();
        
        println!("性能对比:");
        println!("  顺序执行时间: {:?}", sequential_time);
        println!("  并发执行时间: {:?}", concurrent_time);
        println!("  总测试时间: {:?}", total_time);
        
        let speedup = sequential_time.as_secs_f64() / concurrent_time.as_secs_f64();
        println!("  加速比: {:.2}x", speedup);
        
        Ok(())
    }
}

/// 并发测试示例
pub async fn concurrency_test_example() -> Result<()> {
    println!("\n=== 并发测试示例 ===");
    
    let tester = ConcurrencyTester;
    
    // 运行各种并发测试
    ConcurrencyTester::test_mutex_safety().await?;
    ConcurrencyTester::test_rwlock_safety().await?;
    ConcurrencyTester::test_semaphore_control().await?;
    ConcurrencyTester::test_deadlock_detection().await?;
    ConcurrencyTester::test_race_condition().await?;
    ConcurrencyTester::test_atomic_operations().await?;
    ConcurrencyTester::test_concurrency_performance().await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mutex_safety() {
        let result = ConcurrencyTester::test_mutex_safety().await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_rwlock_safety() {
        let result = ConcurrencyTester::test_rwlock_safety().await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_semaphore_control() {
        let result = ConcurrencyTester::test_semaphore_control().await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_atomic_operations() {
        let result = ConcurrencyTester::test_atomic_operations().await;
        assert!(result.is_ok());
    }
}
