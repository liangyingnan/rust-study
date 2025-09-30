//! 异步HTTP客户端模块
//! 
//! 提供异步HTTP请求功能，包括：
//! - 基本HTTP请求
//! - 并发请求处理
//! - 错误处理和重试
//! - 超时管理

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::Instant;

/// HTTP响应信息
#[derive(Debug, Deserialize, Serialize)]
pub struct HttpResponse {
    pub url: String,
    pub status: u16,
    pub response_time_ms: u64,
    pub content_length: Option<usize>,
}

/// 异步HTTP客户端
pub struct AsyncHttpClient {
    client: Client,
    timeout: Duration,
}

impl AsyncHttpClient {
    /// 创建新的HTTP客户端
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            timeout: Duration::from_secs(30),
        }
    }
    
    /// 创建带超时的HTTP客户端
    pub fn with_timeout(timeout: Duration) -> Self {
        Self {
            client: Client::new(),
            timeout,
        }
    }
    
    /// 异步获取单个URL的数据
    pub async fn fetch_url(&self, url: &str) -> Result<HttpResponse> {
        let start = Instant::now();
        
        let response = self.client
            .get(url)
            .timeout(self.timeout)
            .send()
            .await?;
        
        let status = response.status().as_u16();
        let content_length = response.content_length().map(|len| len as usize);
        let response_time = start.elapsed().as_millis() as u64;
        
        // 读取响应体（可选）
        let _body = response.text().await?;
        
        Ok(HttpResponse {
            url: url.to_string(),
            status,
            response_time_ms: response_time,
            content_length,
        })
    }
    
    /// 并发获取多个URL的数据
    pub async fn fetch_multiple_urls(&self, urls: Vec<String>) -> Result<Vec<HttpResponse>> {
        let mut handles = Vec::new();
        
        // 为每个URL创建异步任务
        for url in urls {
            let client = self.client.clone();
            let timeout = self.timeout;
            let handle = tokio::spawn(async move {
                let start = Instant::now();
                let response = client
                    .get(&url)
                    .timeout(timeout)
                    .send()
                    .await?;
                
                let status = response.status().as_u16();
                let content_length = response.content_length().map(|len| len as usize);
                let response_time = start.elapsed().as_millis() as u64;
                let _body = response.text().await?;
                
                Ok::<HttpResponse, anyhow::Error>(HttpResponse {
                    url,
                    status,
                    response_time_ms: response_time,
                    content_length,
                })
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
    
    /// 使用join!宏并发执行多个异步操作
    pub async fn concurrent_requests(&self, urls: Vec<&str>) -> Result<Vec<HttpResponse>> {
        let mut handles = Vec::new();
        
        for url in urls {
            let client = self.client.clone();
            let timeout = self.timeout;
            let url = url.to_string();
            let handle = tokio::spawn(async move {
                let start = Instant::now();
                let response = client
                    .get(&url)
                    .timeout(timeout)
                    .send()
                    .await?;
                
                let status = response.status().as_u16();
                let content_length = response.content_length().map(|len| len as usize);
                let response_time = start.elapsed().as_millis() as u64;
                let _body = response.text().await?;
                
                Ok::<HttpResponse, anyhow::Error>(HttpResponse {
                    url,
                    status,
                    response_time_ms: response_time,
                    content_length,
                })
            });
            handles.push(handle);
        }
        
        let mut results = Vec::new();
        for handle in handles {
            match handle.await? {
                Ok(response) => results.push(response),
                Err(e) => eprintln!("请求失败: {}", e),
            }
        }
        
        Ok(results)
    }
    
    /// 带重试的HTTP请求
    pub async fn fetch_with_retry(&self, url: &str, max_retries: u32) -> Result<HttpResponse> {
        let mut last_error = None;
        
        for attempt in 1..=max_retries {
            match self.fetch_url(url).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < max_retries {
                        let delay = Duration::from_millis(100 * attempt as u64);
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap())
    }
}

impl Default for AsyncHttpClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_http_client_creation() {
        let client = AsyncHttpClient::new();
        assert_eq!(client.timeout, Duration::from_secs(30));
    }
    
    #[tokio::test]
    async fn test_http_client_with_timeout() {
        let timeout = Duration::from_secs(10);
        let client = AsyncHttpClient::with_timeout(timeout);
        assert_eq!(client.timeout, timeout);
    }
}
