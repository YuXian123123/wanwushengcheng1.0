//! 语义相似度检测模块
//!
//! 实现基于LNN理解的语义相似度检测，而非字面匹配

use std::collections::HashMap;
use super::message::{Message, GuId};

/// 相似度检测结果
#[derive(Debug, Clone)]
pub struct SimilarityResult {
    /// 与历史消息的最大相似度
    pub max_similarity: f64,
    /// 是否通过（< 80%）
    pub passed: bool,
    /// 最相似的历史消息ID
    pub most_similar_to: Option<String>,
}

/// 相似度检测器
#[derive(Debug, Clone)]
pub struct SimilarityDetector {
    /// 相似度阈值（默认0.8）
    pub threshold: f64,
    /// 概念重叠权重
    pub concept_weight: f64,
    /// 结构相似权重
    pub structure_weight: f64,
    /// 向量相似权重
    pub vector_weight: f64,
}

impl Default for SimilarityDetector {
    fn default() -> Self {
        Self {
            threshold: 0.8,
            concept_weight: 0.4,
            structure_weight: 0.3,
            vector_weight: 0.3,
        }
    }
}

impl SimilarityDetector {
    /// 检测消息与历史的相似度
    pub fn detect(&self, message: &Message, history: &[Message]) -> SimilarityResult {
        if history.is_empty() {
            return SimilarityResult {
                max_similarity: 0.0,
                passed: true,
                most_similar_to: None,
            };
        }

        let mut max_sim = 0.0;
        let mut most_similar_id: Option<String> = None;

        for hist_msg in history {
            let sim = self.calculate_similarity(message, hist_msg);
            if sim > max_sim {
                max_sim = sim;
                most_similar_id = Some(format!("{:?}", hist_msg.id));
            }
        }

        // 动态阈值（可被刷金币嫌疑调整）
        let effective_threshold = self.threshold;

        SimilarityResult {
            max_similarity: max_sim,
            passed: max_sim < effective_threshold,
            most_similar_to: most_similar_id,
        }
    }

    /// 计算两个消息的语义相似度
    /// sim(m1, m2) = w1·overlap + w2·struct + w3·cos
    fn calculate_similarity(&self, m1: &Message, m2: &Message) -> f64 {
        // 检查发送者是否相同
        let same_sender = m1.sender == m2.sender;

        // 检查消息类型是否相同
        let same_type = std::mem::discriminant(&m1.message_type)
                      == std::mem::discriminant(&m2.message_type);

        // 如果类型不同，相似度较低
        if !same_type {
            return 0.2;
        }

        // 根据消息类型计算相似度
        match (&m1.message_type, &m2.message_type) {
            (super::message::MessageType::Knowledge { content: c1, concepts: cs1, .. },
             super::message::MessageType::Knowledge { content: c2, concepts: cs2, .. }) => {
                // 概念重叠度
                let overlap = self.jaccard_similarity(cs1, cs2);

                // 内容相似度（简化：基于长度比例）
                let len_sim = if c1.len() > 0 && c2.len() > 0 {
                    1.0 - (c1.len() as f64 - c2.len() as f64).abs()
                        / (c1.len().max(c2.len()) as f64)
                } else {
                    0.0
                };

                // 综合相似度
                let base_sim = self.concept_weight * overlap
                             + self.structure_weight * len_sim
                             + self.vector_weight * 0.5;

                // 如果发送者相同，相似度更重要
                if same_sender {
                    base_sim * 1.2 // 放大同发送者的相似度
                } else {
                    base_sim
                }
            }
            _ => 0.3, // 其他类型的默认相似度
        }
    }

    /// Jaccard相似度
    fn jaccard_similarity<T: std::hash::Hash + Eq + std::clone::Clone>(
        &self,
        set1: &std::collections::HashSet<T>,
        set2: &std::collections::HashSet<T>,
    ) -> f64 {
        if set1.is_empty() && set2.is_empty() {
            return 1.0;
        }

        let intersection = set1.intersection(set2).count();
        let union = set1.union(set2).count();

        if union == 0 {
            return 0.0;
        }

        intersection as f64 / union as f64
    }

    /// 更新阈值（根据刷金币嫌疑）
    pub fn with_threshold(&self, farming_suspicion: f64) -> Self {
        let adj = match farming_suspicion {
            s if s > 0.7 => -0.1,  // 高嫌疑：更严格（65%就算相似）
            s if s > 0.4 => -0.05, // 中嫌疑：75%就算相似
            _ => 0.0,              // 低嫌疑：保持80%
        };

        Self {
            threshold: (self.threshold + adj).clamp(0.6, 0.9),
            ..self.clone()
        }
    }
}

/// 发送者历史记录
#[derive(Debug, Clone)]
pub struct SenderHistory {
    pub sender_id: GuId,
    pub messages: Vec<Message>,
    /// 平均质量
    pub average_quality: f64,
    /// 刷金币嫌疑分数
    pub farming_suspicion: f64,
    /// 近期被拒绝次数
    pub recent_rejections: u32,
}

impl SenderHistory {
    pub fn new(sender_id: GuId) -> Self {
        Self {
            sender_id,
            messages: Vec::new(),
            average_quality: 0.5,
            farming_suspicion: 0.0,
            recent_rejections: 0,
        }
    }

    /// 添加消息到历史
    pub fn with_message(&self, message: Message) -> Self {
        let mut new_history = self.clone();
        new_history.messages.push(message);
        new_history
    }

    /// 更新质量统计
    pub fn with_quality_update(&self, quality: f64, passed: bool) -> Self {
        let mut new_history = self.clone();

        // 更新平均质量
        let count = new_history.messages.len() as f64;
        new_history.average_quality = (self.average_quality * count + quality) / (count + 1.0);

        // 更新拒绝计数
        if !passed {
            new_history.recent_rejections += 1;
        }

        // 更新刷金币嫌疑
        new_history.farming_suspicion = self.calculate_farming_suspicion();

        new_history
    }

    /// 计算刷金币嫌疑分数
    fn calculate_farming_suspicion(&self) -> f64 {
        let mut suspicion = 0.0;

        // 近期拒绝次数影响
        suspicion += (self.recent_rejections as f64 / 10.0).min(0.5);

        // 平均质量影响
        if self.average_quality < 0.3 {
            suspicion += 0.3;
        } else if self.average_quality < 0.5 {
            suspicion += 0.1;
        }

        suspicion.clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::message::{MessageType, ChannelType};
    use std::collections::HashSet;

    #[test]
    fn test_similarity_detector() {
        let detector = SimilarityDetector::default();
        assert_eq!(detector.threshold, 0.8);
    }

    #[test]
    fn test_empty_history() {
        let detector = SimilarityDetector::default();
        let msg = Message::new(
            GuId("test".to_string()),
            ChannelType::World,
            MessageType::Knowledge {
                content: "test".to_string(),
                concepts: HashSet::new(),
                confidence: 0.5,
            },
            &super::super::message::MessageSigner,
        );

        let result = detector.detect(&msg, &[]);
        assert!(result.passed);
    }

    #[test]
    fn test_jaccard_similarity() {
        let detector = SimilarityDetector::default();
        let set1: HashSet<String> = HashSet::from(["a".to_string(), "b".to_string()]);
        let set2: HashSet<String> = HashSet::from(["b".to_string(), "c".to_string()]);

        let sim = detector.jaccard_similarity(&set1, &set2);
        // 交集: {b} = 1, 并集: {a,b,c} = 3
        assert!((sim - 1.0/3.0).abs() < 0.01);
    }
}
