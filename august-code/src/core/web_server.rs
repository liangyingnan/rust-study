//! 异步Web服务器模块
//! 
//! 提供异步Web服务器功能，包括：
//! - 带缓存的HTTP请求处理
//! - 并发请求管理
//! - 限流器实现
//! - 任务调度器

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};

/// 缓存条目
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry {
    data: String,
    timestamp: u64,
    ttl: u64,
}

/// 异步Web服务器
#[derive(Debug, Clone)]
pub struct AsyncWebServer {
    client: Client,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

impl AsyncWebServer {
    /// 创建新的Web服务器
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// 异步获取数据，带缓存
    pub async fn fetch_with_cache(&self, url: &str) -> Result<String> {
        // 检查缓存
        if let Some(cached) = self.get_from_cache(url).await {
            println!("从缓存获取: {}", url);
            return Ok(cached);
        }
        
        // 缓存未命中，发起请求
        println!("发起网络请求: {}", url);
        let start = Instant::now();
        
        let response = self.client
            .get(url)
            .timeout(Duration::from_secs(10))
            .send()
            .await?;
        
        let response_time = start.elapsed();
        let content = response.text().await?;
        
        // 存储到缓存
        self.store_in_cache(url, &content, 300).await; // 5分钟 TTL
        
        println!("请求完成: {} (耗时: {:?})", url, response_time);
        Ok(content)
    }
    
    /// 从缓存获取数据
    async fn get_from_cache(&self, url: &str) -> Option<String> {
        let cache = self.cache.read().await;
        if let Some(entry) = cache.get(url) {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            if now - entry.timestamp < entry.ttl {
                return Some(entry.data.clone());
            }
        }
        None
    }
    
    /// 存储数据到缓存
    async fn store_in_cache(&self, url: &str, data: &str, ttl: u64) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let entry = CacheEntry {
            data: data.to_string(),
            timestamp: now,
            ttl,
        };
        
        let mut cache = self.cache.write().await;
        cache.insert(url.to_string(), entry);
    }
    
    /// 并发处理多个请求
    pub async fn process_multiple_requests(&self, urls: Vec<&str>) -> Result<Vec<String>> {
        let mut handles = Vec::new();
        
        for url in urls {
            let server = self.clone();
            let url = url.to_string();
            let handle = tokio::spawn(async move {
                server.fetch_with_cache(&url).await
            });
            handles.push(handle);
        }
        
        let mut results = Vec::new();
        for handle in handles {
            match handle.await? {
                Ok(content) => results.push(content),
                Err(e) => {
                    eprintln!("请求失败: {}", e);
                    results.push("请求失败".to_string());
                }
            }
        }
        
        Ok(results)
    }
    
    /// 清理过期缓存
    pub async fn cleanup_cache(&self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut cache = self.cache.write().await;
        cache.retain(|_, entry| now - entry.timestamp < entry.ttl);
        
        println!("缓存清理完成，剩余条目: {}", cache.len());
    }
    
    /// 获取缓存统计信息
    pub async fn cache_stats(&self) -> (usize, usize) {
        let cache = self.cache.read().await;
        let total_entries = cache.len();
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let valid_entries = cache.values()
            .filter(|entry| now - entry.timestamp < entry.ttl)
            .count();
        
        (total_entries, valid_entries)
    }
}

/// 异步任务调度器
pub struct TaskScheduler {
    tasks: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,
}

impl TaskScheduler {
    /// 创建新的任务调度器
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// 添加周期性任务
    pub async fn add_periodic_task<F>(&self, name: &str, interval: Duration, task: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        let name = name.to_string();
        let tasks = Arc::clone(&self.tasks);
        
        let handle = tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            loop {
                interval_timer.tick().await;
                println!("执行周期性任务: {}", name);
                task();
            }
        });
        
        let mut tasks_guard = tasks.write().await;
        tasks_guard.push(handle);
    }
    
    /// 等待所有任务完成
    pub async fn wait_for_all(&self) {
        let tasks = self.tasks.read().await;
        for handle in tasks.iter() {
            let _ = handle.await;
        }
    }
}

/// 异步限流器
pub struct RateLimiter {
    requests: Arc<RwLock<Vec<Instant>>>,
    max_requests: usize,
    time_window: Duration,
}

impl RateLimiter {
    /// 创建新的限流器
    pub fn new(max_requests: usize, time_window: Duration) -> Self {
        Self {
            requests: Arc::new(RwLock::new(Vec::new())),
            max_requests,
            time_window,
        }
    }
    
    /// 检查是否允许请求
    pub async fn allow_request(&self) -> bool {
        let now = Instant::now();
        let mut requests = self.requests.write().await;
        
        // 清理过期的请求记录
        requests.retain(|&time| now.duration_since(time) < self.time_window);
        
        if requests.len() < self.max_requests {
            requests.push(now);
            true
        } else {
            false
        }
    }
    
    /// 等待直到允许请求
    pub async fn wait_for_permission(&self) {
        while !self.allow_request().await {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_cache_functionality() {
        let server = AsyncWebServer::new();
        
        // 测试缓存存储和获取
        server.store_in_cache("test", "test_data", 60).await;
        
        let cached = server.get_from_cache("test").await;
        assert_eq!(cached, Some("test_data".to_string()));
        
        // 测试缓存统计
        let (total, valid) = server.cache_stats().await;
        assert_eq!(total, 1);
        assert_eq!(valid, 1);
    }
    
    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = RateLimiter::new(2, Duration::from_secs(1));
        
        // 应该允许前两个请求
        assert!(limiter.allow_request().await);
        assert!(limiter.allow_request().await);
        
        // 第三个请求应该被限制
        assert!(!limiter.allow_request().await);
    }
}
