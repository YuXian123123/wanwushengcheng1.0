//! 知识传承系统
//!
//! 实现蛊虫死亡时的知识传承机制：
//! - 双轨制：遗传轨（个体继承）+ 文化轨（世界知识库）
//! - 知识验证：防止污染传播
//! - 知识抽象：从原始经验提炼通用原则
//!
//! 核心公式：
//! Valid_Knowledge = Knowledge × Verified × Trust_Score
//! 传承效率：η = I(K_heir; K_original) / H(K_original)

pub mod inheritance;
pub mod validation;
pub mod abstraction;

pub use inheritance::{KnowledgeInheritance, GeneticTrack, CulturalTrack, InheritanceStats};
pub use validation::{KnowledgeValidator, ValidationResult, KnowledgeTrust};
pub use abstraction::{KnowledgeAbstraction, AbstractionLevel, AbstractedKnowledge, AbstractionStats};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ============================================================================
// 基础类型
// ============================================================================

/// 知识项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knowledge {
    /// 知识ID
    pub id: Uuid,
    /// 知识内容
    pub content: String,
    /// 知识类型
    pub knowledge_type: KnowledgeType,
    /// 知识来源（蛊虫ID）
    pub source: Uuid,
    /// 创建时间戳
    pub created_at: u64,
    /// 验证状态
    pub verified: bool,
    /// 信任分数（0-1）
    pub trust_score: f64,
    /// 使用次数
    pub usage_count: u64,
    /// 成功次数
    pub success_count: u64,
}

/// 知识类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KnowledgeType {
    /// 技能知识（如何做某事）
    Skill,
    /// 事实知识（是什么）
    Fact,
    /// 经验知识（过去的经历）
    Experience,
    /// 策略知识（决策规则）
    Strategy,
    /// 元知识（关于知识的知识）
    Meta,
}

/// 知识优先级（用于传承排序）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum KnowledgePriority {
    /// 低优先级
    Low,
    /// 中优先级
    Medium,
    /// 高优先级
    High,
    /// 关键知识
    Critical,
}

impl Knowledge {
    pub fn new(content: String, knowledge_type: KnowledgeType, source: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            content,
            knowledge_type,
            source,
            created_at: current_timestamp(),
            verified: false,
            trust_score: 0.5,
            usage_count: 0,
            success_count: 0,
        }
    }

    /// 计算知识价值
    pub fn calculate_value(&self) -> f64 {
        if self.usage_count == 0 {
            return self.trust_score;
        }

        let success_rate = self.success_count as f64 / self.usage_count as f64;
        success_rate * self.trust_score
    }

    /// 记录使用
    pub fn record_use(&mut self, success: bool) {
        self.usage_count += 1;
        if success {
            self.success_count += 1;
        }
    }
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ============================================================================
// 配置
// ============================================================================

/// 知识传承配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeConfig {
    /// 最低信任分数阈值
    pub min_trust_score: f64,
    /// 遗传轨最大继承者数量
    pub max_heirs: usize,
    /// 文化轨抽象延迟（秒）
    pub abstraction_delay: f64,
    /// 知识验证采样次数
    pub validation_samples: usize,
    /// 知识衰减率（每秒）
    pub knowledge_decay_rate: f64,
}

impl Default for KnowledgeConfig {
    fn default() -> Self {
        Self {
            min_trust_score: 0.3,
            max_heirs: 3,
            abstraction_delay: 10.0,
            validation_samples: 5,
            knowledge_decay_rate: 0.001,
        }
    }
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knowledge_creation() {
        let knowledge = Knowledge::new(
            "如何识别危险".to_string(),
            KnowledgeType::Skill,
            Uuid::new_v4(),
        );

        assert!(!knowledge.verified);
        assert_eq!(knowledge.trust_score, 0.5);
        assert_eq!(knowledge.usage_count, 0);
    }

    #[test]
    fn test_knowledge_value_calculation() {
        let mut knowledge = Knowledge::new(
            "test".to_string(),
            KnowledgeType::Skill,
            Uuid::new_v4(),
        );

        // 未使用时的价值
        let value_no_use = knowledge.calculate_value();
        assert_eq!(value_no_use, 0.5);

        // 记录使用
        knowledge.record_use(true);
        knowledge.record_use(true);
        knowledge.record_use(false);

        let value_after_use = knowledge.calculate_value();
        // 成功率 2/3 = 0.667, 信任分数 0.5
        assert!(value_after_use > 0.0 && value_after_use < 1.0);
    }

    #[test]
    fn test_knowledge_priority_ordering() {
        assert!(KnowledgePriority::Critical > KnowledgePriority::High);
        assert!(KnowledgePriority::High > KnowledgePriority::Medium);
        assert!(KnowledgePriority::Medium > KnowledgePriority::Low);
    }
}
