// 计算器模块 - 包含基本数学运算

// 公开的加法函数
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

// 公开的减法函数
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

// 创建一个子模块用于高级运算
pub mod advanced {
    // 公开的乘法函数
    pub fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }

    // 公开的除法函数
    pub fn divide(a: f64, b: f64) -> f64 {
        if b == 0.0 {
            panic!("除数不能为零");
        }
        a / b
    }

    // 私有函数，只在模块内可见
    fn power_of_two(x: f64) -> f64 {
        x * x
    }

    // 公开函数可以调用私有函数
    pub fn square(x: f64) -> f64 {
        power_of_two(x)
    }
}

// 私有模块，只在当前文件可见
mod utils {
    // 私有函数
    pub fn is_zero(x: f64) -> bool {
        x == 0.0
    }
}

// 测试模块
#[cfg(test)]
mod tests {
    // 导入父模块中的所有内容
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_advanced_multiply() {
        assert_eq!(advanced::multiply(2.0, 3.0), 6.0);
    }
} 