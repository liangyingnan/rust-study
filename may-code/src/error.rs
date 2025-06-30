use thiserror::Error;

/// 配置管理器的自定义错误类型
/// 演示了 thiserror 库的使用和错误传播
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("文件不存在: {path}")]
    FileNotFound { path: String },

    #[error("不支持的文件格式: {format}")]
    UnsupportedFormat { format: String },

    #[error("JSON 解析错误: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("YAML 解析错误: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("TOML 解析错误: {0}")]
    TomlDeError(#[from] toml::de::Error),

    #[error("TOML 序列化错误: {0}")]
    TomlSerError(#[from] toml::ser::Error),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("验证错误: {message}")]
    ValidationError { message: String },

    #[error("转换错误: {0}")]
    ConversionError(String),
}

/// Result 类型别名，简化错误处理
pub type ConfigResult<T> = Result<T, ConfigError>;

/// 用于演示 Option 类型高级用法的辅助函数
pub fn validate_config_path(path: Option<&str>) -> ConfigResult<String> {
    path.ok_or_else(|| ConfigError::ValidationError {
        message: "配置文件路径不能为空".to_string(),
    })
    .map(|p| p.to_string())
}

/// 演示错误链式传播的示例函数
pub fn check_file_extension(path: &str) -> ConfigResult<String> {
    let extension = std::path::Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| ConfigError::ValidationError {
            message: format!("无法获取文件扩展名: {}", path),
        })?;

    match extension.to_lowercase().as_str() {
        "json" | "yaml" | "yml" | "toml" => Ok(extension.to_string()),
        _ => Err(ConfigError::UnsupportedFormat {
            format: extension.to_string(),
        }),
    }
} 