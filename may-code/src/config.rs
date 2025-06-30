use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use crate::error::{ConfigError, ConfigResult};

/// 泛型 trait 定义 - 配置解析器的统一接口
/// 演示了 Traits 的使用和泛型约束
pub trait ConfigParser<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Debug + Clone,
{
    /// 从字符串解析配置
    fn parse_from_str(&self, content: &str) -> ConfigResult<T>;
    
    /// 将配置序列化为字符串
    fn serialize_to_string(&self, config: &T) -> ConfigResult<String>;
    
    /// 获取解析器支持的文件格式
    fn supported_format(&self) -> &'static str;
    
    /// 验证配置的合法性（可选实现）
    fn validate(&self, config: &T) -> ConfigResult<()> {
        println!("使用默认验证逻辑: {:?}", config);
        Ok(())
    }
}

/// 泛型配置管理器结构体
/// 演示了泛型结构体的定义和使用
#[derive(Debug, Clone)]
pub struct ConfigManager<T, P>
where
    T: Serialize + for<'de> Deserialize<'de> + Debug + Clone,
    P: ConfigParser<T>,
{
    parser: P,
    config: Option<T>,
    file_path: Option<String>,
}

impl<T, P> ConfigManager<T, P>
where
    T: Serialize + for<'de> Deserialize<'de> + Debug + Clone,
    P: ConfigParser<T>,
{
    /// 创建新的配置管理器实例
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            config: None,
            file_path: None,
        }
    }

    /// 从文件加载配置（演示错误传播和 Option 处理）
    pub fn load_from_file(&mut self, path: &str) -> ConfigResult<&T> {
        // 验证文件路径
        if !std::path::Path::new(path).exists() {
            return Err(ConfigError::FileNotFound {
                path: path.to_string(),
            });
        }

        // 读取文件内容
        let content = std::fs::read_to_string(path)?;
        
        // 解析配置
        let config = self.parser.parse_from_str(&content)?;
        
        // 验证配置
        self.parser.validate(&config)?;
        
        // 保存配置和文件路径
        self.config = Some(config);
        self.file_path = Some(path.to_string());

        // 返回配置引用
        self.config
            .as_ref()
            .ok_or_else(|| ConfigError::ConversionError("配置加载失败".to_string()))
    }

    /// 保存配置到文件
    pub fn save_to_file(&self, path: Option<&str>) -> ConfigResult<()> {
        let config = self.config.as_ref().ok_or_else(|| {
            ConfigError::ValidationError {
                message: "没有可保存的配置".to_string(),
            }
        })?;

        let target_path = path
            .or(self.file_path.as_deref())
            .ok_or_else(|| ConfigError::ValidationError {
                message: "未指定保存路径".to_string(),
            })?;

        let content = self.parser.serialize_to_string(config)?;
        std::fs::write(target_path, content)?;

        println!("配置已保存到: {}", target_path);
        Ok(())
    }

    /// 获取配置引用（演示 Option 的高级用法）
    pub fn get_config(&self) -> Option<&T> {
        self.config.as_ref()
    }

    /// 更新配置
    pub fn update_config(&mut self, config: T) -> ConfigResult<()> {
        self.parser.validate(&config)?;
        self.config = Some(config);
        Ok(())
    }

    /// 获取解析器信息
    pub fn parser_info(&self) -> &'static str {
        self.parser.supported_format()
    }
}

/// 示例配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub settings: HashMap<String, String>,
    pub features: Vec<String>,
    pub debug: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut settings = HashMap::new();
        settings.insert("theme".to_string(), "dark".to_string());
        settings.insert("language".to_string(), "zh-CN".to_string());

        Self {
            name: "配置管理器示例".to_string(),
            version: "1.0.0".to_string(),
            settings,
            features: vec!["logging".to_string(), "caching".to_string()],
            debug: false,
        }
    }
}

/// 演示泛型函数的使用
pub fn create_config_manager<T, P>(parser: P) -> ConfigManager<T, P>
where
    T: Serialize + for<'de> Deserialize<'de> + Debug + Clone,
    P: ConfigParser<T>,
{
    ConfigManager::new(parser)
}

/// 演示 Option 和 Result 组合使用的辅助函数
pub fn find_config_value<'a, T>(
    config: Option<&'a HashMap<String, T>>,
    key: &str,
) -> ConfigResult<Option<&'a T>>
where
    T: Debug,
{
    match config {
        Some(map) => {
            let value = map.get(key);
            if let Some(v) = &value {
                println!("找到配置项 '{}': {:?}", key, v);
            }
            Ok(value)
        }
        None => Err(ConfigError::ValidationError {
            message: "配置映射为空".to_string(),
        }),
    }
} 