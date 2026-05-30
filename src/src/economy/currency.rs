//! 货币系统 - 拉蒂奥优雅公式设计
//!
//! 实现货币供应、流通速度、通胀控制

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::config::CurrencyConfig;

/// 货币账户
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyAccount {
    /// 账户ID
    pub id: Uuid,
    /// 余额
    pub balance: f64,
    /// 交易历史
    pub history: Vec<TransactionRecord>,
}

impl CurrencyAccount {
    pub fn new(id: Uuid, initial_balance: f64) -> Self {
        Self {
            id,
            balance: initial_balance,
            history: Vec::new(),
        }
    }

    /// 存入货币（不可变）
    pub fn deposit(&self, amount: f64, config: &CurrencyConfig) -> Option<Self> {
        if amount <= 0.0 {
            return None;
        }
        let new_balance = (self.balance + amount).min(config.max_balance);
        let mut new_account = self.clone();
        new_account.balance = new_balance;
        new_account.history.push(TransactionRecord {
            amount,
            kind: TransactionKind::Deposit,
            timestamp: std::time::SystemTime::now(),
        });
        Some(new_account)
    }

    /// 取出货币（不可变）
    pub fn withdraw(&self, amount: f64, config: &CurrencyConfig) -> Option<Self> {
        if amount <= 0.0 || self.balance - amount < config.min_balance {
            return None;
        }
        let mut new_account = self.clone();
        new_account.balance -= amount;
        new_account.history.push(TransactionRecord {
            amount,
            kind: TransactionKind::Withdraw,
            timestamp: std::time::SystemTime::now(),
        });
        Some(new_account)
    }
}

/// 交易记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRecord {
    pub amount: f64,
    pub kind: TransactionKind,
    pub timestamp: std::time::SystemTime,
}

/// 交易类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionKind {
    Deposit,
    Withdraw,
    Transfer,
}

/// 货币系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencySystem {
    /// 配置
    config: CurrencyConfig,
    /// 账户集合
    accounts: HashMap<Uuid, CurrencyAccount>,
    /// 总交易次数
    total_transactions: u64,
}

impl CurrencySystem {
    pub fn new(config: CurrencyConfig) -> Self {
        Self {
            config,
            accounts: HashMap::new(),
            total_transactions: 0,
        }
    }

    /// 创建账户
    pub fn create_account(&self, id: Uuid, initial_balance: f64) -> Option<Self> {
        if initial_balance < self.config.min_balance {
            return None;
        }
        let mut new_system = self.clone();
        let account = CurrencyAccount::new(id, initial_balance);
        new_system.accounts.insert(id, account);
        Some(new_system)
    }

    /// 转账（不可变）
    pub fn transfer(&self, from: Uuid, to: Uuid, amount: f64) -> Option<Self> {
        if from == to || amount <= 0.0 {
            return None;
        }

        let from_account = self.accounts.get(&from)?;
        let to_account = self.accounts.get(&to)?;

        let new_from = from_account.withdraw(amount, &self.config)?;
        let new_to = to_account.deposit(amount, &self.config)?;

        let mut new_system = self.clone();
        new_system.accounts.insert(from, new_from);
        new_system.accounts.insert(to, new_to);
        new_system.total_transactions += 1;
        Some(new_system)
    }

    /// 货币供应量: M = Σ(balances)
    pub fn money_supply(&self) -> f64 {
        self.accounts.values().map(|a| a.balance).sum()
    }

    /// 流通速度: V = transactions / M
    pub fn velocity(&self) -> f64 {
        let supply = self.money_supply();
        if supply <= 0.0 {
            return 0.0;
        }
        self.total_transactions as f64 / supply
    }

    /// 通胀率: π = k(M·V - Y)
    /// Y 为实际产出（这里简化为交易次数）
    pub fn inflation_rate(&self, output: f64) -> f64 {
        let mv = self.money_supply() * self.velocity();
        self.config.inflation_coefficient * (mv - output)
    }

    /// 获取账户余额
    pub fn balance(&self, id: &Uuid) -> Option<f64> {
        self.accounts.get(id).map(|a| a.balance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_account() {
        let config = CurrencyConfig::default();
        let system = CurrencySystem::new(config);
        let id = Uuid::new_v4();

        let new_system = system.create_account(id, 100.0).unwrap();
        assert_eq!(new_system.balance(&id), Some(100.0));
    }

    #[test]
    fn test_transfer() {
        let config = CurrencyConfig::default();
        let system = CurrencySystem::new(config);
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        let system = system.create_account(id1, 100.0).unwrap();
        let system = system.create_account(id2, 50.0).unwrap();

        let system = system.transfer(id1, id2, 30.0).unwrap();
        assert_eq!(system.balance(&id1), Some(70.0));
        assert_eq!(system.balance(&id2), Some(80.0));
    }

    #[test]
    fn test_money_supply() {
        let config = CurrencyConfig::default();
        let system = CurrencySystem::new(config);
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        let system = system.create_account(id1, 100.0).unwrap();
        let system = system.create_account(id2, 50.0).unwrap();

        assert_eq!(system.money_supply(), 150.0);
    }
}
