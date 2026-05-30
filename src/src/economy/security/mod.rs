//! 安全模块 - 螺丝咕姆安全协议设计
//!
//! 五层防护机制：质量→相似度→行为→信任→审计

pub mod quality;
pub mod similarity;
pub mod behavior;
pub mod trust;
pub mod audit;

pub use quality::{QualityDetector, QualityScore};
pub use similarity::{SimilarityDetector, SimilarityResult};
pub use behavior::{BehaviorAnalyzer, BehaviorPattern};
pub use trust::{TrustSystem, TrustScore};
pub use audit::{AuditLog, AuditEntry};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::config::SecurityConfig;

/// 五层防护结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCheckResult {
    /// 是否通过
    pub passed: bool,
    /// 质量检测结果
    pub quality: QualityScore,
    /// 相似度检测结果
    pub similarity: SimilarityResult,
    /// 行为分析结果
    pub behavior: BehaviorPattern,
    /// 信任评分
    pub trust: TrustScore,
    /// 风险评分
    pub risk_score: f64,
}

/// 安全系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySystem {
    /// 配置
    config: SecurityConfig,
    /// 质量检测器
    quality_detector: QualityDetector,
    /// 相似度检测器
    similarity_detector: SimilarityDetector,
    /// 行为分析器
    behavior_analyzer: BehaviorAnalyzer,
    /// 信任系统
    trust_system: TrustSystem,
    /// 审计日志
    audit_log: AuditLog,
}

impl SecuritySystem {
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            quality_detector: QualityDetector::new(config.quality_threshold),
            similarity_detector: SimilarityDetector::new(
                config.similarity_threshold_base,
                config.max_similarity,
                config.trust_alpha,
                config.frequency_beta,
            ),
            behavior_analyzer: BehaviorAnalyzer::new(),
            trust_system: TrustSystem::new(config.trust_decay_rate),
            audit_log: AuditLog::new(),
            config,
        }
    }

    /// 执行五层防护检查
    pub fn check(
        &self,
        sender: Uuid,
        content: &str,
        history: &[String],
    ) -> SecurityCheckResult {
        // 第一层：内容质量检测
        let quality = self.quality_detector.detect(content);

        // 第二层：相似度检测
        let sender_history = self.similarity_detector.get_sender_history(&sender);
        let similarity = self.similarity_detector.detect(content, history, &sender_history);

        // 第三层：行为模式分析
        let behavior = self.behavior_analyzer.analyze(&sender, content);

        // 第四层：信任评分
        let trust = self.trust_system.get_score(&sender);

        // 计算风险评分
        let risk_score = self.calculate_risk(&quality, &similarity, &behavior, &trust);

        // 判断是否通过
        let passed = quality.score >= self.config.quality_threshold
            && !similarity.is_violation
            && risk_score < 1.0;

        SecurityCheckResult {
            passed,
            quality,
            similarity,
            behavior,
            trust,
            risk_score,
        }
    }

    /// 计算风险评分: Risk = w₁·Q_violation + w₂·Sim_violation + w₃·B_anomaly + w₄·Frequency
    fn calculate_risk(
        &self,
        quality: &QualityScore,
        similarity: &SimilarityResult,
        behavior: &BehaviorPattern,
        trust: &TrustScore,
    ) -> f64 {
        let q_violation = if quality.score < self.config.quality_threshold { 1.0 } else { 0.0 };
        let sim_violation = if similarity.is_violation { 1.0 } else { 0.0 };
        let b_anomaly = if behavior.is_anomaly { 1.0 } else { 0.0 };
        let frequency = behavior.frequency_factor;

        0.3 * q_violation + 0.3 * sim_violation + 0.2 * b_anomaly + 0.2 * frequency - 0.5 * trust.score
    }

    /// 记录审计日志
    pub fn log_action(&self, user: Uuid, action: &str, content_hash: &str) -> Self {
        let mut new_system = self.clone();
        let entry = AuditEntry::new(user, action, content_hash);
        new_system.audit_log = self.audit_log.add(entry);
        new_system
    }

    /// 更新信任评分
    pub fn update_trust(&self, user: Uuid, is_valid: bool) -> Self {
        let mut new_system = self.clone();
        new_system.trust_system = self.trust_system.update(&user, is_valid);
        new_system
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_system_creation() {
        let config = SecurityConfig::default();
        let system = SecuritySystem::new(config);
        assert!(system.config.max_similarity == 0.8);
    }

    #[test]
    fn test_security_check() {
        let config = SecurityConfig::default();
        let system = SecuritySystem::new(config);
        let sender = Uuid::new_v4();

        let result = system.check(sender, "这是一条有意义的测试内容", &[]);

        // 新内容应该通过相似度检测
        assert!(!result.similarity.is_violation);
    }
}
