//! 世界事件模块
//!
//! 定义 WorldMind 向外部（如 Herness）发送的事件
//!
//! # 设计原则
//!
//! WorldMind 与 Herness 通过协议通信，不直接引用：
//! - WorldMind 返回 WorldEvent 作为执行结果的一部分
//! - 外部运行时负责将事件发送给 Herness 的广播通道

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 世界事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldEvent {
    /// 蛊虫注册
    GuRegistered {
        gu_id: Uuid,
        name: String,
    },
    /// 蛊虫注销（死亡）
    GuUnregistered {
        gu_id: Uuid,
        cause: String,
    },
    /// 交易事件
    Transaction(TransactionData),
    /// 意识涌现
    ConsciousnessEmerged {
        sync_rate: f64,
        emergence_factor: f64,
    },
    /// 世界状态变化
    WorldStateChanged {
        health: f64,
        sync_rate: f64,
        population: usize,
    },
    /// 任务完成
    TaskCompleted {
        gu_id: Uuid,
        task_name: String,
        reward: f64,
    },
    /// 能力习得
    AbilityLearned {
        gu_id: Uuid,
        ability_name: String,
        level: u32,
    },
    /// 知识分享
    KnowledgeShared {
        gu_id: Uuid,
        knowledge_type: String,
        quality: f64,
    },
}

/// 交易数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    /// 交易 ID
    pub id: String,
    /// 时间戳
    pub timestamp: u64,
    /// 来源 ID（"system" 表示系统）
    pub from_id: String,
    /// 来源名称
    pub from_name: String,
    /// 来源余额（交易后）
    pub from_balance: f64,
    /// 目标 ID
    pub to_id: String,
    /// 目标名称
    pub to_name: String,
    /// 目标余额（交易后）
    pub to_balance: f64,
    /// 金额（正=收入，负=支出）
    pub amount: f64,
    /// 交易类型
    pub kind: TransactionKind,
    /// 简短原因
    pub reason: String,
    /// 详细说明
    pub detail: String,
}

/// 交易类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionKind {
    /// 获取（任务奖励、系统发放等）
    Deposit,
    /// 花费（购买资源、技能升级、知识付费等）
    Withdraw,
    /// 转账（蛊虫之间的交易）
    Transfer,
}

impl TransactionData {
    /// 创建任务奖励交易
    pub fn task_reward(
        gu_id: Uuid,
        gu_name: &str,
        balance_before: f64,
        amount: f64,
        task_name: &str,
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: format!("tx_{:08x}_{}", timestamp, gu_id),
            timestamp,
            from_id: "system".to_string(),
            from_name: "系统".to_string(),
            from_balance: 0.0,
            to_id: gu_id.to_string(),
            to_name: gu_name.to_string(),
            to_balance: balance_before + amount,
            amount,
            kind: TransactionKind::Deposit,
            reason: "任务奖励".to_string(),
            detail: format!("完成任务「{}」，获得金币奖励", task_name),
        }
    }

    /// 创建资源购买交易
    pub fn resource_purchase(
        gu_id: Uuid,
        gu_name: &str,
        balance_before: f64,
        amount: f64,
        resource_name: &str,
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: format!("tx_{:08x}_{}", timestamp, gu_id),
            timestamp,
            from_id: gu_id.to_string(),
            from_name: gu_name.to_string(),
            from_balance: (balance_before - amount).max(0.0),
            to_id: "system".to_string(),
            to_name: "系统".to_string(),
            to_balance: 0.0,
            amount: -amount,
            kind: TransactionKind::Withdraw,
            reason: "资源购买".to_string(),
            detail: format!("购买「{}」消耗金币", resource_name),
        }
    }

    /// 创建技能升级交易
    pub fn skill_upgrade(
        gu_id: Uuid,
        gu_name: &str,
        balance_before: f64,
        amount: f64,
        skill_name: &str,
        from_level: u32,
        to_level: u32,
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: format!("tx_{:08x}_{}", timestamp, gu_id),
            timestamp,
            from_id: gu_id.to_string(),
            from_name: gu_name.to_string(),
            from_balance: (balance_before - amount).max(0.0),
            to_id: "system".to_string(),
            to_name: "系统".to_string(),
            to_balance: 0.0,
            amount: -amount,
            kind: TransactionKind::Withdraw,
            reason: "能力提升".to_string(),
            detail: format!("技能升级「{} {}→{}」消耗金币", skill_name, from_level, to_level),
        }
    }

    /// 创建知识付费交易
    pub fn knowledge_payment(
        gu_id: Uuid,
        gu_name: &str,
        balance_before: f64,
        amount: f64,
        knowledge_name: &str,
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: format!("tx_{:08x}_{}", timestamp, gu_id),
            timestamp,
            from_id: gu_id.to_string(),
            from_name: gu_name.to_string(),
            from_balance: (balance_before - amount).max(0.0),
            to_id: "system".to_string(),
            to_name: "系统".to_string(),
            to_balance: 0.0,
            amount: -amount,
            kind: TransactionKind::Withdraw,
            reason: "知识付费".to_string(),
            detail: format!("学习知识「{}」支付学费", knowledge_name),
        }
    }

    /// 创建蛊虫间转账交易
    pub fn transfer(
        from_id: Uuid,
        from_name: &str,
        from_balance_before: f64,
        to_id: Uuid,
        to_name: &str,
        to_balance_before: f64,
        amount: f64,
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: format!("tx_{:08x}_{}", timestamp, from_id),
            timestamp,
            from_id: from_id.to_string(),
            from_name: from_name.to_string(),
            from_balance: (from_balance_before - amount).max(0.0),
            to_id: to_id.to_string(),
            to_name: to_name.to_string(),
            to_balance: to_balance_before + amount,
            amount: -amount, // 对 from 来说是支出
            kind: TransactionKind::Transfer,
            reason: "转账支付".to_string(),
            detail: format!("向{}转账{}金币", to_name, amount as i64),
        }
    }
}

/// 世界执行结果
#[derive(Debug, Clone)]
pub struct WorldResult {
    /// 新的世界状态
    pub world: super::WorldMind,
    /// 产生的事件
    pub events: Vec<WorldEvent>,
}

impl WorldResult {
    pub fn new(world: super::WorldMind) -> Self {
        Self {
            world,
            events: Vec::new(),
        }
    }

    pub fn with_event(mut self, event: WorldEvent) -> Self {
        self.events.push(event);
        self
    }

    pub fn with_events(mut self, events: Vec<WorldEvent>) -> Self {
        self.events.extend(events);
        self
    }
}
