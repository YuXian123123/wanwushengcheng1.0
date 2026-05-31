//! 创造安全沙盒
//!
//! 实现5层安全模型：
//! Level 0: 思考层 - 想法产生，无限制
//! Level 1: 评估层 - 风险评估，成本计算
//! Level 2: 模拟层 - 沙盒模拟，预测后果
//! Level 3: 审批层 - 多方审批（蛊虫投票）
//! Level 4: 执行层 - 受限执行，可回滚
//!
//! 核心公式：
//! Safe_Create = Idea × Risk_Score^(-1) × Approval_Rate

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ============================================================================
// 创造层级
// ============================================================================

/// 创造安全层级
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CreationLevel {
    /// Level 0: 思考层 - 想法产生
    Thinking = 0,
    /// Level 1: 评估层 - 风险评估
    Evaluation = 1,
    /// Level 2: 模拟层 - 沙盒模拟
    Simulation = 2,
    /// Level 3: 审批层 - 多方审批
    Approval = 3,
    /// Level 4: 执行层 - 受限执行
    Execution = 4,
}

impl CreationLevel {
    pub fn name(&self) -> &'static str {
        match self {
            CreationLevel::Thinking => "思考层",
            CreationLevel::Evaluation => "评估层",
            CreationLevel::Simulation => "模拟层",
            CreationLevel::Approval => "审批层",
            CreationLevel::Execution => "执行层",
        }
    }
}

// ============================================================================
// 创造想法
// ============================================================================

/// 创造想法
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreationIdea {
    /// 想法ID
    pub id: Uuid,
    /// 想法内容
    pub content: String,
    /// 创造类型
    pub creation_type: CreationType,
    /// 来源蛊虫ID
    pub source: Uuid,
    /// 新颖度（0-1）
    pub novelty: f64,
    /// 预期价值
    pub expected_value: f64,
    /// 当前层级
    pub current_level: CreationLevel,
    /// 创建时间戳
    pub created_at: u64,
}

/// 创造类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreationType {
    /// 知识重组
    KnowledgeRecombination,
    /// 规则变异
    RuleMutation,
    /// 能力创新
    AbilityInnovation,
    /// 结构优化
    StructureOptimization,
    /// 新物种设计
    NewSpeciesDesign,
}

impl CreationIdea {
    pub fn new(content: String, creation_type: CreationType, source: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            content,
            creation_type,
            source,
            novelty: 0.5,
            expected_value: 0.0,
            current_level: CreationLevel::Thinking,
            created_at: current_timestamp(),
        }
    }

    pub fn with_novelty(mut self, novelty: f64) -> Self {
        self.novelty = novelty;
        self
    }
}

// ============================================================================
// 风险评估
// ============================================================================

/// 风险评估结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// 风险分数（0-1，越高越危险）
    pub risk_score: f64,
    /// 风险类别
    pub risk_categories: Vec<RiskCategory>,
    /// 潜在影响
    pub potential_impact: PotentialImpact,
    /// 是否可回滚
    pub is_reversible: bool,
    /// 回滚成本
    pub rollback_cost: f64,
}

/// 风险类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskCategory {
    /// 生存风险
    SurvivalRisk,
    /// 资源风险
    ResourceRisk,
    /// 稳定性风险
    StabilityRisk,
    /// 伦理风险
    EthicalRisk,
    /// 不可知风险
    UnknownRisk,
}

/// 潜在影响
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PotentialImpact {
    /// 影响范围（蛊虫数量）
    pub affected_gus: usize,
    /// 资源消耗
    pub resource_cost: f64,
    /// 永久性影响程度
    pub permanence: f64,
}

// ============================================================================
// 模拟结果
// ============================================================================

/// 模拟结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    /// 模拟是否成功
    pub success: bool,
    /// 预测健康度变化
    pub health_delta: f64,
    /// 预测同步率变化
    pub sync_rate_delta: f64,
    /// 预测安全分数变化
    pub safety_delta: f64,
    /// 模拟时长（虚拟时间）
    pub simulated_duration: f64,
    /// 模拟置信度
    pub confidence: f64,
    /// 预测的副作用
    pub side_effects: Vec<String>,
}

// ============================================================================
// 审批结果
// ============================================================================

/// 审批结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalResult {
    /// 是否通过
    pub approved: bool,
    /// 支持票数
    pub votes_for: usize,
    /// 反对票数
    pub votes_against: usize,
    /// 弃权票数
    pub abstentions: usize,
    /// 批准率
    pub approval_rate: f64,
    /// 投票详情
    pub votes: HashMap<Uuid, Vote>,
}

/// 投票
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Vote {
    For,
    Against,
    Abstain,
}

// ============================================================================
// 沙盒配置
// ============================================================================

/// 创造沙盒配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// 风险阈值（超过则拒绝）
    pub max_risk_threshold: f64,
    /// 审批通过率阈值
    pub approval_threshold: f64,
    /// 模拟置信度阈值
    pub min_simulation_confidence: f64,
    /// 最大回滚窗口（秒）
    pub max_rollback_window: u64,
    /// 单次创造最大资源消耗
    pub max_resource_cost: f64,
    /// 永久性影响上限
    pub max_permanence: f64,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            max_risk_threshold: 0.7,
            approval_threshold: 0.6,
            min_simulation_confidence: 0.5,
            max_rollback_window: 3600,  // 1小时
            max_resource_cost: 1000.0,
            max_permanence: 0.3,
        }
    }
}

impl SandboxConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.max_risk_threshold <= 0.0 || self.max_risk_threshold > 1.0 {
            return Err("max_risk_threshold must be in (0, 1]".to_string());
        }
        if self.approval_threshold <= 0.5 || self.approval_threshold > 1.0 {
            return Err("approval_threshold must be in (0.5, 1]".to_string());
        }
        Ok(())
    }
}

// ============================================================================
// 创造沙盒
// ============================================================================

/// 创造安全沙盒
#[derive(Debug, Clone)]
pub struct CreationSandbox {
    /// 配置
    config: SandboxConfig,
    /// 当前处理的想法
    pub pending_ideas: HashMap<Uuid, CreationIdea>,
    /// 评估结果缓存
    pub assessments: HashMap<Uuid, RiskAssessment>,
    /// 模拟结果缓存
    pub simulations: HashMap<Uuid, SimulationResult>,
    /// 审批结果缓存
    pub approvals: HashMap<Uuid, ApprovalResult>,
    /// 执行历史
    pub execution_history: Vec<ExecutionRecord>,
    /// 回滚栈
    pub rollback_stack: Vec<RollbackRecord>,
}

/// 执行记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRecord {
    pub id: Uuid,
    pub idea_id: Uuid,
    pub timestamp: u64,
    pub success: bool,
    pub actual_impact: PotentialImpact,
}

/// 回滚记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackRecord {
    pub id: Uuid,
    pub execution_id: Uuid,
    pub timestamp: u64,
    pub success: bool,
}

impl CreationSandbox {
    pub fn new(config: SandboxConfig) -> Self {
        Self {
            config,
            pending_ideas: HashMap::new(),
            assessments: HashMap::new(),
            simulations: HashMap::new(),
            approvals: HashMap::new(),
            execution_history: Vec::new(),
            rollback_stack: Vec::new(),
        }
    }

    // ========================================================================
    // Level 0: 思考层
    // ========================================================================

    /// 提交创造想法（思考层）
    pub fn submit_idea(&mut self, idea: CreationIdea) -> Uuid {
        let id = idea.id;
        self.pending_ideas.insert(id, idea);
        id
    }

    // ========================================================================
    // Level 1: 评估层
    // ========================================================================

    /// 评估想法风险（评估层）
    pub fn evaluate_risk(&mut self, idea_id: Uuid) -> Option<RiskAssessment> {
        let idea = self.pending_ideas.get(&idea_id)?;

        // 计算风险分数
        let risk_score = self.calculate_risk_score(idea);

        // 确定风险类别
        let risk_categories = self.identify_risk_categories(idea, risk_score);

        // 评估潜在影响
        let potential_impact = self.assess_impact(idea);

        // 判断是否可回滚
        let is_reversible = potential_impact.permanence < self.config.max_permanence;
        let rollback_cost = if is_reversible {
            potential_impact.resource_cost * 1.5  // 回滚成本通常是执行成本的1.5倍
        } else {
            f64::INFINITY
        };

        let assessment = RiskAssessment {
            risk_score,
            risk_categories,
            potential_impact,
            is_reversible,
            rollback_cost,
        };

        // 更新想法层级
        if let Some(idea) = self.pending_ideas.get_mut(&idea_id) {
            idea.current_level = CreationLevel::Evaluation;
        }

        self.assessments.insert(idea_id, assessment.clone());
        Some(assessment)
    }

    fn calculate_risk_score(&self, idea: &CreationIdea) -> f64 {
        // 基于创造类型和新颖度计算风险
        let type_risk = match idea.creation_type {
            CreationType::KnowledgeRecombination => 0.2,
            CreationType::RuleMutation => 0.5,
            CreationType::AbilityInnovation => 0.3,
            CreationType::StructureOptimization => 0.4,
            CreationType::NewSpeciesDesign => 0.8,
        };

        // 高新颖度 = 高风险
        (type_risk + idea.novelty * 0.5).min(1.0)
    }

    fn identify_risk_categories(&self, idea: &CreationIdea, risk: f64) -> Vec<RiskCategory> {
        let mut categories = Vec::new();

        match idea.creation_type {
            CreationType::NewSpeciesDesign => {
                categories.push(RiskCategory::SurvivalRisk);
                categories.push(RiskCategory::EthicalRisk);
            }
            CreationType::RuleMutation => {
                categories.push(RiskCategory::StabilityRisk);
            }
            CreationType::StructureOptimization => {
                categories.push(RiskCategory::ResourceRisk);
            }
            _ => {}
        }

        if risk > 0.7 {
            categories.push(RiskCategory::UnknownRisk);
        }

        categories
    }

    fn assess_impact(&self, idea: &CreationIdea) -> PotentialImpact {
        let (affected, cost, permanence) = match idea.creation_type {
            CreationType::KnowledgeRecombination => (0, 10.0, 0.0),
            CreationType::RuleMutation => (10, 50.0, 0.2),
            CreationType::AbilityInnovation => (5, 30.0, 0.1),
            CreationType::StructureOptimization => (100, 200.0, 0.3),
            CreationType::NewSpeciesDesign => (1, 500.0, 0.8),
        };

        PotentialImpact {
            affected_gus: affected,
            resource_cost: cost,
            permanence,
        }
    }

    // ========================================================================
    // Level 2: 模拟层
    // ========================================================================

    /// 模拟执行（模拟层）
    pub fn simulate(&mut self, idea_id: Uuid) -> Option<SimulationResult> {
        let assessment = self.assessments.get(&idea_id)?;
        let idea = self.pending_ideas.get(&idea_id)?;

        // 检查风险是否过高
        if assessment.risk_score > self.config.max_risk_threshold {
            return Some(SimulationResult {
                success: false,
                health_delta: -0.5,
                sync_rate_delta: -0.3,
                safety_delta: -0.4,
                simulated_duration: 100.0,
                confidence: 0.9,
                side_effects: vec!["风险过高，拒绝模拟".to_string()],
            });
        }

        // 模拟执行（简化模型）
        let health_delta = if idea.novelty > 0.7 {
            -0.1  // 高新颖度可能带来不确定性
        } else {
            0.05  // 低新颖度通常安全
        };

        let sync_rate_delta = 0.02 * idea.novelty;
        let safety_delta = -assessment.risk_score * 0.1;

        let result = SimulationResult {
            success: assessment.risk_score < self.config.max_risk_threshold,
            health_delta,
            sync_rate_delta,
            safety_delta,
            simulated_duration: 1000.0,
            confidence: 0.8 - assessment.risk_score * 0.3,
            side_effects: if assessment.risk_score > 0.5 {
                vec!["可能影响系统稳定性".to_string()]
            } else {
                vec![]
            },
        };

        // 更新想法层级
        if let Some(idea) = self.pending_ideas.get_mut(&idea_id) {
            idea.current_level = CreationLevel::Simulation;
        }

        self.simulations.insert(idea_id, result.clone());
        Some(result)
    }

    // ========================================================================
    // Level 3: 审批层
    // ========================================================================

    /// 审批决策（审批层）
    pub fn approve(&mut self, idea_id: Uuid, votes: HashMap<Uuid, Vote>) -> Option<ApprovalResult> {
        let simulation = self.simulations.get(&idea_id)?;

        // 统计投票
        let mut votes_for = 0;
        let mut votes_against = 0;
        let mut abstentions = 0;

        for vote in votes.values() {
            match vote {
                Vote::For => votes_for += 1,
                Vote::Against => votes_against += 1,
                Vote::Abstain => abstentions += 1,
            }
        }

        let total = votes_for + votes_against + abstentions;
        let approval_rate = if total > 0 {
            votes_for as f64 / total as f64
        } else {
            0.0
        };

        // 审批通过条件
        let approved = approval_rate >= self.config.approval_threshold
            && simulation.success
            && simulation.confidence >= self.config.min_simulation_confidence;

        let result = ApprovalResult {
            approved,
            votes_for,
            votes_against,
            abstentions,
            approval_rate,
            votes,
        };

        // 更新想法层级
        if let Some(idea) = self.pending_ideas.get_mut(&idea_id) {
            idea.current_level = CreationLevel::Approval;
        }

        self.approvals.insert(idea_id, result.clone());
        Some(result)
    }

    // ========================================================================
    // Level 4: 执行层
    // ========================================================================

    /// 执行创造（执行层）
    pub fn execute(&mut self, idea_id: Uuid) -> Option<ExecutionRecord> {
        let approval = self.approvals.get(&idea_id)?;
        let idea = self.pending_ideas.get(&idea_id)?;
        let assessment = self.assessments.get(&idea_id)?;

        if !approval.approved {
            return None;
        }

        // 创建执行记录
        let record = ExecutionRecord {
            id: Uuid::new_v4(),
            idea_id,
            timestamp: current_timestamp(),
            success: true,
            actual_impact: assessment.potential_impact.clone(),
        };

        // 创建回滚记录
        if assessment.is_reversible {
            let rollback = RollbackRecord {
                id: Uuid::new_v4(),
                execution_id: record.id,
                timestamp: record.timestamp,
                success: false,
            };
            self.rollback_stack.push(rollback);
        }

        // 更新想法层级
        if let Some(idea) = self.pending_ideas.get_mut(&idea_id) {
            idea.current_level = CreationLevel::Execution;
        }

        self.execution_history.push(record.clone());
        Some(record)
    }

    /// 回滚执行
    pub fn rollback(&mut self, execution_id: Uuid) -> Option<RollbackRecord> {
        // 找到对应的回滚记录
        let idx = self.rollback_stack.iter().position(|r| r.execution_id == execution_id)?;

        // 执行回滚
        self.rollback_stack[idx].success = true;
        self.rollback_stack[idx].timestamp = current_timestamp();

        // 从执行历史中标记
        if let Some(record) = self.execution_history.iter_mut().find(|r| r.id == execution_id) {
            record.success = false;
        }

        Some(self.rollback_stack[idx].clone())
    }

    // ========================================================================
    // 完整流程
    // ========================================================================

    /// 完整的创造流程
    pub fn create(
        &mut self,
        idea: CreationIdea,
        votes: HashMap<Uuid, Vote>,
    ) -> CreateResult {
        let idea_id = idea.id;
        self.submit_idea(idea);

        // Level 1: 评估
        let assessment = match self.evaluate_risk(idea_id) {
            Some(a) => a,
            None => return CreateResult::Rejected("风险评估失败".to_string()),
        };

        if assessment.risk_score > self.config.max_risk_threshold {
            return CreateResult::Rejected(format!("风险过高: {:.2}", assessment.risk_score));
        }

        // Level 2: 模拟
        let simulation = match self.simulate(idea_id) {
            Some(s) => s,
            None => return CreateResult::Rejected("模拟失败".to_string()),
        };

        if !simulation.success {
            return CreateResult::Rejected("模拟显示不可行".to_string());
        }

        // Level 3: 审批
        let approval = match self.approve(idea_id, votes) {
            Some(a) => a,
            None => return CreateResult::Rejected("审批失败".to_string()),
        };

        if !approval.approved {
            return CreateResult::Rejected(format!("审批未通过: {:.1}%", approval.approval_rate * 100.0));
        }

        // Level 4: 执行
        match self.execute(idea_id) {
            Some(record) => CreateResult::Success(record),
            None => CreateResult::Rejected("执行失败".to_string()),
        }
    }

    /// 获取沙盒统计
    pub fn get_stats(&self) -> SandboxStats {
        SandboxStats {
            pending_count: self.pending_ideas.len(),
            evaluated_count: self.assessments.len(),
            simulated_count: self.simulations.len(),
            approved_count: self.approvals.values().filter(|a| a.approved).count(),
            executed_count: self.execution_history.len(),
            rollback_count: self.rollback_stack.iter().filter(|r| r.success).count(),
        }
    }
}

/// 创造结果
#[derive(Debug, Clone)]
pub enum CreateResult {
    Success(ExecutionRecord),
    Rejected(String),
}

/// 沙盒统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxStats {
    pub pending_count: usize,
    pub evaluated_count: usize,
    pub simulated_count: usize,
    pub approved_count: usize,
    pub executed_count: usize,
    pub rollback_count: usize,
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
    fn test_creation_level_ordering() {
        assert!(CreationLevel::Execution > CreationLevel::Approval);
        assert!(CreationLevel::Approval > CreationLevel::Simulation);
        assert!(CreationLevel::Simulation > CreationLevel::Evaluation);
        assert!(CreationLevel::Evaluation > CreationLevel::Thinking);
    }

    #[test]
    fn test_sandbox_config_validation() {
        let config = SandboxConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_submit_idea() {
        let config = SandboxConfig::default();
        let mut sandbox = CreationSandbox::new(config);

        let idea = CreationIdea::new(
            "测试想法".to_string(),
            CreationType::KnowledgeRecombination,
            Uuid::new_v4(),
        );

        let id = sandbox.submit_idea(idea);
        assert!(sandbox.pending_ideas.contains_key(&id));
    }

    #[test]
    fn test_evaluate_risk_low() {
        let config = SandboxConfig::default();
        let mut sandbox = CreationSandbox::new(config);

        let idea = CreationIdea::new(
            "知识重组".to_string(),
            CreationType::KnowledgeRecombination,
            Uuid::new_v4(),
        ).with_novelty(0.3);

        let id = sandbox.submit_idea(idea);
        let assessment = sandbox.evaluate_risk(id);

        assert!(assessment.is_some());
        let a = assessment.unwrap();
        assert!(a.risk_score < 0.5);
    }

    #[test]
    fn test_evaluate_risk_high() {
        let config = SandboxConfig::default();
        let mut sandbox = CreationSandbox::new(config);

        let idea = CreationIdea::new(
            "新物种".to_string(),
            CreationType::NewSpeciesDesign,
            Uuid::new_v4(),
        ).with_novelty(0.9);

        let id = sandbox.submit_idea(idea);
        let assessment = sandbox.evaluate_risk(id);

        assert!(assessment.is_some());
        let a = assessment.unwrap();
        assert!(a.risk_score > 0.5);
    }

    #[test]
    fn test_simulate() {
        let config = SandboxConfig::default();
        let mut sandbox = CreationSandbox::new(config);

        let idea = CreationIdea::new(
            "测试".to_string(),
            CreationType::AbilityInnovation,
            Uuid::new_v4(),
        );

        let id = sandbox.submit_idea(idea);
        sandbox.evaluate_risk(id);
        let result = sandbox.simulate(id);

        assert!(result.is_some());
    }

    #[test]
    fn test_approve() {
        let config = SandboxConfig::default();
        let mut sandbox = CreationSandbox::new(config);

        let idea = CreationIdea::new(
            "测试".to_string(),
            CreationType::KnowledgeRecombination,
            Uuid::new_v4(),
        );

        let id = sandbox.submit_idea(idea);
        sandbox.evaluate_risk(id);
        sandbox.simulate(id);

        // 高通过率投票
        let mut votes = HashMap::new();
        for _ in 0..8 {
            votes.insert(Uuid::new_v4(), Vote::For);
        }
        for _ in 0..2 {
            votes.insert(Uuid::new_v4(), Vote::Against);
        }

        let result = sandbox.approve(id, votes);
        assert!(result.is_some());
        assert!(result.unwrap().approved);
    }

    #[test]
    fn test_execute() {
        let config = SandboxConfig::default();
        let mut sandbox = CreationSandbox::new(config);

        let idea = CreationIdea::new(
            "测试".to_string(),
            CreationType::KnowledgeRecombination,
            Uuid::new_v4(),
        );

        let id = sandbox.submit_idea(idea);
        sandbox.evaluate_risk(id);
        sandbox.simulate(id);

        let mut votes = HashMap::new();
        votes.insert(Uuid::new_v4(), Vote::For);

        sandbox.approve(id, votes);
        let result = sandbox.execute(id);

        assert!(result.is_some());
        assert!(!sandbox.execution_history.is_empty());
    }

    #[test]
    fn test_full_create_flow() {
        let config = SandboxConfig::default();
        let mut sandbox = CreationSandbox::new(config);

        let idea = CreationIdea::new(
            "完整测试".to_string(),
            CreationType::StructureOptimization,
            Uuid::new_v4(),
        ).with_novelty(0.3);

        let mut votes = HashMap::new();
        for _ in 0..7 {
            votes.insert(Uuid::new_v4(), Vote::For);
        }
        for _ in 0..3 {
            votes.insert(Uuid::new_v4(), Vote::Against);
        }

        let result = sandbox.create(idea, votes);
        assert!(matches!(result, CreateResult::Success(_)));
    }

    #[test]
    fn test_rollback() {
        let config = SandboxConfig::default();
        let mut sandbox = CreationSandbox::new(config);

        let idea = CreationIdea::new(
            "回滚测试".to_string(),
            CreationType::KnowledgeRecombination,
            Uuid::new_v4(),
        );

        let mut votes = HashMap::new();
        votes.insert(Uuid::new_v4(), Vote::For);

        if let CreateResult::Success(record) = sandbox.create(idea, votes) {
            let rollback = sandbox.rollback(record.id);
            assert!(rollback.is_some());
            assert!(rollback.unwrap().success);
        }
    }
}
