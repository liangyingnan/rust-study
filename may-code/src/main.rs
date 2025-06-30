mod error;
mod config;
mod parser;
mod cli;

use clap::Parser;
use cli::{Cli, CliHandler};
use error::ConfigResult;

/// 主函数 - 程序入口点
/// 演示了完整的错误处理链和程序结构
fn main() {
    // 解析命令行参数
    let cli = Cli::parse();

    // 显示欢迎信息
    print_welcome();

    // 执行命令并处理错误
    if let Err(e) = CliHandler::run(cli) {
        eprintln!("❌ 程序执行出错: {}", e);
        std::process::exit(1);
    }

    println!("\n🎉 程序执行完成！");
}

/// 显示欢迎信息和学习要点
fn print_welcome() {
    println!("🦀 Rust 配置管理器 - 第五个月学习成果");
    println!("═══════════════════════════════════════════════════════════");
    println!("📚 本项目演示的 Rust 核心概念:");
    println!("  🔹 自定义错误类型 (thiserror)");
    println!("  🔹 Result 和 Option 的高级用法"); 
    println!("  🔹 错误传播 (? 操作符)");
    println!("  🔹 泛型 (Generics) 和泛型约束");
    println!("  🔹 特性 (Traits) 和多态性");
    println!("  🔹 trait objects (Box<dyn Trait>)");
    println!("  🔹 序列化和反序列化 (serde)");
    println!("  🔹 命令行参数解析 (clap)");
    println!("═══════════════════════════════════════════════════════════");
    println!();
}

/// 演示模块导出的主要功能
/// 这里展示了如何在代码中使用我们创建的各种组件
#[allow(dead_code)]
fn demonstrate_usage() -> ConfigResult<()> {
    use config::{AppConfig, create_config_manager};
    use parser::{JsonParser, ParserFactory};
    use error::{ConfigError, validate_config_path};

    // 1. 错误处理演示
    println!("🔧 错误处理演示:");
    
    // 演示 Option 到 Result 的转换
    let path = validate_config_path(Some("test.json"))?;
    println!("  验证路径成功: {}", path);

    // 演示自定义错误创建
    let error = ConfigError::ValidationError {
        message: "这是一个示例错误".to_string(),
    };
    println!("  创建自定义错误: {}", error);

    // 2. 泛型和 Traits 演示
    println!("\n🎯 泛型和 Traits 演示:");
    
    // 创建泛型配置管理器
    let mut manager = create_config_manager::<AppConfig, _>(JsonParser);
    println!("  创建泛型配置管理器成功");
    
    // 使用默认配置
    let config = AppConfig::default();
    manager.update_config(config)?;
    println!("  更新配置成功");

    // 3. 动态解析器演示
    println!("\n⚡ 动态解析器演示:");
    
    // 使用工厂模式创建解析器
    let json_parser = ParserFactory::create_parser::<AppConfig>("json")?;
    println!("  创建 JSON 解析器: {}", json_parser.supported_format());
    
    let yaml_parser = ParserFactory::create_parser::<AppConfig>("yaml")?;
    println!("  创建 YAML 解析器: {}", yaml_parser.supported_format());

    Ok(())
}

/// 单元测试模块
/// 演示了 Rust 的测试功能和错误处理测试
#[cfg(test)]
mod tests {
    use super::*;
    use config::{AppConfig, ConfigParser};
    use parser::{JsonParser, YamlParser, TomlParser};
    use error::ConfigError;

    #[test]
    fn test_config_serialization() {
        let config = AppConfig::default();
        let parser = JsonParser;
        
        // 测试序列化
        let result = parser.serialize_to_string(&config);
        assert!(result.is_ok());
        
        // 测试反序列化
        if let Ok(json_string) = result {
            let parsed_result: Result<AppConfig, _> = parser.parse_from_str(&json_string);
            assert!(parsed_result.is_ok());
        }
    }

    #[test]
    fn test_error_propagation() {
        use error::check_file_extension;
        
        // 测试成功情况
        let result = check_file_extension("test.json");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "json");
        
        // 测试错误情况
        let result = check_file_extension("test.unknown");
        assert!(result.is_err());
        
        if let Err(ConfigError::UnsupportedFormat { format }) = result {
            assert_eq!(format, "unknown");
        } else {
            panic!("期望的是 UnsupportedFormat 错误");
        }
    }

    #[test]
    fn test_generic_parser_factory() {
        use parser::ParserFactory;
        
        // 测试支持的格式
        let json_parser = ParserFactory::create_parser::<AppConfig>("json");
        assert!(json_parser.is_ok());
        
        let yaml_parser = ParserFactory::create_parser::<AppConfig>("yaml");
        assert!(yaml_parser.is_ok());
        
        let toml_parser = ParserFactory::create_parser::<AppConfig>("toml");
        assert!(toml_parser.is_ok());
        
        // 测试不支持的格式
        let unknown_parser = ParserFactory::create_parser::<AppConfig>("unknown");
        assert!(unknown_parser.is_err());
    }

    #[test]
    fn test_option_and_result_combinations() {
        use config::find_config_value;
        use std::collections::HashMap;
        
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        
        // 测试找到值的情况
        let result = find_config_value(Some(&map), "key1");
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
        
        // 测试未找到值的情况
        let result = find_config_value(Some(&map), "key2");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
        
        // 测试传入 None 的情况
        let result = find_config_value::<String>(None, "key1");
        assert!(result.is_err());
    }
}
