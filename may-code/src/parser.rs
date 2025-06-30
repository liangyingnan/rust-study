use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use crate::config::ConfigParser;
use crate::error::{ConfigError, ConfigResult};

/// JSON 解析器
/// 演示了 trait 的具体实现
#[derive(Debug, Clone)]
pub struct JsonParser;

impl<T> ConfigParser<T> for JsonParser
where
    T: Serialize + for<'de> Deserialize<'de> + Debug + Clone,
{
    fn parse_from_str(&self, content: &str) -> ConfigResult<T> {
        let config: T = serde_json::from_str(content)?;
        println!("成功解析 JSON 配置");
        Ok(config)
    }

    fn serialize_to_string(&self, config: &T) -> ConfigResult<String> {
        let content = serde_json::to_string_pretty(config)?;
        println!("成功序列化为 JSON 格式");
        Ok(content)
    }

    fn supported_format(&self) -> &'static str {
        "json"
    }

    fn validate(&self, config: &T) -> ConfigResult<()> {
        println!("执行 JSON 配置验证: {:?}", config);
        // 这里可以添加 JSON 特定的验证逻辑
        Ok(())
    }
}

/// YAML 解析器
#[derive(Debug, Clone)]
pub struct YamlParser;

impl<T> ConfigParser<T> for YamlParser
where
    T: Serialize + for<'de> Deserialize<'de> + Debug + Clone,
{
    fn parse_from_str(&self, content: &str) -> ConfigResult<T> {
        let config: T = serde_yaml::from_str(content)?;
        println!("成功解析 YAML 配置");
        Ok(config)
    }

    fn serialize_to_string(&self, config: &T) -> ConfigResult<String> {
        let content = serde_yaml::to_string(config)?;
        println!("成功序列化为 YAML 格式");
        Ok(content)
    }

    fn supported_format(&self) -> &'static str {
        "yaml"
    }

    fn validate(&self, config: &T) -> ConfigResult<()> {
        println!("执行 YAML 配置验证: {:?}", config);
        // 这里可以添加 YAML 特定的验证逻辑
        Ok(())
    }
}

/// TOML 解析器
#[derive(Debug, Clone)]
pub struct TomlParser;

impl<T> ConfigParser<T> for TomlParser
where
    T: Serialize + for<'de> Deserialize<'de> + Debug + Clone,
{
    fn parse_from_str(&self, content: &str) -> ConfigResult<T> {
        let config: T = toml::from_str(content)?;
        println!("成功解析 TOML 配置");
        Ok(config)
    }

    fn serialize_to_string(&self, config: &T) -> ConfigResult<String> {
        let content = toml::to_string_pretty(config)?;
        println!("成功序列化为 TOML 格式");
        Ok(content)
    }

    fn supported_format(&self) -> &'static str {
        "toml"
    }

    fn validate(&self, config: &T) -> ConfigResult<()> {
        println!("执行 TOML 配置验证: {:?}", config);
        // 这里可以添加 TOML 特定的验证逻辑
        Ok(())
    }
}

/// 动态解析器工厂
/// 演示了泛型和动态分发的结合使用
pub struct ParserFactory;

impl ParserFactory {
    /// 根据文件扩展名创建相应的解析器
    /// 返回 Box<dyn ConfigParser<T>> 展示 trait object 的使用
    pub fn create_parser<T>(format: &str) -> ConfigResult<Box<dyn ConfigParser<T>>>
    where
        T: Serialize + for<'de> Deserialize<'de> + Debug + Clone + 'static,
    {
        match format.to_lowercase().as_str() {
            "json" => Ok(Box::new(JsonParser)),
            "yaml" | "yml" => Ok(Box::new(YamlParser)),
            "toml" => Ok(Box::new(TomlParser)),
            _ => Err(ConfigError::UnsupportedFormat {
                format: format.to_string(),
            }),
        }
    }

    /// 获取支持的格式列表
    pub fn supported_formats() -> Vec<&'static str> {
        vec!["json", "yaml", "yml", "toml"]
    }
}

/// 泛型解析器包装器
/// 演示了泛型结构体的高级用法
pub struct GenericParser<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Debug + Clone,
{
    parser: Box<dyn ConfigParser<T>>,
    format: String,
}

impl<T> GenericParser<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Debug + Clone + 'static,
{
    /// 创建新的泛型解析器
    pub fn new(format: &str) -> ConfigResult<Self> {
        let parser = ParserFactory::create_parser::<T>(format)?;
        Ok(Self {
            parser,
            format: format.to_string(),
        })
    }

    /// 获取解析器格式
    pub fn get_format(&self) -> &str {
        &self.format
    }
}

impl<T> ConfigParser<T> for GenericParser<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Debug + Clone,
{
    fn parse_from_str(&self, content: &str) -> ConfigResult<T> {
        self.parser.parse_from_str(content)
    }

    fn serialize_to_string(&self, config: &T) -> ConfigResult<String> {
        self.parser.serialize_to_string(config)
    }

    fn supported_format(&self) -> &'static str {
        // 这里返回一个静态字符串，在实际应用中可能需要更复杂的处理
        "generic"
    }

    fn validate(&self, config: &T) -> ConfigResult<()> {
        self.parser.validate(config)
    }
}

/// 演示多态行为的辅助函数
pub fn demonstrate_polymorphism<T>(
    parsers: Vec<Box<dyn ConfigParser<T>>>,
    content: &str,
) -> Vec<ConfigResult<T>>
where
    T: Serialize + for<'de> Deserialize<'de> + Debug + Clone,
{
    parsers
        .into_iter()
        .map(|parser| {
            println!("使用 {} 解析器", parser.supported_format());
            parser.parse_from_str(content)
        })
        .collect()
}

/// 批量处理不同格式的配置文件
pub fn batch_process_configs<T>(
    files: Vec<(String, String)>, // (file_path, content)
) -> ConfigResult<Vec<T>>
where
    T: Serialize + for<'de> Deserialize<'de> + Debug + Clone + 'static,
{
    let mut results = Vec::new();

    for (file_path, content) in files {
        // 从文件路径推断格式
        let extension = std::path::Path::new(&file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| ConfigError::ValidationError {
                message: format!("无法推断文件格式: {}", file_path),
            })?;

        // 创建相应的解析器
        let parser = ParserFactory::create_parser::<T>(extension)?;
        
        // 解析配置
        match parser.parse_from_str(&content) {
            Ok(config) => {
                println!("成功处理文件: {}", file_path);
                results.push(config);
            }
            Err(e) => {
                eprintln!("处理文件 {} 时出错: {}", file_path, e);
                return Err(e);
            }
        }
    }

    Ok(results)
} 