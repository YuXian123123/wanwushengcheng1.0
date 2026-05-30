//! 市场动态系统 - 黑塔创新架构设计

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::config::MarketConfig;

/// 市场状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketState {
    /// 参与者数量
    pub participants: u64,
    /// 交易次数
    pub transactions: u64,
    /// 停滞因子
    pub stagnation_factor: f64,
    /// 市场活力
    pub vitality: f64,
    /// 竞争强度
    pub competition: f64,
}

impl MarketState {
    pub fn new() -> Self {
        Self {
            participants: 0,
            transactions: 0,
            stagnation_factor: 0.0,
            vitality: 0.0,
            competition: 0.0,
        }
    }

    /// 计算市场活力: A = λ·Participants + μ·Transactions - ν·Stagnation
    pub fn calculate_vitality(&self, config: &MarketConfig) -> f64 {
        config.vitality_lambda * (
            config.participant_weight * self.participants as f64
            + config.transaction_weight * self.transactions as f64
            - config.stagnation_penalty * self.stagnation_factor
        )
    }

    /// 计算竞争强度: C = (Max(Profits) - Min(Profits)) / Average(Profits)
    pub fn calculate_competition(&self, profits: &[f64]) -> f64 {
        if profits.is_empty() {
            return 0.0;
        }
        let max = profits.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min = profits.iter().cloned().fold(f64::INFINITY, f64::min);
        let avg = profits.iter().sum::<f64>() / profits.len() as f64;

        if avg == 0.0 {
            return 0.0;
        }
        (max - min) / avg
    }
}

impl Default for MarketState {
    fn default() -> Self {
        Self::new()
    }
}

/// 市场参与者
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketParticipant {
    /// 参与者ID
    pub id: Uuid,
    /// 利润
    pub profit: f64,
    /// 交易次数
    pub transaction_count: u64,
    /// 加入时间
    pub join_time: u64,
}

impl MarketParticipant {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            profit: 0.0,
            transaction_count: 0,
            join_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// 记录交易（不可变）
    pub fn record_transaction(&self, profit_delta: f64) -> Self {
        let mut new_participant = self.clone();
        new_participant.profit += profit_delta;
        new_participant.transaction_count += 1;
        new_participant
    }
}

/// 市场系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSystem {
    /// 配置
    config: MarketConfig,
    /// 市场状态
    state: MarketState,
    /// 参与者
    participants: HashMap<Uuid, MarketParticipant>,
}

impl MarketSystem {
    pub fn new(config: MarketConfig) -> Self {
        Self {
            config,
            state: MarketState::new(),
            participants: HashMap::new(),
        }
    }

    /// 注册参与者
    pub fn register(&self, id: Uuid) -> Self {
        let mut new_system = self.clone();
        let participant = MarketParticipant::new(id);
        new_system.participants.insert(id, participant);
        new_system.state.participants = new_system.participants.len() as u64;
        new_system
    }

    /// 移除参与者
    pub fn unregister(&self, id: &Uuid) -> Self {
        let mut new_system = self.clone();
        new_system.participants.remove(id);
        new_system.state.participants = new_system.participants.len() as u64;
        new_system
    }

    /// 记录交易
    pub fn record_transaction(&self, participant_id: &Uuid, profit_delta: f64) -> Self {
        let mut new_system = self.clone();

        if let Some(participant) = self.participants.get(participant_id) {
            let new_participant = participant.record_transaction(profit_delta);
            new_system.participants.insert(*participant_id, new_participant);
            new_system.state.transactions += 1;
        }

        new_system
    }

    /// 更新市场状态
    pub fn update(&self) -> Self {
        let mut new_system = self.clone();

        // 计算停滞因子（基于最近交易频率）
        let avg_transactions = if new_system.participants.is_empty() {
            0.0
        } else {
            new_system.participants.values()
                .map(|p| p.transaction_count as f64)
                .sum::<f64>() / new_system.participants.len() as f64
        };
        new_system.state.stagnation_factor = 1.0 / (1.0 + avg_transactions);

        // 计算市场活力
        new_system.state.vitality = new_system.state.calculate_vitality(&self.config);

        // 计算竞争强度
        let profits: Vec<f64> = new_system.participants.values()
            .map(|p| p.profit)
            .collect();
        new_system.state.competition = new_system.state.calculate_competition(&profits);

        new_system
    }

    /// 获取市场活力
    pub fn vitality(&self) -> f64 {
        self.state.vitality
    }

    /// 获取竞争强度
    pub fn competition(&self) -> f64 {
        self.state.competition
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_system_creation() {
        let config = MarketConfig::default();
        let system = MarketSystem::new(config);
        assert_eq!(system.state.participants, 0);
    }

    #[test]
    fn test_participant_registration() {
        let config = MarketConfig::default();
        let system = MarketSystem::new(config);
        let id = Uuid::new_v4();

        let new_system = system.register(id);
        assert_eq!(new_system.state.participants, 1);
    }

    #[test]
    fn test_transaction_recording() {
        let config = MarketConfig::default();
        let system = MarketSystem::new(config);
        let id = Uuid::new_v4();

        let system = system.register(id);
        let system = system.record_transaction(&id, 10.0);

        let participant = system.participants.get(&id).unwrap();
        assert_eq!(participant.profit, 10.0);
        assert_eq!(participant.transaction_count, 1);
    }
}