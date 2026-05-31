//! 交易系统 - 拉蒂奥优雅公式设计

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::config::TradingConfig;

/// 交易类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeType {
    /// 购买
    Buy,
    /// 出售
    Sell,
    /// 交换
    Exchange,
}

/// 交易记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// 交易ID
    pub id: Uuid,
    /// 买方
    pub buyer: Uuid,
    /// 卖方
    pub seller: Uuid,
    /// 金额
    pub amount: f64,
    /// 交易类型
    pub kind: TradeType,
    /// 交易成本
    pub cost: f64,
    /// 税收
    pub tax: f64,
    /// 净收益
    pub net: f64,
}

impl Trade {
    pub fn new(buyer: Uuid, seller: Uuid, amount: f64, kind: TradeType, config: &TradingConfig) -> Self {
        let id = Uuid::new_v4();
        let cost = Self::calculate_cost(amount, kind, config);
        let tax = Self::calculate_tax(amount, config);
        let net = Self::calculate_net(amount, cost, tax);

        Self {
            id,
            buyer,
            seller,
            amount,
            kind,
            cost,
            tax,
            net,
        }
    }

    /// 交易成本: C = c₀ + c₁·amount + c₂·type
    fn calculate_cost(amount: f64, kind: TradeType, config: &TradingConfig) -> f64 {
        let type_fee = match kind {
            TradeType::Buy => config.type_fee,
            TradeType::Sell => config.type_fee * 0.5,
            TradeType::Exchange => config.type_fee * 0.25,
        };
        config.base_fee + config.amount_rate * amount + type_fee
    }

    /// 税收: T = rate × amount
    fn calculate_tax(amount: f64, config: &TradingConfig) -> f64 {
        config.tax_rate * amount
    }

    /// 净收益: R = benefit - cost - tax
    fn calculate_net(amount: f64, cost: f64, tax: f64) -> f64 {
        amount - cost - tax
    }
}

/// 交易系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingSystem {
    /// 配置
    config: TradingConfig,
    /// 交易历史
    history: Vec<Trade>,
}

impl TradingSystem {
    pub fn new(config: TradingConfig) -> Self {
        Self {
            config,
            history: Vec::new(),
        }
    }

    /// 执行交易（不可变）
    pub fn execute(&self, buyer: Uuid, seller: Uuid, amount: f64, kind: TradeType) -> Self {
        let mut new_system = self.clone();
        let trade = Trade::new(buyer, seller, amount, kind, &self.config);
        new_system.history.push(trade);
        new_system
    }

    /// 获取交易历史
    pub fn get_history(&self) -> &[Trade] {
        &self.history
    }

    /// 获取用户交易记录
    pub fn get_user_trades(&self, user: &Uuid) -> Vec<&Trade> {
        self.history
            .iter()
            .filter(|t| t.buyer == *user || t.seller == *user)
            .collect()
    }

    /// 计算总交易量
    pub fn total_volume(&self) -> f64 {
        self.history.iter().map(|t| t.amount).sum()
    }

    /// 计算总税收
    pub fn total_tax(&self) -> f64 {
        self.history.iter().map(|t| t.tax).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade_creation() {
        let config = TradingConfig::default();
        let buyer = Uuid::new_v4();
        let seller = Uuid::new_v4();

        let trade = Trade::new(buyer, seller, 100.0, TradeType::Buy, &config);

        assert!(trade.cost > 0.0);
        assert!(trade.tax > 0.0);
        assert!(trade.net < trade.amount);
    }

    #[test]
    fn test_trading_system() {
        let config = TradingConfig::default();
        let system = TradingSystem::new(config);
        let buyer = Uuid::new_v4();
        let seller = Uuid::new_v4();

        let system = system.execute(buyer, seller, 100.0, TradeType::Buy);

        assert_eq!(system.history.len(), 1);
        assert_eq!(system.total_volume(), 100.0);
    }

    #[test]
    fn test_trade_types() {
        let config = TradingConfig::default();
        let buyer = Uuid::new_v4();
        let seller = Uuid::new_v4();

        let buy = Trade::new(buyer, seller, 100.0, TradeType::Buy, &config);
        let sell = Trade::new(buyer, seller, 100.0, TradeType::Sell, &config);
        let exchange = Trade::new(buyer, seller, 100.0, TradeType::Exchange, &config);

        // Exchange 应该成本最低
        assert!(exchange.cost < sell.cost);
        assert!(sell.cost < buy.cost);
    }
}