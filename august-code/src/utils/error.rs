//! 错误处理工具模块
//! 
//! 提供统一的错误处理功能：
//! - 自定义错误类型
//! - 错误转换
//! - 错误恢复
//! - 错误日志

use anyhow::Result;
use std::fmt;
use std::time::Duration;
use tokio::time::timeout;

/// 应用错误类型
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("网络错误: {0}")]
    Network(String),
    
    #[error("数据库错误: {0}")]
    Database(String),
    
    #[error("文件系统错误: {0}")]
    FileSystem(String),
    
    #[error("超时错误: {0}")]
    Timeout(String),
    
    #[error("配置错误: {0}")]
    Config(String),
    
    #[error("业务逻辑错误: {0}")]
    Business(String),
    
    #[error("未知错误: {0}")]
    Unknown(String),
}

/// 错误恢复策略
#[derive(Debug, Clone)]
pub enum RetryStrategy {
    /// 固定间隔重试
    Fixed(Duration),
    /// 指数退避重试
    Exponential(Duration, f64),
    /// 线性退避重试
    Linear(Duration, Duration),
}

/// 重试配置
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub strategy: RetryStrategy,
    pub timeout: Option<Duration>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            strategy: RetryStrategy::Fixed(Duration::from_millis(100)),
            timeout: Some(Duration::from_secs(30)),
        }
    }
}

/// 错误处理工具
pub struct ErrorHandler;

impl ErrorHandler {
    /// 带重试的异步操作
    pub async fn with_retry<F, Fut, T>(
        operation: F,
        config: RetryConfig,
    ) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let mut last_error = None;
        
        for attempt in 1..=config.max_attempts {
            let result = if let Some(timeout_duration) = config.timeout {
                timeout(timeout_duration, operation()).await?
            } else {
                operation().await
            };
            
            match result {
                Ok(value) => return Ok(value),
                Err(e) => {
                    last_error = Some(e);
                    
                    if attempt < config.max_attempts {
                        let delay = Self::calculate_delay(&config.strategy, attempt);
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap())
    }
    
    /// 计算重试延迟
    fn calculate_delay(strategy: &RetryStrategy, attempt: u32) -> Duration {
        match strategy {
            RetryStrategy::Fixed(delay) => *delay,
            RetryStrategy::Exponential(base_delay, multiplier) => {
                let delay_ms = base_delay.as_millis() as f64 * multiplier.powi(attempt as i32 - 1);
                Duration::from_millis(delay_ms as u64)
            }
            RetryStrategy::Linear(base_delay, increment) => {
                *base_delay + *increment * (attempt - 1)
            }
        }
    }
    
    /// 错误分类
    pub fn categorize_error(error: &anyhow::Error) -> AppError {
        let error_str = error.to_string().to_lowercase();
        
        if error_str.contains("network") || error_str.contains("connection") {
            AppError::Network(error.to_string())
        } else if error_str.contains("database") || error_str.contains("sql") {
            AppError::Database(error.to_string())
        } else if error_str.contains("file") || error_str.contains("io") {
            AppError::FileSystem(error.to_string())
        } else if error_str.contains("timeout") || error_str.contains("timed out") {
            AppError::Timeout(error.to_string())
        } else if error_str.contains("config") || error_str.contains("configuration") {
            AppError::Config(error.to_string())
        } else {
            AppError::Unknown(error.to_string())
        }
    }
    
    /// 错误恢复
    pub async fn recover_from_error<F, Fut, T>(
        error: &AppError,
        recovery_fn: F,
    ) -> Result<T>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        match error {
            AppError::Network(_) | AppError::Timeout(_) => {
                // 网络错误可以尝试恢复
                recovery_fn().await
            }
            AppError::Database(_) => {
                // 数据库错误可能需要重连
                recovery_fn().await
            }
            _ => {
                // 其他错误通常无法恢复
                Err(anyhow::anyhow!("无法恢复的错误: {}", error))
            }
        }
    }
}

/// 错误日志记录器
pub struct ErrorLogger;

impl ErrorLogger {
    /// 记录错误
    pub fn log_error(error: &anyhow::Error, context: &str) {
        eprintln!("[ERROR] {} - {}", context, error);
        
        // 这里可以添加更复杂的日志记录逻辑
        // 比如写入文件、发送到日志服务等
    }
    
    /// 记录警告
    pub fn log_warning(message: &str, context: &str) {
        println!("[WARNING] {} - {}", context, message);
    }
    
    /// 记录信息
    pub fn log_info(message: &str, context: &str) {
        println!("[INFO] {} - {}", context, message);
    }
}

/// 错误统计
#[derive(Debug, Default)]
pub struct ErrorStats {
    pub total_errors: u64,
    pub network_errors: u64,
    pub database_errors: u64,
    pub file_system_errors: u64,
    pub timeout_errors: u64,
    pub config_errors: u64,
    pub business_errors: u64,
    pub unknown_errors: u64,
}

impl ErrorStats {
    /// 记录错误
    pub fn record_error(&mut self, error: &AppError) {
        self.total_errors += 1;
        
        match error {
            AppError::Network(_) => self.network_errors += 1,
            AppError::Database(_) => self.database_errors += 1,
            AppError::FileSystem(_) => self.file_system_errors += 1,
            AppError::Timeout(_) => self.timeout_errors += 1,
            AppError::Config(_) => self.config_errors += 1,
            AppError::Business(_) => self.business_errors += 1,
            AppError::Unknown(_) => self.unknown_errors += 1,
        }
    }
    
    /// 获取错误率
    pub fn get_error_rate(&self, total_operations: u64) -> f64 {
        if total_operations == 0 {
            0.0
        } else {
            self.total_errors as f64 / total_operations as f64
        }
    }
    
    /// 打印统计信息
    pub fn print_stats(&self) {
        println!("错误统计:");
        println!("  总错误数: {}", self.total_errors);
        println!("  网络错误: {}", self.network_errors);
        println!("  数据库错误: {}", self.database_errors);
        println!("  文件系统错误: {}", self.file_system_errors);
        println!("  超时错误: {}", self.timeout_errors);
        println!("  配置错误: {}", self.config_errors);
        println!("  业务错误: {}", self.business_errors);
        println!("  未知错误: {}", self.unknown_errors);
    }
}

/// 错误处理示例
pub async fn error_handling_example() -> Result<()> {
    println!("\n=== 错误处理示例 ===");
    
    // 创建错误统计
    let mut stats = ErrorStats::default();
    
    // 模拟各种错误
    let errors = vec![
        AppError::Network("连接超时".to_string()),
        AppError::Database("查询失败".to_string()),
        AppError::FileSystem("文件不存在".to_string()),
        AppError::Timeout("操作超时".to_string()),
        AppError::Business("业务规则验证失败".to_string()),
    ];
    
    for error in errors {
        stats.record_error(&error);
        ErrorLogger::log_error(&anyhow::anyhow!(error), "错误处理示例");
    }
    
    // 打印统计信息
    stats.print_stats();
    
    // 测试重试机制
    println!("\n测试重试机制:");
    let config = RetryConfig {
        max_attempts: 3,
        strategy: RetryStrategy::Exponential(Duration::from_millis(100), 2.0),
        timeout: Some(Duration::from_secs(1)),
    };
    
    let result = ErrorHandler::with_retry(
        || async {
            // 模拟可能失败的操作
            tokio::time::sleep(Duration::from_millis(50)).await;
            if rand::random::<f64>() < 0.7 {
                Err(anyhow::anyhow!("模拟失败"))
            } else {
                Ok("操作成功")
            }
        },
        config,
    ).await;
    
    match result {
        Ok(value) => println!("重试成功: {}", value),
        Err(e) => println!("重试失败: {}", e),
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_error_categorization() {
        let error = anyhow::anyhow!("Network connection failed");
        let app_error = ErrorHandler::categorize_error(&error);
        
        match app_error {
            AppError::Network(_) => assert!(true),
            _ => assert!(false, "应该被分类为网络错误"),
        }
    }
    
    #[tokio::test]
    async fn test_retry_config_default() {
        let config = RetryConfig::default();
        assert_eq!(config.max_attempts, 3);
        assert!(config.timeout.is_some());
    }
    
    #[tokio::test]
    async fn test_error_stats() {
        let mut stats = ErrorStats::default();
        let error = AppError::Network("test".to_string());
        
        stats.record_error(&error);
        assert_eq!(stats.total_errors, 1);
        assert_eq!(stats.network_errors, 1);
    }
}
