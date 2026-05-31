//! 定价系统 - 拉蒂奥优雅公式设计

use serde::{Deserialize, Serialize};

use super::config::PricingConfig;

/// 价格状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    /// 当前价格
    pub current: f64,
    /// 基础价格
    pub base: f64,
    /// 需求量
    pub demand: f64,
    /// 供给量
    pub supply: f64,
}

impl Price {
    pub fn new(base: f64, demand: f64, supply: f64) -> Self {
        Self {
            current: base,
            base,
            demand,
            supply,
        }
    }

    /// 供需定价: P = P₀ × (D/S)^α
    pub fn calculate(&self, config: &PricingConfig) -> f64 {
        if self.supply <= 0.0 {
            return config.max_price;
        }

        let ratio = self.demand / self.supply;
        let price = self.base * ratio.powf(config.elasticity_alpha);

        // 限制在合理范围内
        price.clamp(config.min_price, config.max_price)
    }

    /// 动态调整: P' = P × (1 + ΔD - ΔS)
    pub fn adjust(&self, demand_delta: f64, supply_delta: f64, config: &PricingConfig) -> Self {
        let adjustment = 1.0 + demand_delta - supply_delta;
        let new_price = (self.current * adjustment).clamp(config.min_price, config.max_price);

        Self {
            current: new_price,
            base: self.base,
            demand: self.demand + demand_delta,
            supply: self.supply + supply_delta,
        }
    }
}

/// 定价系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingSystem {
    /// 配置
    config: PricingConfig,
    /// 商品价格映射
    prices: std::collections::HashMap<String, Price>,
}

impl PricingSystem {
    pub fn new(config: PricingConfig) -> Self {
        Self {
            config,
            prices: std::collections::HashMap::new(),
        }
    }

    /// 设置商品价格
    pub fn set_price(&self, item: &str, base: f64, demand: f64, supply: f64) -> Self {
        let mut new_system = self.clone();
        let price = Price::new(base, demand, supply);
        new_system.prices.insert(item.to_string(), price);
        new_system
    }

    /// 获取商品价格
    pub fn get_price(&self, item: &str) -> Option<f64> {
        self.prices.get(item).map(|p| p.calculate(&self.config))
    }

    /// 更新供需
    pub fn update_supply_demand(
        &self,
        item: &str,
        demand_delta: f64,
        supply_delta: f64,
    ) -> Self {
        let mut new_system = self.clone();

        if let Some(price) = self.prices.get(item) {
            let new_price = price.adjust(demand_delta, supply_delta, &self.config);
            new_system.prices.insert(item.to_string(), new_price);
        }

        new_system
    }

    /// 市场均衡检测: ∂D/∂P + ∂S/∂P = 0
    /// 简化版本：检测供需是否接近平衡
    pub fn is_equilibrium(&self, item: &str, tolerance: f64) -> bool {
        if let Some(price) = self.prices.get(item) {
            let ratio = (price.demand - price.supply).abs() / price.supply.max(1.0);
            ratio < tolerance
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_calculation() {
        let config = PricingConfig::default();
        let price = Price::new(10.0, 100.0, 50.0);

        let calculated = price.calculate(&config);
        // 需求大于供给，价格应该上涨
        assert!(calculated > price.base);
    }

    #[test]
    fn test_price_adjustment() {
        let config = PricingConfig::default();
        let price = Price::new(10.0, 100.0, 100.0);

        // 需求增加
        let adjusted = price.adjust(10.0, 0.0, &config);
        assert!(adjusted.current > price.current);
    }

    #[test]
    fn test_pricing_system() {
        let config = PricingConfig::default();
        let system = PricingSystem::new(config);

        let system = system.set_price("item1", 10.0, 100.0, 50.0);
        let price = system.get_price("item1");

        assert!(price.is_some());
    }
}