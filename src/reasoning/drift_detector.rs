//! 向量漂移检测器模块
//! 实现漂移检测算法、纠正机制和安全边界

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// 漂移检测结果
#[derive(Debug, Clone)]
pub struct DriftDetectionResult {
    pub is_drifted: bool,
    pub drift_magnitude: f64,
    pub correction_needed: bool,
    pub timestamp: Instant,
}

/// 向量漂移检测器
pub struct DriftDetector {
    /// 历史向量窗口
    history_window: VecDeque<Vec<f64>>,
    /// 窗口大小
    window_size: usize,
    /// 漂移阈值
    drift_threshold: f64,
    /// 安全边界
    safety_boundary: f64,
}

impl DriftDetector {
    /// 创建新的漂移检测器
    pub fn new(window_size: usize, drift_threshold: f64, safety_boundary: f64) -> Self {
        Self {
            history_window: VecDeque::with_capacity(window_size),
            window_size,
            drift_threshold,
            safety_boundary,
        }
    }

    /// 检测向量漂移
    pub fn detect_drift(&mut self, current_vector: &[f64]) -> DriftDetectionResult {
        let timestamp = Instant::now();
        
        // 添加当前向量到历史窗口
        self.add_to_history(current_vector.to_vec());
        
        // 如果历史数据不足，无法检测漂移
        if self.history_window.len() < 2 {
            return DriftDetectionResult {
                is_drifted: false,
                drift_magnitude: 0.0,
                correction_needed: false,
                timestamp,
            };
        }
        
        // 计算漂移量
        let drift_magnitude = self.calculate_drift_magnitude();
        
        // 判断是否发生漂移
        let is_drifted = drift_magnitude > self.drift_threshold;
        
        // 判断是否需要纠正
        let correction_needed = is_drifted || drift_magnitude > self.safety_boundary;
        
        DriftDetectionResult {
            is_drifted,
            drift_magnitude,
            correction_needed,
            timestamp,
        }
    }

    /// 添加向量到历史窗口
    fn add_to_history(&mut self, vector: Vec<f64>) {
        if self.history_window.len() >= self.window_size {
            self.history_window.pop_front();
        }
        self.history_window.push_back(vector);
    }

    /// 计算漂移量（基于均方根误差）
    fn calculate_drift_magnitude(&self) -> f64 {
        if self.history_window.len() < 2 {
            return 0.0;
        }
        
        let latest = self.history_window.back().unwrap();
        let previous = self.history_window.get(self.history_window.len() - 2).unwrap();
        
        // 计算两个向量之间的欧几里得距离
        let mut sum_squared_diff = 0.0;
        for i in 0..latest.len() {
            let diff = latest[i] - previous[i];
            sum_squared_diff += diff * diff;
        }
        
        sum_squared_diff.sqrt()
    }

    /// 纠正向量漂移
    pub fn correct_drift(&self, current_vector: &[f64]) -> Vec<f64> {
        if self.history_window.len() < 2 {
            return current_vector.to_vec();
        }
        
        let latest = self.history_window.back().unwrap();
        let previous = self.history_window.get(self.history_window.len() - 2).unwrap();
        
        // 计算预期方向
        let mut expected_direction = vec![0.0; latest.len()];
        for i in 0..latest.len() {
            expected_direction[i] = latest[i] - previous[i];
        }
        
        // 计算当前向量与预期方向的偏差
        let mut correction = vec![0.0; current_vector.len()];
        for i in 0..current_vector.len() {
            // 向预期方向调整
            correction[i] = current_vector[i] + expected_direction[i] * 0.1; // 10%的调整力度
            
            // 确保不超出安全边界
            if correction[i] > self.safety_boundary {
                correction[i] = self.safety_boundary;
            } else if correction[i] < -self.safety_boundary {
                correction[i] = -self.safety_boundary;
            }
        }
        
        correction
    }

    /// 获取历史窗口的统计信息
    pub fn get_statistics(&self) -> DriftStatistics {
        if self.history_window.len() < 2 {
            return DriftStatistics::default();
        }
        
        let mut magnitudes = Vec::new();
        let history: Vec<&Vec<f64>> = self.history_window.iter().collect();
        
        for i in 1..history.len() {
            let mut sum_squared_diff = 0.0;
            for j in 0..history[i].len() {
                let diff = history[i][j] - history[i-1][j];
                sum_squared_diff += diff * diff;
            }
            magnitudes.push(sum_squared_diff.sqrt());
        }
        
        let mean = magnitudes.iter().sum::<f64>() / magnitudes.len() as f64;
        let variance = magnitudes.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / magnitudes.len() as f64;
        let std_dev = variance.sqrt();
        
        DriftStatistics {
            mean_drift: mean,
            std_deviation: std_dev,
            max_drift: magnitudes.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            min_drift: magnitudes.iter().cloned().fold(f64::INFINITY, f64::min),
            sample_count: magnitudes.len(),
        }
    }
}

/// 漂移统计信息
#[derive(Debug, Clone, Default)]
pub struct DriftStatistics {
    pub mean_drift: f64,
    pub std_deviation: f64,
    pub max_drift: f64,
    pub min_drift: f64,
    pub sample_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_drift_detection_no_drift() {
        let mut detector = DriftDetector::new(5, 1.0, 2.0);
        
        // 添加相似的向量，应该没有漂移
        let vector1 = vec![1.0, 2.0, 3.0];
        let vector2 = vec![1.1, 2.1, 3.1];
        
        let result1 = detector.detect_drift(&vector1);
        assert!(!result1.is_drifted);
        
        let result2 = detector.detect_drift(&vector2);
        assert!(!result2.is_drifted);
    }

    #[test]
    fn test_drift_detection_with_drift() {
        let mut detector = DriftDetector::new(5, 1.0, 2.0);
        
        // 添加相似的向量
        let vector1 = vec![1.0, 2.0, 3.0];
        let vector2 = vec![1.1, 2.1, 3.1];
        
        detector.detect_drift(&vector1);
        detector.detect_drift(&vector2);
        
        // 添加一个明显不同的向量，应该检测到漂移
        let vector3 = vec![5.0, 6.0, 7.0];
        let result = detector.detect_drift(&vector3);
        
        assert!(result.is_drifted);
        assert!(result.drift_magnitude > 1.0);
    }

    #[test]
    fn test_drift_correction() {
        let mut detector = DriftDetector::new(5, 1.0, 5.0);
        
        // 添加一些向量建立趋势
        detector.detect_drift(&vec![1.0, 1.0, 1.0]);
        detector.detect_drift(&vec![1.5, 1.5, 1.5]);
        detector.detect_drift(&vec![2.0, 2.0, 2.0]);
        
        // 漂移的向量
        let drifted_vector = vec![10.0, 10.0, 10.0];
        let corrected_vector = detector.correct_drift(&drifted_vector);
        
        // 纠正后的向量应该更接近预期趋势
        assert_ne!(corrected_vector, drifted_vector);
    }

    #[test]
    fn test_statistics() {
        let mut detector = DriftDetector::new(10, 1.0, 2.0);
        
        // 添加一些测试数据
        for i in 0..5 {
            let vector = vec![i as f64, i as f64 * 2.0, i as f64 * 3.0];
            detector.detect_drift(&vector);
            thread::sleep(Duration::from_millis(10)); // 确保时间戳不同
        }
        
        let stats = detector.get_statistics();
        assert_eq!(stats.sample_count, 4); // 5个向量，4个差值
        assert!(stats.mean_drift > 0.0);
        assert!(stats.max_drift >= stats.min_drift);
    }

    #[test]
    fn test_window_size_limit() {
        let mut detector = DriftDetector::new(3, 1.0, 2.0);
        
        // 添加超过窗口大小的数据
        for i in 0..10 {
            let vector = vec![i as f64; 3];
            detector.detect_drift(&vector);
        }
        
        // 窗口应该保持在指定大小
        assert_eq!(detector.history_window.len(), 3);
    }
}
