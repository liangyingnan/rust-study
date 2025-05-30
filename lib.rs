// 声明我们的模块
pub mod calculator;
pub mod statistics;

// 从模块中重新导出特定函数，使其可以直接从crate根访问
pub use calculator::add;
pub use calculator::subtract;
pub use calculator::advanced::multiply;
pub use calculator::advanced::divide;
pub use statistics::mean;
pub use statistics::median;

// 提供一个简单的版本常量
pub const VERSION: &str = "1.0.0";

// 封装的计算器结构体，用于面向对象风格的使用
pub struct Calculator {
    // 可以添加一些状态，比如历史记录
    pub last_result: Option<f64>,
}

impl Calculator {
    // 构造函数
    pub fn new() -> Self {
        Calculator { last_result: None }
    }
    
    // 方法会保存结果
    pub fn add(&mut self, a: f64, b: f64) -> f64 {
        let result = calculator::add(a, b);
        self.last_result = Some(result);
        result
    }
    
    pub fn subtract(&mut self, a: f64, b: f64) -> f64 {
        let result = calculator::subtract(a, b);
        self.last_result = Some(result);
        result
    }
    
    pub fn multiply(&mut self, a: f64, b: f64) -> f64 {
        let result = calculator::advanced::multiply(a, b);
        self.last_result = Some(result);
        result
    }
    
    pub fn divide(&mut self, a: f64, b: f64) -> f64 {
        let result = calculator::advanced::divide(a, b);
        self.last_result = Some(result);
        result
    }
} 