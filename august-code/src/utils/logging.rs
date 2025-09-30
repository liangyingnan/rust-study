//! 日志工具模块
//! 
//! 提供日志记录功能：
//! - 结构化日志
//! - 日志级别控制
//! - 日志轮转
//! - 异步日志记录

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// 日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub target: String,
    pub message: String,
    pub fields: std::collections::HashMap<String, String>,
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: LogLevel,
    pub format: LogFormat,
    pub output: LogOutput,
    pub max_file_size: u64,
    pub max_files: u32,
    pub buffer_size: usize,
}

/// 日志格式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    Json,
    Text,
    Compact,
}

/// 日志输出
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogOutput {
    Console,
    File(String),
    Both(String),
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            format: LogFormat::Text,
            output: LogOutput::Console,
            max_file_size: 10 * 1024 * 1024, // 10MB
            max_files: 5,
            buffer_size: 1000,
        }
    }
}

/// 异步日志记录器
pub struct AsyncLogger {
    config: LogConfig,
    buffer: Arc<RwLock<Vec<LogEntry>>>,
    last_flush: Arc<RwLock<Instant>>,
}

impl AsyncLogger {
    /// 创建新的日志记录器
    pub fn new(config: LogConfig) -> Self {
        Self {
            config,
            buffer: Arc::new(RwLock::new(Vec::new())),
            last_flush: Arc::new(RwLock::new(Instant::now())),
        }
    }
    
    /// 记录日志
    pub async fn log(&self, level: LogLevel, target: &str, message: &str) {
        if level < self.config.level {
            return;
        }
        
        let entry = LogEntry {
            timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
            level,
            target: target.to_string(),
            message: message.to_string(),
            fields: std::collections::HashMap::new(),
        };
        
        // 添加到缓冲区
        {
            let mut buffer = self.buffer.write().await;
            buffer.push(entry);
            
            // 检查是否需要刷新
            if buffer.len() >= self.config.buffer_size {
                drop(buffer);
                self.flush().await;
            }
        }
        
        // 检查是否需要定期刷新
        let should_flush = {
            let last_flush = self.last_flush.read().await;
            last_flush.elapsed() > Duration::from_secs(1)
        };
        
        if should_flush {
            self.flush().await;
        }
    }
    
    /// 记录带字段的日志
    pub async fn log_with_fields(
        &self,
        level: LogLevel,
        target: &str,
        message: &str,
        fields: std::collections::HashMap<String, String>,
    ) {
        if level < self.config.level {
            return;
        }
        
        let entry = LogEntry {
            timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
            level,
            target: target.to_string(),
            message: message.to_string(),
            fields,
        };
        
        {
            let mut buffer = self.buffer.write().await;
            buffer.push(entry);
            
            if buffer.len() >= self.config.buffer_size {
                drop(buffer);
                self.flush().await;
            }
        }
    }
    
    /// 刷新日志缓冲区
    pub async fn flush(&self) {
        let entries = {
            let mut buffer = self.buffer.write().await;
            let entries = buffer.clone();
            buffer.clear();
            entries
        };
        
        if entries.is_empty() {
            return;
        }
        
        // 更新最后刷新时间
        {
            let mut last_flush = self.last_flush.write().await;
            *last_flush = Instant::now();
        }
        
        // 输出日志
        for entry in entries {
            self.output_log(&entry).await;
        }
    }
    
    /// 输出日志
    async fn output_log(&self, entry: &LogEntry) {
        let formatted = match self.config.format {
            LogFormat::Json => serde_json::to_string(entry).unwrap_or_else(|_| "{}".to_string()),
            LogFormat::Text => self.format_text(entry),
            LogFormat::Compact => self.format_compact(entry),
        };
        
        match &self.config.output {
            LogOutput::Console => {
                println!("{}", formatted);
            }
            LogOutput::File(path) => {
                if let Err(e) = self.write_to_file(path, &formatted).await {
                    eprintln!("写入日志文件失败: {}", e);
                }
            }
            LogOutput::Both(path) => {
                println!("{}", formatted);
                if let Err(e) = self.write_to_file(path, &formatted).await {
                    eprintln!("写入日志文件失败: {}", e);
                }
            }
        }
    }
    
    /// 格式化文本日志
    fn format_text(&self, entry: &LogEntry) -> String {
        let mut formatted = format!(
            "[{}] {} {}: {}",
            entry.timestamp, entry.level, entry.target, entry.message
        );
        
        if !entry.fields.is_empty() {
            let fields_str = entry.fields
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join(" ");
            formatted.push_str(&format!(" {}", fields_str));
        }
        
        formatted
    }
    
    /// 格式化紧凑日志
    fn format_compact(&self, entry: &LogEntry) -> String {
        format!(
            "{} {} {}: {}",
            entry.timestamp, entry.level, entry.target, entry.message
        )
    }
    
    /// 写入文件
    async fn write_to_file(&self, path: &str, content: &str) -> Result<()> {
        use tokio::fs::OpenOptions;
        use tokio::io::AsyncWriteExt;
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .await?;
        
        file.write_all(format!("{}\n", content).as_bytes()).await?;
        file.flush().await?;
        
        Ok(())
    }
}

/// 日志宏
#[macro_export]
macro_rules! log_trace {
    ($logger:expr, $target:expr, $($arg:tt)*) => {
        $logger.log(LogLevel::Trace, $target, &format!($($arg)*)).await;
    };
}

#[macro_export]
macro_rules! log_debug {
    ($logger:expr, $target:expr, $($arg:tt)*) => {
        $logger.log(LogLevel::Debug, $target, &format!($($arg)*)).await;
    };
}

#[macro_export]
macro_rules! log_info {
    ($logger:expr, $target:expr, $($arg:tt)*) => {
        $logger.log(LogLevel::Info, $target, &format!($($arg)*)).await;
    };
}

#[macro_export]
macro_rules! log_warn {
    ($logger:expr, $target:expr, $($arg:tt)*) => {
        $logger.log(LogLevel::Warn, $target, &format!($($arg)*)).await;
    };
}

#[macro_export]
macro_rules! log_error {
    ($logger:expr, $target:expr, $($arg:tt)*) => {
        $logger.log(LogLevel::Error, $target, &format!($($arg)*)).await;
    };
}

/// 性能日志记录器
pub struct PerformanceLogger {
    logger: Arc<AsyncLogger>,
    start_times: Arc<RwLock<std::collections::HashMap<String, Instant>>>,
}

impl PerformanceLogger {
    /// 创建新的性能日志记录器
    pub fn new(logger: Arc<AsyncLogger>) -> Self {
        Self {
            logger,
            start_times: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }
    
    /// 开始计时
    pub async fn start_timer(&self, operation: &str) {
        let mut start_times = self.start_times.write().await;
        start_times.insert(operation.to_string(), Instant::now());
    }
    
    /// 结束计时并记录
    pub async fn end_timer(&self, operation: &str) {
        let duration = {
            let mut start_times = self.start_times.write().await;
            start_times.remove(operation).map(|start| start.elapsed())
        };
        
        if let Some(duration) = duration {
            let mut fields = std::collections::HashMap::new();
            fields.insert("duration_ms".to_string(), duration.as_millis().to_string());
            fields.insert("operation".to_string(), operation.to_string());
            
            self.logger.log_with_fields(
                LogLevel::Info,
                "performance",
                &format!("操作 {} 完成", operation),
                fields,
            ).await;
        }
    }
    
    /// 记录性能指标
    pub async fn record_metric(&self, metric: &str, value: f64, unit: &str) {
        let mut fields = std::collections::HashMap::new();
        fields.insert("metric".to_string(), metric.to_string());
        fields.insert("value".to_string(), value.to_string());
        fields.insert("unit".to_string(), unit.to_string());
        
        self.logger.log_with_fields(
            LogLevel::Info,
            "metrics",
            &format!("指标 {} = {} {}", metric, value, unit),
            fields,
        ).await;
    }
}

/// 日志工具示例
pub async fn logging_utils_example() -> Result<()> {
    println!("\n=== 日志工具示例 ===");
    
    // 创建日志配置
    let config = LogConfig {
        level: LogLevel::Debug,
        format: LogFormat::Text,
        output: LogOutput::Console,
        ..Default::default()
    };
    
    // 创建日志记录器
    let logger = Arc::new(AsyncLogger::new(config));
    
    // 记录各种级别的日志
    logger.log(LogLevel::Trace, "example", "这是一条跟踪日志").await;
    logger.log(LogLevel::Debug, "example", "这是一条调试日志").await;
    logger.log(LogLevel::Info, "example", "这是一条信息日志").await;
    logger.log(LogLevel::Warn, "example", "这是一条警告日志").await;
    logger.log(LogLevel::Error, "example", "这是一条错误日志").await;
    
    // 记录带字段的日志
    let mut fields = std::collections::HashMap::new();
    fields.insert("user_id".to_string(), "12345".to_string());
    fields.insert("action".to_string(), "login".to_string());
    fields.insert("ip".to_string(), "192.168.1.1".to_string());
    
    logger.log_with_fields(
        LogLevel::Info,
        "auth",
        "用户登录",
        fields,
    ).await;
    
    // 性能日志记录
    let perf_logger = PerformanceLogger::new(logger.clone());
    perf_logger.start_timer("database_query").await;
    
    // 模拟数据库查询
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    perf_logger.end_timer("database_query").await;
    
    // 记录性能指标
    perf_logger.record_metric("response_time", 150.5, "ms").await;
    perf_logger.record_metric("memory_usage", 1024.0, "MB").await;
    perf_logger.record_metric("cpu_usage", 75.2, "%").await;
    
    // 刷新日志
    logger.flush().await;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Trace < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Error);
    }
    
    #[test]
    fn test_log_level_display() {
        assert_eq!(LogLevel::Info.to_string(), "INFO");
        assert_eq!(LogLevel::Error.to_string(), "ERROR");
    }
    
    #[tokio::test]
    async fn test_async_logger() {
        let config = LogConfig::default();
        let logger = AsyncLogger::new(config);
        
        logger.log(LogLevel::Info, "test", "测试日志").await;
        logger.flush().await;
    }
    
    #[tokio::test]
    async fn test_performance_logger() {
        let config = LogConfig::default();
        let logger = Arc::new(AsyncLogger::new(config));
        let perf_logger = PerformanceLogger::new(logger);
        
        perf_logger.start_timer("test_operation").await;
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        perf_logger.end_timer("test_operation").await;
    }
}
