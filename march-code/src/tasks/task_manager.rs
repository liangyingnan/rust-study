use crate::models::task::{Task, TaskStatus};
use std::collections::HashMap;

/// 任务管理器
pub struct TaskManager {
    tasks: HashMap<usize, Task>,
    next_id: usize,
}

impl TaskManager {
    /// 创建新的任务管理器
    pub fn new() -> Self {
        TaskManager {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    /// 添加任务
    pub fn add_task(&mut self, task: Task) -> usize {
        let id = self.next_id;
        self.tasks.insert(id, task);
        self.next_id += 1;
        id
    }

    /// 列出所有任务
    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("没有任务记录");
            return;
        }

        println!("任务列表：");
        println!("{:<5} {:<20} {:<10}", "ID", "标题", "状态");
        println!("{:-<5} {:-<20} {:-<10}", "", "", "");

        // 将任务按ID排序
        let mut sorted_tasks: Vec<(&usize, &Task)> = self.tasks.iter().collect();
        sorted_tasks.sort_by_key(|&(id, _)| id);

        for (id, task) in sorted_tasks {
            println!("{:<5} {:<20} {:<10}", id, task.title, task.status);
        }
    }

    /// 更新任务状态
    pub fn update_task_status(&mut self, id: usize, status: TaskStatus) -> bool {
        match self.tasks.get_mut(&id) {
            Some(task) => {
                task.update_status(status);
                true
            }
            None => false,
        }
    }

    /// 删除任务
    pub fn delete_task(&mut self, id: usize) -> bool {
        self.tasks.remove(&id).is_some()
    }

    /// 查看任务详情
    pub fn view_task(&self, id: usize) {
        match self.tasks.get(&id) {
            Some(task) => {
                println!("ID: {}", id);
                task.display_details();
            }
            None => println!("找不到ID为{}的任务", id),
        }
    }

    /// 获取任务总数
    pub fn count(&self) -> usize {
        self.tasks.len()
    }
} 