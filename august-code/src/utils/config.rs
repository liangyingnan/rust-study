//! 配置工具模块
//! 
//! 提供配置管理功能：
//! - 配置加载
//! - 配置验证
//! - 环境变量支持
//! - 配置热重载

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub features: FeatureConfig,
}

/// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub timeout: u64,
    pub max_connections: u32,
}

/// 数据库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub timeout: u64,
    pub retry_attempts: u32,
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<String>,
    pub max_size: u64,
    pub max_files: u32,
}

/// 功能配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureConfig {
    pub enable_cache: bool,
    pub enable_metrics: bool,
    pub enable_tracing: bool,
    pub debug_mode: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "localhost".to_string(),
                port: 8080,
                timeout: 30,
                max_connections: 100,
            },
            database: DatabaseConfig {
                url: "sqlite://:memory:".to_string(),
                max_connections: 10,
                timeout: 5,
                retry_attempts: 3,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: None,
                max_size: 10 * 1024 * 1024, // 10MB
                max_files: 5,
            },
            features: FeatureConfig {
                enable_cache: true,
                enable_metrics: false,
                enable_tracing: false,
                debug_mode: false,
            },
        }
    }
}

/// 配置管理器
pub struct ConfigManager {
    config: Arc<RwLock<AppConfig>>,
    watchers: Arc<RwLock<Vec<Box<dyn ConfigWatcher + Send + Sync>>>>,
}

/// 配置观察者trait
pub trait ConfigWatcher {
    fn on_config_changed(&self, config: &AppConfig);
}

impl ConfigManager {
    /// 创建新的配置管理器
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(AppConfig::default())),
            watchers: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// 从环境变量加载配置
    pub async fn load_from_env(&self) -> Result<()> {
        let mut config = self.config.write().await;
        
        // 服务器配置
        if let Ok(host) = std::env::var("SERVER_HOST") {
            config.server.host = host;
        }
        if let Ok(port) = std::env::var("SERVER_PORT") {
            config.server.port = port.parse()?;
        }
        if let Ok(timeout) = std::env::var("SERVER_TIMEOUT") {
            config.server.timeout = timeout.parse()?;
        }
        if let Ok(max_conn) = std::env::var("SERVER_MAX_CONNECTIONS") {
            config.server.max_connections = max_conn.parse()?;
        }
        
        // 数据库配置
        if let Ok(url) = std::env::var("DATABASE_URL") {
            config.database.url = url;
        }
        if let Ok(max_conn) = std::env::var("DATABASE_MAX_CONNECTIONS") {
            config.database.max_connections = max_conn.parse()?;
        }
        if let Ok(timeout) = std::env::var("DATABASE_TIMEOUT") {
            config.database.timeout = timeout.parse()?;
        }
        if let Ok(retries) = std::env::var("DATABASE_RETRY_ATTEMPTS") {
            config.database.retry_attempts = retries.parse()?;
        }
        
        // 日志配置
        if let Ok(level) = std::env::var("LOG_LEVEL") {
            config.logging.level = level;
        }
        if let Ok(file) = std::env::var("LOG_FILE") {
            config.logging.file = Some(file);
        }
        if let Ok(max_size) = std::env::var("LOG_MAX_SIZE") {
            config.logging.max_size = max_size.parse()?;
        }
        if let Ok(max_files) = std::env::var("LOG_MAX_FILES") {
            config.logging.max_files = max_files.parse()?;
        }
        
        // 功能配置
        if let Ok(enable_cache) = std::env::var("ENABLE_CACHE") {
            config.features.enable_cache = enable_cache.parse()?;
        }
        if let Ok(enable_metrics) = std::env::var("ENABLE_METRICS") {
            config.features.enable_metrics = enable_metrics.parse()?;
        }
        if let Ok(enable_tracing) = std::env::var("ENABLE_TRACING") {
            config.features.enable_tracing = enable_tracing.parse()?;
        }
        if let Ok(debug_mode) = std::env::var("DEBUG_MODE") {
            config.features.debug_mode = debug_mode.parse()?;
        }
        
        Ok(())
    }
    
    /// 从文件加载配置
    pub async fn load_from_file(&self, path: &str) -> Result<()> {
        let content = tokio::fs::read_to_string(path).await?;
        let config: AppConfig = toml::from_str(&content)?;
        
        let mut current_config = self.config.write().await;
        *current_config = config;
        
        self.notify_watchers().await;
        Ok(())
    }
    
    /// 保存配置到文件
    pub async fn save_to_file(&self, path: &str) -> Result<()> {
        let config = self.config.read().await;
        let content = toml::to_string_pretty(&*config)?;
        tokio::fs::write(path, content).await?;
        Ok(())
    }
    
    /// 获取配置
    pub async fn get_config(&self) -> AppConfig {
        self.config.read().await.clone()
    }
    
    /// 更新配置
    pub async fn update_config<F>(&self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut AppConfig),
    {
        let mut config = self.config.write().await;
        updater(&mut config);
        self.notify_watchers().await;
        Ok(())
    }
    
    /// 添加配置观察者
    pub async fn add_watcher(&self, watcher: Box<dyn ConfigWatcher + Send + Sync>) {
        let mut watchers = self.watchers.write().await;
        watchers.push(watcher);
    }
    
    /// 通知所有观察者
    async fn notify_watchers(&self) {
        let config = self.config.read().await;
        let watchers = self.watchers.read().await;
        
        for watcher in watchers.iter() {
            watcher.on_config_changed(&config);
        }
    }
    
    /// 验证配置
    pub async fn validate_config(&self) -> Result<Vec<String>> {
        let config = self.config.read().await;
        let mut errors = Vec::new();
        
        // 验证服务器配置
        if config.server.port == 0 {
            errors.push("服务器端口不能为0".to_string());
        }
        if config.server.timeout == 0 {
            errors.push("服务器超时时间不能为0".to_string());
        }
        if config.server.max_connections == 0 {
            errors.push("最大连接数不能为0".to_string());
        }
        
        // 验证数据库配置
        if config.database.url.is_empty() {
            errors.push("数据库URL不能为空".to_string());
        }
        if config.database.max_connections == 0 {
            errors.push("数据库最大连接数不能为0".to_string());
        }
        if config.database.timeout == 0 {
            errors.push("数据库超时时间不能为0".to_string());
        }
        
        // 验证日志配置
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&config.logging.level.as_str()) {
            errors.push(format!("无效的日志级别: {}", config.logging.level));
        }
        if config.logging.max_size == 0 {
            errors.push("日志文件最大大小不能为0".to_string());
        }
        if config.logging.max_files == 0 {
            errors.push("日志文件最大数量不能为0".to_string());
        }
        
        Ok(errors)
    }
}

/// 配置热重载器
pub struct ConfigReloader {
    config_manager: Arc<ConfigManager>,
    watch_path: String,
}

impl ConfigReloader {
    /// 创建新的配置重载器
    pub fn new(config_manager: Arc<ConfigManager>, watch_path: String) -> Self {
        Self {
            config_manager,
            watch_path,
        }
    }
    
    /// 开始监控配置文件变化
    pub async fn start_watching(&self) -> Result<()> {
        let path = self.watch_path.clone();
        let manager = Arc::clone(&self.config_manager);
        
        tokio::spawn(async move {
            let mut last_modified = std::time::SystemTime::UNIX_EPOCH;
            
            loop {
                if let Ok(metadata) = tokio::fs::metadata(&path).await {
                    if let Ok(modified) = metadata.modified() {
                        if modified > last_modified {
                            last_modified = modified;
                            
                            if let Err(e) = manager.load_from_file(&path).await {
                                eprintln!("重新加载配置失败: {}", e);
                            } else {
                                println!("配置已重新加载");
                            }
                        }
                    }
                }
                
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });
        
        Ok(())
    }
}

/// 配置工具示例
pub async fn config_utils_example() -> Result<()> {
    println!("\n=== 配置工具示例 ===");
    
    // 创建配置管理器
    let config_manager = Arc::new(ConfigManager::new());
    
    // 从环境变量加载配置
    config_manager.load_from_env().await?;
    
    // 获取配置
    let config = config_manager.get_config().await;
    println!("当前配置:");
    println!("  服务器: {}:{}", config.server.host, config.server.port);
    println!("  数据库: {}", config.database.url);
    println!("  日志级别: {}", config.logging.level);
    println!("  缓存启用: {}", config.features.enable_cache);
    
    // 验证配置
    let errors = config_manager.validate_config().await?;
    if errors.is_empty() {
        println!("配置验证通过");
    } else {
        println!("配置验证失败:");
        for error in errors {
            println!("  - {}", error);
        }
    }
    
    // 更新配置
    config_manager.update_config(|config| {
        config.server.port = 9090;
        config.features.debug_mode = true;
    }).await?;
    
    let updated_config = config_manager.get_config().await;
    println!("更新后的端口: {}", updated_config.server.port);
    println!("调试模式: {}", updated_config.features.debug_mode);
    
    // 保存配置到文件
    config_manager.save_to_file("config.toml").await?;
    println!("配置已保存到 config.toml");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.server.host, "localhost");
        assert_eq!(config.server.port, 8080);
        assert!(config.features.enable_cache);
    }
    
    #[tokio::test]
    async fn test_config_manager() {
        let manager = ConfigManager::new();
        let config = manager.get_config().await;
        assert_eq!(config.server.port, 8080);
    }
    
    #[tokio::test]
    async fn test_config_validation() {
        let manager = ConfigManager::new();
        let errors = manager.validate_config().await.unwrap();
        assert!(errors.is_empty());
    }
}
