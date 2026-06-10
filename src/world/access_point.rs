//! 接入点模块 - 蛊虫连接到世界神经网络的接口
//!
//! 每只蛊虫有5个接入点：感知、认知、行为、通信、生存

use crate::world::config::WorldConfig;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 接入点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccessPointType {
    /// 感知接入点 - 接收外部输入
    Perceive,
    /// 认知接入点 - 推理与决策
    Cognitive,
    /// 行为接入点 - 输出行为
    Behavior,
    /// 通信接入点 - 与其他蛊虫通信
    Comm,
    /// 生存接入点 - 生命状态同步
    Survival,
}

impl AccessPointType {
    /// 获取接入点名称
    pub fn name(&self) -> &str {
        match self {
            AccessPointType::Perceive => "感知接入点",
            AccessPointType::Cognitive => "认知接入点",
            AccessPointType::Behavior => "行为接入点",
            AccessPointType::Comm => "通信接入点",
            AccessPointType::Survival => "生存接入点",
        }
    }

    /// 获取默认权重（使用默认配置）
    pub fn default_weight(&self) -> f64 {
        self.default_weight_with_config(&WorldConfig::default())
    }

    /// 获取默认权重（使用指定配置）
    pub fn default_weight_with_config(&self, config: &WorldConfig) -> f64 {
        match self {
            AccessPointType::Perceive => config.network.perceive_weight,
            AccessPointType::Cognitive => config.network.cognitive_weight,
            AccessPointType::Behavior => config.network.behavior_weight,
            AccessPointType::Comm => config.network.comm_weight,
            AccessPointType::Survival => config.network.survival_weight,
        }
    }
}

/// 接入点状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessPointStatus {
    /// 活跃
    Active,
    /// 休眠
    Dormant,
    /// 离线
    Offline,
}

/// 接入点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPoint {
    /// 接入点唯一标识
    pub id: Uuid,
    /// 接入点类型
    pub point_type: AccessPointType,
    /// 所属蛊虫
    pub owner: Uuid,
    /// 连接的其他接入点
    pub connections: Vec<Uuid>,
    /// 连接权重
    pub connection_weights: Vec<f64>,
    /// 信号队列
    pub signal_queue: Vec<Signal>,
    /// 处理容量
    pub capacity: f64,
    /// 当前负载
    pub load: f64,
    /// 状态
    pub status: AccessPointStatus,
    /// 技能加成
    pub skill_bonus: f64,
}

impl AccessPoint {
    /// 创建新接入点
    pub fn new(id: Uuid, point_type: AccessPointType, owner: Uuid) -> Self {
        Self {
            id,
            point_type,
            owner,
            connections: Vec::new(),
            connection_weights: Vec::new(),
            signal_queue: Vec::new(),
            capacity: point_type.default_weight(),
            load: 0.0,
            status: AccessPointStatus::Active,
            skill_bonus: 0.0,
        }
    }

    /// 计算实际容量: Capacity = Base_capacity × (1 + Skill_bonus)
    pub fn effective_capacity(&self) -> f64 {
        self.capacity * (1.0 + self.skill_bonus)
    }

    /// 连接到另一个接入点（不可变）
    pub fn connect(&self, target: Uuid, weight: f64) -> Self {
        let mut new_point = self.clone();
        if !new_point.connections.contains(&target) {
            new_point.connections.push(target);
            new_point.connection_weights.push(weight);
        }
        new_point
    }

    /// 断开连接（不可变）
    pub fn disconnect(&self, target: &Uuid) -> Self {
        let mut new_point = self.clone();
        if let Some(idx) = new_point.connections.iter().position(|id| id == target) {
            new_point.connections.remove(idx);
            new_point.connection_weights.remove(idx);
        }
        new_point
    }

    /// 接收信号（不可变）
    pub fn receive_signal(&self, signal: Signal) -> Self {
        let mut new_point = self.clone();
        new_point.signal_queue.push(signal);
        new_point.load = (new_point.signal_queue.len() as f64) / new_point.effective_capacity();
        new_point
    }

    /// 处理信号队列（不可变）
    pub fn process_signals(&self) -> (Self, Vec<Signal>) {
        let signals = self.signal_queue.clone();
        let mut new_point = self.clone();
        new_point.signal_queue.clear();
        new_point.load = 0.0;
        (new_point, signals)
    }

    /// 更新状态（不可变）
    pub fn set_status(&self, status: AccessPointStatus) -> Self {
        let mut new_point = self.clone();
        new_point.status = status;
        new_point
    }

    /// 更新技能加成（不可变）
    pub fn set_skill_bonus(&self, bonus: f64) -> Self {
        let mut new_point = self.clone();
        new_point.skill_bonus = bonus;
        new_point
    }

    /// 检查是否过载
    pub fn is_overloaded(&self) -> bool {
        self.load > self.effective_capacity()
    }
}

/// 信号类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalType {
    /// 感知信号
    Sensory(SensoryData),
    /// 认知信号
    Cognitive(CognitiveState),
    /// 行为信号
    Behavioral(Action),
    /// 通信信号
    Communication(MessageData),
    /// 生存信号（心跳）
    Survival(Heartbeat),
}

/// 感知数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensoryData {
    pub input_type: String,
    pub content: Vec<u8>,
}

/// 认知状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveState {
    pub intention: Option<String>,
    pub confidence: f64,
    pub reasoning_steps: Vec<String>,
}

/// 行动
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_type: String,
    pub parameters: Vec<f64>,
}

/// 消息数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    pub channel: String,
    pub content: String,
}

/// 心跳
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    pub health: f64,
    pub timestamp: u64,
}

/// 信号
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    /// 信号ID
    pub id: Uuid,
    /// 来源接入点
    pub source: Uuid,
    /// 目标接入点（None为广播）
    pub target: Option<Uuid>,
    /// 信号类型
    pub signal_type: SignalType,
    /// 信号强度
    pub strength: f64,
    /// 时间戳
    pub timestamp: u64,
}

impl Signal {
    pub fn new(source: Uuid, signal_type: SignalType, strength: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            source,
            target: None,
            signal_type,
            strength,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    pub fn with_target(mut self, target: Uuid) -> Self {
        self.target = Some(target);
        self
    }

    /// 计算接收强度: S_received = S_sent × e^(-α×distance) × W_connection
    pub fn received_strength(&self, decay_rate: f64, distance: f64, weight: f64) -> f64 {
        self.strength * (-decay_rate * distance).exp() * weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_point_creation() {
        let id = Uuid::new_v4();
        let owner = Uuid::new_v4();
        let point = AccessPoint::new(id, AccessPointType::Cognitive, owner);

        assert_eq!(point.point_type, AccessPointType::Cognitive);
        assert_eq!(point.status, AccessPointStatus::Active);
    }

    #[test]
    fn test_access_point_connection() {
        let id = Uuid::new_v4();
        let owner = Uuid::new_v4();
        let target = Uuid::new_v4();

        let point = AccessPoint::new(id, AccessPointType::Cognitive, owner);
        let connected = point.connect(target, 0.5);

        assert_eq!(connected.connections.len(), 1);
        assert_eq!(connected.connection_weights[0], 0.5);
    }

    #[test]
    fn test_signal_creation() {
        let source = Uuid::new_v4();
        let signal = Signal::new(
            source,
            SignalType::Survival(Heartbeat { health: 0.8, timestamp: 0 }),
            1.0,
        );

        assert_eq!(signal.source, source);
        assert!(signal.strength > 0.0);
    }

    #[test]
    fn test_five_access_point_types() {
        let types = [
            AccessPointType::Perceive,
            AccessPointType::Cognitive,
            AccessPointType::Behavior,
            AccessPointType::Comm,
            AccessPointType::Survival,
        ];

        assert_eq!(types.len(), 5);
    }
}