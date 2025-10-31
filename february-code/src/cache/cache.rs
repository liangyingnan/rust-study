/// 缓存数据结构
/// 
/// 负责存储和管理文本数据，展示所有权和借用概念
#[derive(Debug)]
pub struct Cache {
    data: String,
}

impl Cache {
    /// 创建新缓存，获取数据所有权
    pub fn new(data: String) -> Self {
        Self { data }
    }

    /// 返回数据的不可变引用
    pub fn get_data(&self) -> &str {
        &self.data
    }

    /// 更新数据，需要可变借用
    pub fn update_data(&mut self, new_data: String) {
        self.data = new_data;
    }
    
    /// 追加数据到现有内容，需要可变借用
    pub fn append_data(&mut self, additional_data: &str) {
        self.data.push_str(additional_data);
    }
    
    /// 清空缓存数据，需要可变借用
    pub fn clear(&mut self) {
        self.data.clear();
    }
    
    /// 检查数据是否为空
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// 获取数据长度
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Drop for Cache {
    fn drop(&mut self) {
        // 演示资源清理，实际应用中可能会有更复杂的操作
        println!("正在清理缓存资源，长度为 {} 字节的数据将被释放", self.len());
    }
}