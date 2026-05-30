//! 消息类型定义模块
//!
//! 定义通信系统中的消息结构，包括消息ID、消息类型、信道类型等核心概念。
//! 所有设计遵循不可变性原则，确保状态变化通过创建新对象实现。

use std::collections::HashSet;
use std::fmt::Debug;
use uuid::Uuid;

/// 蛊虫唯一标识符
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GuId(pub String);

/// 小组唯一标识符
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GroupId(pub String);

/// 消息唯一标识符
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MessageId(pub Uuid);

impl MessageId {
    /// 创建新的消息ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// 信道类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChannelType {
    /// 世界频道 - 全局广播
    World,
    /// 小组频道 - 组内通信
    Group { group_id: GroupId },
    /// 个人频道 - 点对点通信
    Personal { peer_id: GuId },
}

/// 生命周期阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecyclePhase {
    Egg,      // 卵期
    Larva,    // 幼虫期
    Growth,   // 成长期
    Maturity, // 成熟期
    Aging,    // 衰老期
    Dead,     // 死亡期
}

/// 系统事件类型
#[derive(Debug, Clone)]
pub enum SystemEvent {
    LifecycleChange(LifecyclePhase),
    ChannelCreated(ChannelType),
    ChannelClosed(ChannelType),
}

/// 交易行为
#[derive(Debug, Clone)]
pub enum TradeAction {
    Buy,
    Sell,
    Exchange,
}

/// 产品信息
#[derive(Debug, Clone)]
pub struct ProductInfo {
    pub name: String,
    pub description: String,
    pub category: String,
}

/// 消息签名
#[derive(Debug, Clone)]
pub struct MessageSignature {
    pub signature: String,
    pub timestamp: u64,
}

/// 消息签名器
#[derive(Debug, Clone)]
pub struct MessageSigner;

impl MessageSigner {
    /// 对消息进行签名
    pub fn sign(&self, sender: &GuId, channel: &ChannelType, message_type: &MessageType) -> MessageSignature {
        // 简化实现，实际应用中需要使用加密签名
        MessageSignature {
            signature: format!("sig_{}", sender.0),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

/// 消息类型枚举
#[derive(Debug, Clone)]
pub enum MessageType {
    /// 知识消息
    Knowledge {
        content: String,
        concepts: HashSet<String>,
        confidence: f64,
    },
    /// 交易消息
    Trade {
        product: ProductInfo,
        price: f64,
        action: TradeAction,
    },
    /// 状态消息
    Status {
        lifecycle_phase: LifecyclePhase,
        health: f64,
    },
    /// 情绪消息
    Emotion {
        pleasure: f64,
        arousal: f64,
    },
    /// 系统消息
    System {
        event: SystemEvent,
    },
}

/// 消息结构体（不可变）
#[derive(Debug, Clone)]
pub struct Message {
    /// 消息ID
    pub id: MessageId,
    /// 发送者
    pub sender: GuId,
    /// 信道
    pub channel: ChannelType,
    /// 消息类型
    pub message_type: MessageType,
    /// 时间戳
    pub timestamp: u64,
    /// 签名
    pub signature: MessageSignature,
}

impl Message {
    /// 创建新消息（唯一构造方式）
    pub fn new(
        sender: GuId,
        channel: ChannelType,
        message_type: MessageType,
        signer: &MessageSigner,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        Self {
            id: MessageId::new(),
            sender,
            channel,
            message_type,
            timestamp: now,
            signature: signer.sign(&sender, &channel, &message_type),
        }
    }
    
    /// 获取消息内容长度（用于成本计算）
    pub fn content_length(&self) -> usize {
        match &self.message_type {
            MessageType::Knowledge { content, .. } => content.len(),
            MessageType::Trade { product, .. } => product.name.len() + product.description.len(),
            MessageType::Status { .. } => 100, // 固定长度估算
            MessageType::Emotion { .. } => 50,  // 固定长度估算
            MessageType::System { .. } => 80,   // 固定长度估算
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let sender = GuId("sender_1".to_string());
        let channel = ChannelType::World;
        let message_type = MessageType::Knowledge {
            content: "Hello, world!".to_string(),
            concepts: HashSet::from(["greeting".to_string(), "communication".to_string()]),
            confidence: 0.95,
        };
        
        let signer = MessageSigner;
        let message = Message::new(sender.clone(), channel, message_type, &signer);
        
        assert_eq!(message.sender, sender);
        assert_eq!(message.channel, ChannelType::World);
        assert!(matches!(message.message_type, MessageType::Knowledge { .. }));
    }
    
    #[test]
    fn test_message_id_uniqueness() {
        let id1 = MessageId::new();
        let id2 = MessageId::new();
        assert_ne!(id1, id2);
    }
    
    #[test]
    fn test_content_length() {
        let sender = GuId("sender_1".to_string());
        let channel = ChannelType::World;
        let message_type = MessageType::Knowledge {
            content: "Hello, world!".to_string(), // 13 characters
            concepts: HashSet::new(),
            confidence: 0.95,
        };
        
        let signer = MessageSigner;
        let message = Message::new(sender, channel, message_type, &signer);
        assert_eq!(message.content_length(), 13);
    }
}
