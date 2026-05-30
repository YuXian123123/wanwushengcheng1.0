//! 信道状态管理模块
//!
//! 实现信道状态的数学形式化表示和不可变操作

use std::collections::HashSet;
use super::message::{ChannelType, GuId};

/// 信道状态枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelStatus {
    Active,
    Idle,
    Closed,
}

/// 信道状态（数学形式化表示）
/// C = (type, participants, capacity, load, cost, state)
#[derive(Debug, Clone)]
pub struct ChannelState {
    pub channel_type: ChannelType,
    pub participants: HashSet<GuId>,
    pub capacity: u32,
    pub load: u32,
    pub cost_factor: f64,
    pub status: ChannelStatus,
}

impl ChannelState {
    pub fn new(channel_type: ChannelType, capacity: u32, cost_factor: f64) -> Self {
        Self {
            channel_type,
            participants: HashSet::new(),
            capacity,
            load: 0,
            cost_factor,
            status: ChannelStatus::Active,
        }
    }

    /// 负载率 ρ = load / cap
    pub fn load_ratio(&self) -> f64 {
        if self.capacity == 0 { return 0.0; }
        self.load as f64 / self.capacity as f64
    }

    /// 负载因子（用于成本计算）
    pub fn load_factor(&self) -> f64 {
        1.0 + self.load_ratio()
    }

    /// 是否可用: available ⇔ load < cap ∧ state = Active
    pub fn is_available(&self) -> bool {
        self.load < self.capacity && self.status == ChannelStatus::Active
    }

    /// 增加负载（返回新信道状态）
    pub fn with_increased_load(&self) -> Option<Self> {
        if self.load >= self.capacity { return None; }
        Some(Self { load: self.load + 1, ..self.clone() })
    }

    /// 减少负载（返回新信道状态）
    pub fn with_decreased_load(&self) -> Self {
        Self { load: self.load.saturating_sub(1), ..self.clone() }
    }

    /// 添加参与者
    pub fn with_participant(&self, participant: GuId) -> Self {
        let mut new_state = self.clone();
        new_state.participants.insert(participant);
        new_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_creation() {
        let channel = ChannelState::new(ChannelType::World, 100, 5.0);
        assert_eq!(channel.capacity, 100);
        assert!(channel.is_available());
    }

    #[test]
    fn test_immutability() {
        let channel = ChannelState::new(ChannelType::World, 100, 5.0);
        let new_channel = channel.with_increased_load().unwrap();
        assert_eq!(channel.load, 0);
        assert_eq!(new_channel.load, 1);
    }
}
