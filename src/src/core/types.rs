//! 类型定义

use serde::{Deserialize, Serialize};

/// 神经元类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NeuronType {
    /// 感知神经元 - 接收外部输入
    Perception,
    /// 认知神经元 - 内部处理
    Cognitive,
    /// 行为神经元 - 输出行为
    Behavior,
}

/// 可塑性规则（局部学习规则，非梯度下降）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlasticityRule {
    /// 赫布学习: Δw = η · xᵢ · xⱼ
    Hebbian,
    /// STDP: 脉冲时序依赖可塑性
    Stdp,
    /// Oja规则: Δw = η · y · (x - y · w) 防止权重爆炸
    Oja,
}

/// LNN配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LNNConfig {
    /// 拓扑约束
    pub topology: TopologyConfig,
    /// 学习参数
    pub learning: LearningConfig,
    /// 时间参数
    pub timing: TimingConfig,
    /// 安全配置
    pub safety: SafetyConfig,
}

impl Default for LNNConfig {
    fn default() -> Self {
        Self {
            topology: TopologyConfig::default(),
            learning: LearningConfig::default(),
            timing: TimingConfig::default(),
            safety: SafetyConfig::default(),
        }
    }
}

/// 拓扑配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyConfig {
    /// 最小神经元数
    pub min_neurons: usize,
    /// 最大神经元数
    pub max_neurons: usize,
    /// 每个神经元最大连接数
    pub max_connections: usize,
}

impl Default for TopologyConfig {
    fn default() -> Self {
        Self {
            min_neurons: 10,
            max_neurons: 1000,
            max_connections: 100,
        }
    }
}

/// 学习配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConfig {
    /// 初始学习率
    pub initial_rate: f64,
    /// 最小学习率
    pub min_rate: f64,
    /// 最大学习率
    pub max_rate: f64,
}

impl Default for LearningConfig {
    fn default() -> Self {
        Self {
            initial_rate: 0.01,
            min_rate: 0.001,
            max_rate: 0.1,
        }
    }
}

/// 时间配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingConfig {
    /// 时间步长
    pub dt: f64,
    /// 时间常数范围 [min, max]
    pub tau_range: (f64, f64),
}

impl Default for TimingConfig {
    fn default() -> Self {
        Self {
            dt: 0.01,
            tau_range: (0.1, 10.0),
        }
    }
}

/// 安全配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConfig {
    /// 启用监控
    pub enable_monitoring: bool,
    /// 熔断阈值（异常神经元比例）
    pub fuse_threshold: f64,
    /// 审计拓扑变更
    pub audit_topology: bool,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            enable_monitoring: true,
            fuse_threshold: 0.3,
            audit_topology: true,
        }
    }
}

/// 动态拓扑配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyDynamics {
    /// 活跃度阈值（低于此值考虑删除）
    pub neuron_activity_threshold: f64,
    /// 权重阈值（低于此值考虑删除突触）
    pub synapse_weight_threshold: f64,
    /// 生长冷却时间（ms）
    pub growth_cooldown_ms: u64,
    /// 修剪冷却时间（ms）
    pub prune_cooldown_ms: u64,
    /// 每周期最大新增数量
    pub max_growth_per_cycle: usize,
    /// 每周期最大修剪数量
    pub max_prune_per_cycle: usize,
}

impl Default for TopologyDynamics {
    fn default() -> Self {
        Self {
            neuron_activity_threshold: 0.05,
            synapse_weight_threshold: 0.01,
            growth_cooldown_ms: 5000,
            prune_cooldown_ms: 10000,
            max_growth_per_cycle: 3,
            max_prune_per_cycle: 2,
        }
    }
}
