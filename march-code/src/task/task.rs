use std::fmt;
use chrono::{DateTime, Utc};

/// 任务状态枚举
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Todo,       // 待办
    InProgress, // 进行中
    Done,       // 已完成
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Todo => write!(f, "待办"),
            TaskStatus::InProgress => write!(f, "进行中"),
            TaskStatus::Done => write!(f, "已完成"),
        }
    }
}

/// 任务结构体
#[derive(Debug, Clone)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    /// 创建新任务
    pub fn new(title: String, description: String) -> Self {
        let now = Utc::now();
        Task {
            title,
            description,
            status: TaskStatus::Todo, // 默认为待办状态
            created_at: now,
            updated_at: now,
        }
    }

    /// 更新任务状态
    pub fn update_status(&mut self, status: TaskStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    /// 任务详情显示
    pub fn display_details(&self) {
        println!("任务详情：");
        println!("标题: {}", self.title);
        println!("描述: {}", self.description);
        println!("状态: {}", self.status);
        println!("创建时间: {}", self.created_at.format("%Y-%m-%d %H:%M:%S"));
        println!("更新时间: {}", self.updated_at.format("%Y-%m-%d %H:%M:%S"));
    }
}