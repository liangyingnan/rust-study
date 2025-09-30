use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};

/// 模拟的异步数据库连接
#[derive(Debug, Clone)]
pub struct AsyncDatabase {
    data: Arc<RwLock<HashMap<String, User>>>,
    connection_pool: Arc<RwLock<Vec<Connection>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
struct Connection {
    id: String,
    created_at: Instant,
    is_active: bool,
}

impl AsyncDatabase {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            connection_pool: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// 异步获取数据库连接
    pub async fn get_connection(&self) -> Result<DatabaseConnection> {
        let mut pool = self.connection_pool.write().await;
        
        // 查找可用连接
        for conn in pool.iter_mut() {
            if conn.is_active {
                conn.is_active = false; // 标记为使用中
                return Ok(DatabaseConnection {
                    id: conn.id.clone(),
                    database: self.clone(),
                });
            }
        }
        
        // 创建新连接
        let conn_id = format!("conn_{}", pool.len() + 1);
        let connection = Connection {
            id: conn_id.clone(),
            created_at: Instant::now(),
            is_active: false,
        };
        
        pool.push(connection);
        
        Ok(DatabaseConnection {
            id: conn_id,
            database: self.clone(),
        })
    }
    
    /// 释放连接
    async fn release_connection(&self, conn_id: &str) {
        let mut pool = self.connection_pool.write().await;
        for conn in pool.iter_mut() {
            if conn.id == conn_id {
                conn.is_active = true;
                break;
            }
        }
    }
    
    /// 异步查询用户
    pub async fn find_user(&self, id: &str) -> Result<Option<User>> {
        let data = self.data.read().await;
        Ok(data.get(id).cloned())
    }
    
    /// 异步创建用户
    pub async fn create_user(&self, user: User) -> Result<()> {
        let mut data = self.data.write().await;
        data.insert(user.id.clone(), user);
        Ok(())
    }
    
    /// 异步更新用户
    pub async fn update_user(&self, user: User) -> Result<()> {
        let mut data = self.data.write().await;
        if data.contains_key(&user.id) {
            data.insert(user.id.clone(), user);
            Ok(())
        } else {
            Err(anyhow::anyhow!("用户不存在"))
        }
    }
    
    /// 异步删除用户
    pub async fn delete_user(&self, id: &str) -> Result<()> {
        let mut data = self.data.write().await;
        data.remove(id);
        Ok(())
    }
    
    /// 异步批量操作
    pub async fn batch_operations(&self, operations: Vec<DatabaseOperation>) -> Result<Vec<Result<()>>> {
        let mut results = Vec::new();
        
        for op in operations {
            let result = match op {
                DatabaseOperation::Create(user) => self.create_user(user).await,
                DatabaseOperation::Update(user) => self.update_user(user).await,
                DatabaseOperation::Delete(id) => self.delete_user(&id).await,
            };
            results.push(result);
        }
        
        Ok(results)
    }
    
    /// 异步事务处理
    pub async fn transaction<F, Fut, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut Transaction) -> Fut,
        Fut: std::future::Future<Output = Result<R>> + Send,
    {
        let mut tx = Transaction::new(self.clone());
        let result = f(&mut tx).await?;
        tx.commit().await?;
        Ok(result)
    }
}

#[derive(Debug)]
pub enum DatabaseOperation {
    Create(User),
    Update(User),
    Delete(String),
}

/// 数据库连接包装器
pub struct DatabaseConnection {
    id: String,
    database: AsyncDatabase,
}

impl DatabaseConnection {
    /// 异步执行查询
    pub async fn query(&self, sql: &str) -> Result<Vec<User>> {
        // 模拟查询延迟
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        let data = self.database.data.read().await;
        let users: Vec<User> = data.values().cloned().collect();
        
        println!("连接 {} 执行查询: {}", self.id, sql);
        Ok(users)
    }
    
    /// 异步执行更新
    pub async fn execute(&self, sql: &str) -> Result<u64> {
        // 模拟执行延迟
        tokio::time::sleep(Duration::from_millis(5)).await;
        
        println!("连接 {} 执行更新: {}", self.id, sql);
        Ok(1)
    }
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        let database = self.database.clone();
        let conn_id = self.id.clone();
        tokio::spawn(async move {
            database.release_connection(&conn_id).await;
        });
    }
}

/// 数据库事务
pub struct Transaction {
    database: AsyncDatabase,
    operations: Vec<DatabaseOperation>,
}

impl Transaction {
    fn new(database: AsyncDatabase) -> Self {
        Self {
            database,
            operations: Vec::new(),
        }
    }
    
    /// 添加操作到事务
    pub fn add_operation(&mut self, operation: DatabaseOperation) {
        self.operations.push(operation);
    }
    
    /// 提交事务
    pub async fn commit(self) -> Result<()> {
        println!("提交事务，包含 {} 个操作", self.operations.len());
        
        for op in self.operations {
            match op {
                DatabaseOperation::Create(user) => {
                    self.database.create_user(user).await?;
                }
                DatabaseOperation::Update(user) => {
                    self.database.update_user(user).await?;
                }
                DatabaseOperation::Delete(id) => {
                    self.database.delete_user(&id).await?;
                }
            }
        }
        
        Ok(())
    }
}

/// 异步数据库连接池
pub struct ConnectionPool {
    database: AsyncDatabase,
    max_connections: usize,
    connections: Arc<RwLock<Vec<DatabaseConnection>>>,
}

impl ConnectionPool {
    pub fn new(database: AsyncDatabase, max_connections: usize) -> Self {
        Self {
            database,
            max_connections,
            connections: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// 异步获取连接
    pub async fn get_connection(&self) -> Result<PooledConnection> {
        // 尝试从池中获取现有连接
        {
            let mut connections = self.connections.write().await;
            if let Some(conn) = connections.pop() {
                return Ok(PooledConnection {
                    connection: Some(conn),
                    pool: self.clone(),
                });
            }
        }
        
        // 创建新连接
        let connection = self.database.get_connection().await?;
        Ok(PooledConnection {
            connection: Some(connection),
            pool: self.clone(),
        })
    }
    
    /// 归还连接到池
    async fn return_connection(&self, connection: DatabaseConnection) {
        let mut connections = self.connections.write().await;
        if connections.len() < self.max_connections {
            connections.push(connection);
        }
    }
}

impl Clone for ConnectionPool {
    fn clone(&self) -> Self {
        Self {
            database: self.database.clone(),
            max_connections: self.max_connections,
            connections: Arc::clone(&self.connections),
        }
    }
}

/// 池化连接
pub struct PooledConnection {
    connection: Option<DatabaseConnection>,
    pool: ConnectionPool,
}

impl PooledConnection {
    /// 获取连接引用
    pub fn connection(&self) -> &DatabaseConnection {
        self.connection.as_ref().unwrap()
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        if let Some(connection) = self.connection.take() {
            let pool = self.pool.clone();
            tokio::spawn(async move {
                pool.return_connection(connection).await;
            });
        }
    }
}

/// 异步数据库操作示例
pub async fn database_operations_example() -> Result<()> {
    println!("\n=== 异步数据库操作示例 ===");
    
    let db = AsyncDatabase::new();
    
    // 创建用户
    let user1 = User {
        id: "1".to_string(),
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
        created_at: 1234567890,
    };
    
    let user2 = User {
        id: "2".to_string(),
        name: "李四".to_string(),
        email: "lisi@example.com".to_string(),
        created_at: 1234567891,
    };
    
    // 异步创建用户
    db.create_user(user1.clone()).await?;
    db.create_user(user2.clone()).await?;
    println!("用户创建完成");
    
    // 异步查询用户
    let found_user = db.find_user("1").await?;
    println!("查询到用户: {:?}", found_user);
    
    // 异步更新用户
    let mut updated_user = user1.clone();
    updated_user.name = "张三（已更新）".to_string();
    db.update_user(updated_user).await?;
    println!("用户更新完成");
    
    // 异步事务
    db.transaction(|tx| {
        async move {
            tx.add_operation(DatabaseOperation::Create(User {
                id: "3".to_string(),
                name: "王五".to_string(),
                email: "wangwu@example.com".to_string(),
                created_at: 1234567892,
            }));
            
            tx.add_operation(DatabaseOperation::Update(User {
                id: "2".to_string(),
                name: "李四（事务更新）".to_string(),
                email: "lisi@example.com".to_string(),
                created_at: 1234567891,
            }));
            
            Ok(())
        }
    }).await?;
    
    println!("事务执行完成");
    
    // 连接池示例
    let pool = ConnectionPool::new(db.clone(), 5);
    
    // 并发使用连接
    let mut handles = Vec::new();
    for i in 0..3 {
        let pool_clone = pool.clone();
        let handle = tokio::spawn(async move {
            let conn = pool_clone.get_connection().await.unwrap();
            let users = conn.connection().query(&format!("SELECT * FROM users WHERE id = {}", i + 1)).await.unwrap();
            println!("连接 {} 查询结果: {} 个用户", i + 1, users.len());
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_database_operations() {
        let db = AsyncDatabase::new();
        
        let user = User {
            id: "test".to_string(),
            name: "测试用户".to_string(),
            email: "test@example.com".to_string(),
            created_at: 1234567890,
        };
        
        // 测试创建
        assert!(db.create_user(user.clone()).await.is_ok());
        
        // 测试查询
        let found = db.find_user("test").await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "测试用户");
        
        // 测试更新
        let mut updated = user.clone();
        updated.name = "更新的用户".to_string();
        assert!(db.update_user(updated).await.is_ok());
        
        // 测试删除
        assert!(db.delete_user("test").await.is_ok());
        let found = db.find_user("test").await.unwrap();
        assert!(found.is_none());
    }
}
