//! 性能优化示例项目
//! 
//! 本项目演示了如何优化Rust代码的性能，包括：
//! - 内存分配优化
//! - 算法优化
//! - CPU使用优化

/// 优化前的版本：处理数据并计算统计信息
pub mod unoptimized {
    use std::collections::HashMap;

    /// 计算数据集的平均值（未优化版本）
    /// 
    /// 问题：
    /// - 多次遍历数据
    /// - 不必要的类型转换
    pub fn calculate_average(numbers: &Vec<i32>) -> f64 {
        let mut sum = 0;
        for num in numbers {
            sum += *num;
        }
        sum as f64 / numbers.len() as f64
    }

    /// 查找出现频率最高的数字（未优化版本）
    /// 
    /// 问题：
    /// - 多次遍历数据
    /// - 使用HashMap但可以优化
    pub fn find_most_frequent(numbers: &Vec<i32>) -> i32 {
        let mut frequency: HashMap<i32, usize> = HashMap::new();
        
        // 第一次遍历：统计频率
        for &num in numbers {
            *frequency.entry(num).or_insert(0) += 1;
        }
        
        // 第二次遍历：查找最大值
        let mut max_count = 0;
        let mut most_frequent = 0;
        for (&num, &count) in &frequency {
            if count > max_count {
                max_count = count;
                most_frequent = num;
            }
        }
        
        most_frequent
    }

    /// 过滤并转换数据（未优化版本）
    /// 
    /// 问题：
    /// - 多次分配Vec
    /// - 多次遍历
    pub fn filter_and_transform(numbers: &Vec<i32>) -> Vec<i32> {
        // 第一次遍历：过滤
        let filtered: Vec<i32> = numbers.iter()
            .filter(|&&x| x > 0)
            .cloned()
            .collect();
        
        // 第二次遍历：转换
        let transformed: Vec<i32> = filtered.iter()
            .map(|&x| x * 2)
            .collect();
        
        transformed
    }

    /// 处理大量数据（未优化版本）
    /// 
    /// 问题：
    /// - 在循环中创建String
    /// - 使用format!宏（分配较多）
    pub fn process_strings(data: &[i32]) -> Vec<String> {
        let mut result = Vec::new();
        for &value in data {
            result.push(format!("Value: {}", value));
        }
        result
    }
}

/// 优化后的版本：性能优化实践
pub mod optimized {
    use std::collections::HashMap;

    /// 计算数据集的平均值（优化版本）
    /// 
    /// 优化点：
    /// - 单次遍历
    /// - 使用更高效的类型
    pub fn calculate_average(numbers: &[i32]) -> f64 {
        if numbers.is_empty() {
            return 0.0;
        }
        
        let sum: i64 = numbers.iter().map(|&x| x as i64).sum();
        sum as f64 / numbers.len() as f64
    }

    /// 查找出现频率最高的数字（优化版本）
    /// 
    /// 优化点：
    /// - 单次遍历完成统计和查找
    /// - 使用更高效的数据结构访问
    pub fn find_most_frequent(numbers: &[i32]) -> i32 {
        if numbers.is_empty() {
            return 0;
        }
        
        let mut frequency: HashMap<i32, usize> = HashMap::with_capacity(numbers.len() / 2);
        let mut max_count = 0;
        let mut most_frequent = numbers[0];
        
        // 单次遍历：统计频率并跟踪最大值
        for &num in numbers {
            let count = frequency.entry(num).and_modify(|c| *c += 1).or_insert(1);
            if *count > max_count {
                max_count = *count;
                most_frequent = num;
            }
        }
        
        most_frequent
    }

    /// 过滤并转换数据（优化版本）
    /// 
    /// 优化点：
    /// - 单次遍历完成过滤和转换
    /// - 预分配Vec容量
    /// - 避免不必要的克隆
    pub fn filter_and_transform(numbers: &[i32]) -> Vec<i32> {
        let capacity = numbers.len() / 2; // 预估容量
        let mut result = Vec::with_capacity(capacity);
        
        // 单次遍历：同时过滤和转换
        for &x in numbers {
            if x > 0 {
                result.push(x * 2);
            }
        }
        
        result
    }

    /// 处理大量数据（优化版本）
    /// 
    /// 优化点：
    /// - 预分配Vec容量
    /// - 使用String::with_capacity预分配字符串容量
    /// - 避免format!宏，使用更高效的方式
    pub fn process_strings(data: &[i32]) -> Vec<String> {
        let mut result = Vec::with_capacity(data.len());
        for &value in data {
            let mut s = String::with_capacity(15); // 预估容量 "Value: 1234567"
            s.push_str("Value: ");
            s.push_str(&value.to_string());
            result.push(s);
        }
        result
    }

    /// 并行处理数据（使用rayon，需要添加依赖）
    /// 
    /// 注意：此函数需要添加 rayon = "1.8" 到 Cargo.toml
    /// 这里仅作为示例，实际使用时取消注释并添加依赖
    /*
    use rayon::prelude::*;
    
    pub fn parallel_filter_and_transform(numbers: &[i32]) -> Vec<i32> {
        numbers.par_iter()
            .filter(|&&x| x > 0)
            .map(|&x| x * 2)
            .collect()
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average() {
        let data = vec![1, 2, 3, 4, 5];
        assert_eq!(unoptimized::calculate_average(&data), 3.0);
        assert_eq!(optimized::calculate_average(&data), 3.0);
    }

    #[test]
    fn test_most_frequent() {
        let data = vec![1, 2, 2, 3, 3, 3, 4];
        assert_eq!(unoptimized::find_most_frequent(&data), 3);
        assert_eq!(optimized::find_most_frequent(&data), 3);
    }

    #[test]
    fn test_filter_and_transform() {
        let data = vec![-1, 2, -3, 4, 5];
        let unopt = unoptimized::filter_and_transform(&data);
        let opt = optimized::filter_and_transform(&data);
        assert_eq!(unopt, opt);
        assert_eq!(opt, vec![4, 8, 10]);
    }
}

