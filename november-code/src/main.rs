//! Rust 高级特性与宏系统演示程序

// 导入库模块
use macro_examples::{
    advanced_traits::*,
    User,
};

// 在模块级别定义枚举（宏需要在顶层展开）
macro_examples::define_status! {
    pub enum HttpStatus {
        Ok = 200,
        NotFound = 404,
        ServerError = 500,
    }
}

fn main() {
    println!("=== Rust 高级特性与宏系统演示 ===\n");

    // 1. 声明式宏示例
    demonstrate_declarative_macros();

    // 2. 高级特性示例
    demonstrate_advanced_traits();

    // 3. 实用宏示例
    demonstrate_utility_macros();

    println!("\n=== 演示完成 ===");
}

/// 演示声明式宏
fn demonstrate_declarative_macros() {
    println!("--- 声明式宏示例 ---");

    // 日志宏
    macro_examples::log!("这是一条信息日志");
    macro_examples::log!(DEBUG, "这是一条调试日志");
    macro_examples::log!(ERROR, "错误代码: {}, 消息: {}", 404, "未找到");

    // 自定义 vec! 宏
    let v1: Vec<i32> = macro_examples::my_vec!();
    let v2 = macro_examples::my_vec!(1, 2, 3, 4, 5);
    let v3 = macro_examples::my_vec!(0; 10);
    println!("空向量: {:?}", v1);
    println!("多元素向量: {:?}", v2);
    println!("重复元素向量: {:?}", v3);

    // 计算宏
    let sum = macro_examples::calculate!(add 1, 2, 3, 4, 5);
    let product = macro_examples::calculate!(mul 2, 3, 4);
    let max_val = macro_examples::calculate!(max 10, 25, 5, 30, 15);
    println!("求和: {}", sum);
    println!("乘积: {}", product);
    println!("最大值: {}", max_val);

    // 创建用户宏
    let user1 = macro_examples::create_user!("Alice");
    let user2 = macro_examples::create_user!("Bob", 25);
    let user3 = macro_examples::create_user!("Charlie", 30, "charlie@example.com");
    println!("用户1: {:?}", user1);
    println!("用户2: {:?}", user2);
    println!("用户3: {:?}", user3);

    // 重复宏
    print!("重复打印: ");
    macro_examples::repeat!(3, {
        print!("Hello ");
    });
    println!();

    // 使用在模块级别定义的枚举
    println!("HTTP 状态: {}", HttpStatus::Ok);
    println!("HTTP 状态: {}", HttpStatus::NotFound);
}

/// 演示高级特性
fn demonstrate_advanced_traits() {
    println!("\n--- 高级特性示例 ---");

    // 关联类型：迭代器
    let mut counter = Counter::new(5);
    print!("计数器迭代: ");
    use macro_examples::advanced_traits::MyIterator;
    while let Some(val) = MyIterator::next(&mut counter) {
        print!("{} ", val);
    }
    println!();

    // 关联类型：图形
    let rect = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 5.0,
    };
    let point = Point2D { x: 5.0, y: 2.5 };
    println!("矩形面积: {}", rect.area());
    println!("点 ({}, {}) 在矩形内: {}", point.x, point.y, rect.contains(&point));

    // 泛型关联类型：集合
    let mut collection = StringCollection::new();
    collection.push("第一个".to_string());
    collection.push("第二个".to_string());
    collection.push("第三个".to_string());
    println!("集合长度: {}", collection.len());
    if let Some(item) = collection.get(1) {
        println!("集合[1]: {}", item);
    }

    // 特性对象
    let circle = Circle { radius: 5.0 };
    let text = Text {
        content: "Hello, Rust!".to_string(),
    };
    let drawables: Vec<Box<dyn Drawable>> = vec![
        Box::new(circle),
        Box::new(text),
    ];
    println!("\n绘制所有对象:");
    draw_all(&drawables);

    // 特性继承
    let readable_text = Text {
        content: "可读文本".to_string(),
    };
    readable_text.draw_with_label("重要");
    println!("读取内容: {}", readable_text.read());

    // 关联常量
    println!("验证值 50: {}", Validator::is_valid(&Validator, 50));
    println!("验证值 150: {}", Validator::is_valid(&Validator, 150));

    // 转换器
    let num: i32 = 42;
    let converted: i64 = num.convert();
    println!("转换 {} -> {}", num, converted);

    // 处理器
    let processor = IntProcessor;
    let result = processor.process(21);
    println!("处理 21 -> {}", result);
}

/// 演示实用宏
fn demonstrate_utility_macros() {
    println!("\n--- 实用宏示例 ---");

    // 使用 serde 宏
    let person = macro_examples::test_data!(person "张三", 28, "zhangsan@example.com");
    match person.to_json() {
        Ok(json_str) => {
            let json: String = json_str;
            println!("Person JSON: {}", json);
            match macro_examples::Person::from_json(&json) {
                Ok(deserialized) => println!("反序列化成功: {:?}", deserialized),
                Err(e) => println!("反序列化失败: {:?}", e),
            }
        }
        Err(e) => println!("序列化失败: {:?}", e),
    }

    // 使用测试数据宏
    let user = macro_examples::test_data!(user "李四", 35);
    println!("测试用户: {:?}", user);

    // 近似相等断言（在测试中使用）
    let pi = 3.14159;
    let approx_pi = 3.1416;
    macro_examples::assert_approx_eq!(pi, approx_pi, 0.0001);
    println!("近似相等断言通过: {} ≈ {}", pi, approx_pi);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let mut counter = Counter::new(3);
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn test_rectangle() {
        let rect = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 5.0,
        };
        assert_eq!(rect.area(), 50.0);
        assert!(rect.contains(&Point2D { x: 5.0, y: 2.5 }));
        assert!(!rect.contains(&Point2D { x: 15.0, y: 2.5 }));
    }

    #[test]
    fn test_my_vec() {
        let v1: Vec<i32> = macro_examples::my_vec!();
        assert_eq!(v1.len(), 0);

        let v2 = macro_examples::my_vec!(1, 2, 3);
        assert_eq!(v2, vec![1, 2, 3]);

        let v3 = macro_examples::my_vec!(42; 5);
        assert_eq!(v3, vec![42, 42, 42, 42, 42]);
    }

    #[test]
    fn test_calculate() {
        assert_eq!(macro_examples::calculate!(add 1, 2, 3), 6);
        assert_eq!(macro_examples::calculate!(mul 2, 3, 4), 24);
        assert_eq!(macro_examples::calculate!(max 10, 25, 5), 25);
    }
}

