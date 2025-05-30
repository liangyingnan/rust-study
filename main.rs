// 引入我们的库
use rust_modules_demo::{
    // 直接从crate根导入的函数
    add, subtract, multiply, divide, mean, median,
    // 导入结构体
    Calculator,
    // 导入常量
    VERSION,
};

fn main() {
    println!("欢迎使用Rust计算器 v{}", VERSION);
    
    // 使用直接导入的函数
    println!("\n基本运算:");
    println!("5 + 3 = {}", add(5.0, 3.0));
    println!("5 - 3 = {}", subtract(5.0, 3.0));
    println!("5 * 3 = {}", multiply(5.0, 3.0));
    println!("5 / 3 = {}", divide(5.0, 3.0));
    
    // 使用Calculator结构体
    println!("\n使用计算器对象:");
    let mut calc = Calculator::new();
    println!("10 + 5 = {}", calc.add(10.0, 5.0));
    println!("上次计算结果: {:?}", calc.last_result);
    println!("上次结果 - 7 = {}", calc.subtract(calc.last_result.unwrap(), 7.0));
    
    // 使用统计函数
    println!("\n统计计算:");
    let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    println!("数据: {:?}", data);
    println!("平均值: {:?}", mean(&data));
    println!("中位数: {:?}", median(&data));

    // 直接访问模块中的函数
    println!("\n直接从模块访问:");
    println!("9的平方 = {}", rust_modules_demo::calculator::advanced::square(9.0));
}
