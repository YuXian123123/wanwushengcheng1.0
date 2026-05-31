//! 世界安全模块 - 螺丝咕姆设计
//!
//! 三大安全机制：
//! 1. 分层生存机制 (Layered Survival)
//! 2. 信任熵检测 (Trust Entropy Detection)
//! 3. 优雅降级协议 (Graceful Degradation Protocol)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ============================================================================
// 分层生存机制
// ============================================================================

/// 生存层级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SurvivalLevel {
    /// Level 1: 蛊虫心跳
    GuHeartbeat,
    /// Level 2: 接入点冗余
    AccessPointRedundancy,
    /// Level 3: 世界种子
    WorldSeed,
}

/// 分层生存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayeredSurvivalConfig {
    /// 心跳超时（秒）
    pub heartbeat_timeout: u64,
    /// 接入点冗余数量
    pub ap_redundancy_count: usize,
    /// 种子保存间隔（秒）
    pub seed_save_interval: u64,
    /// 最小种群
    pub min_population: u64,
    /// 恢复触发阈值
    pub recovery_threshold: f64,
}

impl Default for LayeredSurvivalConfig {
    fn default() -> Self {
        Self {
            heartbeat_timeout: 30,
            ap_redundancy_count: 2,
            seed_save_interval: 60,
            min_population: 10,
            recovery_threshold: 0.3,
        }
    }
}

/// 分层生存状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayeredSurvivalState {
    /// 配置
    config: LayeredSurvivalConfig,
    /// 各层生存率
    pub survival_rates: HashMap<SurvivalLevel, f64>,
    /// 总体生存率
    pub overall_survival_rate: f64,
    /// 最后一次种子保存时间
    pub last_seed_save: u64,
    /// 世界种子
    pub world_seed: Option<WorldSeed>,
}

/// 世界种子 - 最后手段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSeed {
    pub id: Uuid,
    pub timestamp: u64,
    pub population_count: u64,
    pub knowledge_snapshot: Vec<u8>,
    pub config_hash: u64,
}

impl LayeredSurvivalState {
    pub fn new(config: LayeredSurvivalConfig) -> Self {
        let mut survival_rates = HashMap::new();
        survival_rates.insert(SurvivalLevel::GuHeartbeat, 1.0);
        survival_rates.insert(SurvivalLevel::AccessPointRedundancy, 1.0);
        survival_rates.insert(SurvivalLevel::WorldSeed, 1.0);

        Self {
            config,
            survival_rates,
            overall_survival_rate: 1.0,
            last_seed_save: 0,
            world_seed: None,
        }
    }

    /// 计算总体生存率（螺丝咕姆公式）
    ///
    /// Survival_Rate = 1 - (1 - Gu_Rate)^N × (1 - AP_Rate)^5N × Seed_Factor
    pub fn calculate_overall_survival(&self, gu_count: u64) -> f64 {
        let gu_rate = self.survival_rates.get(&SurvivalLevel::GuHeartbeat).copied().unwrap_or(0.0);
        let ap_rate = self.survival_rates.get(&SurvivalLevel::AccessPointRedundancy).copied().unwrap_or(0.0);
        let seed_factor = self.survival_rates.get(&SurvivalLevel::WorldSeed).copied().unwrap_or(0.0);

        if gu_count == 0 {
            return seed_factor;  // 仅依赖种子
        }

        let n = gu_count as f64;
        let gu_failure = (1.0_f64 - gu_rate).powf(n);
        let ap_failure = (1.0_f64 - ap_rate).powf(5.0_f64 * n);

        1.0 - gu_failure * ap_failure * (1.0 - seed_factor)
    }

    /// 更新蛊虫心跳层生存率
    pub fn update_gu_heartbeat_rate(&self, alive_count: u64, total_count: u64) -> Self {
        let mut new_state = self.clone();
        let rate = if total_count > 0 {
            alive_count as f64 / total_count as f64
        } else {
            0.0
        };
        new_state.survival_rates.insert(SurvivalLevel::GuHeartbeat, rate);
        new_state
    }

    /// 更新接入点冗余层生存率
    pub fn update_ap_redundancy_rate(&self, active_aps: usize, total_aps: usize) -> Self {
        let mut new_state = self.clone();
        let rate = if total_aps > 0 {
            active_aps as f64 / total_aps as f64
        } else {
            0.0
        };
        new_state.survival_rates.insert(SurvivalLevel::AccessPointRedundancy, rate);
        new_state
    }

    /// 保存世界种子
    pub fn save_seed(&self, population: u64, knowledge: Vec<u8>, config_hash: u64, timestamp: u64) -> Self {
        let mut new_state = self.clone();
        new_state.world_seed = Some(WorldSeed {
            id: Uuid::new_v4(),
            timestamp,
            population_count: population,
            knowledge_snapshot: knowledge,
            config_hash,
        });
        new_state.last_seed_save = timestamp;
        new_state.survival_rates.insert(SurvivalLevel::WorldSeed, 1.0);
        new_state
    }

    /// 从种子恢复
    pub fn recover_from_seed(&self) -> Option<&WorldSeed> {
        self.world_seed.as_ref()
    }

    /// 检查是否需要恢复
    pub fn needs_recovery(&self) -> bool {
        self.overall_survival_rate < self.config.recovery_threshold
    }
}

// ============================================================================
// 信任熵检测
// ============================================================================

/// 信任熵配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustEntropyConfig {
    /// 熵阈值（超过此值触发审计）
    pub entropy_threshold: f64,
    /// 最小信任值
    pub min_trust: f64,
    /// 最大信任值
    pub max_trust: f64,
    /// 信任衰减率
    pub trust_decay: f64,
}

impl Default for TrustEntropyConfig {
    fn default() -> Self {
        Self {
            entropy_threshold: 0.8,  // 高熵 = 高不确定性 = 可疑
            min_trust: 0.0,
            max_trust: 1.0,
            trust_decay: 0.01,
        }
    }
}

/// 信任熵状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustEntropyState {
    /// 配置
    config: TrustEntropyConfig,
    /// 各蛊虫的信任分数
    pub trust_scores: HashMap<Uuid, f64>,
    /// 当前信任熵
    pub current_entropy: f64,
    /// 是否触发审计
    pub audit_triggered: bool,
    /// 可疑蛊虫列表
    pub suspicious_gus: Vec<Uuid>,
}

impl TrustEntropyState {
    pub fn new(config: TrustEntropyConfig) -> Self {
        Self {
            config,
            trust_scores: HashMap::new(),
            current_entropy: 0.0,
            audit_triggered: false,
            suspicious_gus: Vec::new(),
        }
    }

    /// 计算信任熵（螺丝咕姆公式）
    ///
    /// Trust_Entropy = -Σ Trust_i × log(Trust_i)
    ///
    /// 高熵表示信任分散，可能存在恶意蛊虫
    pub fn calculate_entropy(&self) -> f64 {
        if self.trust_scores.is_empty() {
            return 0.0;
        }

        let mut entropy = 0.0;
        for &trust in self.trust_scores.values() {
            if trust > 0.0 {
                entropy -= trust * trust.ln();
            }
        }

        entropy
    }

    /// 设置蛊虫信任分数
    pub fn set_trust(&self, gu_id: Uuid, trust: f64) -> Self {
        let mut new_state = self.clone();
        let clamped = trust.clamp(self.config.min_trust, self.config.max_trust);
        new_state.trust_scores.insert(gu_id, clamped);
        new_state
    }

    /// 应用信任衰减
    pub fn apply_decay(&self) -> Self {
        let mut new_state = self.clone();
        for trust in new_state.trust_scores.values_mut() {
            *trust = (*trust - self.config.trust_decay).max(self.config.min_trust);
        }
        new_state
    }

    /// 更新并检测异常
    pub fn update(&self) -> Self {
        let mut new_state = self.clone();
        new_state.current_entropy = new_state.calculate_entropy();

        // 检测是否需要审计
        new_state.audit_triggered = new_state.current_entropy > new_state.config.entropy_threshold;

        // 识别可疑蛊虫（信任分数过低）
        let mean_trust: f64 = if !new_state.trust_scores.is_empty() {
            new_state.trust_scores.values().sum::<f64>() / new_state.trust_scores.len() as f64
        } else {
            0.5
        };

        new_state.suspicious_gus = new_state.trust_scores.iter()
            .filter(|(_, &trust)| trust < mean_trust * 0.5)
            .map(|(id, _)| *id)
            .collect();

        new_state
    }

    /// 奖励蛊虫（提高信任）
    pub fn reward(&self, gu_id: &Uuid, amount: f64) -> Self {
        if let Some(&current) = self.trust_scores.get(gu_id) {
            self.set_trust(*gu_id, (current + amount).min(self.config.max_trust))
        } else {
            self.clone()
        }
    }

    /// 惩罚蛊虫（降低信任）
    pub fn punish(&self, gu_id: &Uuid, amount: f64) -> Self {
        if let Some(&current) = self.trust_scores.get(gu_id) {
            self.set_trust(*gu_id, (current - amount).max(self.config.min_trust))
        } else {
            self.clone()
        }
    }

    /// 隔离可疑蛊虫
    pub fn isolate_suspicious(&self) -> Self {
        let mut new_state = self.clone();
        for gu_id in &new_state.suspicious_gus {
            new_state.trust_scores.insert(*gu_id, new_state.config.min_trust);
        }
        new_state
    }
}

// ============================================================================
// 优雅降级协议
// ============================================================================

/// 降级阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DegradationPhase {
    /// 正常运行
    Normal,
    /// Phase 1: 警告 - 停止非必要任务
    Warning,
    /// Phase 2: 收缩 - 合并蛊虫知识
    Contraction,
    /// Phase 3: 种子 - 保存世界状态
    Seeding,
    /// Phase 4: 死亡 - 释放重生种子
    Death,
}

impl DegradationPhase {
    pub fn severity(&self) -> f64 {
        match self {
            DegradationPhase::Normal => 0.0,
            DegradationPhase::Warning => 0.25,
            DegradationPhase::Contraction => 0.5,
            DegradationPhase::Seeding => 0.75,
            DegradationPhase::Death => 1.0,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            DegradationPhase::Normal => DegradationPhase::Warning,
            DegradationPhase::Warning => DegradationPhase::Contraction,
            DegradationPhase::Contraction => DegradationPhase::Seeding,
            DegradationPhase::Seeding => DegradationPhase::Death,
            DegradationPhase::Death => DegradationPhase::Death,
        }
    }
}

/// 优雅降级配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GracefulDegradationConfig {
    /// 各阶段阈值
    pub phase_thresholds: [f64; 4],
    /// 降级冷却时间（秒）
    pub cooldown_seconds: u64,
    /// 知识合并批次大小
    pub merge_batch_size: usize,
}

impl Default for GracefulDegradationConfig {
    fn default() -> Self {
        Self {
            phase_thresholds: [0.3, 0.2, 0.1, 0.05],
            cooldown_seconds: 10,
            merge_batch_size: 5,
        }
    }
}

/// 优雅降级状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GracefulDegradationState {
    /// 配置
    config: GracefulDegradationConfig,
    /// 当前阶段
    pub current_phase: DegradationPhase,
    /// 进入当前阶段的时间
    pub phase_entered_at: u64,
    /// 降级历史
    pub degradation_history: Vec<DegradationEvent>,
    /// 已保存的知识
    pub preserved_knowledge: Vec<PreservedKnowledge>,
}

/// 降级事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradationEvent {
    pub from_phase: DegradationPhase,
    pub to_phase: DegradationPhase,
    pub timestamp: u64,
    pub trigger_value: f64,
}

/// 保留的知识
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreservedKnowledge {
    pub source_gu: Uuid,
    pub knowledge_type: String,
    pub data: Vec<u8>,
    pub priority: f64,
}

impl GracefulDegradationState {
    pub fn new(config: GracefulDegradationConfig) -> Self {
        Self {
            config,
            current_phase: DegradationPhase::Normal,
            phase_entered_at: 0,
            degradation_history: Vec::new(),
            preserved_knowledge: Vec::new(),
        }
    }

    /// 根据健康度确定阶段
    pub fn determine_phase(&self, health: f64) -> DegradationPhase {
        let thresholds = &self.config.phase_thresholds;

        if health >= thresholds[0] {
            DegradationPhase::Normal
        } else if health >= thresholds[1] {
            DegradationPhase::Warning
        } else if health >= thresholds[2] {
            DegradationPhase::Contraction
        } else if health >= thresholds[3] {
            DegradationPhase::Seeding
        } else {
            DegradationPhase::Death
        }
    }

    /// 更新降级状态
    pub fn update(&self, health: f64, timestamp: u64) -> Self {
        let new_phase = self.determine_phase(health);
        let mut new_state = self.clone();

        if new_phase != self.current_phase {
            // 记录降级事件
            new_state.degradation_history.push(DegradationEvent {
                from_phase: self.current_phase,
                to_phase: new_phase,
                timestamp,
                trigger_value: health,
            });
            new_state.current_phase = new_phase;
            new_state.phase_entered_at = timestamp;
        }

        new_state
    }

    /// 保存知识
    pub fn preserve_knowledge(&self, knowledge: PreservedKnowledge) -> Self {
        let mut new_state = self.clone();
        new_state.preserved_knowledge.push(knowledge);
        // 按优先级排序
        new_state.preserved_knowledge.sort_by(|a, b|
            b.priority.partial_cmp(&a.priority).unwrap_or(std::cmp::Ordering::Equal)
        );
        new_state
    }

    /// 获取需要执行的操作
    pub fn get_required_actions(&self) -> Vec<DegradationAction> {
        match self.current_phase {
            DegradationPhase::Normal => vec![],
            DegradationPhase::Warning => vec![
                DegradationAction::StopNonEssentialTasks,
            ],
            DegradationPhase::Contraction => vec![
                DegradationAction::StopNonEssentialTasks,
                DegradationAction::MergeKnowledge,
            ],
            DegradationPhase::Seeding => vec![
                DegradationAction::StopNonEssentialTasks,
                DegradationAction::MergeKnowledge,
                DegradationAction::SaveSeed,
            ],
            DegradationPhase::Death => vec![
                DegradationAction::ReleaseSeed,
            ],
        }
    }

    /// 是否可以恢复
    pub fn can_recover(&self) -> bool {
        matches!(self.current_phase, DegradationPhase::Normal | DegradationPhase::Warning)
    }
}

/// 降级行动
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DegradationAction {
    StopNonEssentialTasks,
    MergeKnowledge,
    SaveSeed,
    ReleaseSeed,
}

// ============================================================================
// 综合安全状态
// ============================================================================

/// 世界安全状态（螺丝咕姆综合设计）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSafetyState {
    /// 分层生存
    pub layered_survival: LayeredSurvivalState,
    /// 信任熵
    pub trust_entropy: TrustEntropyState,
    /// 优雅降级
    pub graceful_degradation: GracefulDegradationState,
    /// 安全评分（综合）
    pub safety_score: f64,
}

impl WorldSafetyState {
    pub fn new() -> Self {
        Self {
            layered_survival: LayeredSurvivalState::new(LayeredSurvivalConfig::default()),
            trust_entropy: TrustEntropyState::new(TrustEntropyConfig::default()),
            graceful_degradation: GracefulDegradationState::new(GracefulDegradationConfig::default()),
            safety_score: 1.0,
        }
    }

    /// 计算综合安全评分
    ///
    /// Safety = Survival × (1 - Entropy_normalized) × (1 - Degradation_severity)
    pub fn calculate_safety_score(&self) -> f64 {
        let survival = self.layered_survival.overall_survival_rate;
        let entropy_factor = 1.0 - (self.trust_entropy.current_entropy / 2.0).min(1.0);  // 归一化熵
        let degradation_factor = 1.0 - self.graceful_degradation.current_phase.severity();

        survival * entropy_factor * degradation_factor
    }

    /// 全面更新
    pub fn update(&self, health: f64, gu_count: u64, timestamp: u64) -> Self {
        let mut new_state = self.clone();
        new_state.graceful_degradation = self.graceful_degradation.update(health, timestamp);
        new_state.trust_entropy = self.trust_entropy.update();
        new_state.layered_survival.overall_survival_rate =
            new_state.layered_survival.calculate_overall_survival(gu_count);
        new_state.safety_score = new_state.calculate_safety_score();
        new_state
    }

    /// 是否需要紧急干预
    pub fn needs_emergency_intervention(&self) -> bool {
        self.safety_score < 0.2 ||
        self.graceful_degradation.current_phase == DegradationPhase::Death ||
        self.trust_entropy.audit_triggered
    }
}

impl Default for WorldSafetyState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layered_survival_creation() {
        let config = LayeredSurvivalConfig::default();
        let state = LayeredSurvivalState::new(config);
        assert_eq!(state.overall_survival_rate, 1.0);
    }

    #[test]
    fn test_survival_rate_formula() {
        let config = LayeredSurvivalConfig::default();
        let state = LayeredSurvivalState::new(config);

        // 10个蛊虫，全部存活
        let rate = state.calculate_overall_survival(10);
        assert!(rate > 0.99);
    }

    #[test]
    fn test_trust_entropy_calculation() {
        let config = TrustEntropyConfig::default();
        let state = TrustEntropyState::new(config);

        // 均匀分布的信任 -> 最大熵
        let state = state.set_trust(Uuid::new_v4(), 0.5)
                         .set_trust(Uuid::new_v4(), 0.5);
        let entropy = state.calculate_entropy();
        assert!(entropy > 0.0);
    }

    #[test]
    fn test_degradation_phase_progression() {
        let config = GracefulDegradationConfig::default();
        let state = GracefulDegradationState::new(config);

        // 健康度下降触发降级
        let state = state.update(0.15, 0);  // 低于第二阈值
        assert!(state.current_phase == DegradationPhase::Contraction);
    }

    #[test]
    fn test_safety_score_calculation() {
        let state = WorldSafetyState::new();
        let updated = state.update(0.5, 10, 0);
        assert!(updated.safety_score > 0.0);
        assert!(updated.safety_score <= 1.0);
    }

    #[test]
    fn test_emergency_detection() {
        let mut state = WorldSafetyState::new();
        state.safety_score = 0.1;
        assert!(state.needs_emergency_intervention());
    }
}
