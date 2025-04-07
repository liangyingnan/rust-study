/// 文本处理上下文
/// 
/// 提供对文本内容的分析功能，通过借用文本数据
#[derive(Debug)]
pub struct TextContext<'a> {
    // 存储对文本的引用，使用生命周期参数确保引用有效
    content: &'a str,
}

impl<'a> TextContext<'a> {
    /// 创建新的文本上下文，借用文本数据
    pub fn new(content: &'a str) -> Self {
        Self { content }
    }
    
    /// 统计文本中的单词数量
    pub fn count_words(&self) -> usize {
        self.content.split_whitespace().count()
    }
    
    /// 寻找最长的单词
    pub fn longest_word(&self) -> &str {
        self.content
            .split_whitespace()
            .max_by_key(|word| word.len())
            .unwrap_or("")
    }
    
    /// 查找指定单词在文本中的位置
    pub fn find_word(&self, word: &str) -> Option<usize> {
        self.content
            .split_whitespace()
            .position(|w| w == word)
    }
    
    /// 获取文本内容
    pub fn get_content(&self) -> &str {
        self.content
    }
    
    /// 检查文本是否包含特定单词
    pub fn contains_word(&self, word: &str) -> bool {
        self.content
            .split_whitespace()
            .any(|w| w == word)
    }
    
    /// 统计特定单词在文本中出现的次数
    pub fn count_word_occurrences(&self, word: &str) -> usize {
        self.content
            .split_whitespace()
            .filter(|&w| w == word)
            .count()
    }
}