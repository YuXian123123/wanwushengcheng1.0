//! 经济系统 - 天才理事会综合设计
//!
//! # 设计理念
//!
//! - 黑塔：创新架构（市场动态、生存压力）
//! - 螺丝咕姆：安全协议（五层防护）
//! - 拉蒂奥：优雅公式（货币、定价、交易、奖励）
//!
//! # 核心机制
//!
//! 1. 物竞天择：生存压力驱动进化
//! 2. 欲望驱动：金币获取推动能力提升
//! 3. 五层防护：质量→相似度→行为→信任→审计
//! 4. 防刷金币：相似度不超过80%

pub mod config;
pub mod currency;
pub mod market;
pub mod pricing;
pub mod trading;
pub mod survival;
pub mod reward;
pub mod security;

// 重导出主要类型
pub use config::EconomyConfig;
pub use currency::{CurrencySystem, CurrencyAccount};
pub use market::{MarketSystem, MarketState, MarketParticipant};
pub use pricing::{PricingSystem, Price};
pub use trading::{TradingSystem, Trade, TradeType};
pub use survival::{SurvivalSystem, SurvivalState};
pub use reward::{RewardSystem, RewardResult, RewardType, RewardParams};
pub use security::{SecuritySystem, SecurityCheckResult};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 经济系统整合
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomySystem {
    /// 配置
    config: EconomyConfig,
    /// 货币系统
    currency: CurrencySystem,
    /// 市场系统
    market: MarketSystem,
    /// 定价系统
    pricing: PricingSystem,
    /// 交易系统
    trading: TradingSystem,
    /// 生存系统
    survival: SurvivalSystem,
    /// 奖励系统
    reward: RewardSystem,
    /// 安全系统
    security: SecuritySystem,
}

impl EconomySystem {
    /// 创建新的经济系统
    pub fn new() -> Self {
        let config = EconomyConfig::default();
        Self::from_config(config)
    }

    /// 从配置创建
    pub fn from_config(config: EconomyConfig) -> Self {
        Self {
            currency: CurrencySystem::new(config.currency.clone()),
            market: MarketSystem::new(config.market.clone()),
            pricing: PricingSystem::new(config.pricing.clone()),
            trading: TradingSystem::new(config.trading.clone()),
            survival: SurvivalSystem::new(config.survival.clone()),
            reward: RewardSystem::new(config.reward.clone()),
            security: SecuritySystem::new(config.security.clone()),
            config,
        }
    }

    /// 注册实体（蛊虫）
    pub fn register_entity(&self, id: Uuid) -> Self {
        let mut new_system = self.clone();

        // 在各子系统中注册
        new_system.currency = self.currency.create_account(id, 0.0).unwrap_or_else(|| self.currency.clone());
        new_system.market = self.market.register(id);
        new_system.survival = self.survival.register(id);

        new_system
    }

    /// 执行知识分享（带安全检查）
    pub fn share_knowledge(
        &self,
        sender: Uuid,
        content: &str,
        history: &[String],
    ) -> (Self, f64) {
        // 安全检查
        let security_result = self.security.check(sender, content, history);

        // 计算奖励
        let reward_params = RewardParams::new()
            .with_quality(security_result.quality.score)
            .with_novelty(1.0 - security_result.similarity.score);

        let new_reward = self.reward.grant(
            sender,
            RewardType::Knowledge,
            Some(&security_result),
            reward_params,
        );

        // 获取奖励金额
        let reward_amount = new_reward.total_rewards(&sender) - self.reward.total_rewards(&sender);

        // 更新系统
        let mut new_system = self.clone();
        new_system.reward = new_reward;
        new_system.security = self.security.update_trust(sender, security_result.passed);

        (new_system, reward_amount)
    }

    /// 获取实体余额
    pub fn balance(&self, id: &Uuid) -> Option<f64> {
        self.currency.balance(id)
    }

    /// 获取实体生存状态
    pub fn survival_state(&self, id: &Uuid) -> Option<&SurvivalState> {
        self.survival.get(id)
    }

    /// 获取市场活力
    pub fn market_vitality(&self) -> f64 {
        self.market.vitality()
    }

    /// 更新所有系统
    pub fn update(&self) -> Self {
        let mut new_system = self.clone();
        new_system.survival = self.survival.update_all();
        new_system.market = self.market.update();
        new_system
    }

    /// 自然选择
    pub fn natural_selection(&self, pressure_threshold: f64) -> Self {
        let mut new_system = self.clone();
        new_system.survival = self.survival.natural_selection(pressure_threshold);
        new_system
    }

    /// 获取配置
    pub fn config(&self) -> &EconomyConfig {
        &self.config
    }
}

impl Default for EconomySystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_economy_system_creation() {
        let system = EconomySystem::new();
        assert!(system.config().validate().is_ok());
    }

    #[test]
    fn test_entity_registration() {
        let system = EconomySystem::new();
        let id = Uuid::new_v4();

        let new_system = system.register_entity(id);
        assert!(new_system.balance(&id).is_some());
    }

    #[test]
    fn test_knowledge_sharing() {
        let system = EconomySystem::new();
        let sender = Uuid::new_v4();

        let system = system.register_entity(sender);
        let (new_system, reward) = system.share_knowledge(sender, "这是一条有价值的知识内容", &[]);

        // 高质量内容应该获得奖励
        assert!(reward >= 0.0);
    }

    #[test]
    fn test_max_similarity_limit() {
        let config = EconomyConfig::default();
        // 确保相似度上限为80%
        assert_eq!(config.security.max_similarity, 0.8);
    }
}