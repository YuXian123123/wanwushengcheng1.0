//! 相似度检测 - 螺丝咕姆第二层防护
//!
//! 核心机制：相似度不超过80%，动态阈值基于发送者历史

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 相似度检测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityResult {
    /// 相似度分数
    pub score: f64,
    /// 动态阈值
    pub threshold: f64,
    /// 是否违规（超过阈值）
    pub is_violation: bool,
    /// 最大历史相似度
    pub max_historical_similarity: f64,
}

/// 发送者历史记录
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SenderHistory {
    /// 发送次数
    pub send_count: u64,
    /// 历史内容
    pub content_hashes: Vec<String>,
    /// 平均相似度
    pub avg_similarity: f64,
}

impl SenderHistory {
    pub fn new() -> Self {
        Self {
            send_count: 0,
            content_hashes: Vec::new(),
            avg_similarity: 0.0,
        }
    }

    /// 更新历史（不可变）
    pub fn update(&self, content_hash: String, similarity: f64) -> Self {
        let mut new_history = self.clone();
        new_history.send_count += 1;
        new_history.content_hashes.push(content_hash);

        // 更新平均相似度
        let total = new_history.avg_similarity * (new_history.send_count - 1) as f64 + similarity;
        new_history.avg_similarity = total / new_history.send_count as f64;

        new_history
    }
}

/// 相似度检测器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityDetector {
    /// 基础阈值
    base_threshold: f64,
    /// 最大相似度上限（80%）
    max_similarity: f64,
    /// 信任系数 α
    trust_alpha: f64,
    /// 频率系数 β
    frequency_beta: f64,
    /// 发送者历史
    sender_histories: HashMap<Uuid, SenderHistory>,
}

impl SimilarityDetector {
    pub fn new(base_threshold: f64, max_similarity: f64, trust_alpha: f64, frequency_beta: f64) -> Self {
        Self {
            base_threshold,
            max_similarity,
            trust_alpha,
            frequency_beta,
            sender_histories: HashMap::new(),
        }
    }

    /// 获取发送者历史
    pub fn get_sender_history(&self, sender: &Uuid) -> SenderHistory {
        self.sender_histories.get(sender).cloned().unwrap_or_default()
    }

    /// 检测相似度
    pub fn detect(
        &self,
        content: &str,
        history: &[String],
        sender_history: &SenderHistory,
    ) -> SimilarityResult {
        // 计算动态阈值: T = T₀ × (1 + α·trust - β·frequency)
        let frequency = (sender_history.send_count as f64).ln().max(0.0) / 10.0;
        let trust = 1.0 - sender_history.avg_similarity.min(1.0);
        let threshold = self.base_threshold * (1.0 + self.trust_alpha * trust - self.frequency_beta * frequency);

        // 计算与历史内容的相似度
        let content_hash = self.hash_content(content);
        let max_sim = self.calculate_max_similarity(&content_hash, history);

        // 判断是否违规
        let is_violation = max_sim > threshold.min(self.max_similarity);

        SimilarityResult {
            score: max_sim,
            threshold: threshold.min(self.max_similarity),
            is_violation,
            max_historical_similarity: max_sim,
        }
    }

    /// 计算内容哈希（简化版本）
    fn hash_content(&self, content: &str) -> String {
        // 简化的内容哈希：使用字符频率分布
        let mut freq = vec![0u32; 256];
        for b in content.bytes() {
            freq[b as usize] += 1;
        }
        // 取前32个字节的频率作为哈希
        freq[..32].iter()
            .map(|&f| format!("{:02x}", f % 256))
            .collect()
    }

    /// 计算最大相似度: Sim = cosine_similarity(content, history)
    fn calculate_max_similarity(&self, content_hash: &str, history: &[String]) -> f64 {
        if history.is_empty() {
            return 0.0;
        }

        let mut max_sim: f64 = 0.0;
        for hist_hash in history {
            let sim = self.cosine_similarity(content_hash, hist_hash);
            max_sim = max_sim.max(sim);
        }
        max_sim
    }

    /// 余弦相似度计算
    fn cosine_similarity(&self, hash1: &str, hash2: &str) -> f64 {
        if hash1.len() != hash2.len() {
            return 0.0;
        }

        let mut dot_product = 0;
        let mut norm1 = 0u64;
        let mut norm2 = 0u64;

        for (c1, c2) in hash1.chars().zip(hash2.chars()) {
            let v1 = c1.to_digit(16).unwrap_or(0) as u64;
            let v2 = c2.to_digit(16).unwrap_or(0) as u64;
            dot_product += v1 * v2;
            norm1 += v1 * v1;
            norm2 += v2 * v2;
        }

        if norm1 == 0 || norm2 == 0 {
            return 0.0;
        }

        dot_product as f64 / ((norm1 as f64 * norm2 as f64).sqrt())
    }

    /// 更新发送者历史（不可变）
    pub fn update_history(&self, sender: Uuid, content: &str, similarity: f64) -> Self {
        let mut new_detector = self.clone();
        let content_hash = self.hash_content(content);
        let history = self.get_sender_history(&sender);
        let new_history = history.update(content_hash, similarity);
        new_detector.sender_histories.insert(sender, new_history);
        new_detector
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similarity_detector_creation() {
        let detector = SimilarityDetector::new(0.5, 0.8, 0.3, 0.2);
        assert_eq!(detector.max_similarity, 0.8);
    }

    #[test]
    fn test_empty_history() {
        let detector = SimilarityDetector::new(0.5, 0.8, 0.3, 0.2);
        let sender_history = SenderHistory::new();

        let result = detector.detect("new content", &[], &sender_history);
        assert_eq!(result.score, 0.0);
        assert!(!result.is_violation);
    }

    #[test]
    fn test_high_similarity_violation() {
        let detector = SimilarityDetector::new(0.5, 0.8, 0.3, 0.2);
        let sender_history = SenderHistory::new();

        // 相同内容应该被检测为高相似度
        let content = "这是一段测试内容用于检测相似度";
        let history = vec![detector.hash_content(content)];

        let result = detector.detect(content, &history, &sender_history);
        assert!(result.score > 0.9);
        assert!(result.is_violation);
    }

    #[test]
    fn test_max_similarity_limit() {
        let detector = SimilarityDetector::new(0.5, 0.8, 0.3, 0.2);

        // 阈值不能超过80%
        assert!(detector.max_similarity <= 0.8);
    }
}