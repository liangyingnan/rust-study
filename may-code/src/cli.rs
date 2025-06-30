use clap::{Parser, Subcommand};
use crate::config::{AppConfig, create_config_manager, ConfigParser};
use crate::error::{ConfigError, ConfigResult, check_file_extension};
use crate::parser::{JsonParser, YamlParser, TomlParser, ParserFactory};

/// 配置文件管理器 - 展示 Rust 错误处理和泛型的强大功能
#[derive(Parser)]
#[command(name = "config-manager")]
#[command(about = "一个演示 Rust 泛型和错误处理的配置文件管理工具")]
#[command(version = "1.0.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 加载并显示配置文件
    Load {
        /// 配置文件路径
        #[arg(short, long)]
        file: String,
        
        /// 指定文件格式 (json, yaml, toml)
        #[arg(short, long)]
        format: Option<String>,
    },
    
    /// 创建默认配置文件
    Create {
        /// 输出文件路径
        #[arg(short, long)]
        output: String,
        
        /// 文件格式 (json, yaml, toml)
        #[arg(short, long, default_value = "json")]
        format: String,
    },
    
    /// 转换配置文件格式
    Convert {
        /// 输入文件路径
        #[arg(short, long)]
        input: String,
        
        /// 输出文件路径
        #[arg(short, long)]
        output: String,
        
        /// 目标格式 (json, yaml, toml)
        #[arg(short, long)]
        target_format: String,
    },
    
    /// 验证配置文件
    Validate {
        /// 配置文件路径
        #[arg(short, long)]
        file: String,
    },
    
    /// 显示支持的格式
    Formats,
    
    /// 演示泛型和错误处理功能
    Demo {
        /// 演示类型 (basic, advanced, error-handling)
        #[arg(short, long, default_value = "basic")]
        demo_type: String,
    },
}

/// CLI 处理器
pub struct CliHandler;

impl CliHandler {
    /// 执行 CLI 命令
    pub fn run(cli: Cli) -> ConfigResult<()> {
        match cli.command {
            Commands::Load { file, format } => Self::handle_load(file, format),
            Commands::Create { output, format } => Self::handle_create(output, format),
            Commands::Convert { input, output, target_format } => {
                Self::handle_convert(input, output, target_format)
            }
            Commands::Validate { file } => Self::handle_validate(file),
            Commands::Formats => Self::handle_formats(),
            Commands::Demo { demo_type } => Self::handle_demo(demo_type),
        }
    }

    /// 处理加载命令（演示错误传播和 Option 处理）
    fn handle_load(file: String, format: Option<String>) -> ConfigResult<()> {
        println!("🔄 加载配置文件: {}", file);

        // 验证文件扩展名或使用指定格式
        let detected_format = if let Some(fmt) = format {
            fmt
        } else {
            check_file_extension(&file)?
        };

        println!("📄 检测到格式: {}", detected_format);

        // 根据格式创建相应的解析器和配置管理器
        match detected_format.to_lowercase().as_str() {
            "json" => {
                let mut manager = create_config_manager::<AppConfig, _>(JsonParser);
                let config = manager.load_from_file(&file)?;
                Self::display_config(config);
            }
            "yaml" | "yml" => {
                let mut manager = create_config_manager::<AppConfig, _>(YamlParser);
                let config = manager.load_from_file(&file)?;
                Self::display_config(config);
            }
            "toml" => {
                let mut manager = create_config_manager::<AppConfig, _>(TomlParser);
                let config = manager.load_from_file(&file)?;
                Self::display_config(config);
            }
            _ => {
                return Err(ConfigError::UnsupportedFormat {
                    format: detected_format,
                });
            }
        }

        Ok(())
    }

    /// 处理创建命令（演示泛型的使用）
    fn handle_create(output: String, format: String) -> ConfigResult<()> {
        println!("🆕 创建默认配置文件: {} (格式: {})", output, format);

        let default_config = AppConfig::default();
        
        // 使用泛型解析器创建文件
        let parser = ParserFactory::create_parser::<AppConfig>(&format)?;
        let content = parser.serialize_to_string(&default_config)?;
        
        std::fs::write(&output, content)?;
        println!("✅ 配置文件已创建: {}", output);

        Ok(())
    }

    /// 处理转换命令（演示错误处理和泛型组合使用）
    fn handle_convert(input: String, output: String, target_format: String) -> ConfigResult<()> {
        println!("🔄 转换配置文件: {} -> {} (目标格式: {})", input, output, target_format);

        // 检测输入文件格式
        let input_format = check_file_extension(&input)?;
        println!("📥 输入格式: {}", input_format);

        // 读取并解析输入文件
        let content = std::fs::read_to_string(&input)?;
        let input_parser = ParserFactory::create_parser::<AppConfig>(&input_format)?;
        let config = input_parser.parse_from_str(&content)?;

        // 使用目标格式序列化
        let output_parser = ParserFactory::create_parser::<AppConfig>(&target_format)?;
        let output_content = output_parser.serialize_to_string(&config)?;

        // 写入输出文件
        std::fs::write(&output, output_content)?;
        println!("✅ 转换完成: {} -> {}", input, output);

        Ok(())
    }

    /// 处理验证命令
    fn handle_validate(file: String) -> ConfigResult<()> {
        println!("🔍 验证配置文件: {}", file);

        let format = check_file_extension(&file)?;
        let content = std::fs::read_to_string(&file)?;
        let parser = ParserFactory::create_parser::<AppConfig>(&format)?;
        
        match parser.parse_from_str(&content) {
            Ok(config) => {
                parser.validate(&config)?;
                println!("✅ 配置文件验证通过");
                Self::display_config(&config);
            }
            Err(e) => {
                println!("❌ 配置文件验证失败: {}", e);
                return Err(e);
            }
        }

        Ok(())
    }

    /// 显示支持的格式
    fn handle_formats() -> ConfigResult<()> {
        println!("📋 支持的配置文件格式:");
        for format in ParserFactory::supported_formats() {
            println!("  • {}", format);
        }
        Ok(())
    }

    /// 演示功能
    fn handle_demo(demo_type: String) -> ConfigResult<()> {
        match demo_type.as_str() {
            "basic" => Self::demo_basic_usage(),
            "advanced" => Self::demo_advanced_features(),
            "error-handling" => Self::demo_error_handling(),
            _ => {
                println!("❌ 未知的演示类型: {}", demo_type);
                println!("可用类型: basic, advanced, error-handling");
                Ok(())
            }
        }
    }

    /// 基础用法演示
    fn demo_basic_usage() -> ConfigResult<()> {
        println!("🎯 基础用法演示");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // 创建默认配置
        let config = AppConfig::default();
        println!("📦 创建默认配置:");
        Self::display_config(&config);

        // 演示不同格式的序列化
        let json_parser = JsonParser;
        let yaml_parser = YamlParser;
        let toml_parser = TomlParser;

        println!("\n🔄 不同格式序列化演示:");
        if let Ok(json) = json_parser.serialize_to_string(&config) {
            println!("JSON 格式:\n{}", json);
        }

        Ok(())
    }

    /// 高级功能演示
    fn demo_advanced_features() -> ConfigResult<()> {
        println!("🚀 高级功能演示");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // 演示泛型函数
        println!("🔧 泛型配置管理器演示:");
        let mut manager = create_config_manager(JsonParser);
        let mut config = AppConfig::default();
        config.name = "高级配置示例".to_string();
        config.settings.insert("advanced".to_string(), "true".to_string());

        manager.update_config(config)?;
        if let Some(cfg) = manager.get_config() {
            println!("配置管理器中的配置:");
            Self::display_config(cfg);
        }

        // 演示 Option 和 Result 组合使用
        println!("\n🔍 配置查找演示:");
        if let Some(cfg) = manager.get_config() {
            match crate::config::find_config_value(Some(&cfg.settings), "theme") {
                Ok(Some(value)) => println!("找到主题设置: {}", value),
                Ok(None) => println!("未找到主题设置"),
                Err(e) => println!("查找出错: {}", e),
            }
        }

        Ok(())
    }

    /// 错误处理演示
    fn demo_error_handling() -> ConfigResult<()> {
        println!("⚠️  错误处理演示");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // 演示不同类型的错误
        println!("1. 文件不存在错误:");
        match Self::handle_load("nonexistent.json".to_string(), None) {
            Err(e) => println!("   捕获错误: {}", e),
            Ok(_) => println!("   意外成功"),
        }

        println!("\n2. 不支持的格式错误:");
        match check_file_extension("test.unknown") {
            Err(e) => println!("   捕获错误: {}", e),
            Ok(_) => println!("   意外成功"),
        }

        println!("\n3. 验证错误演示:");
        match crate::error::validate_config_path(None) {
            Err(e) => println!("   捕获错误: {}", e),
            Ok(_) => println!("   意外成功"),
        }

        println!("\n✅ 错误处理演示完成");
        Ok(())
    }

    /// 显示配置信息
    fn display_config(config: &AppConfig) {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📋 配置信息:");
        println!("  名称: {}", config.name);
        println!("  版本: {}", config.version);
        println!("  调试模式: {}", if config.debug { "开启" } else { "关闭" });
        
        println!("  功能特性:");
        for feature in &config.features {
            println!("    • {}", feature);
        }
        
        println!("  设置项:");
        for (key, value) in &config.settings {
            println!("    {} = {}", key, value);
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
} 