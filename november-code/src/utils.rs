//! 实用工具模块

use serde::{Deserialize, Serialize};

/// 使用 serde 的派生宏进行序列化/反序列化
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: String,
}

impl Person {
    pub fn new(name: &str, age: u32, email: &str) -> Self {
        Person {
            name: name.to_string(),
            email: email.to_string(),
            age,
        }
    }

    /// 将 Person 序列化为 JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// 从 JSON 反序列化 Person
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// 使用宏简化错误处理
#[macro_export]
macro_rules! try_or_return {
    ($expr:expr, $err_msg:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                eprintln!("{}: {:?}", $err_msg, e);
                return;
            }
        }
    };
}

/// 使用宏简化 Option 处理
#[macro_export]
macro_rules! unwrap_or_return {
    ($expr:expr, $err_msg:expr) => {
        match $expr {
            Some(val) => val,
            None => {
                eprintln!("{}", $err_msg);
                return;
            }
        }
    };
}

/// 使用宏创建测试数据
#[macro_export]
macro_rules! test_data {
    (person $name:expr, $age:expr, $email:expr) => {
        $crate::Person::new($name, $age, $email)
    };
    (user $name:expr, $age:expr) => {
        $crate::User {
            name: $name.to_string(),
            age: $age,
            email: String::new(),
        }
    };
}

