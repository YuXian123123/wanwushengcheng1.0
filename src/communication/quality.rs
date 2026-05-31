//! 内容质量检测模块
//!
//! 实现知识密度、结构化程度、综合质量的计算

use std::collections::HashSet;
use super::message::MessageType;

/// 质量分数
#[derive(Debug, Clone)]
pub struct QualityScore {
    /// 知识密度 ρ
    pub knowledge_density: f64,
    /// 结构化程度 σ
    pub structure_score: f64,
    /// 新颖度 ν
    pub novelty: f64,
    /// 综合质量 Q = wρ·ρ + wσ·σ + wν·ν
    pub overall: f64,
}

/// 动态质量阈值
#[derive(Debug, Clone)]
pub struct QualityThreshold {
    /// 基础阈值
    pub base: f64,
    /// 发送者质量调整
    pub quality_adjustment: f64,
    /// 刷金币嫌疑调整
    pub farming_adjustment: f64,
}

impl QualityThreshold {
    pub fn new() -> Self {
        Self {
            base: 0.5,
            quality_adjustment: 0.0,
            farming_adjustment: 0.0,
        }
    }

    /// 计算最终阈值
    pub fn calculate(&self) -> f64 {
        (self.base + self.quality_adjustment + self.farming_adjustment).clamp(0.3, 0.9)
    }

    /// 根据发送者历史更新阈值
    pub fn with_sender_history(&self, avg_quality: f64, farming_suspicion: f64) -> Self {
        let quality_adj = match avg_quality {
            q if q > 0.8 => -0.1,
            q if q > 0.5 => 0.0,
            q if q > 0.3 => 0.1,
            _ => 0.2,
        };

        let farming_adj = match farming_suspicion {
            s if s > 0.7 => 0.3,
            s if s > 0.4 => 0.1,
            _ => 0.0,
        };

        Self {
            base: self.base,
            quality_adjustment: quality_adj,
            farming_adjustment: farming_adj,
        }
    }
}

/// 质量检测器
#[derive(Debug, Clone)]
pub struct QualityDetector {
    /// 知识密度权重
    pub density_weight: f64,
    /// 结构化权重
    pub structure_weight: f64,
    /// 新颖度权重
    pub novelty_weight: f64,
}

impl Default for QualityDetector {
    fn default() -> Self {
        Self {
            density_weight: 0.4,
            structure_weight: 0.3,
            novelty_weight: 0.3,
        }
    }
}

impl QualityDetector {
    /// 计算消息质量
    pub fn calculate_quality(&self, message_type: &MessageType, novelty: f64) -> QualityScore {
        let (density, structure) = self.extract_quality_features(message_type);

        let overall = self.density_weight * density
                    + self.structure_weight * structure
                    + self.novelty_weight * novelty;

        QualityScore {
            knowledge_density: density,
            structure_score: structure,
            novelty,
            overall,
        }
    }

    /// 提取质量特征
    fn extract_quality_features(&self, message_type: &MessageType) -> (f64, f64) {
        match message_type {
            MessageType::Knowledge { content, concepts, confidence } => {
                let density = if content.is_empty() {
                    0.0
                } else {
                    concepts.len() as f64 / content.len() as f64
                };

                // 结构化程度基于概念数量和置信度
                let structure = if concepts.len() > 0 {
                    (concepts.len() as f64 / 10.0).min(1.0) * confidence
                } else {
                    0.0
                };

                (density.min(1.0), structure)
            }
            MessageType::Trade { product, .. } => {
                // 交易消息的结构化程度较高
                let density = if product.description.is_empty() {
                    0.3
                } else {
                    0.5
                };
                (density, 0.7)
            }
            MessageType::Status { .. } => (0.2, 0.3),
            MessageType::Emotion { .. } => (0.1, 0.2),
            MessageType::System { .. } => (0.3, 0.5),
        }
    }

    /// 检查质量是否达标
    pub fn is_quality_passed(&self, quality: &QualityScore, threshold: &QualityThreshold) -> bool {
        quality.overall > threshold.calculate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_threshold() {
        let threshold = QualityThreshold::new();
        assert_eq!(threshold.calculate(), 0.5);

        let adjusted = threshold.with_sender_history(0.9, 0.1);
        assert!(adjusted.calculate() < 0.5);
    }

    #[test]
    fn test_quality_detection() {
        let detector = QualityDetector::default();
        let msg_type = MessageType::Knowledge {
            content: "HTML is a markup language".to_string(),
            concepts: HashSet::from(["HTML".to_string(), "markup".to_string()]),
            confidence: 0.9,
        };

        let quality = detector.calculate_quality(&msg_type, 0.8);
        assert!(quality.overall > 0.0);
    }
}
