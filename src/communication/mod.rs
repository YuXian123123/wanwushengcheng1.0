//! 通信模块 - 信号共鸣与防刷金币机制
//!
//! 本模块实现蛊虫通信系统，包括：
//! - 信号共鸣编码
//! - 内容质量检测
//! - 语义相似度检测
//! - 防刷金币机制

pub mod message;
pub mod channel;
pub mod signal;
pub mod spectrum;
pub mod quality;
pub mod similarity;

// 重新导出主要类型
pub use message::{Message, MessageId, MessageType, ChannelType, GuId, GroupId};
pub use channel::{ChannelState, ChannelStatus};
pub use signal::{ResonanceSignal, SignalType};
pub use quality::{QualityDetector, QualityScore, QualityThreshold};
pub use similarity::{SimilarityDetector, SimilarityResult};

/// 通信配置
#[derive(Debug, Clone)]
pub struct CommunicationConfig {
    /// 世界频道成本系数
    pub world_cost_factor: f64,
    /// 小组频道成本系数
    pub group_cost_factor: f64,
    /// 个人频道成本系数
    pub personal_cost_factor: f64,
    /// 相似度阈值（默认0.8）
    pub similarity_threshold: f64,
    /// 最小质量阈值
    pub min_quality_threshold: f64,
    /// 最大消息长度
    pub max_message_length: usize,
}

impl Default for CommunicationConfig {
    fn default() -> Self {
        Self {
            world_cost_factor: 5.0,
            group_cost_factor: 2.0,
            personal_cost_factor: 1.0,
            similarity_threshold: 0.8,
            min_quality_threshold: 0.5,
            max_message_length: 10000,
        }
    }
}

/// 通信系统
#[derive(Debug, Clone)]
pub struct CommunicationSystem {
    /// 配置
    pub config: CommunicationConfig,
    /// 信道状态
    pub channels: Vec<ChannelState>,
    /// 消息历史
    pub message_history: Vec<Message>,
}

impl CommunicationSystem {
    /// 创建新的通信系统
    pub fn new(config: CommunicationConfig) -> Self {
        Self {
            config,
            channels: Vec::new(),
            message_history: Vec::new(),
        }
    }

    /// 发送消息（返回新系统状态和消息）
    pub fn with_message_sent(
        &self,
        sender: GuId,
        channel: ChannelType,
        message_type: MessageType,
    ) -> Option<(Self, Message)> {
        let message = Message::new(
            sender,
            channel,
            message_type,
            &message::MessageSigner,
        );

        let mut new_system = self.clone();
        new_system.message_history.push(message.clone());

        Some((new_system, message))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_communication_system_creation() {
        let system = CommunicationSystem::new(CommunicationConfig::default());
        assert_eq!(system.config.similarity_threshold, 0.8);
    }
}
