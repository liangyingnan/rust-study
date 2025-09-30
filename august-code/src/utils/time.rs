//! 时间工具模块
//! 
//! 提供时间相关的工具函数：
//! - 时间格式化
//! - 时间计算
//! - 定时器工具
//! - 性能测量

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, timeout, Interval};

/// 时间工具
pub struct TimeUtils;

impl TimeUtils {
    /// 获取当前时间戳（秒）
    pub fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    
    /// 获取当前时间戳（毫秒）
    pub fn current_timestamp_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
    
    /// 格式化时间戳
    pub fn format_timestamp(timestamp: u64) -> String {
        let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0)
            .unwrap_or_default();
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
    
    /// 格式化持续时间
    pub fn format_duration(duration: Duration) -> String {
        let total_secs = duration.as_secs();
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;
        let millis = duration.subsec_millis();
        
        if hours > 0 {
            format!("{}h {}m {}s {}ms", hours, minutes, seconds, millis)
        } else if minutes > 0 {
            format!("{}m {}s {}ms", minutes, seconds, millis)
        } else if seconds > 0 {
            format!("{}s {}ms", seconds, millis)
        } else {
            format!("{}ms", millis)
        }
    }
    
    /// 创建延迟
    pub async fn delay(ms: u64) {
        sleep(Duration::from_millis(ms)).await;
    }
    
    /// 创建延迟（秒）
    pub async fn delay_seconds(seconds: u64) {
        sleep(Duration::from_secs(seconds)).await;
    }
}

/// 性能测量器
pub struct PerformanceTimer {
    start: Instant,
    name: String,
}

impl PerformanceTimer {
    /// 创建新的性能测量器
    pub fn new(name: &str) -> Self {
        Self {
            start: Instant::now(),
            name: name.to_string(),
        }
    }
    
    /// 获取经过的时间
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
    
    /// 获取经过的时间（毫秒）
    pub fn elapsed_ms(&self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
    
    /// 打印经过的时间
    pub fn print_elapsed(&self) {
        let elapsed = self.elapsed();
        println!("{} 耗时: {}", self.name, TimeUtils::format_duration(elapsed));
    }
}

impl Drop for PerformanceTimer {
    fn drop(&mut self) {
        self.print_elapsed();
    }
}

/// 异步定时器
pub struct AsyncTimer {
    interval: Interval,
    name: String,
    count: u64,
}

impl AsyncTimer {
    /// 创建新的异步定时器
    pub fn new(name: &str, duration: Duration) -> Self {
        Self {
            interval: tokio::time::interval(duration),
            name: name.to_string(),
            count: 0,
        }
    }
    
    /// 等待下一次触发
    pub async fn wait(&mut self) {
        self.interval.tick().await;
        self.count += 1;
        println!("定时器 {} 触发 #{}", self.name, self.count);
    }
    
    /// 获取触发次数
    pub fn count(&self) -> u64 {
        self.count
    }
}

/// 超时包装器
pub struct TimeoutWrapper;

impl TimeoutWrapper {
    /// 带超时的异步操作
    pub async fn with_timeout<F, T>(
        operation: F,
        timeout_duration: Duration,
    ) -> Result<T, TimeoutError>
    where
        F: std::future::Future<Output = T> + Send,
    {
        match timeout(timeout_duration, operation).await {
            Ok(result) => Ok(result),
            Err(_) => Err(TimeoutError {
                duration: timeout_duration,
            }),
        }
    }
}

/// 超时错误
#[derive(Debug)]
pub struct TimeoutError {
    pub duration: Duration,
}

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "操作超时，超时时间: {:?}", self.duration)
    }
}

impl std::error::Error for TimeoutError {}

/// 时间窗口
pub struct TimeWindow {
    start: Instant,
    duration: Duration,
}

impl TimeWindow {
    /// 创建新的时间窗口
    pub fn new(duration: Duration) -> Self {
        Self {
            start: Instant::now(),
            duration,
        }
    }
    
    /// 检查是否在窗口内
    pub fn is_within_window(&self) -> bool {
        self.start.elapsed() < self.duration
    }
    
    /// 获取剩余时间
    pub fn remaining_time(&self) -> Duration {
        let elapsed = self.start.elapsed();
        if elapsed < self.duration {
            self.duration - elapsed
        } else {
            Duration::from_secs(0)
        }
    }
    
    /// 重置时间窗口
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }
}

/// 时间工具示例
pub async fn time_utils_example() -> Result<(), anyhow::Error> {
    println!("\n=== 时间工具示例 ===");
    
    // 基本时间操作
    let timestamp = TimeUtils::current_timestamp();
    println!("当前时间戳: {}", timestamp);
    println!("格式化时间: {}", TimeUtils::format_timestamp(timestamp));
    
    // 性能测量
    {
        let _timer = PerformanceTimer::new("性能测量示例");
        TimeUtils::delay(100).await;
    }
    
    // 手动性能测量
    let timer = PerformanceTimer::new("手动测量");
    TimeUtils::delay(200).await;
    timer.print_elapsed();
    
    // 异步定时器
    let mut async_timer = AsyncTimer::new("示例定时器", Duration::from_millis(500));
    for i in 1..=3 {
        async_timer.wait().await;
        println!("定时器触发第 {} 次", i);
    }
    
    // 超时包装器
    let result = TimeoutWrapper::with_timeout(
        async {
            TimeUtils::delay(100).await;
            "操作完成"
        },
        Duration::from_millis(200),
    ).await;
    
    match result {
        Ok(value) => println!("超时测试成功: {}", value),
        Err(e) => println!("超时测试失败: {}", e),
    }
    
    // 时间窗口
    let mut window = TimeWindow::new(Duration::from_millis(1000));
    println!("时间窗口剩余时间: {:?}", window.remaining_time());
    
    TimeUtils::delay(500).await;
    println!("时间窗口剩余时间: {:?}", window.remaining_time());
    println!("是否在窗口内: {}", window.is_within_window());
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_current_timestamp() {
        let timestamp = TimeUtils::current_timestamp();
        assert!(timestamp > 0);
    }
    
    #[test]
    fn test_format_duration() {
        let duration = Duration::from_millis(1234);
        let formatted = TimeUtils::format_duration(duration);
        assert!(formatted.contains("1s"));
        assert!(formatted.contains("234ms"));
    }
    
    #[tokio::test]
    async fn test_performance_timer() {
        let timer = PerformanceTimer::new("测试");
        TimeUtils::delay(10).await;
        assert!(timer.elapsed_ms() >= 10);
    }
    
    #[tokio::test]
    async fn test_timeout_wrapper() {
        let result = TimeoutWrapper::with_timeout(
            async {
                TimeUtils::delay(50).await;
                "成功"
            },
            Duration::from_millis(100),
        ).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "成功");
    }
    
    #[tokio::test]
    async fn test_time_window() {
        let window = TimeWindow::new(Duration::from_millis(100));
        assert!(window.is_within_window());
        
        TimeUtils::delay(150).await;
        assert!(!window.is_within_window());
    }
}
