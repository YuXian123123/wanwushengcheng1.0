//! 经济系统配置
//!
//! 所有经济参数通过配置管理，禁止硬编码

use serde::{Deserialize, Serialize};

/// 经济系统全局配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomyConfig {
    /// 货币配置
    pub currency: CurrencyConfig,
    /// 市场配置
    pub market: MarketConfig,
    /// 定价配置
    pub pricing: PricingConfig,
    /// 交易配置
    pub trading: TradingConfig,
    /// 生存配置
    pub survival: SurvivalConfig,
    /// 奖励配置
    pub reward: RewardConfig,
    /// 安全配置
    pub security: SecurityConfig,
}

impl Default for EconomyConfig {
    fn default() -> Self {
        Self {
            currency: CurrencyConfig::default(),
            market: MarketConfig::default(),
            pricing: PricingConfig::default(),
            trading: TradingConfig::default(),
            survival: SurvivalConfig::default(),
            reward: RewardConfig::default(),
            security: SecurityConfig::default(),
        }
    }
}

impl EconomyConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<(), String> {
        self.currency.validate()?;
        self.market.validate()?;
        self.pricing.validate()?;
        self.trading.validate()?;
        self.survival.validate()?;
        self.reward.validate()?;
        self.security.validate()?;
        Ok(())
    }
}

/// 货币配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyConfig {
    /// 初始货币供应量
    pub initial_supply: f64,
    /// 通胀调节系数
    pub inflation_coefficient: f64,
    /// 最小余额
    pub min_balance: f64,
    /// 最大余额
    pub max_balance: f64,
}

impl Default for CurrencyConfig {
    fn default() -> Self {
        Self {
            initial_supply: 1000.0,
            inflation_coefficient: 0.01,
            min_balance: 0.0,
            max_balance: 1_000_000.0,
        }
    }
}

impl CurrencyConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.initial_supply <= 0.0 {
            return Err("initial_supply must be positive".to_string());
        }
        if self.min_balance < 0.0 {
            return Err("min_balance cannot be negative".to_string());
        }
        if self.max_balance <= self.min_balance {
            return Err("max_balance must be greater than min_balance".to_string());
        }
        Ok(())
    }
}

/// 市场配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketConfig {
    /// 市场活力系数
    pub vitality_lambda: f64,
    /// 参与者权重
    pub participant_weight: f64,
    /// 交易权重
    pub transaction_weight: f64,
    /// 停滞惩罚
    pub stagnation_penalty: f64,
}

impl Default for MarketConfig {
    fn default() -> Self {
        Self {
            vitality_lambda: 1.0,
            participant_weight: 0.4,
            transaction_weight: 0.4,
            stagnation_penalty: 0.2,
        }
    }
}

impl MarketConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.vitality_lambda <= 0.0 {
            return Err("vitality_lambda must be positive".to_string());
        }
        Ok(())
    }
}

/// 定价配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingConfig {
    /// 基础价格
    pub base_price: f64,
    /// 价格弹性系数
    pub elasticity_alpha: f64,
    /// 最小价格
    pub min_price: f64,
    /// 最大价格
    pub max_price: f64,
}

impl Default for PricingConfig {
    fn default() -> Self {
        Self {
            base_price: 10.0,
            elasticity_alpha: 0.5,
            min_price: 0.01,
            max_price: 10000.0,
        }
    }
}

impl PricingConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.base_price <= 0.0 {
            return Err("base_price must be positive".to_string());
        }
        if self.elasticity_alpha <= 0.0 {
            return Err("elasticity_alpha must be positive".to_string());
        }
        Ok(())
    }
}

/// 交易配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingConfig {
    /// 基础交易费用
    pub base_fee: f64,
    /// 数量费率
    pub amount_rate: f64,
    /// 类型费用
    pub type_fee: f64,
    /// 税率
    pub tax_rate: f64,
}

impl Default for TradingConfig {
    fn default() -> Self {
        Self {
            base_fee: 0.1,
            amount_rate: 0.01,
            type_fee: 0.05,
            tax_rate: 0.1,
        }
    }
}

impl TradingConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.base_fee < 0.0 {
            return Err("base_fee cannot be negative".to_string());
        }
        if self.tax_rate < 0.0 || self.tax_rate > 1.0 {
            return Err("tax_rate must be between 0 and 1".to_string());
        }
        Ok(())
    }
}

/// 生存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalConfig {
    /// 基础生存压力
    pub base_pressure: f64,
    /// 压力增长率
    pub pressure_growth_rate: f64,
    /// 基础寿命
    pub base_lifespan: f64,
    /// 延寿阈值
    pub extension_threshold: f64,
    /// 延寿量
    pub extension_amount: f64,
    /// 货币充足度基准
    pub coin_baseline: f64,
}

impl Default for SurvivalConfig {
    fn default() -> Self {
        Self {
            base_pressure: 1.0,
            pressure_growth_rate: 0.01,
            base_lifespan: 100.0,
            extension_threshold: 50.0,
            extension_amount: 10.0,
            coin_baseline: 100.0,
        }
    }
}

impl SurvivalConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.base_pressure <= 0.0 {
            return Err("base_pressure must be positive".to_string());
        }
        if self.base_lifespan <= 0.0 {
            return Err("base_lifespan must be positive".to_string());
        }
        Ok(())
    }
}

/// 奖励配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardConfig {
    /// 基础任务奖励
    pub base_task_reward: f64,
    /// 基础知识奖励
    pub base_knowledge_reward: f64,
    /// 基础创新奖励
    pub base_innovation_reward: f64,
    /// 质量权重
    pub quality_weight: f64,
    /// 新颖性权重
    pub novelty_weight: f64,
}

impl Default for RewardConfig {
    fn default() -> Self {
        Self {
            base_task_reward: 10.0,
            base_knowledge_reward: 5.0,
            base_innovation_reward: 20.0,
            quality_weight: 0.6,
            novelty_weight: 0.4,
        }
    }
}

impl RewardConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.base_task_reward <= 0.0 {
            return Err("base_task_reward must be positive".to_string());
        }
        Ok(())
    }
}

/// 安全配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// 相似度阈值基数
    pub similarity_threshold_base: f64,
    /// 信任系数
    pub trust_alpha: f64,
    /// 频率系数
    pub frequency_beta: f64,
    /// 最大相似度（防刷金币）
    pub max_similarity: f64,
    /// 质量阈值
    pub quality_threshold: f64,
    /// 信任衰减率
    pub trust_decay_rate: f64,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            similarity_threshold_base: 0.5,
            trust_alpha: 0.3,
            frequency_beta: 0.2,
            max_similarity: 0.8, // 80% 相似度上限
            quality_threshold: 0.3,
            trust_decay_rate: 0.01,
        }
    }
}

impl SecurityConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.similarity_threshold_base <= 0.0 || self.similarity_threshold_base > 1.0 {
            return Err("similarity_threshold_base must be between 0 and 1".to_string());
        }
        if self.max_similarity <= 0.0 || self.max_similarity > 1.0 {
            return Err("max_similarity must be between 0 and 1".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_valid() {
        let config = EconomyConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_security_max_similarity() {
        let config = SecurityConfig::default();
        assert_eq!(config.max_similarity, 0.8); // 80% 上限
    }
}
