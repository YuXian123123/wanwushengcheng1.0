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
pub mod cognis;
pub mod knowledge_encoder;
pub mod knowledge_storage;
pub mod knowledge_pool;
pub mod world_knowledge;
pub mod knowledge_collaboration;
pub mod chat_channel;
pub mod lnn_language;

#[cfg(test)]
mod topic_extraction_test;

#[cfg(test)]
mod knowledge_flow_test;

#[cfg(test)]
mod html_learning_test;

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
pub use cognis::{CogniParticle, EntityType, RelationType, ParseResult, CognisParser};
pub use knowledge_encoder::{KnowledgeEncoder, NeuralSignal, KnowledgeValue};
pub use knowledge_storage::{SkillStorage, SkillDocument, GuIndexDocument, SkillRef, SkillRelation, LearnerInfo};
pub use knowledge_pool::{KnowledgePool, CandidateKnowledge, VoteRecord, CandidateStatus, LearningResult, KnowledgeEvaluator};
pub use world_knowledge::{WorldKnowledgeStore, CandidateVersion, Vote, VotingSession, VotingStatus, KnowledgeGraph, GraphNode, GraphEdge};
pub use knowledge_collaboration::{
    KnowledgeCollaboration, KnowledgeDiscussion, ContentProposal, ContentPart,
    Comment, CommentType, ProposalStatus, DiscussionStatus, Conflict, ConflictParty,
    Stance, Argument, ArgumentType, Resolution, ResolutionMethod, Experience,
    ExperienceType, ConflictPattern, TimelineEvent, TimelineEventType, Revision,
};
pub use chat_channel::{
    ChatSystem, ChatChannel, ChatMessage, ChannelType, MessageContent,
    SenderRole, Reaction, SystemNotificationType,
};
pub use lnn_language::{
    LNNLanguageEmergence, LanguageVocabulary, EmergedMessage, LNNStateSnapshot,
};

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
    /// 知识协作系统（蛊虫讨论生成知识）
    #[serde(skip)]
    knowledge_collaboration: KnowledgeCollaboration,
    /// 聊天系统（蛊虫实时通信）
    #[serde(skip)]
    chat_system: ChatSystem,
    /// LNN 语言涌现器（神经网络状态 → 聊天消息）
    #[serde(skip)]
    language_emergence: LNNLanguageEmergence,
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

/// 候选主题及其得分（用于技能名称提取算法）
struct TopicCandidate {
    word: String,
    score: f64,
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
            knowledge_collaboration: KnowledgeCollaboration::new(),
            chat_system: ChatSystem::new(),
            language_emergence: LNNLanguageEmergence::new(),
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

        // 初始化钱包（使用配置中的出生金币）
        let wallet = GuWallet::new(gu_id, self.config.survival.gu_birth_coins);

        // 初始化技能（初始为空，通过学习获得）
        let skills = Vec::new();

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

        // 注册到知识协作系统
        new_mind.knowledge_collaboration.register_gu(gu_id);

        // 加入聊天频道
        new_mind.chat_system.join_channel("world", gu_id);
        new_mind.chat_system.join_channel("knowledge", gu_id);

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
    pub fn update(&mut self) {
        let now = current_timestamp();

        // 检测并移除死亡蛊虫
        // 注意：暂时跳过死亡检测，因为心跳机制尚未完善
        // let dead_gus = self.detect_dead_gus();

        // 更新健康状态
        self.state = self.state.update_health_status(&self.config);

        // 更新共振场（黑塔设计）- 意识涌现
        self.resonance_field = self.resonance_field.clone().update(1.0, now);

        // 更新安全状态（螺丝咕姆设计）
        let health = self.state.health;
        let gu_count = self.gu_registry.len() as u64;
        self.safety_state = self.safety_state.clone().update(health, gu_count, now);

        // 更新意识层（拉蒂奥设计）
        self.consciousness = self.consciousness.clone().merge_intentions();
        let trust_scores: HashMap<Uuid, f64> = self.gu_registry.iter()
            .map(|(id, info)| (*id, info.trust_score))
            .collect();
        self.consciousness = self.consciousness.process_decisions(&trust_scores);

        // 检查生存条件
        if !self.state.check_survival(&self.config) {
            // 触发恢复协议：生成新蛊虫
            let needed = self.config.survival.min_population - self.state.population;
            for _ in 0..needed {
                let gu_id = Uuid::new_v4();
                // 简化注册：直接添加
                let new_gu = GuInfo {
                    id: gu_id,
                    access_points: vec![],
                    trust_score: 0.5,
                    expertise: HashMap::new(),
                    last_heartbeat: now,
                    color_gene: ColorGene::random_primordial(),
                    parents: (None, None),
                    wallet: GuWallet::new(gu_id, 1000.0),
                    name: "新蛊虫".to_string(),
                    skills: Vec::new(),
                    lnn: GuLNN::new(),
                };
                self.gu_registry.insert(gu_id, new_gu);
                self.state.population += 1;
            }
        }

        // 优雅降级检查（螺丝咕姆设计）
        if self.safety_state.needs_emergency_intervention() {
            // 保存种子
            self.safety_state.layered_survival =
                self.safety_state.layered_survival.clone().save_seed(
                    self.state.population,
                    vec![],  // 知识快照
                    0,       // 配置哈希
                    now,
                );
        }

        // 自动分配待领取的任务（世界模型自治）
        self.auto_assign_pending_tasks();

        // LNN 状态扰动 → 聊天消息涌现（黑塔设计：网络状态驱动语言涌现）
        self.emerge_chat_from_lnn();
    }

    /// 从 LNN 状态涌现聊天消息
    ///
    /// 每次世界更新时，根据蛊虫的神经网络状态决定是否产生聊天消息
    /// 这是真正的"意识扰动"到"语言表达"的涌现
    fn emerge_chat_from_lnn(&mut self) {
        // 收集所有蛊虫的状态快照（包含技能信息）
        let snapshots: Vec<LNNStateSnapshot> = self.gu_registry.iter()
            .map(|(id, info)| LNNStateSnapshot::from_gu_info(*id, &info.name, &info.lnn, &info.skills))
            .collect();

        let gu_count = snapshots.len();
        if gu_count == 0 {
            return;
        }

        // 使用时间戳作为随机种子
        let now_ns = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        // 随机选择 3-5 个蛊虫尝试发言（增加多样性）
        let speak_count = 3 + (now_ns % 3) as usize;

        // 为每个蛊虫生成一个随机索引
        for i in 0..speak_count.min(gu_count) {
            let idx = ((now_ns as usize) + i * 7) % gu_count;
            let snapshot = &snapshots[idx];

            // 准备知识上下文
            let knowledge_context: Option<(&[String], Option<&str>)> = if !snapshot.recent_topics.is_empty() {
                Some((&snapshot.recent_topics, snapshot.knowledge_summary.as_deref()))
            } else {
                None
            };

            // 使用语言涌现器尝试生成消息
            if let Some(emerged) = self.language_emergence.emerge_message(
                snapshot.state_vector,
                snapshot.activity,
                snapshot.behavior_tendency,
                snapshot.gu_id,
                &snapshot.gu_name,
                snapshot.current_skill.as_deref(),
                knowledge_context,
            ) {
                // 发送到对应频道
                self.chat_system.send_message(
                    &emerged.channel_id,
                    emerged.sender_id,
                    &emerged.sender_name,
                    emerged.sender_role,
                    emerged.content,
                );
            }
        }

        // 共识消息：每 30 秒允许一次（避免刷屏）
        let update_count = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        if update_count % 30 == 0 && self.resonance_field.sync_rate > 0.85 && !snapshots.is_empty() {
            let avg_activity: f64 = snapshots.iter().map(|s| s.activity).sum::<f64>() / snapshots.len() as f64;

            if avg_activity > 0.5 {
                // 提取所有知识主题作为共识话题
                let all_topics: Vec<&String> = snapshots.iter()
                    .flat_map(|s| s.recent_topics.iter())
                    .take(3)
                    .collect();

                let topic = all_topics.first()
                    .map(|s| s.as_str())
                    .unwrap_or("当前讨论");

                if let Some(consensus) = self.language_emergence.emerge_consensus(
                    topic,
                    snapshots.len(),
                    self.resonance_field.sync_rate,
                ) {
                    self.chat_system.send_message(
                        &consensus.channel_id,
                        consensus.sender_id,
                        &consensus.sender_name,
                        consensus.sender_role,
                        consensus.content,
                    );
                }
            }
        }
    }

    /// 自动分配待领取的任务
    ///
    /// 世界模型根据蛊虫的能力和信任度，自动分配最合适的蛊虫
    fn auto_assign_pending_tasks(&mut self) {
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

    /// 获取蛊虫注册表（只读）
    pub fn gu_registry(&self) -> &HashMap<Uuid, GuInfo> {
        &self.gu_registry
    }

    /// 获取知识协作系统（只读）
    pub fn knowledge_collaboration(&self) -> &KnowledgeCollaboration {
        &self.knowledge_collaboration
    }

    /// 获取知识协作系统（可变）
    pub fn knowledge_collaboration_mut(&mut self) -> &mut KnowledgeCollaboration {
        &mut self.knowledge_collaboration
    }

    /// 获取聊天系统（只读）
    pub fn chat_system(&self) -> &ChatSystem {
        &self.chat_system
    }

    /// 获取聊天系统（可变）
    pub fn chat_system_mut(&mut self) -> &mut ChatSystem {
        &mut self.chat_system
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

        // 1. 更新每个蛊虫的内部网络 + 注入随机扰动
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        for (idx, gu) in self.gu_registry.values_mut().enumerate() {
            // 注入随机扰动（模拟世界的随机刺激）
            // 使用蛊虫索引和时间戳生成伪随机扰动
            let perturbation_base = ((now as f64) * (idx as f64 + 1.0)).sin();
            let perturbation = perturbation_base * 0.1; // 扰动强度 0.1

            // 随机选择一个神经元类型注入扰动
            let neuron_idx = (now as usize + idx) % 5;
            let neuron_type = match neuron_idx {
                0 => crate::core::NeuronType::Perception,
                1 => crate::core::NeuronType::Cognitive,
                2 => crate::core::NeuronType::Behavior,
                3 => crate::core::NeuronType::Comm,
                _ => crate::core::NeuronType::Survival,
            };

            gu.lnn.receive_world_signal(neuron_type, perturbation);

            // 正常网络更新
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

    /// 执行蛊虫行为，返回交易事件
    pub fn execute_gu_action(&mut self, gu_id: Uuid, action: GuAction) -> Option<TransactionData> {
        if !self.gu_registry.contains_key(&gu_id) {
            return None;
        }

        match action {
            GuAction::AcceptTask(task_id) => {
                // 蛊虫接取任务（不产生交易，只是状态变更）
                match self.assign_task(task_id, gu_id) {
                    Ok(()) => None,
                    Err(_) => None,
                }
            }
            GuAction::BuyResource => {
                let resource = Resource::random_resource();
                let gu = self.gu_registry.get_mut(&gu_id).unwrap();
                let result = GuBehavior::buy_resource(&mut gu.wallet, &gu.name, &resource);
                result.transaction
            }
            GuAction::Learn(action) => {
                let gu = self.gu_registry.get_mut(&gu_id).unwrap();
                let gu_name = gu.name.clone();
                let skills = &mut gu.skills;
                let result = GuBehavior::learn(skills, &gu_name, action);
                // 学习行为不产生交易事件（不消耗金币）
                result.transaction
            }
            GuAction::Transfer { to_id, amount } => {
                if !self.gu_registry.contains_key(&to_id) {
                    return None;
                }

                let from_name = self.gu_registry.get(&gu_id).unwrap().name.clone();
                let to_name = self.gu_registry.get(&to_id).unwrap().name.clone();

                // 检查余额
                let from_balance_before = self.gu_registry.get(&gu_id).unwrap().wallet.balance;
                if from_balance_before < amount {
                    return None;
                }

                // 先更新 from_wallet
                {
                    let from_wallet = &mut self.gu_registry.get_mut(&gu_id).unwrap().wallet;
                    from_wallet.balance -= amount;
                    from_wallet.total_expense += amount;
                }

                // 再更新 to_wallet
                let (from_balance_after, to_balance_after) = {
                    let from_balance = self.gu_registry.get(&gu_id).unwrap().wallet.balance;
                    let to_wallet = &mut self.gu_registry.get_mut(&to_id).unwrap().wallet;
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
        }
    }

    /// 随机执行一个蛊虫的随机行为（用于模拟）
    pub fn random_action(&mut self) -> Option<TransactionData> {
        self.emergent_action()
    }

    /// 从神经网络状态涌现行为（黑塔设计：网络状态驱动行为涌现）
    ///
    /// 行为不再随机选择，而是从蛊虫的 LNN 状态涌现：
    /// 1. 更新蛊虫神经网络（接收世界信号）
    /// 2. 根据网络状态决定行为倾向
    /// 3. 根据行为倾向执行具体行为
    pub fn emergent_action(&mut self) -> Option<TransactionData> {
        let gu_ids: Vec<Uuid> = self.gu_registry.keys().copied().collect();
        if gu_ids.is_empty() {
            return None;
        }

        // 选择一个蛊虫（可以考虑优先选择活跃度高的）
        let gu_id = gu_ids[rand::random::<usize>() % gu_ids.len()];

        // 获取当前世界信号
        let world_health = self.health();
        // 使用配置中的最小种群作为基准计算种群压力
        let population_pressure = self.population() as f64
            / (self.config.survival.min_population.max(10) * 10) as f64;

        // 更新蛊虫神经网络
        let gu = self.gu_registry.get_mut(&gu_id).unwrap();

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
                let pending_tasks: Vec<Uuid> = self.tasks.iter()
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
                let gu = self.gu_registry.get(&gu_id).unwrap();
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
                return None;
            }
            BehaviorTendency::Survival => {
                // 生存优先：尝试接取任务获取金币
                let pending_tasks: Vec<Uuid> = self.tasks.iter()
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

        self.execute_gu_action(gu_id, action)
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
    /// 同时蛊虫会学习任务所需的技能
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

        // 保存任务信息用于学习
        let task_name = task.name.clone();
        let task_description = task.description.clone();
        let task_skills = task.required_skills.clone();

        // 标记任务完成
        task.complete();

        // 发放奖励并学习技能
        let reward = task.reward;
        let gu = self.gu_registry.get_mut(&gu_id)
            .ok_or_else(|| format!("蛊虫 {} 不存在", gu_id))?;

        // 学习技能：根据任务需求或任务名称创建知识点
        if !task_skills.is_empty() {
            // 任务有技能要求，学习这些技能
            for skill_name in &task_skills {
                // 检查是否已有该技能
                if let Some(existing_skill) = gu.skills.iter_mut().find(|s| &s.name == skill_name) {
                    // 已有技能，添加新知识点
                    let node = KnowledgeNode::new(
                        format!("{}实践经验", task_name),
                        format!("通过完成任务「{}」获得", task_name),
                        KnowledgeNodeType::Experience,
                    );
                    existing_skill.extend_knowledge(node, None);
                } else {
                    // 没有该技能，创建新技能
                    let foundation = KnowledgeNode::new(
                        format!("{}基础", skill_name),
                        format!("通过完成任务「{}」入门", task_name),
                        KnowledgeNodeType::Foundation,
                    );
                    let new_skill = Skill::new(
                        skill_name.clone(),
                        task_description.clone(),
                        foundation,
                    );
                    gu.skills.push(new_skill);
                }
            }
        } else {
            // 任务没有技能要求，根据任务名称创建技能
            let foundation = KnowledgeNode::new(
                format!("{}基础", task_name),
                task_description.clone(),
                KnowledgeNodeType::Foundation,
            );
            let new_skill = Skill::new(
                task_name.clone(),
                task_description.clone(),
                foundation,
            );
            gu.skills.push(new_skill);
        }

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
    ///
    /// # 知识消耗流程（改造后）
    ///
    /// ```text
    /// 知识文件
    ///    │
    ///    ▼
    /// ┌─────────────────────────────────────┐
    /// │ 1. 认知素分解 (CognisParser)        │
    /// │    - 提取实体 (Entity)              │
    /// │    - 提取属性 (Attribute)           │
    /// │    - 提取关系 (Relation)            │
    /// └─────────────────────────────────────┘
    ///    │
    ///    ▼
    /// ┌─────────────────────────────────────┐
    /// │ 2. 信号编码 (KnowledgeEncoder)      │
    /// │    - Entity → Perceive+Cognitive    │
    /// │    - Relation → Cognitive+Comm      │
    /// │    - 归一化信号强度                  │
    /// └─────────────────────────────────────┘
    ///    │
    ///    ▼
    /// ┌─────────────────────────────────────┐
    /// │ 3. LNN 融合学习                     │
    /// │    - 输入神经信号                   │
    /// │    - 赫布学习更新权重               │
    /// │    - 更新活跃度                     │
    /// └─────────────────────────────────────┘
    ///    │
    ///    ▼
    /// ┌─────────────────────────────────────┐
    /// │ 4. 知识存储                         │
    /// │    - 创建技能文档                   │
    /// │    - 持久化到 HTML 文件             │
    /// │    - 更新蛊虫索引                   │
    /// └─────────────────────────────────────┘
    /// ```
    pub fn receive_knowledge_file(&mut self, file_event: &crate::herness_web::KnowledgeFileEvent) -> FileDigestResult {
        use cognis::CognisParser;
        use knowledge_encoder::KnowledgeEncoder;
        use lnn_language::LNNLanguageEmergence;

        // ===== 获取所有蛊虫，全部参与学习 =====
        let all_gu_ids: Vec<Uuid> = self.gu_registry.keys().copied().collect();
        let gu_count = all_gu_ids.len();

        if gu_count == 0 {
            return FileDigestResult {
                success: false,
                message: "没有可用的蛊虫进行学习".to_string(),
                files_processed: file_event.index,
                should_continue: false,
                skill_name: None,
            };
        }

        // ===== 第一阶段：认知素分解（统一解析，所有蛊虫共享） =====
        let mut parser = CognisParser::new();
        let parse_result = parser.parse(&file_event.content);

        // ===== 第二阶段：信号编码（统一编码） =====
        let encoder = KnowledgeEncoder::new();
        let neural_signals = encoder.encode(&parse_result);
        let knowledge_value = encoder.calculate_knowledge_value(&parse_result);

        // ===== 第三阶段：所有蛊虫 LNN 融合学习 =====
        // 每个蛊虫都接收神经信号，但由于网络结构差异，产生的状态各不相同
        for gu_id in &all_gu_ids {
            if let Some(gu) = self.gu_registry.get_mut(gu_id) {
                // 输入编码后的神经信号（每个蛊虫的响应不同）
                for signal in &neural_signals {
                    gu.lnn.receive_world_signal(signal.neuron_type, signal.strength);
                }

                // 额外的学习信号（基于知识价值）
                gu.lnn.receive_world_signal(
                    crate::core::NeuronType::Cognitive,
                    knowledge_value.score * 0.3,
                );

                // 触发多次更新以促进赫布学习（每个蛊虫学习过程不同）
                for _ in 0..parse_result.particles.len().min(10) {
                    gu.lnn.update(0.01);
                }
            }
        }

        // ===== 第四阶段：知识协作讨论（LNN 神经网络驱动） =====
        let skill_name = match &parse_result.main_topic {
            Some(topic) => topic.clone(),
            None => Self::extract_skill_name(&file_event.relative_path, &file_event.filename, &file_event.content),
        };

        // 提取知识内容用于讨论
        let definition = Self::create_knowledge_summary(&parse_result, &file_event.content);
        let topics: Vec<String> = parse_result.keywords.iter().take(5).cloned().collect();

        // 发送系统通知：讨论开始
        self.chat_system.send_system_notification(
            "knowledge",
            chat_channel::SystemNotificationType::DiscussionStarted,
            format!("📌 新知识讨论开始: {} (共 {} 只蛊虫参与)", skill_name, gu_count),
        );

        // 创建讨论，由第一只蛊虫发起
        let initiator_id = all_gu_ids[0];

        // 开始讨论
        self.knowledge_collaboration.start_discussion(&skill_name, initiator_id);

        // ===== 使用 LNN 神经网络生成每个蛊虫的讨论内容 =====
        let language_emergence = LNNLanguageEmergence::default();

        // 收集每个蛊虫生成的消息
        let mut gu_messages: Vec<(Uuid, String, String)> = Vec::new(); // (gu_id, gu_name, message)

        for (idx, gu_id) in all_gu_ids.iter().enumerate() {
            // 跳过发起者（已经加入讨论）
            if idx > 0 {
                self.knowledge_collaboration.join_discussion(&skill_name, *gu_id);
            }

            // 获取蛊虫信息
            let (gu_name, state_vector, activity, behavior_tendency) = {
                if let Some(gu) = self.gu_registry.get(gu_id) {
                    (
                        gu.name.clone(),
                        gu.lnn.state_vector(),
                        gu.lnn.get_overall_activity(),
                        gu.lnn.decide_behavior(),
                    )
                } else {
                    continue;
                }
            };

            // 使用 LNN 神经网络状态涌现消息
            if let Some(emerged_msg) = language_emergence.emerge_message(
                state_vector,
                activity,
                behavior_tendency,
                *gu_id,
                &gu_name,
                Some(&file_event.filename),
                Some((&topics, Some(definition.as_str()))),
            ) {
                // 将涌现的消息内容转换为字符串
                let message_text = match emerged_msg.content {
                    chat_channel::MessageContent::Text(text) => text,
                    chat_channel::MessageContent::Proposal { topic, content, .. } => {
                        format!("📝 关于「{}」的提议: {}", topic, content)
                    }
                    chat_channel::MessageContent::Vote { support, proposal_id, reason } => {
                        format!("{} {} ({})", if support { "✅ 同意" } else { "❌ 反对" }, proposal_id, reason)
                    }
                    _ => format!("关于「{}」的思考", skill_name),
                };

                gu_messages.push((*gu_id, gu_name.clone(), message_text));

                // 发起者额外发送提议
                if idx == 0 {
                    let proposal_id = self.knowledge_collaboration.propose(
                        &skill_name,
                        *gu_id,
                        &gu_name,
                        ContentPart::Definition,
                        definition.clone(),
                    );

                    if let Some(pid) = proposal_id {
                        self.chat_system.send_message(
                            "knowledge",
                            *gu_id,
                            &gu_name,
                            chat_channel::SenderRole::Gu,
                            chat_channel::MessageContent::Proposal {
                                topic: skill_name.clone(),
                                part: ContentPart::Definition,
                                content: definition.clone(),
                                proposal_id: pid,
                            },
                        );
                    }
                }
            } else {
                // 如果神经网络活跃度不足，发送简短的参与消息
                gu_messages.push((*gu_id, gu_name.to_string(), format!("我正在学习「{}」...", skill_name)));
            }
        }

        // 发送所有蛊虫的讨论消息
        for (gu_id, gu_name, message) in gu_messages {
            self.chat_system.send_message(
                "knowledge",
                gu_id,
                &gu_name,
                chat_channel::SenderRole::Gu,
                chat_channel::MessageContent::Text(message),
            );
        }

        // ===== 第五阶段：所有蛊虫投票 =====
        // 获取该主题所有提议并让所有蛊虫投票
        if let Some(discussion) = self.knowledge_collaboration.get_discussion(&skill_name) {
            // 收集提议 ID
            let proposal_ids: Vec<String> = discussion.proposals.values()
                .flat_map(|ps| ps.iter().map(|p| p.id.clone()))
                .collect();

            // 让每个蛊虫投票（投票理由由 LNN 状态驱动）
            for proposal_id in &proposal_ids {
                for voter_id in &all_gu_ids {
                    // 获取蛊虫的神经网络状态来决定投票理由
                    let (voter_name, state_vector, activity) = {
                        if let Some(gu) = self.gu_registry.get(voter_id) {
                            (gu.name.clone(), gu.lnn.state_vector(), gu.lnn.get_overall_activity())
                        } else {
                            continue;
                        }
                    };

                    // 根据神经网络状态生成投票理由
                    let cognitive = state_vector[1]; // Cognitive 神经元状态
                    let comm = state_vector[3]; // Comm 神经元状态

                    let reason = if cognitive > 0.5 && comm > 0.5 {
                        format!("经过思考，我认为这个提议很有价值 (认知度: {:.0}%)", cognitive * 100.0)
                    } else if cognitive > 0.3 {
                        format!("我同意这个观点，值得深入探讨")
                    } else {
                        format!("支持该提议 (活跃度: {:.0}%)", activity * 100.0)
                    };

                    // 投票支持
                    self.knowledge_collaboration.vote_support(&skill_name, proposal_id, *voter_id);

                    // 在聊天频道显示投票
                    self.chat_system.send_message(
                        "knowledge",
                        *voter_id,
                        &voter_name,
                        chat_channel::SenderRole::Gu,
                        chat_channel::MessageContent::Vote {
                            proposal_id: proposal_id.clone(),
                            support: true,
                            reason,
                        },
                    );
                }
            }
        }

        // 检查共识
        if let Some(_consensus) = self.knowledge_collaboration.check_consensus(&skill_name) {
            // 在聊天频道发送共识达成通知
            self.chat_system.send_message(
                "knowledge",
                Uuid::nil(),
                "系统",
                chat_channel::SenderRole::System,
                chat_channel::MessageContent::ConsensusReached {
                    topic: skill_name.clone(),
                    participants: all_gu_ids.len(),
                },
            );

            // 共识达成，保存到世界知识库
            if let Some(_path) = self.knowledge_collaboration.save_to_world(&skill_name) {
                // 发送知识入库通知
                self.chat_system.send_system_notification(
                    "knowledge",
                    chat_channel::SystemNotificationType::KnowledgeStored,
                    format!("✅ 知识「{}」已入库，所有蛊虫已习得", skill_name),
                );

                // ===== 更新所有蛊虫的技能 =====
                for gu_id in &all_gu_ids {
                    if let Some(gu) = self.gu_registry.get_mut(gu_id) {
                        let knowledge_node = KnowledgeNode::new(
                            file_event.filename.clone(),
                            definition.clone(),
                            KnowledgeNodeType::Foundation,
                        );

                        // 查找或创建技能
                        let skill_idx = gu.skills.iter()
                            .position(|s| s.name == skill_name);

                        if let Some(idx) = skill_idx {
                            gu.skills[idx].extend_knowledge(knowledge_node, None);
                        } else {
                            let new_skill = Skill::new(
                                skill_name.clone(),
                                format!("通过协作学习 {} 获得", file_event.filename),
                                knowledge_node,
                            );
                            gu.skills.push(new_skill);
                        }

                        // 更新信任度（每只蛊虫都获得奖励）
                        self.safety_state.trust_entropy =
                            self.safety_state.trust_entropy.reward(gu_id, knowledge_value.score * 0.05);
                    }
                }
            }
        }

        FileDigestResult {
            success: true,
            message: format!("文件 {} 已通过 {} 只蛊虫协作学习处理", file_event.filename, gu_count),
            files_processed: file_event.index,
            should_continue: true,
            skill_name: Some(skill_name),
        }
    }

    /// 创建知识摘要（从解析结果生成）
    fn create_knowledge_summary(
        parse_result: &cognis::ParseResult,
        content: &str,
    ) -> String {
        let mut summary_parts = Vec::new();

        // 主题
        if let Some(topic) = &parse_result.main_topic {
            summary_parts.push(format!("主题: {}", topic));
        }

        // 代码语言
        if !parse_result.code_languages.is_empty() {
            summary_parts.push(format!("语言: {}", parse_result.code_languages.join(", ")));
        }

        // 关键词
        if !parse_result.keywords.is_empty() {
            summary_parts.push(format!("关键词: {}", parse_result.keywords.iter().take(5).cloned().collect::<Vec<_>>().join(", ")));
        }

        // 实体数量
        let entity_count = parse_result.particles.iter()
            .filter(|p| matches!(p, cognis::CogniParticle::Entity { .. }))
            .count();
        if entity_count > 0 {
            summary_parts.push(format!("实体数: {}", entity_count));
        }

        // 原始内容摘要
        let content_preview: String = content.chars().take(300).collect();
        summary_parts.push(format!("\n{}", content_preview));

        summary_parts.join("\n")
    }

    /// 从文件路径中提取技能名称（三天才裁决方法 - 天才委员会设计）
    ///
    /// 算法步骤（三天才裁决）：
    /// 1. 黑塔：创新性评估 - 代码语言识别（权重最高）
    /// 2. 螺丝咕姆：可信度评估 - 领域关键词匹配
    /// 3. 拉蒂奥：优雅度评估 - 位置和频率加权
    /// 4. 综合裁决：选择综合得分最高的主题
    fn extract_skill_name(relative_path: &str, _filename: &str, content: &str) -> String {
        // 获取三种方法的结果
        let abstraction_result = Self::abstraction_ladder_topic(content);
        let analogy_result = Self::cross_domain_analogy_topic(content);
        let elegance_result = Self::elegance_scorer_topic(content);

        // 三天才裁决
        let main_topic = Self::trinity_decide_topic(
            content,
            &abstraction_result,
            &analogy_result,
            &elegance_result,
        );

        // 从相对路径提取子主题
        let sub_topic = Self::extract_sub_topic(relative_path);

        // 组合技能名
        match (main_topic, sub_topic) {
            (Some(main), Some(sub)) => format!("{}-{}", main, sub),
            (Some(main), None) => main,
            (None, Some(sub)) => sub,
            (None, None) => "通用知识".to_string(),
        }
    }

    // ==================== 三种主题提取方法 ====================

    /// 方法1: 抽象阶梯 - 从代码语言和标题提取
    fn abstraction_ladder_topic(content: &str) -> Option<String> {
        // 维度1: 代码块语言（权重最高）
        let languages = Self::extract_code_languages(content);
        if let Some(lang) = languages.first() {
            return Some(lang.clone());
        }

        // 维度2: Markdown 标题关键词
        let title_words = Self::extract_title_words(content);
        if let Some(word) = title_words.first() {
            return Some(word.clone());
        }

        None
    }

    /// 方法2: 跨域类比 - 基于领域关键词匹配
    fn cross_domain_analogy_topic(content: &str) -> Option<String> {
        // 领域知识库（中心词）
        let domain_centers: [(&str, &[&str]); 5] = [
            ("HTML", &["tag", "element", "attribute", "document", "html", "body", "head", "div", "span"]),
            ("CSS", &["style", "selector", "property", "value", "flex", "grid", "margin", "padding"]),
            ("JavaScript", &["function", "variable", "const", "let", "var", "async", "promise", "callback"]),
            ("Rust", &["fn", "let", "mut", "struct", "impl", "trait", "borrow", "ownership"]),
            ("Python", &["def", "class", "import", "self", "lambda", "list", "dict"]),
        ];

        let content_lower = content.to_lowercase();
        let mut best_domain: Option<String> = None;
        let mut best_score = 0.0;

        for (domain, keywords) in &domain_centers {
            let mut matches = 0;

            // 检查领域名称本身是否出现
            if content_lower.contains(&domain.to_lowercase()) {
                matches += 3; // 领域名本身出现，权重加3
            }

            // 检查关键词
            for keyword in *keywords {
                if content_lower.contains(keyword) {
                    matches += 1;
                }
            }
            let score = matches as f32 / (keywords.len() + 3) as f32;
            if score > best_score && score >= 0.15 {
                best_score = score;
                best_domain = Some(domain.to_string());
            }
        }

        best_domain
    }

    /// 方法3: 优雅评分 - 基于位置和频率
    fn elegance_scorer_topic(content: &str) -> Option<String> {
        let candidates = Self::extract_topic_candidates(content);
        Self::select_best_topic(&candidates)
    }

    // ==================== 三天才裁决 ====================

    /// 三天才裁决：综合评估选择最佳主题
    ///
    /// 权重分配：
    /// - 跨域类比（螺丝咕姆）：权重 0.5（最高，因为基于领域知识库）
    /// - 抽象阶梯（黑塔）：权重 0.3
    /// - 优雅评分（拉蒂奥）：权重 0.2
    fn trinity_decide_topic(
        content: &str,
        abstraction_result: &Option<String>,
        analogy_result: &Option<String>,
        elegance_result: &Option<String>,
    ) -> Option<String> {
        // 收集所有候选及其加权得分
        let mut candidates: Vec<(String, f32)> = Vec::new();

        // 黑塔评分：创新性（代码语言识别）
        if let Some(topic) = abstraction_result {
            let base_score = Self::black_tower_score(topic, content);
            let weighted = base_score * 0.3; // 权重 30%
            candidates.push((topic.clone(), weighted));
        }

        // 螺丝咕姆评分：可信度（领域匹配）- 最高权重
        if let Some(topic) = analogy_result {
            let base_score = Self::screwllum_score(topic, content);
            let weighted = base_score * 0.5; // 权重 50%
            candidates.push((topic.clone(), weighted));
        }

        // 拉蒂奥评分：优雅度（位置频率）
        if let Some(topic) = elegance_result {
            let base_score = Self::latio_score(topic);
            let weighted = base_score * 0.2; // 权重 20%
            candidates.push((topic.clone(), weighted));
        }

        // 选择得分最高的
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        candidates.first().map(|(topic, _)| topic.clone())
    }

    /// 黑塔评分：创新性评估
    fn black_tower_score(topic: &str, content: &str) -> f32 {
        // 代码语言识别得分最高
        let code_languages = ["html", "css", "javascript", "rust", "python", "java", "go"];
        let topic_lower = topic.to_lowercase();

        let mut score = 0.5;
        if code_languages.contains(&topic_lower.as_str()) {
            score = 0.9; // 代码语言，高分
        } else if content.to_lowercase().matches(&topic_lower).count() > 5 {
            score = 0.7; // 出现多次，中等分
        }
        score
    }

    /// 螺丝咕姆评分：可信度评估
    fn screwllum_score(topic: &str, content: &str) -> f32 {
        // 检查主题在内容中出现的次数
        let count = content.to_lowercase().matches(&topic.to_lowercase()).count();
        if count > 10 {
            0.9
        } else if count > 5 {
            0.7
        } else if count > 0 {
            0.5
        } else {
            0.1
        }
    }

    /// 拉蒂奥评分：优雅度评估
    fn latio_score(topic: &str) -> f32 {
        // 短而精的主题得分更高
        let len = topic.len();
        if len <= 5 {
            0.9
        } else if len <= 10 {
            0.7
        } else {
            0.5
        }
    }

    /// 从内容中提取主题候选词（多维度特征提取）
    fn extract_topic_candidates(content: &str) -> Vec<TopicCandidate> {
        let mut candidates: Vec<TopicCandidate> = Vec::new();

        // 维度1: 从代码块语言标识提取（权重最高 = 4.0）
        let code_languages = Self::extract_code_languages(content);
        for lang in code_languages {
            candidates.push(TopicCandidate { word: lang, score: 4.0 });
        }

        // 维度2: 从 Markdown 标题中提取（权重 = 3.0）
        let title_words = Self::extract_title_words(content);
        for word in title_words {
            candidates.push(TopicCandidate { word, score: 3.0 });
        }

        // 维度3: 从高频词提取（权重 = 词频 × 0.5）
        let frequent_words = Self::extract_frequent_words(content, 5);
        for (word, count) in frequent_words {
            if count >= 2 {
                candidates.push(TopicCandidate { word, score: count as f64 * 0.5 });
            }
        }

        candidates
    }

    /// 从 Markdown 标题中提取关键词
    fn extract_title_words(content: &str) -> Vec<String> {
        let mut words = Vec::new();

        for line in content.lines().take(50) {
            let trimmed = line.trim();
            if trimmed.starts_with('#') {
                let title = trimmed.trim_start_matches('#').trim();
                for word in title.split_whitespace() {
                    let word = word.trim_matches(|c: char| !c.is_alphanumeric());
                    if word.len() >= 2 && word.len() <= 20 {
                        let has_upper = word.chars().any(|c| c.is_uppercase());
                        let is_alnum = word.chars().all(|c| c.is_ascii_alphanumeric());
                        if has_upper && is_alnum {
                            let normalized = Self::normalize_topic_word(word);
                            if !words.contains(&normalized) {
                                words.push(normalized);
                            }
                        }
                    }
                }
            }
        }

        words.truncate(3);
        words
    }

    /// 从代码块中提取语言标识
    fn extract_code_languages(content: &str) -> Vec<String> {
        let mut languages = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("```") {
                let lang = trimmed.trim_start_matches('`').trim();
                if !lang.is_empty() && lang.len() <= 20 {
                    let normalized = Self::normalize_topic_word(lang);
                    if !languages.contains(&normalized) {
                        languages.push(normalized);
                    }
                }
            }
        }

        languages.truncate(2);
        languages
    }

    /// 提取高频词（排除停用词）
    fn extract_frequent_words(content: &str, top_n: usize) -> Vec<(String, usize)> {
        use std::collections::HashMap;

        let stop_words = [
            "the", "a", "an", "is", "are", "was", "were", "be", "been", "being",
            "have", "has", "had", "do", "does", "did", "will", "would", "could",
            "should", "may", "might", "must", "shall", "can", "to", "of", "in",
            "for", "on", "with", "at", "by", "from", "as", "and", "but", "or",
            "not", "this", "that", "的", "是", "在", "了", "和", "与", "或", "有",
            "这", "那", "个", "上", "下", "中", "为", "以", "于", "对", "也", "就",
        ];

        let mut word_count: HashMap<String, usize> = HashMap::new();
        let content_preview: String = content.chars().take(2000).collect();

        for word in content_preview.split(|c: char| c.is_whitespace() || c.is_ascii_punctuation()) {
            let word = word.trim();
            if word.len() < 2 || word.len() > 20 { continue; }
            let word_lower = word.to_lowercase();
            if stop_words.contains(&word_lower.as_str()) { continue; }

            let has_upper = word.chars().any(|c| c.is_uppercase());
            let is_ascii = word.chars().all(|c| c.is_ascii_alphanumeric());
            if has_upper && is_ascii {
                let normalized = Self::normalize_topic_word(word);
                *word_count.entry(normalized).or_insert(0) += 1;
            }
        }

        let mut sorted: Vec<(String, usize)> = word_count.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        sorted.truncate(top_n);
        sorted
    }

    /// 规范化主题词
    fn normalize_topic_word(word: &str) -> String {
        let mut chars = word.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => {
                let first_upper = first.to_uppercase().to_string();
                let rest: String = chars.map(|c| c.to_lowercase().next().unwrap_or(c)).collect();
                format!("{}{}", first_upper, rest)
            }
        }
    }

    /// 选择最佳主题
    fn select_best_topic(candidates: &[TopicCandidate]) -> Option<String> {
        candidates
            .iter()
            .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal))
            .map(|c| c.word.clone())
    }

    /// 从相对路径中提取子主题（父目录名）
    fn extract_sub_topic(relative_path: &str) -> Option<String> {
        let parts: Vec<&str> = relative_path.split('/').filter(|s| !s.is_empty()).collect();

        if parts.len() > 1 {
            let parent_dir = parts[0];
            // 直接使用父目录名，不做硬编码映射
            return Some(parent_dir.to_string());
        }

        None
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
                println!("[WorldMind] 警告：没有蛊虫可供学习！");
                Uuid::nil()
            })
    }

    /// 获取蛊虫技能数量（用于调试）
    pub fn get_total_skills(&self) -> usize {
        self.gu_registry.values().map(|gu| gu.skills.len()).sum()
    }

    /// 获取所有蛊虫的技能摘要
    pub fn get_skills_summary(&self) -> Vec<(Uuid, String, usize)> {
        self.gu_registry.iter()
            .filter_map(|(id, gu)| {
                if gu.skills.is_empty() {
                    None
                } else {
                    Some((*id, gu.name.clone(), gu.skills.len()))
                }
            })
            .collect()
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
    /// 创建的技能名称（如果有）
    pub skill_name: Option<String>,
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
        let mut mind = WorldMind::with_config(config);

        // 更新后应该自动生成蛊虫以满足最小种群
        mind.update();
        assert!(mind.population() >= mind.config.survival.min_population);
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
        mind.update();
        // 由于初始相位相同，同步率应该很高
        assert!(mind.consciousness_sync_rate() > 0.5);
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
        let _transaction = mind.emergent_action();

        // 验证行为确实执行了（可能会有交易或者没有，取决于 LNN 状态）
        // 主要验证不会 panic
        assert_eq!(mind.population(), 2);
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

pub mod skill_extractor;
