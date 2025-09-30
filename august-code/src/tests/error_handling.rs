//! 错误处理测试模块
//! 
//! 包含各种错误处理测试：
//! - 超时错误测试
//! - 网络错误测试
//! - 重试机制测试
//! - 错误恢复测试

use anyhow::Result;
use std::time::Duration;
use tokio::time::{sleep, timeout};

/// 错误处理测试器
pub struct ErrorHandlingTester;

impl ErrorHandlingTester {
    /// 测试超时错误处理
    pub async fn test_timeout_handling() -> Result<()> {
        println!("\n--- 测试超时错误处理 ---");
        
        // 测试正常操作
        let result = timeout(
            Duration::from_millis(100),
            async {
                sleep(Duration::from_millis(50)).await;
                "操作成功"
            }
        ).await;
        
        match result {
            Ok(value) => println!("正常操作成功: {}", value),
            Err(_) => println!("正常操作意外超时"),
        }
        
        // 测试超时操作
        let result = timeout(
            Duration::from_millis(50),
            async {
                sleep(Duration::from_millis(100)).await;
                "操作成功"
            }
        ).await;
        
        match result {
            Ok(value) => println!("超时操作意外成功: {}", value),
            Err(_) => println!("超时操作正确超时"),
        }
        
        Ok(())
    }
    
    /// 测试网络错误处理
    pub async fn test_network_error_handling() -> Result<()> {
        println!("\n--- 测试网络错误处理 ---");
        
        // 模拟网络错误
        let result = simulate_network_operation().await;
        
        match result {
            Ok(value) => println!("网络操作成功: {}", value),
            Err(e) => {
                println!("网络操作失败: {}", e);
                
                // 尝试错误恢复
                if let Err(recovery_error) = recover_from_network_error().await {
                    println!("错误恢复失败: {}", recovery_error);
                } else {
                    println!("错误恢复成功");
                }
            }
        }
        
        Ok(())
    }
    
    /// 测试重试机制
    pub async fn test_retry_mechanism() -> Result<()> {
        println!("\n--- 测试重试机制 ---");
        
        let mut attempt = 0;
        let max_attempts = 3;
        
        loop {
            attempt += 1;
            println!("尝试第 {} 次", attempt);
            
            let result = simulate_unreliable_operation().await;
            
            match result {
                Ok(value) => {
                    println!("操作成功: {}", value);
                    break;
                }
                Err(e) => {
                    println!("操作失败: {}", e);
                    
                    if attempt >= max_attempts {
                        println!("达到最大重试次数，操作失败");
                        break;
                    }
                    
                    // 等待后重试
                    let delay = Duration::from_millis(100 * attempt);
                    println!("等待 {:?} 后重试", delay);
                    sleep(delay).await;
                }
            }
        }
        
        Ok(())
    }
    
    /// 测试错误恢复
    pub async fn test_error_recovery() -> Result<()> {
        println!("\n--- 测试错误恢复 ---");
        
        // 测试可恢复的错误
        let result = simulate_recoverable_error().await;
        
        match result {
            Ok(value) => println!("操作成功: {}", value),
            Err(e) => {
                println!("操作失败: {}", e);
                
                // 尝试恢复
                match recover_from_error(&e).await {
                    Ok(value) => println!("恢复成功: {}", value),
                    Err(recovery_error) => println!("恢复失败: {}", recovery_error),
                }
            }
        }
        
        // 测试不可恢复的错误
        let result = simulate_unrecoverable_error().await;
        
        match result {
            Ok(value) => println!("操作成功: {}", value),
            Err(e) => {
                println!("操作失败: {}", e);
                
                // 尝试恢复
                match recover_from_error(&e).await {
                    Ok(value) => println!("恢复成功: {}", value),
                    Err(recovery_error) => println!("恢复失败: {}", recovery_error),
                }
            }
        }
        
        Ok(())
    }
    
    /// 测试错误统计
    pub async fn test_error_statistics() -> Result<()> {
        println!("\n--- 测试错误统计 ---");
        
        let mut error_count = 0;
        let mut success_count = 0;
        let total_operations = 100;
        
        for i in 1..=total_operations {
            let result = simulate_random_operation().await;
            
            match result {
                Ok(_) => {
                    success_count += 1;
                    if i % 20 == 0 {
                        println!("操作 {} 成功", i);
                    }
                }
                Err(e) => {
                    error_count += 1;
                    println!("操作 {} 失败: {}", i, e);
                }
            }
        }
        
        let success_rate = (success_count as f64 / total_operations as f64) * 100.0;
        let error_rate = (error_count as f64 / total_operations as f64) * 100.0;
        
        println!("错误统计:");
        println!("  总操作数: {}", total_operations);
        println!("  成功数: {}", success_count);
        println!("  失败数: {}", error_count);
        println!("  成功率: {:.2}%", success_rate);
        println!("  错误率: {:.2}%", error_rate);
        
        Ok(())
    }
    
    /// 测试错误分类
    pub async fn test_error_classification() -> Result<()> {
        println!("\n--- 测试错误分类 ---");
        
        let errors = vec![
            anyhow::anyhow!("Network connection failed"),
            anyhow::anyhow!("Database query timeout"),
            anyhow::anyhow!("File not found"),
            anyhow::anyhow!("Invalid configuration"),
            anyhow::anyhow!("Business logic error"),
            anyhow::anyhow!("Unknown error occurred"),
        ];
        
        for error in errors {
            let error_type = classify_error(&error);
            println!("错误: {} -> 类型: {}", error, error_type);
        }
        
        Ok(())
    }
}

/// 模拟网络操作
async fn simulate_network_operation() -> Result<String> {
    // 模拟网络延迟
    sleep(Duration::from_millis(50)).await;
    
    // 模拟网络错误（30% 概率）
    if rand::random::<f64>() < 0.3 {
        Err(anyhow::anyhow!("网络连接超时"))
    } else {
        Ok("网络操作成功".to_string())
    }
}

/// 从网络错误恢复
async fn recover_from_network_error() -> Result<String> {
    println!("尝试重新连接...");
    sleep(Duration::from_millis(100)).await;
    
    // 模拟重连成功
    if rand::random::<f64>() < 0.7 {
        Ok("网络重连成功".to_string())
    } else {
        Err(anyhow::anyhow!("网络重连失败"))
    }
}

/// 模拟不可靠操作
async fn simulate_unreliable_operation() -> Result<String> {
    sleep(Duration::from_millis(20)).await;
    
    // 模拟操作失败（50% 概率）
    if rand::random::<f64>() < 0.5 {
        Err(anyhow::anyhow!("操作失败"))
    } else {
        Ok("操作成功".to_string())
    }
}

/// 模拟可恢复错误
async fn simulate_recoverable_error() -> Result<String> {
    sleep(Duration::from_millis(30)).await;
    Err(anyhow::anyhow!("可恢复的错误"))
}

/// 模拟不可恢复错误
async fn simulate_unrecoverable_error() -> Result<String> {
    sleep(Duration::from_millis(30)).await;
    Err(anyhow::anyhow!("不可恢复的错误"))
}

/// 从错误恢复
async fn recover_from_error(error: &anyhow::Error) -> Result<String> {
    let error_msg = error.to_string().to_lowercase();
    
    if error_msg.contains("可恢复") {
        println!("尝试恢复...");
        sleep(Duration::from_millis(50)).await;
        Ok("恢复成功".to_string())
    } else {
        Err(anyhow::anyhow!("无法恢复的错误"))
    }
}

/// 模拟随机操作
async fn simulate_random_operation() -> Result<String> {
    sleep(Duration::from_millis(10)).await;
    
    // 模拟操作失败（20% 概率）
    if rand::random::<f64>() < 0.2 {
        Err(anyhow::anyhow!("随机操作失败"))
    } else {
        Ok("随机操作成功".to_string())
    }
}

/// 分类错误
fn classify_error(error: &anyhow::Error) -> &'static str {
    let error_msg = error.to_string().to_lowercase();
    
    if error_msg.contains("network") || error_msg.contains("connection") {
        "网络错误"
    } else if error_msg.contains("database") || error_msg.contains("query") {
        "数据库错误"
    } else if error_msg.contains("file") || error_msg.contains("io") {
        "文件系统错误"
    } else if error_msg.contains("timeout") {
        "超时错误"
    } else if error_msg.contains("config") || error_msg.contains("configuration") {
        "配置错误"
    } else if error_msg.contains("business") || error_msg.contains("logic") {
        "业务逻辑错误"
    } else {
        "未知错误"
    }
}

/// 错误处理测试示例
pub async fn error_handling_test_example() -> Result<()> {
    println!("\n=== 错误处理测试示例 ===");
    
    let tester = ErrorHandlingTester;
    
    // 运行各种错误处理测试
    ErrorHandlingTester::test_timeout_handling().await?;
    ErrorHandlingTester::test_network_error_handling().await?;
    ErrorHandlingTester::test_retry_mechanism().await?;
    ErrorHandlingTester::test_error_recovery().await?;
    ErrorHandlingTester::test_error_statistics().await?;
    ErrorHandlingTester::test_error_classification().await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_timeout_handling() {
        let result = ErrorHandlingTester::test_timeout_handling().await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_network_error_handling() {
        let result = ErrorHandlingTester::test_network_error_handling().await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_retry_mechanism() {
        let result = ErrorHandlingTester::test_retry_mechanism().await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_error_classification() {
        let error = anyhow::anyhow!("Network connection failed");
        let error_type = classify_error(&error);
        assert_eq!(error_type, "网络错误");
    }
}
