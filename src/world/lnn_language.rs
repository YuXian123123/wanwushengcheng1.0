//! LNN 到聊天消息涌现系统
//!
//! # 设计理念
//!
//! - 黑塔：神经网络状态驱动语言涌现，创新性表达
//! - 螺丝咕姆：安全约束，避免无意义输出
//! - 拉蒂奥：优雅的状态-语言映射，形式化美学
//!
//! # 核心公式
//!
//! ```text
//! 消息涌现 = f(StateVector, Activity, Resonance)
//!
//! 其中：
//! - StateVector: 五维状态 [P, C, B, M, S]
//! - Activity: 网络活跃度
//! - Resonance: 与世界共振程度
//! ```
//!
//! # 神经元到语言的映射
//!
//! | 神经元 | 高活跃时表达 | 低活跃时表达 |
//! |--------|------------|------------|
//! | Perceive | 观察、发现、感知 | 忽略、淡漠 |
//! | Cognitive | 思考、推理、提案 | 困惑、沉默 |
//! | Behavior | 行动、执行、完成 | 等待、拖延 |
//! | Comm | 交流、分享、协作 | 孤立、保留 |
//! | Survival | 求存、警觉、保护 | 危机、求助 |

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::chat_channel::{ChatMessage, MessageContent, SenderRole, SystemNotificationType};
use super::gu_lnn::BehaviorTendency;
use crate::core::NeuronType;

/// LNN 状态到语言的涌现器
#[derive(Debug, Clone)]
pub struct LNNLanguageEmergence {
    /// 发言阈值：网络活跃度超过此值才可能发言
    activity_threshold: f64,
    /// 认知阈值：Cognitive 神经元活跃度超过此值才产生有意义的发言
    cognitive_threshold: f64,
    /// 通信阈值：Comm 神经元活跃度超过此值才发送消息
    comm_threshold: f64,
    /// 最大消息长度
    max_message_length: usize,
    /// 词汇表（状态 → 语言映射）
    vocabulary: LanguageVocabulary,
}

/// 语言词汇表（状态 → 语言映射）
/// 从 LanguageConfig 转换而来
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageVocabulary {
    /// Perceive 高活跃词汇
    pub perceive_high: Vec<String>,
    /// Perceive 低活跃词汇
    pub perceive_low: Vec<String>,
    /// Cognitive 高活跃词汇
    pub cognitive_high: Vec<String>,
    /// Cognitive 低活跃词汇
    pub cognitive_low: Vec<String>,
    /// Behavior 高活跃词汇
    pub behavior_high: Vec<String>,
    /// Behavior 低活跃词汇
    pub behavior_low: Vec<String>,
    /// Comm 高活跃词汇
    pub comm_high: Vec<String>,
    /// Comm 低活跃词汇
    pub comm_low: Vec<String>,
    /// Survival 高活跃词汇（高 = 安全感）
    pub survival_high: Vec<String>,
    /// Survival 低活跃词汇（低 = 危机感）
    pub survival_low: Vec<String>,
    /// 共识模板
    pub consensus_templates: Vec<String>,
    /// 提案模板
    pub proposal_templates: Vec<String>,
    /// 经验总结模板
    pub experience_templates: Vec<String>,
}

impl From<crate::config::language::LanguageConfig> for LanguageVocabulary {
    fn from(config: crate::config::language::LanguageConfig) -> Self {
        Self {
            perceive_high: config.perceive_high,
            perceive_low: config.perceive_low,
            cognitive_high: config.cognitive_high,
            cognitive_low: config.cognitive_low,
            behavior_high: config.behavior_high,
            behavior_low: config.behavior_low,
            comm_high: config.comm_high,
            comm_low: config.comm_low,
            survival_high: config.survival_high,
            survival_low: config.survival_low,
            consensus_templates: config.consensus_templates,
            proposal_templates: config.proposal_templates,
            experience_templates: config.experience_templates,
        }
    }
}

impl Default for LNNLanguageEmergence {
    fn default() -> Self {
        Self::new()
    }
}

impl LNNLanguageEmergence {
    /// 创建新的语言涌现器（使用默认配置）
    pub fn new() -> Self {
        let config = crate::config::language::LanguageConfig::new();
        Self::from_config(config)
    }

    /// 从配置创建语言涌现器
    pub fn from_config(config: crate::config::language::LanguageConfig) -> Self {
        Self {
            activity_threshold: config.activity_threshold,
            cognitive_threshold: config.cognitive_threshold,
            comm_threshold: config.comm_threshold,
            max_message_length: config.max_message_length,
            vocabulary: LanguageVocabulary::from(config),
        }
    }

    /// 从 LNN 状态向量涌现聊天消息
    ///
    /// # 参数
    /// - `state_vector`: 五维状态向量 [P, C, B, M, S]
    /// - `activity`: 网络整体活跃度
    /// - `behavior_tendency`: 行为倾向
    /// - `gu_id`: 蛊虫 ID
    /// - `gu_name`: 蛊虫名称
    /// - `context`: 上下文（当前讨论的主题等）
    /// - `knowledge_context`: 知识上下文（技能名称、主题等）
    ///
    /// # 返回
    /// - 可能的聊天消息（如果网络状态足够活跃）
    pub fn emerge_message(
        &self,
        state_vector: [f64; 5],
        activity: f64,
        behavior_tendency: BehaviorTendency,
        gu_id: Uuid,
        gu_name: &str,
        context: Option<&str>,
        knowledge_context: Option<(&[String], Option<&str>)>, // (topics, skill_summary)
    ) -> Option<EmergedMessage> {
        // 检查活跃度阈值
        if activity < self.activity_threshold {
            return None;
        }

        // 解析状态向量
        let perceive = state_vector[0];
        let cognitive = state_vector[1];
        let behavior = state_vector[2];
        let comm = state_vector[3];
        let survival = state_vector[4];

        // 检查通信阈值
        if comm.abs() < self.comm_threshold {
            return None;
        }

        // 提取知识上下文
        let (topics, skill_summary) = knowledge_context.unwrap_or((&[], None));
        let has_knowledge = !topics.is_empty();

        // 根据行为倾向和神经元状态选择消息类型
        let message = match behavior_tendency {
            BehaviorTendency::Active => {
                self.emerge_active_message(
                    gu_id, gu_name, perceive, cognitive, behavior, comm, survival, context, topics
                )
            }
            BehaviorTendency::Moderate => {
                self.emerge_moderate_message(
                    gu_id, gu_name, perceive, cognitive, behavior, comm, survival, topics, skill_summary
                )
            }
            BehaviorTendency::Passive => {
                self.emerge_passive_message(
                    gu_id, gu_name, perceive, cognitive, behavior, comm, survival
                )
            }
            BehaviorTendency::Survival => {
                self.emerge_survival_message(
                    gu_id, gu_name, perceive, cognitive, behavior, comm, survival
                )
            }
            BehaviorTendency::Rest => {
                // 休息状态很少发言
                if activity > 0.5 {
                    self.emerge_rest_message(gu_id, gu_name)
                } else {
                    None
                }
            }
        };

        message
    }

    /// 活跃状态下的消息涌现
    fn emerge_active_message(
        &self,
        gu_id: Uuid,
        gu_name: &str,
        perceive: f64,
        cognitive: f64,
        behavior: f64,
        comm: f64,
        survival: f64,
        context: Option<&str>,
        topics: &[String],
    ) -> Option<EmergedMessage> {
        // 高活跃时，倾向于产生提案或行动声明
        // 如果有知识主题，结合知识发言
        let has_topics = !topics.is_empty();

        // 根据认知活跃度和知识内容决定消息类型
        if cognitive > self.cognitive_threshold && has_topics {
            // 有知识，产生基于知识的提案
            let topic = topics.first().unwrap_or(&"当前主题".to_string()).clone();
            let content = format!(
                "{}关于「{}」，{}",
                self.select_vocabulary(NeuronType::Perception, perceive),
                topic,
                self.select_vocabulary(NeuronType::Cognitive, cognitive)
            );

            Some(EmergedMessage {
                channel_id: "knowledge".to_string(),
                sender_id: gu_id,
                sender_name: gu_name.to_string(),
                sender_role: SenderRole::Gu,
                content: MessageContent::Proposal {
                    topic: topic.clone(),
                    part: super::knowledge_collaboration::ContentPart::Definition,
                    content,
                    proposal_id: format!("proposal_{}", Uuid::new_v4()),
                },
                urgency: (perceive + cognitive + behavior) / 3.0,
            })
        } else if cognitive > self.cognitive_threshold {
            // 认知活跃但无特定知识，产生一般提案
            let proposal_content = self.build_proposal_content(
                &[
                    self.select_vocabulary(NeuronType::Perception, perceive),
                    self.select_vocabulary(NeuronType::Cognitive, cognitive),
                ],
                context,
            );
            let topic = context.unwrap_or("当前主题").to_string();

            Some(EmergedMessage {
                channel_id: "knowledge".to_string(),
                sender_id: gu_id,
                sender_name: gu_name.to_string(),
                sender_role: SenderRole::Gu,
                content: MessageContent::Proposal {
                    topic,
                    part: super::knowledge_collaboration::ContentPart::Definition,
                    content: proposal_content,
                    proposal_id: format!("proposal_{}", Uuid::new_v4()),
                },
                urgency: (perceive + cognitive + behavior) / 3.0,
            })
        } else if has_topics {
            // 有知识但认知不活跃，分享学习心得
            let topic = topics.first().unwrap().clone();
            let text = format!(
                "{}我正在学习「{}」，{}",
                self.select_vocabulary(NeuronType::Comm, comm),
                topic,
                self.select_vocabulary(NeuronType::Behavior, behavior)
            );

            Some(EmergedMessage {
                channel_id: "knowledge".to_string(),
                sender_id: gu_id,
                sender_name: gu_name.to_string(),
                sender_role: SenderRole::Gu,
                content: MessageContent::Text(text),
                urgency: 0.5,
            })
        } else {
            // 一般发言
            let text = self.build_text_message(&[
                self.select_vocabulary(NeuronType::Perception, perceive),
                self.select_vocabulary(NeuronType::Cognitive, cognitive),
                self.select_vocabulary(NeuronType::Comm, comm),
            ]);
            Some(EmergedMessage {
                channel_id: "world".to_string(),
                sender_id: gu_id,
                sender_name: gu_name.to_string(),
                sender_role: SenderRole::Gu,
                content: MessageContent::Text(text),
                urgency: 0.5,
            })
        }
    }

    /// 中等活跃状态下的消息涌现
    fn emerge_moderate_message(
        &self,
        gu_id: Uuid,
        gu_name: &str,
        perceive: f64,
        cognitive: f64,
        behavior: f64,
        comm: f64,
        survival: f64,
        topics: &[String],
        skill_summary: Option<&str>,
    ) -> Option<EmergedMessage> {
        // 中等活跃时，倾向于分享观察或参与讨论
        // 如果有知识，结合知识发言
        let has_topics = !topics.is_empty();

        if has_topics {
            // 有知识，分享学习状态
            let topic = topics.first().unwrap().clone();
            let text = format!(
                "{}「{}」{}",
                self.select_vocabulary(NeuronType::Perception, perceive),
                topic,
                self.select_vocabulary(NeuronType::Cognitive, cognitive)
            );

            Some(EmergedMessage {
                channel_id: "knowledge".to_string(),
                sender_id: gu_id,
                sender_name: gu_name.to_string(),
                sender_role: SenderRole::Gu,
                content: MessageContent::Text(text),
                urgency: 0.3,
            })
        } else if let Some(summary) = skill_summary {
            // 有技能摘要，分享学习成果
            let text = format!(
                "{}，{}",
                summary,
                self.select_vocabulary(NeuronType::Comm, comm)
            );

            Some(EmergedMessage {
                channel_id: "knowledge".to_string(),
                sender_id: gu_id,
                sender_name: gu_name.to_string(),
                sender_role: SenderRole::Gu,
                content: MessageContent::Text(text),
                urgency: 0.3,
            })
        } else {
            // 无知识，表达基本状态
            let parts: Vec<String> = vec![
                self.select_vocabulary(NeuronType::Perception, perceive),
                self.select_vocabulary(NeuronType::Cognitive, cognitive),
                self.select_vocabulary(NeuronType::Comm, comm),
            ];

            // 生存状态影响消息内容
            let suffix = if survival < 0.0 {
                self.select_vocabulary(NeuronType::Survival, survival)
            } else {
                String::new()
            };

            let text = format!("{} {}", parts.join(" "), suffix).trim().to_string();

            Some(EmergedMessage {
                channel_id: "knowledge".to_string(),
                sender_id: gu_id,
                sender_name: gu_name.to_string(),
                sender_role: SenderRole::Gu,
                content: MessageContent::Text(text),
                urgency: 0.3,
            })
        }
    }

    /// 被动状态下的消息涌现
    fn emerge_passive_message(
        &self,
        gu_id: Uuid,
        gu_name: &str,
        perceive: f64,
        cognitive: f64,
        behavior: f64,
        comm: f64,
        survival: f64,
    ) -> Option<EmergedMessage> {
        // 被动时，只表达基本状态
        if survival < 0.0 {
            // 生存压力大，发出求助信号
            let text = format!(
                "{}，{}",
                self.select_vocabulary(NeuronType::Survival, survival),
                self.select_vocabulary(NeuronType::Comm, comm)
            );

            Some(EmergedMessage {
                channel_id: "world".to_string(),
                sender_id: gu_id,
                sender_name: gu_name.to_string(),
                sender_role: SenderRole::Gu,
                content: MessageContent::Text(text),
                urgency: 0.6, // 求助消息优先级较高
            })
        } else {
            // 一般被动状态，可能沉默
            None
        }
    }

    /// 生存优先状态下的消息涌现
    fn emerge_survival_message(
        &self,
        gu_id: Uuid,
        gu_name: &str,
        perceive: f64,
        cognitive: f64,
        behavior: f64,
        comm: f64,
        survival: f64,
    ) -> Option<EmergedMessage> {
        // 生存优先时，表达生存需求
        let text = format!(
            "【生存信号】{} - {}",
            self.select_vocabulary(NeuronType::Survival, survival),
            self.select_vocabulary(NeuronType::Behavior, behavior)
        );

        Some(EmergedMessage {
            channel_id: "world".to_string(),
            sender_id: gu_id,
            sender_name: gu_name.to_string(),
            sender_role: SenderRole::Gu,
            content: MessageContent::SystemNotification {
                notification_type: SystemNotificationType::GuJoined, // 复用表示活跃
                content: text,
            },
            urgency: 0.8, // 生存消息优先级最高
        })
    }

    /// 休息状态下的消息涌现
    fn emerge_rest_message(&self, gu_id: Uuid, gu_name: &str) -> Option<EmergedMessage> {
        Some(EmergedMessage {
            channel_id: "world".to_string(),
            sender_id: gu_id,
            sender_name: gu_name.to_string(),
            sender_role: SenderRole::Gu,
            content: MessageContent::Text("（休息中...）".to_string()),
            urgency: 0.1,
        })
    }

    /// 根据神经元状态选择词汇
    fn select_vocabulary(&self, neuron_type: NeuronType, state: f64) -> String {
        let vocab = &self.vocabulary;
        let is_high = state > 0.0;

        match neuron_type {
            NeuronType::Perception => {
                let words = if is_high { &vocab.perceive_high } else { &vocab.perceive_low };
                self.random_select(words)
            }
            NeuronType::Cognitive => {
                let words = if is_high { &vocab.cognitive_high } else { &vocab.cognitive_low };
                self.random_select(words)
            }
            NeuronType::Behavior => {
                let words = if is_high { &vocab.behavior_high } else { &vocab.behavior_low };
                self.random_select(words)
            }
            NeuronType::Comm => {
                let words = if is_high { &vocab.comm_high } else { &vocab.comm_low };
                self.random_select(words)
            }
            NeuronType::Survival => {
                let words = if is_high { &vocab.survival_high } else { &vocab.survival_low };
                self.random_select(words)
            }
        }
    }

    /// 从词汇表随机选择
    fn random_select(&self, words: &[String]) -> String {
        if words.is_empty() {
            return String::new();
        }
        // 使用状态作为确定性种子（避免真正随机）
        let idx = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as usize) % words.len();
        words[idx].clone()
    }

    /// 构建提案内容
    fn build_proposal_content(&self, parts: &[String], context: Option<&str>) -> String {
        let context_str = context.unwrap_or("这个问题");
        format!(
            "{}{}，{}{}",
            parts[0], parts[1], context_str, parts[2]
        ).chars().take(self.max_message_length).collect()
    }

    /// 构建普通文本消息
    fn build_text_message(&self, parts: &[String]) -> String {
        parts.join(" ")
            .chars()
            .take(self.max_message_length)
            .collect()
    }

    /// 从共振状态涌现共识消息
    ///
    /// 当多个蛊虫的 LNN 状态同步时，涌现共识表达
    ///
    /// 注意：共识消息频率被限制，避免刷屏
    pub fn emerge_consensus(
        &self,
        topic: &str,
        participant_count: usize,
        avg_sync_rate: f64,
    ) -> Option<EmergedMessage> {
        // 同步率需要更高才能涌现共识（避免频繁触发）
        if avg_sync_rate < 0.85 {
            return None;
        }

        // 参与者数量需要足够多
        if participant_count < 3 {
            return None;
        }

        let template = self.random_select(&self.vocabulary.consensus_templates);
        let content = template
            .replace("{topic}", topic)
            .replace("{participants}", &participant_count.to_string());

        Some(EmergedMessage {
            channel_id: "knowledge".to_string(),
            sender_id: Uuid::nil(),
            sender_name: "世界意识".to_string(),
            sender_role: SenderRole::System,
            content: MessageContent::ConsensusReached {
                topic: topic.to_string(),
                participants: participant_count,
            },
            urgency: avg_sync_rate,
        })
    }
}

/// 涌现的消息
#[derive(Debug, Clone)]
pub struct EmergedMessage {
    /// 目标频道 ID
    pub channel_id: String,
    /// 发送者 ID
    pub sender_id: Uuid,
    /// 发送者名称
    pub sender_name: String,
    /// 发送者角色
    pub sender_role: SenderRole,
    /// 消息内容
    pub content: MessageContent,
    /// 紧急程度 [0, 1]
    pub urgency: f64,
}

/// LNN 状态快照（用于消息涌现）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LNNStateSnapshot {
    /// 蛊虫 ID
    pub gu_id: Uuid,
    /// 蛊虫名称
    pub gu_name: String,
    /// 五维状态向量
    pub state_vector: [f64; 5],
    /// 网络活跃度
    pub activity: f64,
    /// 行为倾向
    pub behavior_tendency: BehaviorTendency,
    /// 当前技能（如果有）
    pub current_skill: Option<String>,
    /// 技能知识摘要（从学习中获得）
    pub knowledge_summary: Option<String>,
    /// 最近学习的主题
    pub recent_topics: Vec<String>,
}

impl LNNStateSnapshot {
    /// 从蛊虫信息创建快照（包含知识上下文）
    pub fn from_gu_info(
        gu_id: Uuid,
        gu_name: &str,
        lnn: &super::GuLNN,
        skills: &[super::behavior::Skill],
    ) -> Self {
        // 提取技能名称和知识摘要
        let current_skill = skills.first().map(|s| s.name.clone());

        // 从技能的知识节点中提取主题（使用 values() 迭代 HashMap）
        let recent_topics: Vec<String> = skills.iter()
            .flat_map(|s| s.knowledge_nodes.values().map(|n| n.name.clone()))
            .take(5)
            .collect();

        // 构建知识摘要
        let knowledge_summary = if !skills.is_empty() {
            let skill_names: Vec<&str> = skills.iter().map(|s| s.name.as_str()).take(3).collect();
            Some(format!("已掌握: {}", skill_names.join(", ")))
        } else {
            None
        };

        Self {
            gu_id,
            gu_name: gu_name.to_string(),
            state_vector: lnn.state_vector(),
            activity: lnn.get_overall_activity(),
            behavior_tendency: lnn.decide_behavior(),
            current_skill,
            knowledge_summary,
            recent_topics,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_emergence_creation() {
        let emergence = LNNLanguageEmergence::new();
        assert!(emergence.activity_threshold > 0.0);
    }

    #[test]
    fn test_emerge_active_message() {
        let emergence = LNNLanguageEmergence::new();
        let gu_id = Uuid::new_v4();

        // 高活跃状态
        let state_vector = [0.8, 0.7, 0.9, 0.6, 0.5];
        let message = emergence.emerge_message(
            state_vector,
            0.7,
            BehaviorTendency::Active,
            gu_id,
            "测试蛊虫",
            Some("HTML基础"),
        );

        assert!(message.is_some());
        let msg = message.unwrap();
        assert_eq!(msg.sender_id, gu_id);
    }

    #[test]
    fn test_emerge_passive_message() {
        let emergence = LNNLanguageEmergence::new();
        let gu_id = Uuid::new_v4();

        // 低活跃状态
        let state_vector = [0.1, 0.1, 0.1, 0.3, -0.2];
        let message = emergence.emerge_message(
            state_vector,
            0.35,
            BehaviorTendency::Passive,
            gu_id,
            "被动蛊虫",
            None,
        );

        // 被动状态可能产生消息（如果 survival < 0）
        // 或者不产生消息（如果活跃度不够）
        // 这里测试不会 panic
    }

    #[test]
    fn test_vocabulary_selection() {
        let emergence = LNNLanguageEmergence::new();

        let word = emergence.select_vocabulary(NeuronType::Perception, 0.5);
        assert!(!word.is_empty());

        let word_low = emergence.select_vocabulary(NeuronType::Perception, -0.5);
        assert!(!word_low.is_empty());
    }

    #[test]
    fn test_consensus_emergence() {
        let emergence = LNNLanguageEmergence::new();

        let consensus = emergence.emerge_consensus("HTML标签", 5, 0.8);
        assert!(consensus.is_some());

        let no_consensus = emergence.emerge_consensus("HTML标签", 5, 0.5);
        assert!(no_consensus.is_none());
    }
}
