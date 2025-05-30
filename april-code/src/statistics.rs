// 统计模块 - 包含基本统计计算功能

// 计算平均值
pub fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    
    let sum: f64 = values.iter().sum();
    Some(sum / values.len() as f64)
}

// 计算中位数
pub fn median(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    
    // 创建可变副本并排序
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        // 偶数个元素，取中间两个的平均值
        Some((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        // 奇数个元素，取中间值
        Some(sorted[mid])
    }
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mean() {
        let values = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(mean(&values), Some(3.0));
    }
    
    #[test]
    fn test_median_odd() {
        let values = [1.0, 3.0, 5.0, 7.0, 9.0];
        assert_eq!(median(&values), Some(5.0));
    }
    
    #[test]
    fn test_median_even() {
        let values = [1.0, 3.0, 5.0, 7.0];
        assert_eq!(median(&values), Some(4.0));
    }
} 