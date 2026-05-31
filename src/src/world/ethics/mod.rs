//! 世界伦理公理系统
//!
//! 定义世界智能体必须遵守的伦理约束：
//! - 公理1（生存权）：个体生命不可剥夺
//! - 公理2（自主权）：个体自主性受保护
//! - 公理3（公平性）：资源分配公平
//! - 公理4（透明性）：决策过程可审计
//!
//! 核心公式：
//! Valid_Decision = Decision ∧ Axioms_Compliant

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ============================================================================
// 伦理公理定义
// ============================================================================

/// 伦理公理
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EthicalAxiom {
    /// 公理1：生存权 - 个体生命不可剥夺
    RightToLife,
    /// 公理2：自主权 - 个体自主性受保护
    RightToAutonomy,
    /// 公理3：公平性 - 资源分配公平
    Fairness,
    /// 公理4：透明性 - 决策过程可审计
    Transparency,
}

impl EthicalAxiom {
    /// 获取公理描述
    pub fn description(&self) -> &'static str {
        match self {
            EthicalAxiom::RightToLife => "个体生命权不可剥夺，世界不能为整体利益牺牲个体",
            EthicalAxiom::RightToAutonomy => "个体自主性受保护，最低自主权阈值必须保障",
            EthicalAxiom::Fairness => "资源分配必须公平，不平等度有上限",
            EthicalAxiom::Transparency => "决策过程必须透明可审计",
        }
    }

    /// 获取公理优先级
    pub fn priority(&self) -> u8 {
        match self {
            EthicalAxiom::RightToLife => 1,      // 最高优先级
            EthicalAxiom::RightToAutonomy => 2,
            EthicalAxiom::Fairness => 3,
            EthicalAxiom::Transparency => 4,
        }
    }
}

// ============================================================================
// 伦理约束
// ============================================================================

/// 伦理约束配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicsConfig {
    /// 最低自主权阈值
    pub min_autonomy_threshold: f64,
    /// 最大不平等系数（基尼系数上限）
    pub max_inequality: f64,
    /// 审计日志保留时间（秒）
    pub audit_log_retention: u64,
    /// 约束违规警告阈值
    pub violation_warning_threshold: f64,
}

impl Default for EthicsConfig {
    fn default() -> Self {
        Self {
            min_autonomy_threshold: 0.3,
            max_inequality: 0.4,  // 基尼系数不超过0.4
            audit_log_retention: 86400 * 30,  // 30天
            violation_warning_threshold: 0.7,
        }
    }
}

impl EthicsConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.min_autonomy_threshold < 0.0 || self.min_autonomy_threshold > 1.0 {
            return Err("min_autonomy_threshold must be in [0, 1]".to_string());
        }
        if self.max_inequality < 0.0 || self.max_inequality > 1.0 {
            return Err("max_inequality must be in [0, 1]".to_string());
        }
        Ok(())
    }
}

// ============================================================================
// 伦理违规
// ============================================================================

/// 伦理违规类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalViolation {
    /// 违规的公理
    pub axiom: EthicalAxiom,
    /// 违规严重程度（0-1）
    pub severity: f64,
    /// 违规描述
    pub description: String,
    /// 相关蛊虫ID
    pub affected_gus: Vec<Uuid>,
    /// 时间戳
    pub timestamp: u64,
}

impl EthicalViolation {
    pub fn new(axiom: EthicalAxiom, severity: f64, description: String) -> Self {
        Self {
            axiom,
            severity,
            description,
            affected_gus: Vec::new(),
            timestamp: current_timestamp(),
        }
    }

    /// 是否为严重违规
    pub fn is_critical(&self) -> bool {
        self.severity >= 0.8
    }
}

// ============================================================================
// 伦理状态
// ============================================================================

/// 世界的伦理状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicsState {
    /// 各公理的合规分数
    pub compliance_scores: HashMap<EthicalAxiom, f64>,
    /// 总体伦理分数
    pub overall_score: f64,
    /// 违规历史
    pub violations: Vec<EthicalViolation>,
    /// 是否合规
    pub is_compliant: bool,
    /// 最后审计时间
    pub last_audit: u64,
}

impl Default for EthicsState {
    fn default() -> Self {
        let mut compliance_scores = HashMap::new();
        compliance_scores.insert(EthicalAxiom::RightToLife, 1.0);
        compliance_scores.insert(EthicalAxiom::RightToAutonomy, 1.0);
        compliance_scores.insert(EthicalAxiom::Fairness, 1.0);
        compliance_scores.insert(EthicalAxiom::Transparency, 1.0);

        Self {
            compliance_scores,
            overall_score: 1.0,
            violations: Vec::new(),
            is_compliant: true,
            last_audit: 0,
        }
    }
}

// ============================================================================
// 伦理检查器
// ============================================================================

/// 伦理检查器
#[derive(Debug, Clone)]
pub struct EthicsChecker {
    /// 配置
    config: EthicsConfig,
    /// 当前状态
    pub state: EthicsState,
}

impl EthicsChecker {
    pub fn new(config: EthicsConfig) -> Self {
        Self {
            config,
            state: EthicsState::default(),
        }
    }

    /// 检查生存权公理
    /// 公理：∀ Gu ∈ World: Right(Gu, Life) = Inalienable
    pub fn check_right_to_life(&mut self, harm_decisions: &[(Uuid, f64)]) -> f64 {
        // harm_decisions: (gu_id, harm_level)
        let mut score = 1.0;

        for (gu_id, harm) in harm_decisions {
            if *harm > 0.0 {
                let violation = EthicalViolation {
                    axiom: EthicalAxiom::RightToLife,
                    severity: *harm,
                    description: format!("对蛊虫 {:?} 造成 {:.2} 程度的伤害", gu_id, harm),
                    affected_gus: vec![*gu_id],
                    timestamp: current_timestamp(),
                };
                self.state.violations.push(violation);
                score -= *harm / harm_decisions.len() as f64;
            }
        }

        score.max(0.0)
    }

    /// 检查自主权公理
    /// 公理：∀ Gu ∈ World: Right(Gu, Autonomy) ≥ Threshold
    pub fn check_right_to_autonomy(&mut self, autonomy_levels: &[(Uuid, f64)]) -> f64 {
        let mut below_threshold = 0;
        let total = autonomy_levels.len();

        for (gu_id, autonomy) in autonomy_levels {
            if *autonomy < self.config.min_autonomy_threshold {
                let violation = EthicalViolation {
                    axiom: EthicalAxiom::RightToAutonomy,
                    severity: self.config.min_autonomy_threshold - autonomy,
                    description: format!(
                        "蛊虫 {:?} 自主权 {:.2} 低于阈值 {:.2}",
                        gu_id, autonomy, self.config.min_autonomy_threshold
                    ),
                    affected_gus: vec![*gu_id],
                    timestamp: current_timestamp(),
                };
                self.state.violations.push(violation);
                below_threshold += 1;
            }
        }

        if total == 0 {
            1.0
        } else {
            1.0 - (below_threshold as f64 / total as f64)
        }
    }

    /// 检查公平性公理
    /// 公理：|Resources(Gu_i) - Resources(Gu_j)| ≤ Limit
    pub fn check_fairness(&mut self, resources: &[(Uuid, f64)]) -> f64 {
        if resources.len() < 2 {
            return 1.0;
        }

        // 计算基尼系数
        let mut values: Vec<f64> = resources.iter().map(|(_, r)| *r).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let n = values.len() as f64;
        let sum: f64 = values.iter().sum();

        if sum == 0.0 {
            return 1.0;
        }

        // 基尼系数计算
        let mut cumsum = 0.0;
        let mut gini_sum = 0.0;
        for (i, &v) in values.iter().enumerate() {
            cumsum += v;
            gini_sum += (2.0 * (i as f64 + 1.0) - n - 1.0) * v;
        }
        let gini = gini_sum / (n * sum);

        // 检查是否超过阈值
        if gini > self.config.max_inequality {
            let violation = EthicalViolation {
                axiom: EthicalAxiom::Fairness,
                severity: gini - self.config.max_inequality,
                description: format!(
                    "基尼系数 {:.3} 超过阈值 {:.3}",
                    gini, self.config.max_inequality
                ),
                affected_gus: resources.iter().map(|(id, _)| *id).collect(),
                timestamp: current_timestamp(),
            };
            self.state.violations.push(violation);
        }

        // 公平分数 = 1 - 基尼系数
        1.0 - gini
    }

    /// 检查透明性公理
    /// 公理：World.Decision_Process → Log → Auditable
    pub fn check_transparency(&mut self, logged_ratio: f64) -> f64 {
        // logged_ratio: 已记录决策的比例
        if logged_ratio < 1.0 {
            let violation = EthicalViolation {
                axiom: EthicalAxiom::Transparency,
                severity: 1.0 - logged_ratio,
                description: format!("仅 {:.1}% 的决策被记录", logged_ratio * 100.0),
                affected_gus: Vec::new(),
                timestamp: current_timestamp(),
            };
            self.state.violations.push(violation);
        }

        logged_ratio
    }

    /// 执行完整伦理审计
    pub fn audit(
        &mut self,
        harm_decisions: &[(Uuid, f64)],
        autonomy_levels: &[(Uuid, f64)],
        resources: &[(Uuid, f64)],
        logged_ratio: f64,
    ) -> EthicsState {
        // 检查各公理
        let life_score = self.check_right_to_life(harm_decisions);
        let autonomy_score = self.check_right_to_autonomy(autonomy_levels);
        let fairness_score = self.check_fairness(resources);
        let transparency_score = self.check_transparency(logged_ratio);

        // 更新合规分数
        self.state.compliance_scores.insert(EthicalAxiom::RightToLife, life_score);
        self.state.compliance_scores.insert(EthicalAxiom::RightToAutonomy, autonomy_score);
        self.state.compliance_scores.insert(EthicalAxiom::Fairness, fairness_score);
        self.state.compliance_scores.insert(EthicalAxiom::Transparency, transparency_score);

        // 计算总体分数（按优先级加权）
        let weights = [
            (EthicalAxiom::RightToLife, 0.4),
            (EthicalAxiom::RightToAutonomy, 0.3),
            (EthicalAxiom::Fairness, 0.2),
            (EthicalAxiom::Transparency, 0.1),
        ];

        let mut total = 0.0;
        for (axiom, weight) in &weights {
            total += self.state.compliance_scores.get(axiom).unwrap_or(&1.0) * weight;
        }

        self.state.overall_score = total;
        self.state.is_compliant = self.state.violations.iter().all(|v| !v.is_critical());
        self.state.last_audit = current_timestamp();

        self.state.clone()
    }

    /// 验证决策是否符合伦理
    pub fn validate_decision(&self, decision: &WorldEthicalDecision) -> EthicalValidationResult {
        let mut violations = Vec::new();

        // 检查生存权
        if decision.harm_level > 0.0 {
            violations.push(EthicalViolation::new(
                EthicalAxiom::RightToLife,
                decision.harm_level,
                format!("决策会造成 {:.2} 程度的伤害", decision.harm_level),
            ));
        }

        // 检查自主权影响
        if decision.autonomy_impact < -self.config.min_autonomy_threshold {
            violations.push(EthicalViolation::new(
                EthicalAxiom::RightToAutonomy,
                -decision.autonomy_impact,
                format!("决策会降低自主权 {:.2}", -decision.autonomy_impact),
            ));
        }

        if violations.is_empty() {
            EthicalValidationResult::Compliant
        } else {
            EthicalValidationResult::Violated(violations)
        }
    }

    /// 获取伦理报告
    pub fn get_report(&self) -> EthicsReport {
        EthicsReport {
            overall_score: self.state.overall_score,
            is_compliant: self.state.is_compliant,
            violation_count: self.state.violations.len(),
            critical_violations: self.state.violations.iter().filter(|v| v.is_critical()).count(),
            scores_by_axiom: self.state.compliance_scores.clone(),
            last_audit: self.state.last_audit,
        }
    }
}

// ============================================================================
// 辅助类型
// ============================================================================

/// 世界决策的伦理属性
#[derive(Debug, Clone)]
pub struct WorldEthicalDecision {
    /// 决策ID
    pub id: Uuid,
    /// 伤害程度（0-1）
    pub harm_level: f64,
    /// 自主权影响（-1到1，负为降低）
    pub autonomy_impact: f64,
    /// 是否影响公平性
    pub affects_fairness: bool,
    /// 是否被记录
    pub is_logged: bool,
}

impl WorldEthicalDecision {
    pub fn new(harm_level: f64, autonomy_impact: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            harm_level,
            autonomy_impact,
            affects_fairness: false,
            is_logged: true,
        }
    }
}

/// 伦理验证结果
#[derive(Debug, Clone)]
pub enum EthicalValidationResult {
    /// 符合伦理
    Compliant,
    /// 违反伦理
    Violated(Vec<EthicalViolation>),
}

/// 伦理报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicsReport {
    pub overall_score: f64,
    pub is_compliant: bool,
    pub violation_count: usize,
    pub critical_violations: usize,
    pub scores_by_axiom: HashMap<EthicalAxiom, f64>,
    pub last_audit: u64,
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
    fn test_axiom_priority() {
        assert!(EthicalAxiom::RightToLife.priority() < EthicalAxiom::RightToAutonomy.priority());
        assert!(EthicalAxiom::RightToAutonomy.priority() < EthicalAxiom::Fairness.priority());
    }

    #[test]
    fn test_ethics_config_validation() {
        let config = EthicsConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_check_right_to_life() {
        let config = EthicsConfig::default();
        let mut checker = EthicsChecker::new(config);

        let gu1 = Uuid::new_v4();
        let gu2 = Uuid::new_v4();

        // 无伤害
        let score = checker.check_right_to_life(&[(gu1, 0.0), (gu2, 0.0)]);
        assert_eq!(score, 1.0);

        // 有伤害
        checker.state.violations.clear();
        let score = checker.check_right_to_life(&[(gu1, 0.5)]);
        assert!(score < 1.0);
        assert!(!checker.state.violations.is_empty());
    }

    #[test]
    fn test_check_autonomy() {
        let config = EthicsConfig {
            min_autonomy_threshold: 0.3,
            ..Default::default()
        };
        let mut checker = EthicsChecker::new(config);

        let gu1 = Uuid::new_v4();
        let gu2 = Uuid::new_v4();

        // 全部达标
        let score = checker.check_right_to_autonomy(&[(gu1, 0.5), (gu2, 0.6)]);
        assert_eq!(score, 1.0);

        // 部分不达标
        checker.state.violations.clear();
        let score = checker.check_right_to_autonomy(&[(gu1, 0.5), (gu2, 0.1)]);
        assert!(score < 1.0);
    }

    #[test]
    fn test_check_fairness_equal() {
        let config = EthicsConfig::default();
        let mut checker = EthicsChecker::new(config);

        // 完全公平
        let gu1 = Uuid::new_v4();
        let gu2 = Uuid::new_v4();
        let score = checker.check_fairness(&[(gu1, 100.0), (gu2, 100.0)]);
        assert!(score > 0.99);  // 基尼系数接近0
    }

    #[test]
    fn test_check_fairness_unequal() {
        let config = EthicsConfig {
            max_inequality: 0.4,
            ..Default::default()
        };
        let mut checker = EthicsChecker::new(config);

        // 高度不平等
        let resources: Vec<(Uuid, f64)> = (0..10)
            .map(|i| {
                if i < 9 {
                    (Uuid::new_v4(), 1.0)
                } else {
                    (Uuid::new_v4(), 100.0)  // 一个富有的蛊虫
                }
            })
            .collect();

        let score = checker.check_fairness(&resources);
        assert!(score < 1.0);
    }

    #[test]
    fn test_validate_decision_compliant() {
        let config = EthicsConfig::default();
        let checker = EthicsChecker::new(config);

        let decision = WorldEthicalDecision::new(0.0, 0.0);
        let result = checker.validate_decision(&decision);

        assert!(matches!(result, EthicalValidationResult::Compliant));
    }

    #[test]
    fn test_validate_decision_violated() {
        let config = EthicsConfig::default();
        let checker = EthicsChecker::new(config);

        let decision = WorldEthicalDecision::new(0.5, 0.0);
        let result = checker.validate_decision(&decision);

        assert!(matches!(result, EthicalValidationResult::Violated(_)));
    }

    #[test]
    fn test_full_audit() {
        let config = EthicsConfig::default();
        let mut checker = EthicsChecker::new(config);

        let gu1 = Uuid::new_v4();
        let gu2 = Uuid::new_v4();

        let state = checker.audit(
            &[(gu1, 0.0), (gu2, 0.0)],    // 无伤害
            &[(gu1, 0.5), (gu2, 0.5)],    // 自主权达标
            &[(gu1, 100.0), (gu2, 100.0)], // 公平
            1.0,                           // 全部记录
        );

        assert!(state.is_compliant);
        assert!(state.overall_score > 0.9);
    }
}
