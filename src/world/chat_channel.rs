//! 聊天频道系统 - 蛊虫实时通信
//!
//! # 设计理念
//!
//! - 黑塔：涌现式讨论，创新性提议
//! - 螺丝咕姆：安全验证，冲突记录
//! - 拉蒂奥：优雅排版，形式化表达
//!
//! # 核心功能
//!
//! 1. 频道管理：世界频道、协作频道、私有频道
//! 2. 消息系统：文本、公式、提案、投票
//! 3. 实时同步：WebSocket 推送

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use super::knowledge_collaboration::{
    ContentPart, ProposalStatus, ResolutionMethod,
    TimelineEventType,
};

/// 聊天频道
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChannel {
    /// 频道 ID
    pub id: String,
    /// 频道名称
    pub name: String,
    /// 频道类型
    pub channel_type: ChannelType,
    /// 消息列表
    pub messages: Vec<ChatMessage>,
    /// 在线蛊虫
    pub online_participants: Vec<Uuid>,
    /// 创建时间
    pub created_at: u64,
}

/// 频道类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelType {
    /// 世界频道（全局广播）
    World,
    /// 协作频道（知识讨论）
    Collaboration,
    /// 私有频道（一对一）
    Private,
    /// 系统频道（日志通知）
    System,
}

/// 聊天消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// 消息 ID
    pub id: String,
    /// 发送者 ID
    pub sender_id: Uuid,
    /// 发送者名称
    pub sender_name: String,
    /// 发送者角色
    pub sender_role: SenderRole,
    /// 消息内容
    pub content: MessageContent,
    /// 发送时间
    pub sent_at: u64,
    /// 回复的消息 ID
    pub reply_to: Option<String>,
    /// 反应（表情）
    pub reactions: Vec<Reaction>,
}

/// 发送者角色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SenderRole {
    /// 黑塔（创新天才）
    BlackTower,
    /// 螺丝咕姆（安全天才）
    Screwllum,
    /// 拉蒂奥（优雅天才）
    Latio,
    /// 普通蛊虫
    Gu,
    /// 系统
    System,
}

/// 消息内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageContent {
    /// 普通文本
    Text(String),
    /// 公式
    Formula {
        /// 公式文本
        formula: String,
        /// 说明
        description: String,
    },
    /// 代码块
    Code {
        /// 代码内容
        code: String,
        /// 语言
        language: String,
    },
    /// 知识提案
    Proposal {
        /// 主题
        topic: String,
        /// 部分
        part: ContentPart,
        /// 内容
        content: String,
        /// 提案 ID
        proposal_id: String,
    },
    /// 投票
    Vote {
        /// 提案 ID
        proposal_id: String,
        /// 是否支持
        support: bool,
        /// 理由
        reason: String,
    },
    /// 反对意见
    Opposition {
        /// 提案 ID
        proposal_id: String,
        /// 反对理由
        reason: String,
        /// 建议修改
        suggestion: Option<String>,
    },
    /// 冲突解决
    ConflictResolution {
        /// 冲突 ID
        conflict_id: String,
        /// 解决方式
        method: ResolutionMethod,
        /// 最终内容
        final_content: String,
    },
    /// 共识达成通知
    ConsensusReached {
        /// 主题
        topic: String,
        /// 参与者数量
        participants: usize,
    },
    /// 系统通知
    SystemNotification {
        /// 通知类型
        notification_type: SystemNotificationType,
        /// 内容
        content: String,
    },
    /// 经验总结
    ExperienceSummary {
        /// 主题
        topic: String,
        /// 经验列表
        lessons: Vec<String>,
    },
}

/// 系统通知类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemNotificationType {
    /// 讨论开始
    DiscussionStarted,
    /// 蛊虫加入
    GuJoined,
    /// 蛊虫离开
    GuLeft,
    /// 提议提交
    ProposalSubmitted,
    /// 投票更新
    VoteUpdated,
    /// 冲突发生
    ConflictOccurred,
    /// 冲突解决
    ConflictResolved,
    /// 共识达成
    ConsensusReached,
    /// 知识入库
    KnowledgeStored,
}

/// 反应（表情）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    /// 表情
    pub emoji: String,
    /// 反应者
    pub reactor_id: Uuid,
}

/// 聊天系统
#[derive(Debug, Clone, Default)]
pub struct ChatSystem {
    /// 频道列表
    channels: HashMap<String, ChatChannel>,
    /// 蛊虫所在频道
    gu_channels: HashMap<Uuid, Vec<String>>,
}

impl ChatSystem {
    /// 创建新的聊天系统
    pub fn new() -> Self {
        let mut system = Self {
            channels: HashMap::new(),
            gu_channels: HashMap::new(),
        };

        // 创建默认频道
        system.create_channel("world", "世界意识", ChannelType::World);
        system.create_channel("knowledge", "知识讨论", ChannelType::Collaboration);
        system.create_channel("genius-council", "天才议会", ChannelType::Collaboration);
        system.create_channel("system", "系统日志", ChannelType::System);

        system
    }

    /// 创建频道
    pub fn create_channel(&mut self, id: &str, name: &str, channel_type: ChannelType) {
        self.channels.insert(id.to_string(), ChatChannel {
            id: id.to_string(),
            name: name.to_string(),
            channel_type,
            messages: Vec::new(),
            online_participants: Vec::new(),
            created_at: Self::now(),
        });
    }

    /// 蛊虫加入频道
    pub fn join_channel(&mut self, channel_id: &str, gu_id: Uuid) -> Option<()> {
        let channel = self.channels.get_mut(channel_id)?;

        if !channel.online_participants.contains(&gu_id) {
            channel.online_participants.push(gu_id);
        }

        // 记录蛊虫所在的频道
        self.gu_channels
            .entry(gu_id)
            .or_default()
            .push(channel_id.to_string());

        Some(())
    }

    /// 发送消息
    pub fn send_message(
        &mut self,
        channel_id: &str,
        sender_id: Uuid,
        sender_name: &str,
        sender_role: SenderRole,
        content: MessageContent,
    ) -> Option<String> {
        let channel = self.channels.get_mut(channel_id)?;

        let message = ChatMessage {
            id: format!("msg_{}", Uuid::new_v4()),
            sender_id,
            sender_name: sender_name.to_string(),
            sender_role,
            content,
            sent_at: Self::now(),
            reply_to: None,
            reactions: Vec::new(),
        };

        let message_id = message.id.clone();
        channel.messages.push(message);

        Some(message_id)
    }

    /// 发送系统通知
    pub fn send_system_notification(
        &mut self,
        channel_id: &str,
        notification_type: SystemNotificationType,
        content: String,
    ) -> Option<String> {
        self.send_message(
            channel_id,
            Uuid::nil(),
            "系统",
            SenderRole::System,
            MessageContent::SystemNotification {
                notification_type,
                content,
            },
        )
    }

    /// 获取频道消息
    pub fn get_messages(&self, channel_id: &str, limit: usize) -> Vec<&ChatMessage> {
        self.channels
            .get(channel_id)
            .map(|c| c.messages.iter().rev().take(limit).rev().collect())
            .unwrap_or_default()
    }

    /// 获取频道
    pub fn get_channel(&self, channel_id: &str) -> Option<&ChatChannel> {
        self.channels.get(channel_id)
    }

    /// 获取所有频道
    pub fn get_all_channels(&self) -> Vec<&ChatChannel> {
        self.channels.values().collect()
    }

    /// 生成 HTML 格式的消息
    pub fn render_message_html(message: &ChatMessage) -> String {
        let avatar = match message.sender_role {
            SenderRole::BlackTower => "🗼",
            SenderRole::Screwllum => "🔧",
            SenderRole::Latio => "📐",
            SenderRole::System => "⚙️",
            SenderRole::Gu => "🐛",
        };

        let role_class = match message.sender_role {
            SenderRole::BlackTower => "black-tower",
            SenderRole::Screwllum => "screwllum",
            SenderRole::Latio => "latio",
            SenderRole::System => "system",
            SenderRole::Gu => "gu",
        };

        let content_html = match &message.content {
            MessageContent::Text(text) => format!(
                r#"<div class="message-text">{}</div>"#,
                text.replace('\n', "<br>")
            ),
            MessageContent::Formula { formula, description } => format!(
                r#"<div class="message-formula">
                    <div class="formula">{}</div>
                    <div class="description">{}</div>
                </div>"#,
                formula, description
            ),
            MessageContent::Code { code, language } => format!(
                r#"<div class="message-code">
                    <div class="language">{}</div>
                    <pre><code>{}</code></pre>
                </div>"#,
                language, code
            ),
            MessageContent::Proposal { topic, part, content, proposal_id } => format!(
                r#"<div class="message-proposal" data-proposal-id="{}">
                    <h4>📋 提案: {} - {}</h4>
                    <p>{}</p>
                    <div class="proposal-actions">
                        <button class="vote-btn approve" onclick="vote('{}', true)">✓ 支持</button>
                        <button class="vote-btn reject" onclick="vote('{}', false)">✗ 反对</button>
                    </div>
                </div>"#,
                proposal_id, topic, part, content, proposal_id, proposal_id
            ),
            MessageContent::Vote { proposal_id, support, reason } => format!(
                r#"<div class="message-vote {}">
                    <span class="vote-icon">{}</span>
                    <span class="vote-reason">{}</span>
                </div>"#,
                if *support { "support" } else { "oppose" },
                if *support { "✓" } else { "✗" },
                reason
            ),
            MessageContent::Opposition { proposal_id, reason, suggestion } => format!(
                r#"<div class="message-opposition">
                    <h4>⚠️ 反对意见</h4>
                    <p>{}</p>
                    {}
                </div>"#,
                reason,
                suggestion.as_ref()
                    .map(|s| format!("<p><b>建议:</b> {}</p>", s))
                    .unwrap_or_default()
            ),
            MessageContent::ConflictResolution { conflict_id, method, final_content } => format!(
                r#"<div class="message-resolution">
                    <h4>✅ 冲突解决 - {:?}</h4>
                    <p>{}</p>
                </div>"#,
                method, final_content
            ),
            MessageContent::ConsensusReached { topic, participants } => format!(
                r#"<div class="message-consensus">
                    <h4>🎉 共识达成!</h4>
                    <p>主题: {}</p>
                    <p>参与者: {} 只蛊虫</p>
                </div>"#,
                topic, participants
            ),
            MessageContent::SystemNotification { notification_type, content } => format!(
                r#"<div class="system-notification {:?}">
                    <span>{}</span>
                </div>"#,
                notification_type, content
            ),
            MessageContent::ExperienceSummary { topic, lessons } => format!(
                r#"<div class="message-experience">
                    <h4>📚 经验总结: {}</h4>
                    <ul>{}</ul>
                </div>"#,
                topic,
                lessons.iter()
                    .map(|l| format!("<li>{}</li>", l))
                    .collect::<Vec<_>>()
                    .join("")
            ),
        };

        format!(
            r#"<div class="message {}">
                <div class="message-avatar {}">{}</div>
                <div class="message-content">
                    <div class="message-header">
                        <span class="message-author">{}</span>
                        <span class="message-time">{}</span>
                    </div>
                    {}
                </div>
            </div>"#,
            role_class,
            role_class,
            avatar,
            message.sender_name,
            message.sent_at,
            content_html
        )
    }

    /// 生成频道 HTML
    pub fn render_channel_html(&self, channel_id: &str) -> Option<String> {
        let channel = self.channels.get(channel_id)?;

        let messages_html = channel.messages.iter()
            .map(|m| Self::render_message_html(m))
            .collect::<Vec<_>>()
            .join("\n");

        Some(format!(
            r#"<div class="chat-container" data-channel="{}">
                <div class="chat-header">
                    <h3>{}</h3>
                    <span class="online-count">{} 在线</span>
                </div>
                <div class="messages-container">
                    {}
                </div>
                <div class="input-container">
                    <textarea class="message-input" placeholder="输入消息..."></textarea>
                    <button class="send-btn">发送</button>
                </div>
            </div>"#,
            channel_id,
            channel.name,
            channel.online_participants.len(),
            messages_html
        ))
    }

    /// 获取当前时间戳
    fn now() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_system() {
        let system = ChatSystem::new();
        assert!(system.get_channel("world").is_some());
        assert!(system.get_channel("knowledge").is_some());
    }

    #[test]
    fn test_send_message() {
        let mut system = ChatSystem::new();
        let gu_id = Uuid::new_v4();

        system.join_channel("world", gu_id);

        let msg_id = system.send_message(
            "world",
            gu_id,
            "测试蛊虫",
            SenderRole::Gu,
            MessageContent::Text("你好，世界！".to_string()),
        );

        assert!(msg_id.is_some());

        let messages = system.get_messages("world", 10);
        assert_eq!(messages.len(), 1);
    }

    #[test]
    fn test_send_proposal() {
        let mut system = ChatSystem::new();
        let gu_id = Uuid::new_v4();

        system.join_channel("knowledge", gu_id);

        let msg_id = system.send_message(
            "knowledge",
            gu_id,
            "黑塔",
            SenderRole::BlackTower,
            MessageContent::Proposal {
                topic: "HTML基础".to_string(),
                part: ContentPart::Definition,
                content: "HTML是超文本标记语言".to_string(),
                proposal_id: "proposal_001".to_string(),
            },
        );

        assert!(msg_id.is_some());

        let html = system.render_channel_html("knowledge");
        assert!(html.unwrap().contains("proposal_001"));
    }
}
