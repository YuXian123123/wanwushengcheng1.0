//! 行为模式分析 - 螺丝咕姆第三层防护

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 行为分析配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorConfig {
    /// 最大历史记录数
    pub max_history_size: usize,
    /// 高频阈值（每分钟）
    pub high_frequency_threshold: f64,
    /// 重复阈值
    pub repetition_threshold: f64,
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            max_history_size: 100,
            high_frequency_threshold: 10.0,
            repetition_threshold: 0.5,
        }
    }
}

/// 行为模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorPattern {
    /// 发送频率因子
    pub frequency_factor: f64,
    /// 时间分布均匀度
    pub timing_uniformity: f64,
    /// 内容多样性
    pub content_diversity: f64,
    /// 是否异常
    pub is_anomaly: bool,
    /// 异常类型
    pub anomaly_type: Option<AnomalyType>,
}

/// 异常类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalyType {
    /// 高频发送
    HighFrequency,
    /// 内容重复
    ContentRepetition,
    /// 时间异常（如固定间隔）
    TimingPattern,
    /// 综合异常
    Combined,
}

/// 用户行为记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBehavior {
    /// 发送次数
    pub send_count: u64,
    /// 最近发送时间戳
    pub last_send_time: u64,
    /// 发送间隔历史
    pub intervals: Vec<u64>,
    /// 内容哈希集合
    pub content_hashes: Vec<String>,
    /// 最大历史记录数
    max_history_size: usize,
}

impl UserBehavior {
    pub fn new(max_history_size: usize) -> Self {
        Self {
            send_count: 0,
            last_send_time: 0,
            intervals: Vec::new(),
            content_hashes: Vec::new(),
            max_history_size,
        }
    }

    /// 记录发送行为（不可变）
    pub fn record(&self, timestamp: u64, content_hash: String) -> Self {
        let mut new_behavior = self.clone();
        new_behavior.send_count += 1;

        // 计算间隔
        if self.last_send_time > 0 {
            new_behavior.intervals.push(timestamp - self.last_send_time);
            // 保留最近的间隔（使用配置）
            while new_behavior.intervals.len() > new_behavior.max_history_size {
                new_behavior.intervals.remove(0);
            }
        }

        new_behavior.last_send_time = timestamp;
        new_behavior.content_hashes.push(content_hash);

        // 保留最近的内容哈希（使用配置）
        while new_behavior.content_hashes.len() > new_behavior.max_history_size {
            new_behavior.content_hashes.remove(0);
        }

        new_behavior
    }
}

impl Default for UserBehavior {
    fn default() -> Self {
        Self::new(100)
    }
}

/// 行为分析器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorAnalyzer {
    /// 用户行为记录
    user_behaviors: HashMap<Uuid, UserBehavior>,
    /// 配置
    config: BehaviorConfig,
}

impl BehaviorAnalyzer {
    pub fn new(config: BehaviorConfig) -> Self {
        Self {
            user_behaviors: HashMap::new(),
            config,
        }
    }

    /// 使用默认配置创建
    pub fn with_defaults() -> Self {
        Self::new(BehaviorConfig::default())
    }

    /// 分析行为模式
    pub fn analyze(&self, user: &Uuid, content: &str) -> BehaviorPattern {
        let behavior = self.user_behaviors.get(user).cloned().unwrap_or_else(|| {
            UserBehavior::new(self.config.max_history_size)
        });

        // 计算频率因子
        let frequency_factor = self.calculate_frequency_factor(&behavior);

        // 计算时间分布均匀度
        let timing_uniformity = self.calculate_timing_uniformity(&behavior);

        // 计算内容多样性
        let content_diversity = self.calculate_content_diversity(&behavior, content);

        // 检测异常
        let (is_anomaly, anomaly_type) = self.detect_anomaly(
            frequency_factor,
            timing_uniformity,
            content_diversity,
        );

        BehaviorPattern {
            frequency_factor,
            timing_uniformity,
            content_diversity,
            is_anomaly,
            anomaly_type,
        }
    }

    /// 计算频率因子
    fn calculate_frequency_factor(&self, behavior: &UserBehavior) -> f64 {
        if behavior.send_count == 0 {
            return 0.0;
        }
        // 简化：基于发送次数的对数
        (behavior.send_count as f64).ln().max(0.0) / 10.0
    }

    /// 计算时间分布均匀度
    fn calculate_timing_uniformity(&self, behavior: &UserBehavior) -> f64 {
        if behavior.intervals.len() < 2 {
            return 1.0;
        }

        // 计算间隔的标准差
        let mean = behavior.intervals.iter().sum::<u64>() as f64 / behavior.intervals.len() as f64;
        let variance = behavior.intervals.iter()
            .map(|&i| (i as f64 - mean).powi(2))
            .sum::<f64>() / behavior.intervals.len() as f64;
        let std_dev = variance.sqrt();

        // 标准差越小，均匀度越高（可能是机器人行为）
        let uniformity = 1.0 / (1.0 + std_dev / mean.max(1.0));
        uniformity
    }

    /// 计算内容多样性
    fn calculate_content_diversity(&self, behavior: &UserBehavior, new_content: &str) -> f64 {
        if behavior.content_hashes.is_empty() {
            return 1.0;
        }

        // 计算新内容与历史内容的重复率
        let new_hash = self.simple_hash(new_content);
        let duplicates = behavior.content_hashes.iter()
            .filter(|&h| h == &new_hash)
            .count();

        1.0 - (duplicates as f64 / behavior.content_hashes.len() as f64).min(1.0)
    }

    /// 检测异常: Anomaly = |behavior - expected| > σ
    fn detect_anomaly(
        &self,
        frequency: f64,
        timing: f64,
        diversity: f64,
    ) -> (bool, Option<AnomalyType>) {
        // 高频异常（使用配置阈值）
        if frequency > self.config.high_frequency_threshold / 10.0 {
            return (true, Some(AnomalyType::HighFrequency));
        }

        // 内容重复异常（使用配置阈值）
        if diversity < self.config.repetition_threshold {
            return (true, Some(AnomalyType::ContentRepetition));
        }

        // 时间模式异常（太规律）
        if timing > 0.9 && frequency > 0.3 {
            return (true, Some(AnomalyType::TimingPattern));
        }

        (false, None)
    }

    /// 简单哈希函数
    fn simple_hash(&self, content: &str) -> String {
        format!("{:x}", content.len())
    }

    /// 记录用户行为（不可变）
    pub fn record(&self, user: Uuid, timestamp: u64, content: &str) -> Self {
        let mut new_analyzer = self.clone();
        let content_hash = self.simple_hash(content);
        let behavior = self.user_behaviors.get(&user).cloned().unwrap_or_else(|| {
            UserBehavior::new(self.config.max_history_size)
        });
        let new_behavior = behavior.record(timestamp, content_hash);
        new_analyzer.user_behaviors.insert(user, new_behavior);
        new_analyzer
    }
}

impl Default for BehaviorAnalyzer {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_behavior_analyzer_creation() {
        let analyzer = BehaviorAnalyzer::with_defaults();
        let user = Uuid::new_v4();

        let pattern = analyzer.analyze(&user, "test content");
        assert!(!pattern.is_anomaly);
    }

    #[test]
    fn test_frequency_factor() {
        let mut analyzer = BehaviorAnalyzer::with_defaults();
        let user = Uuid::new_v4();

        // 模拟高频发送
        for i in 0..20 {
            analyzer = analyzer.record(user, i as u64, &format!("content {}", i));
        }

        let pattern = analyzer.analyze(&user, "new content");
        assert!(pattern.frequency_factor > 0.0);
    }
}