# Copilot 使用说明（为本仓库定制）

本文件为 AI 编码代理（Copilot / Agents）提供针对本仓库的快速上手指南，着重指出本仓库的结构、约定、关键文件和常见编辑陷阱。请保持简洁且可执行。

**Big Picture**
- **项目类型**: 一个同时包含库和二进制示例的 Rust 小项目（`Cargo.toml` 定义 `lib` 和 `[[bin]]`）。
- **目标**: 演示 Rust 声明式宏、Trait/关联类型和 serde 的使用。主要模块位于 `src/`（`lib.rs` 把子模块导出）。

**关键文件/位置（快速引用）**
- **库入口**: src/lib.rs — 导出 `declarative_macros`, `advanced_traits`, `utils`。
- **示例程序**: src/main.rs — 演示宏与 trait 的使用；包含运行示例与若干 `#[cfg(test)]` 单测。
- **宏集合**: src/declarative_macros.rs — 所有 `macro_rules!` 示例（使用 `#[macro_export]`，从 crate 根导出）。
- **高级特性**: src/advanced_traits.rs — 关联类型、GAT、trait 继承等示例。
- **实用工具**: src/utils.rs — `serde` 派生用法、`test_data!`、`try_or_return!` 等习惯用法。

**构建 / 测试 / 调试 快速命令**
```powershell
# 在仓库根目录运行示例
cargo run

# 运行全部测试
cargo test

# 快速类型检查
cargo check
```

**仓库特定约定与模式（务必遵守）**
- 宏：宏都用 `#[macro_export]` 导出并期望从 crate 根可用；宏内部若引用库类型请用 `crate::...`（示例见 `src/utils.rs::test_data!`）。
- 早退模式：仓库内多个宏（如 `try_or_return!`、`unwrap_or_return!`）通过打印错误并 `return` 来早退——当你修改这些宏或在其它模块复制类似逻辑时，遵守相同的早退语义。
- 命名：库在 `Cargo.toml` 中名为 `macro-examples`，lib 名称为 `macro_examples`（注意下划线/连字符的差异）。在代码中 `use macro_examples::...`。
- GAT 与最低 Rust 版本：`src/advanced_traits.rs` 中用到 GAT（泛型关联类型），请确保目标环境支持相应的 Rust 版本（最小约需 Rust 1.65+）。

**修改与扩展建议（AI 代理写补丁时）**
- 修改宏时：确保保留 `#[macro_export]`（若期望从 crate 根可用），并保持对 `String`/`&str` 的一致处理（多数宏将 `.to_string()` 用于构造字符串）。
- 添加新导出符号：更新 `src/lib.rs` 导出表（`pub use ...`），以便 `main.rs` 与外部调用保持一致。
- 依赖调整：若新增 serde 结构或启用特性，请同步更新 `Cargo.toml`（本仓库显式使用 `serde = { version = "1.0", features = ["derive"] }`）。

**可调查的具体示例（便于定位）**
- 要查看自定义 vec 宏实现在: src/declarative_macros.rs（`my_vec!`）。
- 要查看断言宏与近似比较用例: src/declarative_macros.rs（`assert_approx_eq!`），main.rs 的示例会调用它。
- 要查看 serde 用法与 JSON 辅助方法: src/utils.rs（`Person::to_json`, `Person::from_json`）。

**常见坑 / 不要做的事**
- 不要在宏内部直接使用非导出的路径（例如 `crate::private_mod::X`）除非确定该路径在 lib 根可见；优先使用公开类型或在 `lib.rs` 复导出。
- 修改 trait 定义需注意兼容性（多数示例为教学用途，不应在破坏现有 trait API 的情况下改变签名）。

**想知道更多？**
- 阅读仓库 README.md 以获取运行样例与上下文。

----
请先检查上述条目是否覆盖你需要 AI 代理掌握的主要点；指出任何遗漏或需要更详尽示例的区域，我会据此迭代。
