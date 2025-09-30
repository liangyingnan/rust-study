//! 异步任务调度器模块
//! 
//! 提供异步任务调度功能，包括：
//! - 周期性任务调度
//! - 一次性任务调度
//! - 任务队列管理
//! - 任务优先级管理

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};

/// 任务优先级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
}

/// 任务状态
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// 任务信息
#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub id: String,
    pub name: String,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub created_at: Instant,
    pub started_at: Option<Instant>,
    pub completed_at: Option<Instant>,
}

/// 异步任务调度器
pub struct AsyncTaskScheduler {
    tasks: Arc<RwLock<Vec<TaskInfo>>>,
    running_tasks: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,
    task_counter: Arc<RwLock<u64>>,
}

impl AsyncTaskScheduler {
    /// 创建新的任务调度器
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(Vec::new())),
            running_tasks: Arc::new(RwLock::new(Vec::new())),
            task_counter: Arc::new(RwLock::new(0)),
        }
    }
    
    /// 添加周期性任务
    pub async fn add_periodic_task<F>(
        &self,
        name: &str,
        interval: Duration,
        task: F,
        priority: TaskPriority,
    ) -> Result<String>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let task_id = self.generate_task_id().await;
        let task_info = TaskInfo {
            id: task_id.clone(),
            name: name.to_string(),
            priority,
            status: TaskStatus::Pending,
            created_at: Instant::now(),
            started_at: None,
            completed_at: None,
        };
        
        // 添加到任务列表
        {
            let mut tasks = self.tasks.write().await;
            tasks.push(task_info);
        }
        
        // 启动任务
        let tasks = Arc::clone(&self.tasks);
        let running_tasks = Arc::clone(&self.running_tasks);
        let task_id_clone = task_id.clone();
        let name = name.to_string();
        
        let handle = tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            
            // 更新任务状态为运行中
            {
                let mut tasks = tasks.write().await;
                if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id_clone) {
                    task.status = TaskStatus::Running;
                    task.started_at = Some(Instant::now());
                }
            }
            
            loop {
                interval_timer.tick().await;
                println!("执行周期性任务: {} (ID: {})", name, task_id_clone);
                task();
            }
        });
        
        // 添加到运行中的任务列表
        {
            let mut running_tasks = running_tasks.write().await;
            running_tasks.push(handle);
        }
        
        Ok(task_id)
    }
    
    /// 添加一次性任务
    pub async fn add_one_time_task<F>(
        &self,
        name: &str,
        delay: Duration,
        task: F,
        priority: TaskPriority,
    ) -> Result<String>
    where
        F: FnOnce() + Send + Sync + 'static,
    {
        let task_id = self.generate_task_id().await;
        let task_info = TaskInfo {
            id: task_id.clone(),
            name: name.to_string(),
            priority,
            status: TaskStatus::Pending,
            created_at: Instant::now(),
            started_at: None,
            completed_at: None,
        };
        
        // 添加到任务列表
        {
            let mut tasks = self.tasks.write().await;
            tasks.push(task_info);
        }
        
        // 启动任务
        let tasks = Arc::clone(&self.tasks);
        let running_tasks = Arc::clone(&self.running_tasks);
        let task_id_clone = task_id.clone();
        let name = name.to_string();
        
        let handle = tokio::spawn(async move {
            tokio::time::sleep(delay).await;
            
            // 更新任务状态为运行中
            {
                let mut tasks = tasks.write().await;
                if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id_clone) {
                    task.status = TaskStatus::Running;
                    task.started_at = Some(Instant::now());
                }
            }
            
            println!("执行一次性任务: {} (ID: {})", name, task_id_clone);
            task();
            
            // 更新任务状态为完成
            {
                let mut tasks = tasks.write().await;
                if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id_clone) {
                    task.status = TaskStatus::Completed;
                    task.completed_at = Some(Instant::now());
                }
            }
        });
        
        // 添加到运行中的任务列表
        {
            let mut running_tasks = running_tasks.write().await;
            running_tasks.push(handle);
        }
        
        Ok(task_id)
    }
    
    /// 获取任务信息
    pub async fn get_task_info(&self, task_id: &str) -> Option<TaskInfo> {
        let tasks = self.tasks.read().await;
        tasks.iter().find(|t| t.id == task_id).cloned()
    }
    
    /// 获取所有任务信息
    pub async fn get_all_tasks(&self) -> Vec<TaskInfo> {
        let tasks = self.tasks.read().await;
        tasks.clone()
    }
    
    /// 按优先级获取任务
    pub async fn get_tasks_by_priority(&self, priority: TaskPriority) -> Vec<TaskInfo> {
        let tasks = self.tasks.read().await;
        tasks
            .iter()
            .filter(|t| t.priority == priority)
            .cloned()
            .collect()
    }
    
    /// 获取运行中的任务数量
    pub async fn get_running_task_count(&self) -> usize {
        let tasks = self.tasks.read().await;
        tasks.iter().filter(|t| t.status == TaskStatus::Running).count()
    }
    
    /// 等待所有任务完成
    pub async fn wait_for_all(&self) {
        let running_tasks = self.running_tasks.read().await;
        for handle in running_tasks.iter() {
            let _ = handle.await;
        }
    }
    
    /// 生成任务ID
    async fn generate_task_id(&self) -> String {
        let mut counter = self.task_counter.write().await;
        *counter += 1;
        format!("task_{}", *counter)
    }
}

impl Default for AsyncTaskScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// 任务队列管理器
pub struct TaskQueue {
    queue: Arc<RwLock<Vec<TaskInfo>>>,
    max_size: Option<usize>,
}

impl TaskQueue {
    /// 创建新的任务队列
    pub fn new() -> Self {
        Self {
            queue: Arc::new(RwLock::new(Vec::new())),
            max_size: None,
        }
    }
    
    /// 创建带最大大小的任务队列
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            queue: Arc::new(RwLock::new(Vec::new())),
            max_size: Some(max_size),
        }
    }
    
    /// 添加任务到队列
    pub async fn enqueue(&self, task: TaskInfo) -> Result<()> {
        let mut queue = self.queue.write().await;
        
        if let Some(max_size) = self.max_size {
            if queue.len() >= max_size {
                return Err(anyhow::anyhow!("任务队列已满"));
            }
        }
        
        queue.push(task);
        Ok(())
    }
    
    /// 从队列中取出任务
    pub async fn dequeue(&self) -> Option<TaskInfo> {
        let mut queue = self.queue.write().await;
        queue.pop()
    }
    
    /// 按优先级获取下一个任务
    pub async fn dequeue_by_priority(&self) -> Option<TaskInfo> {
        let mut queue = self.queue.write().await;
        
        if queue.is_empty() {
            return None;
        }
        
        // 按优先级排序，优先级高的先执行
        queue.sort_by(|a, b| b.priority.cmp(&a.priority));
        queue.pop()
    }
    
    /// 获取队列大小
    pub async fn size(&self) -> usize {
        let queue = self.queue.read().await;
        queue.len()
    }
    
    /// 检查队列是否为空
    pub async fn is_empty(&self) -> bool {
        let queue = self.queue.read().await;
        queue.is_empty()
    }
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_task_scheduler_creation() {
        let scheduler = AsyncTaskScheduler::new();
        assert_eq!(scheduler.get_running_task_count().await, 0);
    }
    
    #[tokio::test]
    async fn test_task_queue_operations() {
        let queue = TaskQueue::new();
        assert!(queue.is_empty().await);
        
        let task = TaskInfo {
            id: "test_task".to_string(),
            name: "测试任务".to_string(),
            priority: TaskPriority::Normal,
            status: TaskStatus::Pending,
            created_at: Instant::now(),
            started_at: None,
            completed_at: None,
        };
        
        assert!(queue.enqueue(task).await.is_ok());
        assert!(!queue.is_empty().await);
        assert_eq!(queue.size().await, 1);
        
        let dequeued = queue.dequeue().await;
        assert!(dequeued.is_some());
        assert!(queue.is_empty().await);
    }
}
