//! 高级特性（Traits）和关联类型示例

/// 关联类型示例：自定义迭代器特性
pub trait MyIterator {
    type Item;  // 关联类型

    fn next(&mut self) -> Option<Self::Item>;
}

/// 实现一个简单的计数器迭代器
pub struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    pub fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl MyIterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

/// 关联类型示例：图形特性
pub trait Shape {
    type Point;  // 关联类型：点
    type Area;   // 关联类型：面积类型

    fn area(&self) -> Self::Area;
    fn contains(&self, point: &Self::Point) -> bool;
}

/// 矩形实现
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Shape for Rectangle {
    type Point = Point2D;
    type Area = f64;

    fn area(&self) -> Self::Area {
        self.width * self.height
    }

    fn contains(&self, point: &Self::Point) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
}

/// 泛型关联类型（GAT）示例：集合特性
pub trait Collection {
    type Item<'a>
    where
        Self: 'a;

    fn get(&self, index: usize) -> Option<Self::Item<'_>>;
    fn len(&self) -> usize;
}

// 注意：GAT 需要 Rust 1.65+ 版本

/// 字符串集合实现
pub struct StringCollection {
    items: Vec<String>,
}

impl StringCollection {
    pub fn new() -> Self {
        StringCollection {
            items: Vec::new(),
        }
    }

    pub fn push(&mut self, item: String) {
        self.items.push(item);
    }
}

impl Collection for StringCollection {
    type Item<'a> = &'a str;

    fn get(&self, index: usize) -> Option<Self::Item<'_>> {
        self.items.get(index).map(|s| s.as_str())
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

/// 高级特性：默认实现和特性边界
pub trait Drawable {
    fn draw(&self);
    
    // 默认实现
    fn draw_with_label(&self, label: &str) {
        println!("标签: {}", label);
        self.draw();
    }
}

pub struct Circle {
    pub radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("绘制圆形，半径: {}", self.radius);
    }
}

/// 特性对象示例
pub fn draw_all(items: &[Box<dyn Drawable>]) {
    for item in items {
        item.draw();
    }
}

/// 高级特性：特性继承
pub trait Readable: Drawable {
    fn read(&self) -> String;
}

pub struct Text {
    pub content: String,
}

impl Drawable for Text {
    fn draw(&self) {
        println!("显示文本: {}", self.content);
    }
}

impl Readable for Text {
    fn read(&self) -> String {
        self.content.clone()
    }
}

/// 高级特性：关联常量
pub trait Constants {
    const MAX_VALUE: u32;
    const MIN_VALUE: u32;
    
    fn is_valid(&self, value: u32) -> bool {
        value >= Self::MIN_VALUE && value <= Self::MAX_VALUE
    }
}

pub struct Validator;

impl Constants for Validator {
    const MAX_VALUE: u32 = 100;
    const MIN_VALUE: u32 = 0;
}

/// 高级特性：特性方法中的泛型
pub trait Converter {
    fn convert<T: From<Self>>(self) -> T
    where
        Self: Sized;
}

impl Converter for i32 {
    fn convert<T: From<Self>>(self) -> T {
        T::from(self)
    }
}

/// 高级特性：条件实现（使用 where 子句）
pub trait Processor {
    type Input;
    type Output;

    fn process(&self, input: Self::Input) -> Self::Output;
}

pub struct IntProcessor;

impl Processor for IntProcessor {
    type Input = i32;
    type Output = i64;

    fn process(&self, input: Self::Input) -> Self::Output {
        input as i64 * 2
    }
}

/// 高级特性：特性组合
pub trait Cloneable: Clone {
    fn duplicate(&self) -> Self
    where
        Self: Sized,
    {
        self.clone()
    }
}

impl<T: Clone> Cloneable for T {}

