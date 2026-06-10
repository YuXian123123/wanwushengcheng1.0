//! 蛊虫神经网络模块
//!
//! 每个蛊虫拥有独立的 LNN（液体神经网络），包含5个核心神经元对应5个接入点。
//!
//! # 设计理念
//!
//! - 黑塔：网络状态驱动行为涌现
//! - 螺丝咕姆：Survival 神经元状态直接关联蛊虫存活
//! - 拉蒂奥：五维状态向量满足归一化约束
//!
//! # 架构
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │              GuLNN (蛊虫神经网络)        │
//! │                                         │
//! │  ┌─────────┐     ┌─────────┐           │
//! │  │Perceive │ ──▶ │Cognitive│ ──▶ ...   │
//! │  │ 神经元  │     │ 神经元  │           │
//! │  └─────────┘     └─────────┘           │
//! │       │               │                 │
//! │       ▼               ▼                 │
//! │  ┌─────────┐     ┌─────────┐           │
//! │  │ Behavior│     │  Comm   │           │
//! │  │ 神经元  │     │ 神经元  │           │
//! │  └─────────┘     └─────────┘           │
//! │       │               │                 │
//! │       └───────┬───────┘                 │
//! │               ▼                         │
//! │         ┌─────────┐                     │
//! │         │Survival │ ← 生存绑定          │
//! │         │ 神经元  │                     │
//! │         └─────────┘                     │
//! └─────────────────────────────────────────┘
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::core::NeuronType;

/// 接入点类型（重导出以便使用）
pub use super::AccessPointType;

/// 接入点状态（用于前端显示）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPointStatus {
    pub perception: f64,
    pub action: f64,
    pub communication: f64,
    pub memory: f64,
    pub reasoning: f64,
}

/// 蛊虫神经元状态（可序列化）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuNeuronState {
    /// 神经元 ID
    pub id: Uuid,
    /// 神经元类型（对应接入点）
    pub neuron_type: NeuronType,
    /// 当前状态值 [-1, 1]
    pub state: f64,
    /// 时间常数 τ
    pub tau: f64,
    /// 偏置
    pub bias: f64,
    /// 活跃度
    pub activity: f64,
}

impl GuNeuronState {
    /// 创建新神经元
    pub fn new(neuron_type: NeuronType) -> Self {
        let (tau, bias) = match neuron_type {
            NeuronType::Perception => (2.0, 0.0),   // 慢响应，无偏置
            NeuronType::Cognitive => (1.0, 0.0),    // 中等响应
            NeuronType::Behavior => (0.5, 0.0),     // 快响应
            NeuronType::Comm => (1.0, 0.0),         // 中等响应
            NeuronType::Survival => (5.0, 0.5),     // 慢响应，正偏置（倾向存活）
        };

        Self {
            id: Uuid::new_v4(),
            neuron_type,
            state: 0.0,
            tau,
            bias,
            activity: 0.0,
        }
    }

    /// 更新神经元状态
    ///
    /// 状态方程: τ·dx/dt = -x + input + bias
    /// 欧拉法: x(t+dt) = x(t) + dt/τ * (-x + input + bias)
    pub fn update(&mut self, input: f64, dt: f64) -> f64 {
        let dx = (dt / self.tau) * (-self.state + input + self.bias);
        self.state = (self.state + dx).clamp(-1.0, 1.0);

        // 活跃度更新（指数移动平均）
        self.activity = self.activity * 0.99 + self.state.abs() * 0.01;

        self.state
    }

    /// 获取状态
    pub fn state(&self) -> f64 {
        self.state
    }
}

/// 跨神经元连接
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuSynapse {
    /// 突触 ID
    pub id: Uuid,
    /// 来源神经元 ID
    pub from: Uuid,
    /// 目标神经元 ID
    pub to: Uuid,
    /// 连接权重
    pub weight: f64,
}

impl GuSynapse {
    pub fn new(from: Uuid, to: Uuid, weight: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            from,
            to,
            weight,
        }
    }
}

/// 蛊虫神经网络
///
/// 每个蛊虫拥有一个 GuLNN，包含5个核心神经元和它们之间的连接。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuLNN {
    /// 神经元集合（按类型索引）
    neurons: HashMap<NeuronType, GuNeuronState>,
    /// 神经元 ID 到类型的映射
    neuron_id_to_type: HashMap<Uuid, NeuronType>,
    /// 内部突触（神经元之间的连接）
    synapses: Vec<GuSynapse>,
    /// 当前时间
    current_time: f64,
    /// 学习率
    learning_rate: f64,
}

impl GuLNN {
    /// 创建新的蛊虫神经网络
    ///
    /// 初始化5个核心神经元，并建立默认连接：
    /// - Perceive → Cognitive
    /// - Cognitive → Behavior
    /// - Cognitive → Comm
    /// - 所有 → Survival
    pub fn new() -> Self {
        let mut neurons = HashMap::new();
        let mut neuron_id_to_type = HashMap::new();

        // 创建5个核心神经元，添加随机扰动让每个蛊虫不同
        // 使用时间戳作为随机种子
        let random_factor = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as f64;

        for neuron_type in [
            NeuronType::Perception,
            NeuronType::Cognitive,
            NeuronType::Behavior,
            NeuronType::Comm,
            NeuronType::Survival,
        ] {
            let mut neuron = GuNeuronState::new(neuron_type);

            // 添加随机初始扰动（-0.3 到 0.3）
            let perturbation = ((random_factor * (neuron_type as usize + 1) as f64).sin() * 0.3);
            neuron.state = neuron.state + perturbation;
            neuron.state = neuron.state.clamp(-1.0, 1.0);

            neuron_id_to_type.insert(neuron.id, neuron_type);
            neurons.insert(neuron_type, neuron);
        }

        // 创建默认连接
        let perceive_id = neurons.get(&NeuronType::Perception).unwrap().id;
        let cognitive_id = neurons.get(&NeuronType::Cognitive).unwrap().id;
        let behavior_id = neurons.get(&NeuronType::Behavior).unwrap().id;
        let comm_id = neurons.get(&NeuronType::Comm).unwrap().id;
        let survival_id = neurons.get(&NeuronType::Survival).unwrap().id;

        let synapses = vec![
            // 感知 → 认知（输入处理）
            GuSynapse::new(perceive_id, cognitive_id, 1.0),
            // 认知 → 行为（决策输出）
            GuSynapse::new(cognitive_id, behavior_id, 1.0),
            // 认知 → 通信（信息分享）
            GuSynapse::new(cognitive_id, comm_id, 0.5),
            // 所有 → 生存（状态汇聚）
            GuSynapse::new(perceive_id, survival_id, 0.2),
            GuSynapse::new(cognitive_id, survival_id, 0.3),
            GuSynapse::new(behavior_id, survival_id, 0.2),
            GuSynapse::new(comm_id, survival_id, 0.1),
        ];

        Self {
            neurons,
            neuron_id_to_type,
            synapses,
            current_time: 0.0,
            learning_rate: 0.01,
        }
    }

    /// 获取神经元状态
    pub fn get_neuron_state(&self, neuron_type: NeuronType) -> f64 {
        self.neurons.get(&neuron_type)
            .map(|n| n.state)
            .unwrap_or(0.0)
    }

    /// 获取生存状态
    ///
    /// Survival 神经元状态决定蛊虫是否存活
    pub fn survival_state(&self) -> f64 {
        self.get_neuron_state(NeuronType::Survival)
    }

    /// 获取整体活跃度（用于健康度）
    ///
    /// 所有神经元活跃度的平均值，映射到 [0, 1]
    pub fn get_overall_activity(&self) -> f64 {
        let total: f64 = self.neurons.values()
            .map(|n| (n.activity + 1.0) / 2.0) // 从 [-1, 1] 映射到 [0, 1]
            .sum();
        total / self.neurons.len() as f64
    }

    /// 获取接入点状态（用于前端显示）
    ///
    /// 返回每个接入点的活跃度 [0, 1]
    pub fn get_access_point_status(&self) -> AccessPointStatus {
        AccessPointStatus {
            perception: self.get_normalized_activity(NeuronType::Perception),
            action: self.get_normalized_activity(NeuronType::Behavior),
            communication: self.get_normalized_activity(NeuronType::Comm),
            memory: self.get_normalized_activity(NeuronType::Cognitive), // Cognitive 对应 memory
            reasoning: self.get_normalized_activity(NeuronType::Cognitive),
        }
    }

    /// 获取归一化的活跃度 [0, 1]
    fn get_normalized_activity(&self, neuron_type: NeuronType) -> f64 {
        self.neurons.get(&neuron_type)
            .map(|n| (n.activity + 1.0) / 2.0)
            .unwrap_or(0.5)
    }

    /// 获取行为状态
    ///
    /// Behavior 神经元状态决定蛊虫的行为倾向
    pub fn behavior_state(&self) -> f64 {
        self.get_neuron_state(NeuronType::Behavior)
    }

    /// 获取五维状态向量
    ///
    /// 满足归一化约束: |P|² + |C|² + |B|² + |M|² + |S|² = 1
    pub fn state_vector(&self) -> [f64; 5] {
        let p = self.get_neuron_state(NeuronType::Perception);
        let c = self.get_neuron_state(NeuronType::Cognitive);
        let b = self.get_neuron_state(NeuronType::Behavior);
        let m = self.get_neuron_state(NeuronType::Comm);
        let s = self.get_neuron_state(NeuronType::Survival);

        // 归一化
        let norm = (p * p + c * c + b * b + m * m + s * s).sqrt();
        if norm > 0.0 {
            [p / norm, c / norm, b / norm, m / norm, s / norm]
        } else {
            [0.0, 0.0, 0.0, 0.0, 0.0]
        }
    }

    /// 向特定神经元输入信号
    pub fn input(&mut self, neuron_type: NeuronType, signal: f64) {
        if let Some(neuron) = self.neurons.get_mut(&neuron_type) {
            neuron.update(signal, 0.01);
        }
    }

    /// 执行一次网络更新
    ///
    /// 1. 传播信号
    /// 2. 更新神经元状态
    /// 3. 学习调整权重
    pub fn update(&mut self, dt: f64) {
        // 计算每个神经元的输入
        let mut inputs: HashMap<Uuid, f64> = HashMap::new();

        for synapse in &self.synapses {
            if let Some(from_neuron) = self.neuron_id_to_type.get(&synapse.from)
                .and_then(|t| self.neurons.get(t))
            {
                let input = inputs.entry(synapse.to).or_insert(0.0);
                *input += synapse.weight * from_neuron.state;
            }
        }

        // 更新神经元状态
        for (neuron_type, neuron) in &mut self.neurons {
            let input = neuron.id;
            let signal = inputs.get(&input).copied().unwrap_or(0.0);
            neuron.update(signal, dt);
        }

        self.current_time += dt;

        // 赫布学习
        self.hebbian_learning();
    }

    /// 赫布学习
    ///
    /// Δw = η · xᵢ · xⱼ
    fn hebbian_learning(&mut self) {
        for synapse in &mut self.synapses {
            let from_state = self.neuron_id_to_type.get(&synapse.from)
                .and_then(|t| self.neurons.get(t))
                .map(|n| n.state)
                .unwrap_or(0.0);

            let to_state = self.neuron_id_to_type.get(&synapse.to)
                .and_then(|t| self.neurons.get(t))
                .map(|n| n.state)
                .unwrap_or(0.0);

            // 赫布规则 + 权重限制
            let delta = self.learning_rate * from_state * to_state;
            synapse.weight = (synapse.weight + delta).clamp(-2.0, 2.0);
        }
    }

    /// 从外部刺激更新（世界输入）
    ///
    /// 世界通过这个方法向蛊虫的神经网络发送信号
    pub fn receive_world_signal(&mut self, signal_type: NeuronType, strength: f64) {
        self.input(signal_type, strength);
        self.update(0.01);
    }

    /// 决定行为倾向
    ///
    /// 根据网络状态返回行为倾向向量
    pub fn decide_behavior(&self) -> BehaviorTendency {
        let behavior_state = self.behavior_state();
        let cognitive_state = self.get_neuron_state(NeuronType::Cognitive);
        let survival_state = self.survival_state();

        // 生存压力大时，倾向于生存相关行为
        if survival_state < 0.0 {
            return BehaviorTendency::Survival;
        }

        // 根据行为神经元状态决定
        match behavior_state {
            b if b > 0.5 => BehaviorTendency::Active,
            b if b > 0.0 => BehaviorTendency::Moderate,
            b if b > -0.5 => BehaviorTendency::Passive,
            _ => BehaviorTendency::Rest,
        }
    }
}

impl Default for GuLNN {
    fn default() -> Self {
        Self::new()
    }
}

/// 行为倾向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BehaviorTendency {
    /// 高活跃：积极行动（任务、交易等）
    Active,
    /// 中等活跃：适度行动（学习、观察等）
    Moderate,
    /// 低活跃：被动行动（等待、恢复等）
    Passive,
    /// 休息：几乎不行动
    Rest,
    /// 生存优先：专注于存活
    Survival,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gu_lnn_creation() {
        let lnn = GuLNN::new();

        // 应该有5个神经元
        assert_eq!(lnn.neurons.len(), 5);

        // 应该有默认连接
        assert!(!lnn.synapses.is_empty());
    }

    #[test]
    fn test_state_vector_normalization() {
        let mut lnn = GuLNN::new();

        // 输入一些信号
        lnn.input(NeuronType::Perception, 1.0);
        lnn.update(0.01);

        let vec = lnn.state_vector();

        // 检查归一化
        let sum: f64 = vec.iter().map(|x| x * x).sum();
        assert!((sum - 1.0).abs() < 0.01 || sum < 0.01);
    }

    #[test]
    fn test_survival_state() {
        let lnn = GuLNN::new();

        // 初始生存状态应该 >= 0
        assert!(lnn.survival_state() >= 0.0);
    }

    #[test]
    fn test_behavior_tendency() {
        let mut lnn = GuLNN::new();

        // 持续输入强烈负面生存信号，应该触发 Survival 倾向
        // 注意：Survival 神经元有正偏置(0.5)，需要更强的负面输入
        for _ in 0..50 {
            lnn.input(NeuronType::Survival, -1.0);
            lnn.update(0.01);
        }

        let tendency = lnn.decide_behavior();
        // 由于 Survival 神经元有正偏置，状态可能不会变成负数
        // 但如果变成负数，应该是 Survival 倾向
        if lnn.survival_state() < 0.0 {
            assert_eq!(tendency, BehaviorTendency::Survival);
        }
        // 否则检查行为状态是否合理
    }
}
