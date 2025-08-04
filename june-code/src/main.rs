fn main() {
    println!("Rust闭包与迭代器示例程序");
    
    // 1. 闭包基础示例
    println!("\n1. 闭包基础");
    // 定义一个简单闭包
    let add_one = |x| x + 1;
    println!("闭包结果: {}", add_one(5));
    
    // 带类型标注的闭包
    let add_two: fn(i32) -> i32 = |x| x + 2;
    println!("带类型标注的闭包结果: {}", add_two(5));
    
    // 捕获环境变量的闭包
    let x = 4;
    let equal_to_x = |z| z == x;
    println!("闭包捕获环境变量: {}", equal_to_x(4));
    
    // 2. 使用闭包作为参数
    println!("\n2. 闭包作为函数参数");
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 使用闭包求和
    let sum = calculate_with_closure(&numbers, |acc, &item| acc + item);
    println!("使用闭包计算和: {}", sum);
    
    // 使用闭包求积
    let product = calculate_with_closure(&numbers, |acc, &item| acc * item);
    println!("使用闭包计算积: {}", product);
    
    // 3. 迭代器基础
    println!("\n3. 迭代器基础");
    let v = vec![1, 2, 3, 4, 5];
    
    // 基本迭代
    print!("基本迭代: ");
    for i in &v {
        print!("{} ", i);
    }
    println!();
    
    // 迭代器适配器
    println!("\n4. 迭代器适配器");
    // map - 对每个元素应用函数
    let squared: Vec<i32> = v.iter().map(|x| x * x).collect();
    println!("map示例 (平方): {:?}", squared);
    
    // filter - 过滤元素
    let even: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
    println!("filter示例 (偶数): {:?}", even);
    
    // 链式调用 - 先过滤再映射
    let even_squared: Vec<i32> = v.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("链式调用 (偶数平方): {:?}", even_squared);
    
    // 5. 闭包和迭代器结合的实际应用
    println!("\n5. 实际应用示例");
    
    // 创建一个商品列表
    let products = vec![
        Product { name: "手机".to_string(), price: 2999, in_stock: true },
        Product { name: "笔记本".to_string(), price: 5999, in_stock: true },
        Product { name: "耳机".to_string(), price: 999, in_stock: false },
        Product { name: "平板".to_string(), price: 3999, in_stock: true },
    ];
    
    // 查找有货商品
    let in_stock_products: Vec<&Product> = products.iter()
        .filter(|product| product.in_stock)
        .collect();
    println!("有货商品数量: {}", in_stock_products.len());
    
    // 计算有货商品的总价值
    let total_value: i32 = products.iter()
        .filter(|p| p.in_stock)
        .map(|p| p.price)
        .sum();
    println!("有货商品总价值: {}元", total_value);
    
    // 找出价格最高的商品
    if let Some(most_expensive) = products.iter().max_by_key(|p| p.price) {
        println!("最贵商品: {}, 价格: {}元", most_expensive.name, most_expensive.price);
    }
    
    // 自定义排序 - 按价格从高到低
    let mut sorted_products = products.clone();
    sorted_products.sort_by(|a, b| b.price.cmp(&a.price));
    
    println!("商品按价格排序:");
    for product in sorted_products {
        println!("  {} - {}元 - {}", 
            product.name, 
            product.price, 
            if product.in_stock { "有货" } else { "无货" }
        );
    }
}

// 用于闭包示例的函数
fn calculate_with_closure<F>(numbers: &[i32], closure: F) -> i32 
where
    F: Fn(i32, &i32) -> i32
{
    let mut result = if numbers.is_empty() { 
        return 0 
    } else if closure(0, &0) == 0 { 
        // 对于乘法运算，初始值设为1
        1 
    } else { 
        // 对于加法等其他运算，初始值设为0
        0 
    };
    
    for number in numbers {
        result = closure(result, number);
    }
    result
}

// 定义一个商品结构体用于实际应用示例
#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: i32,
    in_stock: bool,
}