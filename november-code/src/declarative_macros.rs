//! 声明式宏（macro_rules!）示例

/// 创建一个简单的日志宏，可以记录不同级别的日志
#[macro_export]
macro_rules! log {
    // 无级别，默认为 INFO
    ($msg:expr) => {
        println!("[INFO] {}", $msg);
    };
    // 带级别
    ($level:ident, $msg:expr) => {
        println!("[{}] {}", stringify!($level), $msg);
    };
    // 带格式化的消息
    ($level:ident, $($arg:tt)*) => {
        println!("[{}] {}", stringify!($level), format!($($arg)*));
    };
}

/// 创建一个 vec! 的简化版本，支持多种初始化方式
#[macro_export]
macro_rules! my_vec {
    // 空向量
    () => {
        Vec::new()
    };
    // 单个元素重复 n 次
    ($elem:expr; $n:expr) => {
        {
            let mut v = Vec::with_capacity($n);
            for _ in 0..$n {
                v.push($elem.clone());
            }
            v
        }
    };
    // 多个元素
    ($($x:expr),+ $(,)?) => {
        {
            let mut v = Vec::new();
            $(v.push($x);)+
            v
        }
    };
}

/// 创建一个计算表达式的宏，支持多种运算符
#[macro_export]
macro_rules! calculate {
    // 加法
    (add $($x:expr),+ $(,)?) => {
        {
            let mut sum = 0;
            $(sum += $x;)+
            sum
        }
    };
    // 乘法
    (mul $($x:expr),+ $(,)?) => {
        {
            let mut product = 1;
            $(product *= $x;)+
            product
        }
    };
    // 最大值
    (max $first:expr $(, $x:expr)+ $(,)?) => {
        {
            let mut max_val = $first;
            $(
                if $x > max_val {
                    max_val = $x;
                }
            )+
            max_val
        }
    };
}

/// 创建一个用于创建结构体实例的宏，自动处理默认值
#[macro_export]
macro_rules! create_user {
    // 只有必需字段
    ($name:expr) => {
        User {
            name: $name.to_string(),
            age: 0,
            email: String::new(),
        }
    };
    // 包含年龄
    ($name:expr, $age:expr) => {
        User {
            name: $name.to_string(),
            age: $age,
            email: String::new(),
        }
    };
    // 包含所有字段
    ($name:expr, $age:expr, $email:expr) => {
        User {
            name: $name.to_string(),
            age: $age,
            email: $email.to_string(),
        }
    };
}

/// 用户结构体（用于宏示例）
#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub email: String,
}

/// 创建一个用于模式匹配的宏，简化 match 表达式
#[macro_export]
macro_rules! match_result {
    // 成功情况
    (ok $expr:expr => $body:block) => {
        match $expr {
            Ok(val) => $body,
            Err(e) => {
                eprintln!("错误: {:?}", e);
                return;
            }
        }
    };
    // 成功和错误都处理
    (ok $expr:expr => $ok_body:block, err => $err_body:block) => {
        match $expr {
            Ok(val) => $ok_body,
            Err(e) => $err_body,
        }
    };
}

/// 创建一个用于测试断言的宏
#[macro_export]
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr, $epsilon:expr) => {
        {
            let left_val: f64 = $left as f64;
            let right_val: f64 = $right as f64;
            let diff = (left_val - right_val).abs();
            if diff > $epsilon as f64 {
                panic!(
                    "断言失败: {} 和 {} 的差值 {} 超过了允许的误差 {}",
                    left_val, right_val, diff, $epsilon
                );
            }
        }
    };
    ($left:expr, $right:expr) => {
        assert_approx_eq!($left, $right, 0.0001);
    };
}

/// 创建一个用于重复代码块的宏
#[macro_export]
macro_rules! repeat {
    ($n:expr, $body:block) => {
        {
            for _ in 0..$n {
                $body
            }
        }
    };
}

/// 创建一个用于定义枚举变体的宏，自动实现 Display trait
#[macro_export]
macro_rules! define_status {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident = $val:expr),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant = $val),+,
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        $name::$variant => write!(f, "{}", $val),
                    )+
                }
            }
        }
    };
}

