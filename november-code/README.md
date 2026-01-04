# Rust 高级特性与宏系统示例项目

本项目展示了 Rust 的高级特性与宏系统的使用，包括声明式宏、高级特性（Traits）和关联类型等。

## 项目结构

```
november-code/
├── Cargo.toml          # 项目配置文件
├── README.md           # 项目说明
└── src/
    ├── lib.rs          # 库入口
    ├── main.rs         # 演示程序
    ├── declarative_macros.rs  # 声明式宏示例
    ├── advanced_traits.rs     # 高级特性示例
    └── utils.rs              # 实用工具
```

## 功能特性

### 声明式宏
- `log!` - 日志宏，支持多级别和格式化
- `my_vec!` - 自定义向量创建宏
- `calculate!` - 计算宏，支持多种运算
- `create_user!` - 用户创建宏
- `match_result!` - 结果匹配宏
- `assert_approx_eq!` - 近似相等断言宏
- `repeat!` - 重复代码块宏
- `define_status!` - 状态枚举宏

### 高级特性
- 关联类型（Associated Types）
- 泛型关联类型（GAT）
- 特性对象（Trait Objects）
- 特性继承
- 关联常量
- 默认实现

### 过程宏使用
- 使用 `serde` 的派生宏进行序列化/反序列化

## 运行方式

### 运行演示程序
```powershell
cd 'F:\工作\学习计划\11月\november-code'
cargo run
```

### 运行测试
```powershell
cargo test
```

### 检查代码
```powershell
cargo check
```

## 依赖

- `serde` - 序列化框架
- `serde_json` - JSON 序列化支持

## 学习文档

详细的学习文档请参考：`高级特性与宏系统.md`

