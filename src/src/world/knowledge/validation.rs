//! 知识验证机制
//!
//! 验证知识正确性，防止污染传播
//!
//! 核心公式：
//! Valid_Knowledge = Knowledge × Verified × Trust_Score
//! 信任熵 = -Σ Trust_i × log(Trust_i)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::{Knowledge, KnowledgeType, KnowledgeConfig};

// ============================================================================
// 验证结果
// ============================================================================

/// 知识验证结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// 是否通过验证
    pub passed: bool,
    /// 验证分数（0-1）
    pub score: f64,
    /// 验证方法
    pub method: ValidationMethod,
    /// 验证时间戳
    pub timestamp: u64,
    /// 验证失败原因
    pub failure_reasons: Vec<String>,
}

/// 验证方法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationMethod {
    /// 共识验证（多个蛊虫验证）
    Consensus,
    /// 实践验证（实际使用测试）
    Empirical,
    /// 逻辑验证（逻辑一致性检查）
    Logical,
    /// 来源验证（信任链验证）
    SourceChain,
    /// 混合验证（多种方法组合）
    Hybrid,
}

// ============================================================================
// 知识信任
// ============================================================================

/// 知识信任状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeTrust {
    /// 信任分数（0-1）
    pub score: f64,
    /// 信任来源
    pub sources: Vec<TrustSource>,
    /// 最后更新时间戳
    pub last_update: u64,
}

/// 信任来源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustSource {
    /// 来源蛊虫ID
    pub gu_id: Uuid,
    /// 来源信任分数
    pub trust: f64,
    /// 时间戳
    pub timestamp: u64,
}

impl KnowledgeTrust {
    pub fn new() -> Self {
        Self {
            score: 0.5,
            sources: Vec::new(),
            last_update: 0,
        }
    }

    /// 添加信任来源
    pub fn add_source(&mut self, gu_id: Uuid, trust: f64, timestamp: u64) {
        self.sources.push(TrustSource {
            gu_id,
            trust,
            timestamp,
        });
        self.recalculate_score();
        self.last_update = timestamp;
    }

    /// 重新计算信任分数
    fn recalculate_score(&mut self) {
        if self.sources.is_empty() {
            self.score = 0.5;
            return;
        }

        // 加权平均
        let total_weight: f64 = self.sources.iter().map(|s| s.trust).sum();
        if total_weight > 0.0 {
            self.score = total_weight / self.sources.len() as f64;
        }
    }

    /// 计算信任熵
    pub fn calculate_entropy(&self) -> f64 {
        if self.sources.is_empty() {
            return 0.0;
        }

        // H = -Σ p_i × log(p_i)
        let mut entropy = 0.0;
        for source in &self.sources {
            if source.trust > 0.0 {
                entropy -= source.trust * source.trust.ln();
            }
        }
        entropy
    }
}

// ============================================================================
// 知识验证器
// ============================================================================

/// 知识验证器
#[derive(Debug, Clone)]
pub struct KnowledgeValidator {
    /// 配置
    config: KnowledgeConfig,
    /// 验证历史
    validation_history: HashMap<Uuid, Vec<ValidationResult>>,
    /// 信任状态
    trust_states: HashMap<Uuid, KnowledgeTrust>,
    /// 验证阈值
    validation_threshold: f64,
}

impl KnowledgeValidator {
    pub fn new(config: KnowledgeConfig) -> Self {
        Self {
            config,
            validation_history: HashMap::new(),
            trust_states: HashMap::new(),
            validation_threshold: 0.6,
        }
    }

    /// 验证知识
    pub fn validate(&mut self, knowledge: &Knowledge, method: ValidationMethod) -> ValidationResult {
        let mut failure_reasons = Vec::new();
        let mut score = 0.0;

        // 根据验证方法计算分数
        match method {
            ValidationMethod::Consensus => {
                score = self.consensus_validation(knowledge, &mut failure_reasons);
            }
            ValidationMethod::Empirical => {
                score = self.empirical_validation(knowledge, &mut failure_reasons);
            }
            ValidationMethod::Logical => {
                score = self.logical_validation(knowledge, &mut failure_reasons);
            }
            ValidationMethod::SourceChain => {
                score = self.source_chain_validation(knowledge, &mut failure_reasons);
            }
            ValidationMethod::Hybrid => {
                // 混合验证：多种方法的加权平均
                let consensus = self.consensus_validation(knowledge, &mut failure_reasons);
                let empirical = self.empirical_validation(knowledge, &mut failure_reasons);
                let logical = self.logical_validation(knowledge, &mut failure_reasons);

                score = 0.4 * consensus + 0.4 * empirical + 0.2 * logical;
            }
        }

        let passed = score >= self.validation_threshold;
        let result = ValidationResult {
            passed,
            score,
            method,
            timestamp: current_timestamp(),
            failure_reasons,
        };

        // 记录验证历史
        self.validation_history
            .entry(knowledge.id)
            .or_insert_with(Vec::new)
            .push(result.clone());

        result
    }

    /// 共识验证
    fn consensus_validation(&self, knowledge: &Knowledge, failures: &mut Vec<String>) -> f64 {
        // 检查是否有足够的验证者
        let trust = self.trust_states.get(&knowledge.id);
        match trust {
            Some(t) => {
                if t.sources.len() < 2 {
                    failures.push("验证者数量不足".to_string());
                }
                t.score
            }
            None => {
                failures.push("无信任记录".to_string());
                0.0
            }
        }
    }

    /// 实践验证
    fn empirical_validation(&self, knowledge: &Knowledge, failures: &mut Vec<String>) -> f64 {
        if knowledge.usage_count == 0 {
            failures.push("无使用记录".to_string());
            return 0.5; // 未知，给中等分数
        }

        let success_rate = knowledge.success_count as f64 / knowledge.usage_count as f64;
        if success_rate < 0.5 {
            failures.push(format!("成功率过低: {:.2}", success_rate));
        }
        success_rate
    }

    /// 逻辑验证
    fn logical_validation(&self, knowledge: &Knowledge, failures: &mut Vec<String>) -> f64 {
        // 检查内容是否为空
        if knowledge.content.is_empty() {
            failures.push("知识内容为空".to_string());
            return 0.0;
        }

        // 检查内容长度是否合理
        if knowledge.content.len() < 5 {
            failures.push("知识内容过短".to_string());
            return 0.3;
        }

        // 基础逻辑检查通过
        0.8
    }

    /// 来源链验证
    fn source_chain_validation(&self, knowledge: &Knowledge, failures: &mut Vec<String>) -> f64 {
        // 检查来源信任分数
        if knowledge.trust_score < self.config.min_trust_score {
            failures.push(format!(
                "来源信任分数过低: {:.2} < {:.2}",
                knowledge.trust_score, self.config.min_trust_score
            ));
            return knowledge.trust_score;
        }

        // 来源可信
        knowledge.trust_score
    }

    /// 添加信任来源
    pub fn add_trust_source(&mut self, knowledge_id: Uuid, gu_id: Uuid, trust: f64) {
        let trust_state = self
            .trust_states
            .entry(knowledge_id)
            .or_insert_with(KnowledgeTrust::new);
        trust_state.add_source(gu_id, trust, current_timestamp());
    }

    /// 获取知识的有效分数
    pub fn get_valid_score(&self, knowledge: &Knowledge) -> f64 {
        // Valid_Knowledge = Knowledge × Verified × Trust_Score
        let verified = if knowledge.verified { 1.0 } else { 0.5 };
        knowledge.calculate_value() * verified * knowledge.trust_score
    }

    /// 计算系统信任熵
    pub fn calculate_system_trust_entropy(&self) -> f64 {
        let mut total_entropy = 0.0;
        for trust in self.trust_states.values() {
            total_entropy += trust.calculate_entropy();
        }
        total_entropy
    }

    /// 检测信任异常
    pub fn detect_trust_anomaly(&self) -> Vec<Uuid> {
        let mut anomalies = Vec::new();
        for (id, trust) in &self.trust_states {
            // 信任熵过高表示信任分布不均匀
            let entropy = trust.calculate_entropy();
            if entropy > 1.0 || trust.score < self.config.min_trust_score {
                anomalies.push(*id);
            }
        }
        anomalies
    }
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_creation() {
        let result = ValidationResult {
            passed: true,
            score: 0.8,
            method: ValidationMethod::Consensus,
            timestamp: 1,
            failure_reasons: Vec::new(),
        };

        assert!(result.passed);
        assert_eq!(result.score, 0.8);
    }

    #[test]
    fn test_knowledge_trust_creation() {
        let trust = KnowledgeTrust::new();
        assert_eq!(trust.score, 0.5);
        assert!(trust.sources.is_empty());
    }

    #[test]
    fn test_knowledge_trust_add_source() {
        let mut trust = KnowledgeTrust::new();
        trust.add_source(Uuid::new_v4(), 0.8, 1);
        trust.add_source(Uuid::new_v4(), 0.6, 2);

        assert_eq!(trust.sources.len(), 2);
        // 平均信任分数
        assert!((trust.score - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_knowledge_validator_creation() {
        let config = KnowledgeConfig::default();
        let validator = KnowledgeValidator::new(config);
        assert!(validator.validation_history.is_empty());
    }

    #[test]
    fn test_logical_validation() {
        let config = KnowledgeConfig::default();
        let mut validator = KnowledgeValidator::new(config);

        let knowledge = Knowledge::new(
            "这是一个有效的知识".to_string(),
            KnowledgeType::Skill,
            Uuid::new_v4(),
        );

        let result = validator.validate(&knowledge, ValidationMethod::Logical);
        assert!(result.score > 0.5);
    }

    #[test]
    fn test_empirical_validation() {
        let config = KnowledgeConfig::default();
        let mut validator = KnowledgeValidator::new(config);

        let mut knowledge = Knowledge::new(
            "test".to_string(),
            KnowledgeType::Skill,
            Uuid::new_v4(),
        );

        // 记录使用
        knowledge.record_use(true);
        knowledge.record_use(true);
        knowledge.record_use(false);

        let result = validator.validate(&knowledge, ValidationMethod::Empirical);
        // 成功率 2/3 ≈ 0.67
        assert!(result.score > 0.5 && result.score < 0.8);
    }

    #[test]
    fn test_valid_score_calculation() {
        let config = KnowledgeConfig::default();
        let validator = KnowledgeValidator::new(config);

        let mut knowledge = Knowledge::new(
            "test".to_string(),
            KnowledgeType::Skill,
            Uuid::new_v4(),
        );
        knowledge.verified = true;
        knowledge.trust_score = 0.8;
        knowledge.record_use(true);

        let valid_score = validator.get_valid_score(&knowledge);
        assert!(valid_score > 0.0 && valid_score <= 1.0);
    }

    #[test]
    fn test_trust_entropy_calculation() {
        let mut trust = KnowledgeTrust::new();
        trust.add_source(Uuid::new_v4(), 0.5, 1);
        trust.add_source(Uuid::new_v4(), 0.5, 2);

        let entropy = trust.calculate_entropy();
        // 最大熵在均匀分布时
        assert!(entropy > 0.0);
    }
}
