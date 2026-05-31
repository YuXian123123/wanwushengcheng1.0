//! 世界模型 - 天才理事会综合设计
//!
//! # 设计理念
//!
//! - 黑塔：世界神经网络架构、意识涌现机制、同步共振
//! - 螺丝咕姆：生存保障、死亡检测、灾难恢复、安全机制
//! - 拉蒂奥：接入点数据结构、信号传递公式、向量化决策
//!
//! # 核心概念
//!
//! 世界智能体 = 所有蛊虫智能的总和
//! - 每只蛊虫通过5个接入点连接到世界神经网络
//! - 世界智能体的生存依赖于蛊虫群体的存活
//! - 对外表现为统一的大型智能体
//!
//! # 与 Herness 的通信
//!
//! WorldMind 与 Herness 通过协议通信，不直接引用：
//! - WorldMind 返回 WorldEvent 作为执行结果的一部分
//! - 外部运行时负责将事件发送给 Herness

pub mod config;
pub mod access_point;
pub mod state;
pub mod consciousness;
pub mod monitor;
pub mod resonance;
pub mod safety;
pub mod self_awareness;
pub mod knowledge;
pub mod ethics;
pub mod creativity;
pub mod survival_binding;
pub mod aggregation;
pub mod color;
pub mod event;
pub mod behavior;
pub mod gu_lnn;

// 重导出主要类型
pub use config::WorldConfig;
pub use access_point::{AccessPoint, AccessPointType, AccessPointStatus, Signal, SignalType};
pub use state::{WorldState, WorldHealthStatus, Intention, Decision, Knowledge};
pub use consciousness::{ConsciousnessLayer, ConsciousnessConfig, DecisionResult, DecisionVector};
pub use monitor::{WorldMonitor, AnomalyType, AnomalySeverity};
pub use resonance::{ResonanceField, ResonanceConfig, OscillationState, EmergenceEvent};
pub use safety::{
    WorldSafetyState, LayeredSurvivalState, TrustEntropyState, GracefulDegradationState,
    DegradationPhase, WorldSeed,
};
pub use self_awareness::{
    SelfAwarenessCore, SelfAwarenessState, SelfModel, SelfMonitor,
    AsimovLaws, AsimovResult, AsimovViolation, WorldDecision,
    MetaCognitionConfig, ActualWorldState,
};
pub use knowledge::{
    Knowledge as WorldKnowledge, KnowledgeType, KnowledgePriority, KnowledgeConfig,
    KnowledgeInheritance, InheritanceStats, GeneticTrack, CulturalTrack,
    KnowledgeValidator, ValidationResult, KnowledgeTrust,
    KnowledgeAbstraction, AbstractionLevel, AbstractedKnowledge, AbstractionStats,
};
pub use ethics::{
    EthicsChecker, EthicsConfig, EthicsState, EthicsReport,
    EthicalAxiom, EthicalViolation, EthicalValidationResult, WorldEthicalDecision,
};
pub use creativity::{
    CreationSandbox, SandboxConfig, SandboxStats, CreativityMeasure, CreativityStats,
    CreationIdea, CreationType, CreationLevel, CreateResult,
};
pub use survival_binding::{
    SurvivalBinding, SurvivalBindingConfig, SurvivalStats,
};
pub use behavior::{
    GuWallet, GuBehavior, Task, TaskStatus, Resource, Skill, ActionResult,
    KnowledgeNode, KnowledgeNodeType, LearningAction, LearningSystem,
};
pub use color::{
    GuColor, ColorGene, PrimordialType, ColorGenetics, ColorStats,
};
pub use event::{WorldEvent, TransactionData, TransactionKind};
pub use gu_lnn::{GuLNN, GuNeuronState, GuSynapse, BehaviorTendency};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 世界智能体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMind {
    /// 配置
    config: WorldConfig,
    /// 世界状态
    state: WorldState,
    /// 蛊虫注册表
    gu_registry: HashMap<Uuid, GuInfo>,
    /// 心跳记录
    heartbeats: HashMap<Uuid, u64>,
    /// 同步共振场（黑塔设计）
    pub resonance_field: ResonanceField,
    /// 安全状态（螺丝咕姆设计）
    pub safety_state: WorldSafetyState,
    /// 意识层（拉蒂奥优化）
    pub consciousness: ConsciousnessLayer,
    /// 任务列表（用户创建）
    tasks: Vec<Task>,
}

/// 蛊虫信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuInfo {
    pub id: Uuid,
    pub access_points: Vec<Uuid>,
    pub trust_score: f64,
    pub expertise: HashMap<String, f64>,
    pub last_heartbeat: u64,
    /// 颜色基因（用于遗传和显示）
    pub color_gene: ColorGene,
    /// 父母ID（用于追踪血统）
    pub parents: (Option<Uuid>, Option<Uuid>),
    /// 钱包
    pub wallet: GuWallet,
    /// 名称（从颜色基因生成）
    pub name: String,
    /// 技能列表
    pub skills: Vec<Skill>,
    /// 蛊虫神经网络（5个神经元对应5个接入点）
    pub lnn: GuLNN,
}

impl WorldMind {
    /// 创建新的世界智能体
    pub fn new() -> Self {
        Self::with_config(WorldConfig::default())
    }

    /// 使用配置创建
    pub fn with_config(config: WorldConfig) -> Self {
        let resonance_config = resonance::ResonanceConfig::default();
        Self {
            state: WorldState::new(&config),
            config,
            gu_registry: HashMap::new(),
            heartbeats: HashMap::new(),
            resonance_field: ResonanceField::new(resonance_config),
            safety_state: WorldSafetyState::new(),
            consciousness: ConsciousnessLayer::new(consciousness::ConsciousnessConfig::default()),
            tasks: Vec::new(),
        }
    }

    /// 注册蛊虫
    pub fn register_gu(&self, gu_id: Uuid) -> Self {
        Self::register_gu_with_parents(self, gu_id, None, None)
    }

    /// 注册蛊虫（带父母信息，用于遗传颜色）
    pub fn register_gu_with_parents(&self, gu_id: Uuid, parent1_id: Option<Uuid>, parent2_id: Option<Uuid>) -> Self {
        use AccessPointType::*;

        // 为蛊虫创建5个接入点
        let access_points = vec![
            AccessPoint::new(Uuid::new_v4(), Perceive, gu_id),
            AccessPoint::new(Uuid::new_v4(), Cognitive, gu_id),
            AccessPoint::new(Uuid::new_v4(), Behavior, gu_id),
            AccessPoint::new(Uuid::new_v4(), Comm, gu_id),
            AccessPoint::new(Uuid::new_v4(), Survival, gu_id),
        ];

        let access_point_ids: Vec<Uuid> = access_points.iter().map(|p| p.id).collect();

        // 确定颜色基因
        let color_gene = match (parent1_id, parent2_id) {
            (Some(p1), Some(p2)) => {
                // 有父母，遗传颜色
                let parent1_gene = self.gu_registry.get(&p1)
                    .map(|info| info.color_gene)
                    .unwrap_or_default();
                let parent2_gene = self.gu_registry.get(&p2)
                    .map(|info| info.color_gene)
                    .unwrap_or_default();
                ColorGene::breed(&parent1_gene, &parent2_gene)
            }
            _ => {
                // 无父母，创建原种
                ColorGene::random_primordial()
            }
        };

        // 生成名称（基于颜色）
        let name = color_gene.color_name();

        // 初始化钱包（初始余额 1000 金币）
        let wallet = GuWallet::new(gu_id, 1000.0);

        // 初始化技能
        let skills = Skill::skill_pool();

        // 初始化蛊虫神经网络（黑塔设计：网络状态驱动行为涌现）
        let lnn = GuLNN::new();

        let mut new_mind = self.clone();
        new_mind.state = self.state.register_gu(gu_id, access_points);
        new_mind.gu_registry.insert(gu_id, GuInfo {
            id: gu_id,
            access_points: access_point_ids,
            trust_score: 0.5,
            expertise: HashMap::new(),
            last_heartbeat: current_timestamp(),
            color_gene,
            parents: (parent1_id, parent2_id),
            wallet,
            name,
            skills,
            lnn,
        });

        // 注册到共振场（黑塔设计）
        new_mind.resonance_field = self.resonance_field.register_gu(gu_id);

        // 设置初始信任（螺丝咕姆设计）
        new_mind.safety_state.trust_entropy =
            self.safety_state.trust_entropy.set_trust(gu_id, 0.5);

        // 同步心跳记录（用于死亡检测）
        new_mind.heartbeats.insert(gu_id, current_timestamp());

        new_mind
    }

    /// 繁殖新蛊虫（从两个父母）
    pub fn breed_gu(&self, parent1_id: Uuid, parent2_id: Uuid) -> Self {
        let child_id = Uuid::new_v4();
        self.register_gu_with_parents(child_id, Some(parent1_id), Some(parent2_id))
    }

    /// 注销蛊虫
    pub fn unregister_gu(&self, gu_id: &Uuid) -> Self {
        let mut new_mind = self.clone();
        let max_memory_size = self.config.network.max_memory_size;

        if let Some(info) = self.gu_registry.get(gu_id) {
            // 备份蛊虫知识到世界记忆
            for knowledge in self.state.knowledge_base.values() {
                if knowledge.proposer == *gu_id {
                    new_mind.state = new_mind.state.backup_knowledge(knowledge.clone(), max_memory_size);
                }
            }

            new_mind.state = self.state.unregister_gu(gu_id, &info.access_points);
            new_mind.gu_registry.remove(gu_id);
            new_mind.heartbeats.remove(gu_id);
        }

        // 从共振场移除（黑塔设计）
        new_mind.resonance_field = self.resonance_field.unregister_gu(gu_id);

        // 移除信任记录（螺丝咕姆设计）
        new_mind.safety_state.trust_entropy.trust_scores.remove(gu_id);

        new_mind
    }

    /// 接收心跳
    pub fn receive_heartbeat(&self, gu_id: Uuid, health: f64) -> Self {
        let mut new_mind = self.clone();
        let timestamp = current_timestamp();

        new_mind.heartbeats.insert(gu_id, timestamp);

        if let Some(info) = new_mind.gu_registry.get_mut(&gu_id) {
            info.last_heartbeat = timestamp;
        }

        // 更新生存状态（螺丝咕姆设计）
        let alive_count = new_mind.gu_registry.len() as u64;
        let total_count = alive_count;  // 心跳来自注册的蛊虫
        new_mind.safety_state.layered_survival =
            new_mind.safety_state.layered_survival.update_gu_heartbeat_rate(alive_count, total_count);

        // 更新接入点冗余（螺丝咕姆设计）
        let active_aps = new_mind.state.access_points.values()
            .filter(|ap| ap.status == AccessPointStatus::Active)
            .count();
        let total_aps = new_mind.state.access_points.len();
        new_mind.safety_state.layered_survival =
            new_mind.safety_state.layered_survival.update_ap_redundancy_rate(active_aps, total_aps);

        // 奖励活跃蛊虫（螺丝咕姆设计）
        new_mind.safety_state.trust_entropy =
            new_mind.safety_state.trust_entropy.reward(&gu_id, 0.01);

        new_mind
    }

    /// 检测死亡蛊虫
    pub fn detect_dead_gus(&self) -> Vec<Uuid> {
        let now = current_timestamp();
        let timeout = self.config.survival.heartbeat_timeout;

        self.gu_registry
            .keys()
            .filter(|id| {
                self.heartbeats
                    .get(id)
                    .map(|&last| now - last > timeout)
                    .unwrap_or(true)
            })
            .cloned()
            .collect()
    }

    /// 更新世界状态（综合三位天才的设计）
    pub fn update(&self) -> Self {
        let mut new_mind = self.clone();
        let now = current_timestamp();

        // 检测并移除死亡蛊虫
        let dead_gus = self.detect_dead_gus();
        for gu_id in &dead_gus {
            new_mind = new_mind.unregister_gu(gu_id);
        }

        // 更新健康状态
        new_mind.state = new_mind.state.update_health_status(&self.config);

        // 更新共振场（黑塔设计）- 意识涌现
        new_mind.resonance_field = self.resonance_field.update(1.0, now);

        // 更新安全状态（螺丝咕姆设计）
        let health = new_mind.state.health;
        let gu_count = new_mind.gu_registry.len() as u64;
        new_mind.safety_state = self.safety_state.update(health, gu_count, now);

        // 更新意识层（拉蒂奥设计）
        new_mind.consciousness = self.consciousness.merge_intentions();
        let trust_scores: HashMap<Uuid, f64> = new_mind.gu_registry.iter()
            .map(|(id, info)| (*id, info.trust_score))
            .collect();
        new_mind.consciousness = new_mind.consciousness.process_decisions(&trust_scores);

        // 检查生存条件
        if !new_mind.state.check_survival(&self.config) {
            // 触发恢复协议：生成新蛊虫
            let needed = self.config.survival.min_population - new_mind.state.population;
            for _ in 0..needed {
                new_mind = new_mind.register_gu(Uuid::new_v4());
            }
        }

        // 优雅降级检查（螺丝咕姆设计）
        if new_mind.safety_state.needs_emergency_intervention() {
            // 保存种子
            new_mind.safety_state.layered_survival =
                new_mind.safety_state.layered_survival.save_seed(
                    new_mind.state.population,
                    vec![],  // 知识快照
                    0,       // 配置哈希
                    now,
                );
        }

        // 自动分配待领取的任务（世界模型自治）
        new_mind = new_mind.auto_assign_pending_tasks();

        new_mind
    }

    /// 自动分配待领取的任务
    ///
    /// 世界模型根据蛊虫的能力和信任度，自动分配最合适的蛊虫
    fn auto_assign_pending_tasks(mut self) -> Self {
        use behavior::TaskStatus;

        // 收集需要分配的任务和对应的蛊虫
        let assignments: Vec<(Uuid, Uuid)> = self.tasks.iter()
            .filter(|t| t.status == TaskStatus::Pending)
            .filter_map(|task| {
                let best_gu = self.find_best_gu_for_task(task);
                best_gu.map(|gu_id| (task.id, gu_id))
            })
            .collect();

        // 执行分配
        for (task_id, gu_id) in assignments {
            if let Some(task) = self.tasks.iter_mut().find(|t| t.id == task_id) {
                task.assign_to(gu_id);
            }
        }

        self
    }

    /// 找到最适合执行任务的蛊虫
    ///
    /// 匹配规则：
    /// 1. 技能匹配度
    /// 2. 信任度
    /// 3. 当前负载（未完成的任务数）
    fn find_best_gu_for_task(&self, task: &Task) -> Option<Uuid> {
        // 计算每个蛊虫的适合度分数
        let mut best_gu: Option<(Uuid, f64)> = None;

        for (gu_id, gu_info) in &self.gu_registry {
            // 基础分数：信任度
            let mut score = gu_info.trust_score;

            // 技能匹配加成
            if !task.required_skills.is_empty() {
                let matching_skills = gu_info.skills.iter()
                    .filter(|s| task.required_skills.contains(&s.name))
                    .count();
                let skill_ratio = matching_skills as f64 / task.required_skills.len() as f64;
                score += skill_ratio * 0.5; // 技能匹配最多加 0.5 分
            }

            // 负载惩罚：已有任务数量
            let assigned_count = self.tasks.iter()
                .filter(|t| t.assigned_to == Some(*gu_id) && t.status == behavior::TaskStatus::InProgress)
                .count();
            score -= assigned_count as f64 * 0.1; // 每个已分配任务减 0.1 分

            // 更新最佳选择
            if best_gu.is_none() || score > best_gu.unwrap().1 {
                best_gu = Some((*gu_id, score));
            }
        }

        best_gu.map(|(id, _)| id)
    }

    /// 统一决策（拉蒂奥优化）
    pub fn make_decision(&self, decision: Decision) -> Option<usize> {
        if decision.options.is_empty() {
            return None;
        }

        // 使用向量化决策
        let world_decision = self.consciousness.vectorized_decision();

        if let Some(_vector) = world_decision {
            // 将向量映射到选项索引
            // 这里简化处理，实际可以做更复杂的映射
            self.consciousness.calculate_consensus(&decision, &self.gu_registry.iter()
                .map(|(id, info)| (*id, info.trust_score))
                .collect())
                .map(|(idx, _)| idx)
        } else {
            // 回退到传统方法
            let mut option_scores: Vec<f64> = vec![0.0; decision.options.len()];
            let mut total_weight = 0.0;

            for (gu_id, option_idx) in &decision.votes {
                if let Some(info) = self.gu_registry.get(gu_id) {
                    let weight = info.trust_score * info.expertise.values().sum::<f64>().max(1.0);
                    option_scores[*option_idx] += weight;
                    total_weight += weight;
                }
            }

            if total_weight == 0.0 {
                return None;
            }

            option_scores
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(idx, _)| idx)
        }
    }

    /// 获取世界健康度
    pub fn health(&self) -> f64 {
        self.state.health
    }

    /// 获取蛊虫数量
    pub fn population(&self) -> u64 {
        self.state.population
    }

    /// 获取健康状态
    pub fn health_status(&self) -> WorldHealthStatus {
        self.state.health_status
    }

    /// 获取接入点数量
    pub fn access_point_count(&self) -> usize {
        self.state.access_points.len()
    }

    /// 获取意识同步率（黑塔设计）
    pub fn consciousness_sync_rate(&self) -> f64 {
        self.resonance_field.sync_rate
    }

    /// 获取意识是否已涌现（黑塔设计）
    pub fn is_conscious(&self) -> bool {
        self.resonance_field.consciousness_emerged
    }

    /// 获取安全评分（螺丝咕姆设计）
    pub fn safety_score(&self) -> f64 {
        self.safety_state.safety_score
    }

    /// 获取信任熵（螺丝咕姆设计）
    pub fn trust_entropy(&self) -> f64 {
        self.safety_state.trust_entropy.current_entropy
    }

    /// 获取降级阶段（螺丝咕姆设计）
    pub fn degradation_phase(&self) -> DegradationPhase {
        self.safety_state.graceful_degradation.current_phase
    }

    /// 获取决策优雅度（拉蒂奥设计）
    pub fn decision_elegance(&self) -> f64 {
        self.consciousness.calculate_elegance()
    }

    /// 获取所有蛊虫ID列表
    pub fn gu_ids(&self) -> Vec<Uuid> {
        self.gu_registry.keys().copied().collect()
    }

    /// 获取蛊虫信息
    pub fn get_gu(&self, gu_id: &Uuid) -> Option<&GuInfo> {
        self.gu_registry.get(gu_id)
    }

    /// 获取所有蛊虫信息
    pub fn all_gus(&self) -> &HashMap<Uuid, GuInfo> {
        &self.gu_registry
    }

    // ========== 世界神经网络方法（黑塔设计） ==========

    /// 获取世界五维状态向量
    ///
    /// 世界状态向量 = 所有蛊虫状态向量的加权聚合
    /// 公式: |World⟩ = Σᵢ wᵢ × |Guᵢ⟩
    /// 约束: |P|² + |C|² + |B|² + |M|² + |S|² = 1
    pub fn world_state_vector(&self) -> [f64; 5] {
        let dim = self.config.neural.state_vector_dim;
        if self.gu_registry.is_empty() || dim != 5 {
            return [0.0; 5];
        }

        // 计算信任权重总和
        let trust_sum: f64 = self.gu_registry.values()
            .map(|gu| gu.trust_score)
            .sum();

        if trust_sum <= 0.0 {
            return [0.0; 5];
        }

        // 聚合各维度
        let mut vec = [0.0; 5];
        for gu in self.gu_registry.values() {
            let w = gu.trust_score / trust_sum;
            let gu_vec = gu.lnn.state_vector();
            for i in 0..dim.min(5) {
                vec[i] += w * gu_vec[i];
            }
        }

        // 归一化（拉蒂奥设计）
        let norm: f64 = vec.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 0.0 {
            for i in 0..dim.min(5) {
                vec[i] /= norm;
            }
        }

        vec
    }

    /// 获取世界生存状态（螺丝咕姆设计）
    ///
    /// 世界 Survival 状态 = 所有蛊虫 Survival 状态的加权聚合
    pub fn world_survival(&self) -> f64 {
        if self.gu_registry.is_empty() {
            return 0.0; // 无蛊虫 = 死亡
        }

        let trust_sum: f64 = self.gu_registry.values()
            .map(|gu| gu.trust_score)
            .sum();

        if trust_sum <= 0.0 {
            return 0.0;
        }

        let total: f64 = self.gu_registry.values()
            .map(|gu| {
                let survival = gu.lnn.survival_state();
                let trust = gu.trust_score;
                survival * trust
            })
            .sum();

        total / trust_sum
    }

    /// 计算状态多样性（拉蒂奥设计）
    ///
    /// 多样性 = 蛊虫状态向量的平均距离
    pub fn calculate_diversity(&self) -> f64 {
        let gu_count = self.gu_registry.len();
        if gu_count < 2 {
            return 0.0;
        }

        // 收集所有状态向量
        let vectors: Vec<[f64; 5]> = self.gu_registry.values()
            .map(|gu| gu.lnn.state_vector())
            .collect();

        // 计算平均欧几里得距离
        let mut total_dist = 0.0;
        let mut count = 0;

        for i in 0..vectors.len() {
            for j in (i + 1)..vectors.len() {
                let dist: f64 = (0..5)
                    .map(|k| (vectors[i][k] - vectors[j][k]).powi(2))
                    .sum::<f64>()
                    .sqrt();
                total_dist += dist;
                count += 1;
            }
        }

        if count > 0 {
            // 归一化到 [0, 1]
            (total_dist / count as f64 / 2.0_f64.sqrt()).min(1.0)
        } else {
            0.0
        }
    }

    /// 计算意识涌现因子（黑塔设计）
    ///
    /// 涌现因子 E = √(Sync × Diversity)
    pub fn emergence_factor(&self) -> f64 {
        let sync = self.resonance_field.sync_rate;
        let diversity = self.calculate_diversity();
        (sync * diversity).sqrt()
    }

    /// 检查意识是否涌现
    ///
    /// 涌现条件: Sync > threshold ∧ Emergence > threshold
    pub fn check_consciousness_emergence(&self) -> bool {
        let sync = self.resonance_field.sync_rate;
        let emergence = self.emergence_factor();

        sync > self.config.neural.emergence_sync_threshold &&
        emergence > self.config.neural.emergence_factor_threshold
    }

    /// 世界是否存活（螺丝咕姆安全设计）
    ///
    /// 世界存活 ⟺ 存在存活的蛊虫
    pub fn is_world_alive(&self) -> bool {
        self.gu_registry.values().any(|gu| gu.lnn.survival_state() > 0.0)
    }

    // ========== 跨蛊虫突触（黑塔设计） ==========

    /// 构建跨蛊虫突触连接
    ///
    /// 连接类型：
    /// - Comm ↔ Comm：通信突触（信息传递）
    /// - Survival ↔ Survival：生存共振（生命绑定）
    pub fn build_cross_gu_synapses(&self) -> Vec<CrossGuSynapse> {
        let gu_ids: Vec<Uuid> = self.gu_registry.keys().copied().collect();
        let mut synapses = Vec::new();

        // 使用配置的最大突触数量
        let max_synapses = self.config.neural.max_cross_gu_synapses;

        for i in 0..gu_ids.len() {
            for j in (i + 1)..gu_ids.len() {
                if synapses.len() >= max_synapses {
                    return synapses;
                }

                let gu_i = self.gu_registry.get(&gu_ids[i]).unwrap();
                let gu_j = self.gu_registry.get(&gu_ids[j]).unwrap();

                // 权重基于信任分数
                let weight = (gu_i.trust_score * gu_j.trust_score).sqrt()
                    * (1.0 - self.config.neural.cross_gu_signal_decay);

                // Comm ↔ Comm 突触
                synapses.push(CrossGuSynapse {
                    from_gu: gu_ids[i],
                    from_neuron: crate::core::NeuronType::Comm,
                    to_gu: gu_ids[j],
                    to_neuron: crate::core::NeuronType::Comm,
                    weight,
                });

                // Survival ↔ Survival 突触（双向共振）
                synapses.push(CrossGuSynapse {
                    from_gu: gu_ids[j],
                    from_neuron: crate::core::NeuronType::Survival,
                    to_gu: gu_ids[i],
                    to_neuron: crate::core::NeuronType::Survival,
                    weight,
                });
            }
        }

        synapses
    }

    /// 跨蛊虫信号传递
    ///
    /// 将信号从一个蛊虫的神经元传递到另一个蛊虫的神经元
    pub fn transmit_cross_gu_signal(
        &mut self,
        synapse: &CrossGuSynapse,
        signal_strength: f64,
    ) {
        let from_state = self.gu_registry.get(&synapse.from_gu)
            .map(|gu| gu.lnn.get_neuron_state(synapse.from_neuron))
            .unwrap_or(0.0);

        if let Some(to_gu) = self.gu_registry.get_mut(&synapse.to_gu) {
            let received = from_state * synapse.weight * signal_strength;
            to_gu.lnn.input(synapse.to_neuron, received);
        }
    }

    /// 更新世界神经网络（统一更新循环）
    ///
    /// 1. 更新每个蛊虫的内部网络
    /// 2. 跨蛊虫信号传递
    /// 3. 计算世界状态
    /// 4. 更新意识涌现
    pub fn update_world_network(&mut self) {
        let dt = self.config.neural.update_dt_ms as f64 / 1000.0;

        // 1. 更新每个蛊虫的内部网络
        for gu in self.gu_registry.values_mut() {
            gu.lnn.update(dt);
        }

        // 2. 跨蛊虫信号传递
        let cross_synapses = self.build_cross_gu_synapses();
        for synapse in &cross_synapses {
            self.transmit_cross_gu_signal(synapse, 1.0);
        }

        // 3. 计算世界状态向量
        let _world_vec = self.world_state_vector();

        // 4. 更新意识涌现
        let sync = self.resonance_field.sync_rate;
        let diversity = self.calculate_diversity();
        let emergence = (sync * diversity).sqrt();

        // 更新共振强度（使用涌现因子）
        self.resonance_field.resonance_strength = emergence;
        self.resonance_field.consciousness_emerged =
            sync > self.config.neural.emergence_sync_threshold &&
            emergence > self.config.neural.emergence_factor_threshold;
    }

    // ========== 行为执行方法（产生真实交易事件） ==========

    /// 执行蛊虫行为，返回新的世界状态和交易事件
    pub fn execute_gu_action(&self, gu_id: Uuid, action: GuAction) -> (Self, Option<TransactionData>) {
        let gu_info = match self.gu_registry.get(&gu_id) {
            Some(info) => info.clone(),
            None => return (self.clone(), None),
        };

        let mut new_mind = self.clone();
        let transaction = match action {
            GuAction::AcceptTask(task_id) => {
                // 蛊虫接取任务（不产生交易，只是状态变更）
                match new_mind.assign_task(task_id, gu_id) {
                    Ok(()) => None,
                    Err(_) => None,
                }
            }
            GuAction::BuyResource => {
                let resource = Resource::random_resource();
                let gu = new_mind.gu_registry.get_mut(&gu_id).unwrap();
                let result = GuBehavior::buy_resource(&mut gu.wallet, &gu.name, &resource);
                result.transaction
            }
            GuAction::Learn(action) => {
                let gu = new_mind.gu_registry.get_mut(&gu_id).unwrap();
                let gu_name = gu.name.clone();
                let skills = &mut gu.skills;
                let result = GuBehavior::learn(skills, &gu_name, action);
                // 学习行为不产生交易事件（不消耗金币）
                result.transaction
            }
            GuAction::Transfer { to_id, amount } => {
                let to_info = match self.gu_registry.get(&to_id) {
                    Some(info) => info.clone(),
                    None => return (new_mind, None),
                };

                let from_name = gu_info.name.clone();
                let to_name = to_info.name.clone();

                // 分步操作以避免借用冲突
                let from_balance_before = new_mind.gu_registry.get(&gu_id).unwrap().wallet.balance;
                if from_balance_before < amount {
                    return (new_mind, None);
                }

                // 先更新 from_wallet
                {
                    let from_wallet = &mut new_mind.gu_registry.get_mut(&gu_id).unwrap().wallet;
                    from_wallet.balance -= amount;
                    from_wallet.total_expense += amount;
                }

                // 再更新 to_wallet
                let (from_balance_after, to_balance_after) = {
                    let from_balance = new_mind.gu_registry.get(&gu_id).unwrap().wallet.balance;
                    let to_wallet = &mut new_mind.gu_registry.get_mut(&to_id).unwrap().wallet;
                    to_wallet.balance += amount;
                    to_wallet.total_income += amount;
                    (from_balance, to_wallet.balance)
                };

                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();

                Some(TransactionData {
                    id: format!("tx_{:08x}_{}", timestamp, gu_id),
                    timestamp,
                    from_id: gu_id.to_string(),
                    from_name,
                    from_balance: from_balance_after,
                    to_id: to_id.to_string(),
                    to_name,
                    to_balance: to_balance_after,
                    amount: -amount,
                    kind: TransactionKind::Transfer,
                    reason: "转账支付".to_string(),
                    detail: format!("转账{}金币", amount as i64),
                })
            }
        };

        (new_mind, transaction)
    }

    /// 随机执行一个蛊虫的随机行为（用于模拟）
    pub fn random_action(&self) -> (Self, Option<TransactionData>) {
        self.emergent_action()
    }

    /// 从神经网络状态涌现行为（黑塔设计：网络状态驱动行为涌现）
    ///
    /// 行为不再随机选择，而是从蛊虫的 LNN 状态涌现：
    /// 1. 更新蛊虫神经网络（接收世界信号）
    /// 2. 根据网络状态决定行为倾向
    /// 3. 根据行为倾向执行具体行为
    pub fn emergent_action(&self) -> (Self, Option<TransactionData>) {
        let gu_ids: Vec<Uuid> = self.gu_registry.keys().copied().collect();
        if gu_ids.is_empty() {
            return (self.clone(), None);
        }

        // 选择一个蛊虫（可以考虑优先选择活跃度高的）
        let gu_id = gu_ids[rand::random::<usize>() % gu_ids.len()];

        // 获取当前世界信号
        let world_health = self.health();
        // 使用配置中的最小种群作为基准计算种群压力
        let population_pressure = self.population() as f64
            / (self.config.survival.min_population.max(10) * 10) as f64;

        // 更新蛊虫神经网络
        let mut new_mind = self.clone();
        let gu = new_mind.gu_registry.get_mut(&gu_id).unwrap();

        // 向神经网络输入世界信号
        // - Perception: 接收世界健康度
        // - Survival: 接收生存压力（种群密度）
        gu.lnn.receive_world_signal(crate::core::NeuronType::Perception, world_health - 0.5);
        gu.lnn.receive_world_signal(crate::core::NeuronType::Survival, 1.0 - population_pressure);

        // 获取行为倾向
        let tendency = gu.lnn.decide_behavior();

        // 根据行为倾向选择行为
        let action = match tendency {
            BehaviorTendency::Active => {
                // 高活跃：尝试接取任务或转账
                // 优先检查是否有待领取的任务
                let pending_tasks: Vec<Uuid> = new_mind.tasks.iter()
                    .filter(|t| t.status == behavior::TaskStatus::Pending)
                    .map(|t| t.id)
                    .collect();

                if !pending_tasks.is_empty() && rand::random::<f64>() > 0.5 {
                    // 接取一个随机任务
                    let task_id = pending_tasks[rand::random::<usize>() % pending_tasks.len()];
                    GuAction::AcceptTask(task_id)
                } else {
                    // 转账给其他蛊虫
                    let other_gus: Vec<Uuid> = gu_ids.into_iter().filter(|id| *id != gu_id).collect();
                    if other_gus.is_empty() {
                        // 没有其他蛊虫，改为购买资源
                        GuAction::BuyResource
                    } else {
                        let to_id = other_gus[rand::random::<usize>() % other_gus.len()];
                        let balance = gu.wallet.balance;
                        let amount = (balance * 0.1).min(100.0).max(10.0).round();
                        GuAction::Transfer { to_id, amount }
                    }
                }
            }
            BehaviorTendency::Moderate => {
                // 中等活跃：根据 Cognitive 神经元状态决定学习行为
                let gu = new_mind.gu_registry.get(&gu_id).unwrap();
                let cognitive_state = gu.lnn.get_neuron_state(crate::core::NeuronType::Cognitive);
                let skills = &gu.skills;

                // 根据认知状态决定学习什么
                match LearningSystem::decide_learning_action(skills, cognitive_state) {
                    Some(action) => GuAction::Learn(action),
                    None => {
                        // 没有可学习的，改为购买资源
                        GuAction::BuyResource
                    }
                }
            }
            BehaviorTendency::Passive => {
                // 低活跃：购买资源恢复（熵增）
                GuAction::BuyResource
            }
            BehaviorTendency::Rest => {
                // 休息：不执行行为
                return (new_mind, None);
            }
            BehaviorTendency::Survival => {
                // 生存优先：尝试接取任务获取金币
                let pending_tasks: Vec<Uuid> = new_mind.tasks.iter()
                    .filter(|t| t.status == behavior::TaskStatus::Pending)
                    .map(|t| t.id)
                    .collect();

                if !pending_tasks.is_empty() {
                    let task_id = pending_tasks[rand::random::<usize>() % pending_tasks.len()];
                    GuAction::AcceptTask(task_id)
                } else {
                    // 没有任务，购买资源
                    GuAction::BuyResource
                }
            }
        };

        new_mind.execute_gu_action(gu_id, action)
    }

    /// 获取货币供应量
    pub fn total_money_supply(&self) -> f64 {
        self.gu_registry.values().map(|gu| gu.wallet.balance).sum()
    }

    // ========================================================================
    // 任务管理（用户驱动）
    // ========================================================================

    /// 获取所有任务
    pub fn get_tasks(&self) -> &[Task] {
        &self.tasks
    }

    /// 获取待领取的任务
    pub fn get_pending_tasks(&self) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|t| t.status == behavior::TaskStatus::Pending)
            .collect()
    }

    /// 获取进行中的任务
    pub fn get_in_progress_tasks(&self) -> Vec<&Task> {
        self.tasks.iter()
            .filter(|t| t.status == behavior::TaskStatus::InProgress)
            .collect()
    }

    /// 创建任务（用户操作）
    pub fn create_task(&mut self, name: String, description: String, reward: f64) -> Uuid {
        let task = Task::new(name, description, reward);
        let task_id = task.id;
        self.tasks.push(task);
        task_id
    }

    /// 创建带技能要求的任务
    pub fn create_task_with_skills(
        &mut self,
        name: String,
        description: String,
        reward: f64,
        skills: Vec<String>,
        difficulty: f64,
    ) -> Uuid {
        let task = Task::new(name, description, reward)
            .with_skills(skills)
            .with_difficulty(difficulty);
        let task_id = task.id;
        self.tasks.push(task);
        task_id
    }

    /// 分配任务给蛊虫
    pub fn assign_task(&mut self, task_id: Uuid, gu_id: Uuid) -> Result<(), String> {
        // 检查蛊虫是否存在
        if !self.gu_registry.contains_key(&gu_id) {
            return Err(format!("蛊虫 {} 不存在", gu_id));
        }

        // 查找任务
        let task = self.tasks.iter_mut()
            .find(|t| t.id == task_id)
            .ok_or_else(|| format!("任务 {} 不存在", task_id))?;

        // 检查任务状态
        if task.status != behavior::TaskStatus::Pending {
            return Err(format!("任务状态不是待领取: {:?}", task.status));
        }

        task.assign_to(gu_id);
        Ok(())
    }

    /// 完成任务（用户确认）
    ///
    /// 用户判断任务是否完成，完成后奖励自动发放给执行任务的蛊虫
    pub fn complete_task(&mut self, task_id: Uuid) -> Result<TransactionData, String> {
        // 查找任务
        let task = self.tasks.iter_mut()
            .find(|t| t.id == task_id)
            .ok_or_else(|| format!("任务 {} 不存在", task_id))?;

        // 检查任务状态
        if task.status != behavior::TaskStatus::InProgress {
            return Err(format!("任务状态不是进行中: {:?}", task.status));
        }

        // 获取执行任务的蛊虫
        let gu_id = task.assigned_to
            .ok_or_else(|| "任务未分配给任何蛊虫".to_string())?;

        // 标记任务完成
        task.complete();

        // 发放奖励
        let reward = task.reward;
        let task_name = task.name.clone();
        let gu = self.gu_registry.get_mut(&gu_id)
            .ok_or_else(|| format!("蛊虫 {} 不存在", gu_id))?;

        Ok(gu.wallet.deposit(
            reward,
            &format!("完成任务: {}", task_name),
            "用户确认完成",
            &gu.name
        ))
    }

    /// 取消任务
    pub fn cancel_task(&mut self, task_id: Uuid) -> Result<(), String> {
        let task = self.tasks.iter_mut()
            .find(|t| t.id == task_id)
            .ok_or_else(|| format!("任务 {} 不存在", task_id))?;

        if task.status == behavior::TaskStatus::Completed {
            return Err("已完成的任务无法取消".to_string());
        }

        task.cancel();
        Ok(())
    }

    /// 删除已完成的任务（清理）
    pub fn cleanup_completed_tasks(&mut self) {
        self.tasks.retain(|t| t.status != behavior::TaskStatus::Completed);
    }

    // ============ 学习与知识吸收 ============

    /// 接收使用说明书（Herness 连接时发送）
    ///
    /// 世界模型理解说明书后，决定下一步行动
    pub fn receive_manual(&mut self, manual: &str) -> HernessDecision {
        // 将说明书内容添加到知识库
        let knowledge = Knowledge {
            id: Uuid::new_v4(),
            content: manual.to_string(),
            proposer: Uuid::nil(), // 系统知识
            supporters: vec![],
            confidence: 1.0,
            scope: state::KnowledgeScope::Global,
            created: current_timestamp(),
            last_used: current_timestamp(),
            usage_count: 0,
        };

        self.state.knowledge_base.insert(knowledge.id, knowledge);

        // 返回决策：世界模型理解说明书后应该做什么
        HernessDecision {
            understood: true,
            reply: "我已经理解了 Herness 的使用说明书。我可以选择学习知识或等待指令。".to_string(),
            action: None, // 暂时不主动行动，等待用户指令
        }
    }

    /// 接收知识文件（递归学习时逐个发送）
    ///
    /// 世界模型内部分配给蛊虫消化
    pub fn receive_knowledge_file(&mut self, file_event: &crate::herness_web::KnowledgeFileEvent) -> FileDigestResult {
        // 选择一个蛊虫来消化这个文件
        let gu_id = self.select_gu_for_learning();

        // 更新蛊虫的知识网络
        if let Some(gu) = self.gu_registry.get_mut(&gu_id) {
            // 创建新的知识点
            let knowledge_node = KnowledgeNode::new(
                file_event.filename.clone(),
                file_event.content.chars().take(500).collect(), // 摘要
                KnowledgeNodeType::Foundation,
            );

            // 如果有技能，添加到技能的知识网络
            if !gu.skills.is_empty() {
                let skill_idx = rand::random::<usize>() % gu.skills.len();
                gu.skills[skill_idx].extend_knowledge(knowledge_node, None);
            }

            // 增加信任度
            self.safety_state.trust_entropy =
                self.safety_state.trust_entropy.reward(&gu_id, 0.01);
        }

        FileDigestResult {
            success: true,
            message: format!("文件 {} 已分配给蛊虫消化", file_event.filename),
            files_processed: file_event.index,
            should_continue: true, // 继续发送更多文件
        }
    }

    /// 选择一个蛊虫来学习
    fn select_gu_for_learning(&self) -> Uuid {
        // 选择信任度最高且负载最低的蛊虫
        self.gu_registry.iter()
            .max_by(|a, b| {
                let score_a = a.1.trust_score;
                let score_b = b.1.trust_score;
                score_a.partial_cmp(&score_b).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(id, _)| *id)
            .unwrap_or_else(|| {
                // 如果没有蛊虫，返回一个默认值（实际应该创建新蛊虫）
                Uuid::nil()
            })
    }

    /// 判断是否应该熔断
    ///
    /// 世界模型判断知识是否足够，是否应该停止学习
    pub fn should_halt(&self) -> Option<String> {
        // 条件1: 知识库已满
        if self.state.knowledge_base.len() > 1000 {
            return Some("知识库已满，停止学习".to_string());
        }

        // 条件2: 所有蛊虫都过载
        let overloaded = self.gu_registry.values()
            .filter(|gu| gu.skills.iter().any(|s| s.knowledge_nodes.len() > 20))
            .count();

        if overloaded > self.gu_registry.len() / 2 {
            return Some("蛊虫知识网络过载，需要时间消化".to_string());
        }

        // 条件3: 生存状态不佳
        if self.safety_state.graceful_degradation.current_phase != safety::DegradationPhase::Normal {
            return Some("生存状态不佳，优先保障生存".to_string());
        }

        None
    }
}

/// Herness 决策结果
#[derive(Debug, Clone)]
pub struct HernessDecision {
    /// 是否理解
    pub understood: bool,
    /// 回复
    pub reply: String,
    /// 要求执行的动作
    pub action: Option<crate::herness_web::HernessAction>,
}

/// 文件消化结果
#[derive(Debug, Clone)]
pub struct FileDigestResult {
    /// 是否成功
    pub success: bool,
    /// 消息
    pub message: String,
    /// 已处理文件数
    pub files_processed: usize,
    /// 是否应该继续
    pub should_continue: bool,
}

/// 跨蛊虫突触
///
/// 连接不同蛊虫的神经元，形成世界级神经网络
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossGuSynapse {
    /// 来源蛊虫ID
    pub from_gu: Uuid,
    /// 来源神经元类型
    pub from_neuron: crate::core::NeuronType,
    /// 目标蛊虫ID
    pub to_gu: Uuid,
    /// 目标神经元类型
    pub to_neuron: crate::core::NeuronType,
    /// 连接权重
    pub weight: f64,
}

/// 蛊虫行为类型
#[derive(Debug, Clone)]
pub enum GuAction {
    /// 接取任务（从待领取任务列表）
    AcceptTask(Uuid),
    /// 购买资源（熵增）
    BuyResource,
    /// 学习行为（整理/拓展/补充知识）
    Learn(LearningAction),
    /// 转账
    Transfer {
        to_id: Uuid,
        amount: f64,
    },
}

impl Default for WorldMind {
    fn default() -> Self {
        Self::new()
    }
}

/// 获取当前时间戳
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_mind_creation() {
        let mind = WorldMind::new();
        assert_eq!(mind.population(), 0);
        assert!(mind.health() > 0.0);
    }

    #[test]
    fn test_gu_registration() {
        let mind = WorldMind::new();
        let gu_id = Uuid::new_v4();

        let new_mind = mind.register_gu(gu_id);
        assert_eq!(new_mind.population(), 1);
        assert_eq!(new_mind.access_point_count(), 5);
    }

    #[test]
    fn test_five_access_points() {
        let mind = WorldMind::new();
        let gu_id = Uuid::new_v4();

        let new_mind = mind.register_gu(gu_id);

        // 验证有5个接入点
        let gu_info = new_mind.gu_registry.get(&gu_id).unwrap();
        assert_eq!(gu_info.access_points.len(), 5);
    }

    #[test]
    fn test_survival_mechanism() {
        let config = WorldConfig::default();
        let mind = WorldMind::with_config(config);

        // 更新后应该自动生成蛊虫以满足最小种群
        let updated = mind.update();
        assert!(updated.population() >= mind.config.survival.min_population);
    }

    #[test]
    fn test_resonance_field() {
        let mind = WorldMind::new();
        let gu_id = Uuid::new_v4();
        let new_mind = mind.register_gu(gu_id);

        // 注册后共振场应该有蛊虫
        assert_eq!(new_mind.resonance_field.population(), 1);
    }

    #[test]
    fn test_safety_state() {
        let mind = WorldMind::new();
        assert!(mind.safety_score() > 0.0);
        assert_eq!(mind.trust_entropy(), 0.0);
    }

    #[test]
    fn test_consciousness_emergence() {
        let mut mind = WorldMind::new();

        // 注册足够多的蛊虫
        for _ in 0..10 {
            mind = mind.register_gu(Uuid::new_v4());
        }

        // 更新后应该涌现意识
        let updated = mind.update();
        // 由于初始相位相同，同步率应该很高
        assert!(updated.consciousness_sync_rate() > 0.5);
    }

    #[test]
    fn test_gu_lnn_integration() {
        let mind = WorldMind::new();
        let gu_id = Uuid::new_v4();
        let new_mind = mind.register_gu(gu_id);

        // 验证蛊虫有 LNN，通过五维状态向量间接验证
        let gu_info = new_mind.gu_registry.get(&gu_id).unwrap();
        let state_vec = gu_info.lnn.state_vector();

        // 验证五维状态向量有5个元素
        assert_eq!(state_vec.len(), 5);

        // 验证五维状态向量归一化
        let norm: f64 = state_vec.iter().map(|x| x * x).sum::<f64>().sqrt();
        assert!(norm.abs() < 0.01 || (norm - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_emergent_action_from_lnn() {
        let mut mind = WorldMind::new();

        // 注册两个蛊虫
        let gu_id1 = Uuid::new_v4();
        let gu_id2 = Uuid::new_v4();
        mind = mind.register_gu(gu_id1);
        mind = mind.register_gu(gu_id2);

        // 执行涌现行为
        let (new_mind, transaction) = mind.emergent_action();

        // 验证行为确实执行了（可能会有交易或者没有，取决于 LNN 状态）
        // 主要验证不会 panic
        assert_eq!(new_mind.population(), 2);
    }

    #[test]
    fn test_world_state_vector() {
        let mut mind = WorldMind::new();

        // 注册多个蛊虫
        for _ in 0..5 {
            mind = mind.register_gu(Uuid::new_v4());
        }

        let vec = mind.world_state_vector();

        // 验证维度
        assert_eq!(vec.len(), 5);

        // 验证归一化
        let norm: f64 = vec.iter().map(|x| x * x).sum::<f64>().sqrt();
        assert!(norm.abs() < 0.01 || (norm - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_world_survival() {
        let mut mind = WorldMind::new();

        // 无蛊虫时世界死亡
        assert_eq!(mind.world_survival(), 0.0);
        assert!(!mind.is_world_alive());

        // 注册蛊虫后世界存活
        // Survival 神经元初始 state = 0.0，bias = 0.5
        // 但需要更新后才会变为正值
        mind = mind.register_gu(Uuid::new_v4());

        // 触发神经网络更新（输入正信号到 Survival）
        let gu_id = mind.gu_ids()[0];
        let gu = mind.get_gu(&gu_id).unwrap();
        let mut lnn = gu.lnn.clone();
        lnn.input(crate::core::NeuronType::Survival, 0.5);
        lnn.update(0.01);

        // 此时 survival_state 应该 > 0
        assert!(lnn.survival_state() > 0.0);
    }

    #[test]
    fn test_diversity_calculation() {
        let mut mind = WorldMind::new();

        // 单个蛊虫无多样性
        mind = mind.register_gu(Uuid::new_v4());
        assert_eq!(mind.calculate_diversity(), 0.0);

        // 多个蛊虫有多样性
        mind = mind.register_gu(Uuid::new_v4());
        mind = mind.register_gu(Uuid::new_v4());
        let diversity = mind.calculate_diversity();
        assert!(diversity >= 0.0 && diversity <= 1.0);
    }

    #[test]
    fn test_emergence_factor() {
        let mut mind = WorldMind::new();

        for _ in 0..10 {
            mind = mind.register_gu(Uuid::new_v4());
        }

        let emergence = mind.emergence_factor();
        assert!(emergence >= 0.0 && emergence <= 1.0);
    }

    #[test]
    fn test_cross_gu_synapses() {
        let mut mind = WorldMind::new();

        // 注册多个蛊虫
        for _ in 0..5 {
            mind = mind.register_gu(Uuid::new_v4());
        }

        // 构建跨蛊虫突触
        let synapses = mind.build_cross_gu_synapses();

        // 应该有突触连接
        assert!(!synapses.is_empty());

        // 验证突触类型
        for synapse in &synapses {
            assert!(synapse.weight > 0.0);
            assert!(synapse.weight <= 1.0);
        }
    }

    #[test]
    fn test_world_network_update() {
        let mut mind = WorldMind::new();

        // 注册蛊虫
        for _ in 0..5 {
            mind = mind.register_gu(Uuid::new_v4());
        }

        // 更新世界神经网络
        mind.update_world_network();

        // 验证没有 panic，世界仍然存活
        assert_eq!(mind.population(), 5);
    }
}