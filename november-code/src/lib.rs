//! Rust 高级特性与宏系统示例库
//! 
//! 本库展示了：
//! 1. 声明式宏（macro_rules!）
//! 2. 过程宏的使用
//! 3. 高级特性（Traits）和关联类型
//! 4. 使用宏简化代码的实用示例

// 使用 #[macro_use] 导入宏模块（宏会通过 #[macro_export] 自动导出到 crate 根）
#[macro_use]
pub mod declarative_macros;

#[macro_use]
pub mod utils;

pub mod advanced_traits;

// 重新导出非宏项
pub use declarative_macros::User;
pub use advanced_traits::*;
pub use utils::Person;

