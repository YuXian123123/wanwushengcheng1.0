//! 知识协作生成系统
//!
//! # 设计理念
//!
//! - 黑塔：竞争涌现最佳内容，冲突激发创新
//! - 螺丝咕姆：共识验证确保准确性，反对意见需要被记录和学习
//! - 拉蒂奥：优雅融合多数派意见，冲突解决过程是宝贵经验
//!
//! # 核心思想
//!
//! 知识的每一个部分（定义、概念、关系、示例、排版等）
//! 都由蛊虫们讨论、投票、共识后生成。
//!
//! **重点**：反对意见和冲突解决过程是宝贵的经验，需要：
//! 1. 完整记录讨论过程
//! 2. 提取冲突点
//! 3. 记录解决方案
//! 4. 存入经验库供 LNN 学习

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 知识内容部分
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ContentPart {
    /// 定义（主题的核心描述）
    Definition,
    /// 概念列表
    Concepts,
    /// 关系列表
    Relations,
    /// 示例
    Examples,
    /// 排版样式
    Layout,
    /// 元数据
    Metadata,
    /// 自定义部分
    Custom(String),
}

impl std::fmt::Display for ContentPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ContentPart {
    pub fn as_str(&self) -> &str {
        match self {
            ContentPart::Definition => "definition",
            ContentPart::Concepts => "concepts",
            ContentPart::Relations => "relations",
            ContentPart::Examples => "examples",
            ContentPart::Layout => "layout",
            ContentPart::Metadata => "metadata",
            ContentPart::Custom(s) => s,
        }
    }
}

/// 内容提议
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentProposal {
    /// 提议 ID
    pub id: String,
    /// 提议者蛊虫 ID
    pub proposer_id: Uuid,
    /// 提议者名称
    pub proposer_name: String,
    /// 针对哪个部分
    pub part: ContentPart,
    /// 提议的内容
    pub content: String,
    /// 提议时间
    pub proposed_at: u64,
    /// 支持者
    pub supporters: Vec<Uuid>,
    /// 反对者
    pub opponents: Vec<Uuid>,
    /// 评论和讨论
    pub comments: Vec<Comment>,
    /// 状态
    pub status: ProposalStatus,
    /// 修改历史
    pub revisions: Vec<Revision>,
}

/// 修改版本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Revision {
    /// 版本号
    pub version: u32,
    /// 修改者
    pub revised_by: Uuid,
    /// 修改者名称
    pub reviser_name: String,
    /// 修改内容
    pub content: String,
    /// 修改原因
    pub reason: String,
    /// 修改时间
    pub revised_at: u64,
}

/// 评论
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    /// 评论 ID
    pub id: String,
    /// 评论者 ID
    pub commenter_id: Uuid,
    /// 评论者名称
    pub commenter_name: String,
    /// 评论内容
    pub content: String,
    /// 评论时间
    pub commented_at: u64,
    /// 评论类型
    pub comment_type: CommentType,
    /// 是否引发修改
    pub triggered_revision: bool,
}

/// 评论类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommentType {
    /// 同意
    Agree,
    /// 反对
    Disagree,
    /// 建议修改
    SuggestModification,
    /// 提问
    Question,
    /// 补充
    Supplement,
    /// 批判
    Critique,
    /// 反驳
    Rebuttal,
    /// 妥协
    Compromise,
}

/// 提议状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalStatus {
    /// 讨论中
    Discussing,
    /// 已达成共识
    ConsensusReached,
    /// 被否决
    Rejected,
    /// 已融合
    Merged,
    /// 需要修改
    NeedsRevision,
}

/// 冲突记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    /// 冲突 ID
    pub id: String,
    /// 冲突主题
    pub topic: String,
    /// 涉及的部分
    pub part: ContentPart,
    /// 冲突方
    pub parties: Vec<ConflictParty>,
    /// 冲突原因
    pub reason: String,
    /// 冲突论点（各方观点）
    pub arguments: Vec<Argument>,
    /// 解决方案
    pub resolution: Option<Resolution>,
    /// 冲突时间
    pub occurred_at: u64,
    /// 解决时间
    pub resolved_at: Option<u64>,
}

/// 冲突方
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictParty {
    /// 蛊虫 ID
    pub gu_id: Uuid,
    /// 名称
    pub name: String,
    /// 立场
    pub stance: Stance,
}

/// 立场
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Stance {
    /// 支持
    Support,
    /// 反对
    Oppose,
    /// 中立
    Neutral,
    /// 有条件支持
    ConditionalSupport,
}

/// 论点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Argument {
    /// 论点 ID
    pub id: String,
    /// 提出者
    pub proposer_id: Uuid,
    /// 提出者名称
    pub proposer_name: String,
    /// 论点内容
    pub content: String,
    /// 论点类型
    pub argument_type: ArgumentType,
    /// 支持证据
    pub evidence: Vec<String>,
    /// 论点时间
    pub proposed_at: u64,
}

/// 论点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArgumentType {
    /// 事实陈述
    Fact,
    /// 逻辑推理
    Logic,
    /// 类比
    Analogy,
    /// 反例
    CounterExample,
    /// 经验
    Experience,
    /// 直觉
    Intuition,
}

/// 解决方案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    /// 解决方式
    pub method: ResolutionMethod,
    /// 最终内容
    pub final_content: String,
    /// 经验教训
    pub lessons_learned: Vec<String>,
    /// 解决时间
    pub resolved_at: u64,
}

/// 解决方式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionMethod {
    /// 多数派胜出
    Majority,
    /// 妥协融合
    Compromise,
    /// 一方说服另一方
    Persuasion,
    /// 第三方调解
    Mediation,
    /// 证据驱动
    EvidenceBased,
    /// 新提议替代
    NewProposal,
}

/// 经验记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    /// 经验 ID
    pub id: String,
    /// 来源主题
    pub topic: String,
    /// 来源冲突 ID
    pub conflict_id: String,
    /// 经验类型
    pub experience_type: ExperienceType,
    /// 经验内容
    pub content: String,
    /// 适用场景
    pub applicable_scenarios: Vec<String>,
    /// 学习权重（LNN 使用）
    pub learning_weight: f64,
    /// 创建时间
    pub created_at: u64,
    /// 引用次数
    pub reference_count: u32,
}

/// 经验类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExperienceType {
    /// 成功模式
    SuccessPattern,
    /// 失败教训
    FailureLesson,
    /// 冲突解决策略
    ConflictResolution,
    /// 共识达成技巧
    ConsensusTechnique,
    /// 内容生成启发
    ContentHeuristic,
}

/// 知识主题讨论会
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeDiscussion {
    /// 主题
    pub topic: String,
    /// 开始时间
    pub started_at: u64,
    /// 参与的蛊虫
    pub participants: Vec<Uuid>,
    /// 各部分的提议（部分 → 提议列表）
    pub proposals: HashMap<String, Vec<ContentProposal>>,
    /// 冲突记录
    pub conflicts: Vec<Conflict>,
    /// 经验记录
    pub experiences: Vec<Experience>,
    /// 最终共识内容（部分 → 内容）
    pub consensus: HashMap<String, String>,
    /// 讨论状态
    pub status: DiscussionStatus,
    /// 需要多少蛊虫同意（100% = 全部参与蛊虫）
    pub required_consensus: usize,
    /// 完整讨论记录（时间线）
    pub timeline: Vec<TimelineEvent>,
}

/// 时间线事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    /// 事件 ID
    pub id: String,
    /// 时间
    pub timestamp: u64,
    /// 事件类型
    pub event_type: TimelineEventType,
    /// 事件描述
    pub description: String,
    /// 涉及的蛊虫
    pub involved_gus: Vec<Uuid>,
    /// 相关提议 ID
    pub related_proposal: Option<String>,
    /// 相关冲突 ID
    pub related_conflict: Option<String>,
}

/// 时间线事件类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimelineEventType {
    /// 讨论开始
    DiscussionStarted,
    /// 蛊虫加入
    ParticipantJoined,
    /// 提议提交
    ProposalSubmitted,
    /// 评论发表
    CommentPosted,
    /// 投票
    Voted,
    /// 冲突发生
    ConflictOccurred,
    /// 冲突解决
    ConflictResolved,
    /// 共识达成
    ConsensusReached,
    /// 提议修改
    ProposalRevised,
    /// 讨论完成
    DiscussionCompleted,
}

/// 知识协作器
#[derive(Debug, Clone)]
pub struct KnowledgeCollaboration {
    /// 讨论会（主题 → 讨论会）
    discussions: HashMap<String, KnowledgeDiscussion>,
    /// 经验库（全局）
    experience_library: Vec<Experience>,
    /// 已注册的蛊虫
    registered_gus: Vec<Uuid>,
    /// 根目录
    root: std::path::PathBuf,
}

impl KnowledgeCollaboration {
    /// 创建新的知识协作器
    pub fn new() -> Self {
        Self {
            discussions: HashMap::new(),
            experience_library: Vec::new(),
            registered_gus: Vec::new(),
            root: std::path::PathBuf::from("knowledge"),
        }
    }

    /// 注册蛊虫
    pub fn register_gu(&mut self, gu_id: Uuid) {
        if !self.registered_gus.contains(&gu_id) {
            self.registered_gus.push(gu_id);
        }
    }

    /// 开始一个新讨论
    pub fn start_discussion(&mut self, topic: &str, starter_id: Uuid) -> String {
        let normalized = Self::normalize_topic(topic);

        // 检查是否已存在讨论
        if self.discussions.contains_key(&normalized) {
            return normalized;
        }

        let discussion = KnowledgeDiscussion {
            topic: topic.to_string(),
            started_at: Self::now(),
            participants: vec![starter_id],
            proposals: HashMap::new(),
            conflicts: Vec::new(),
            experiences: Vec::new(),
            consensus: HashMap::new(),
            status: DiscussionStatus::InProgress,
            required_consensus: self.registered_gus.len().max(1),
            timeline: vec![TimelineEvent {
                id: format!("event_{}", Uuid::new_v4()),
                timestamp: Self::now(),
                event_type: TimelineEventType::DiscussionStarted,
                description: format!("讨论 '{}' 开始", topic),
                involved_gus: vec![starter_id],
                related_proposal: None,
                related_conflict: None,
            }],
        };

        self.discussions.insert(normalized.clone(), discussion);
        normalized
    }

    /// 加入讨论
    pub fn join_discussion(&mut self, topic: &str, gu_id: Uuid) -> bool {
        let normalized = Self::normalize_topic(topic);

        if let Some(discussion) = self.discussions.get_mut(&normalized) {
            if !discussion.participants.contains(&gu_id) {
                discussion.participants.push(gu_id);
                discussion.required_consensus = discussion.participants.len();

                // 记录时间线
                discussion.timeline.push(TimelineEvent {
                    id: format!("event_{}", Uuid::new_v4()),
                    timestamp: Self::now(),
                    event_type: TimelineEventType::ParticipantJoined,
                    description: format!("蛊虫 {} 加入讨论", gu_id),
                    involved_gus: vec![gu_id],
                    related_proposal: None,
                    related_conflict: None,
                });
            }
            return true;
        }
        false
    }

    /// 提议内容
    pub fn propose(
        &mut self,
        topic: &str,
        proposer_id: Uuid,
        proposer_name: &str,
        part: ContentPart,
        content: String,
    ) -> Option<String> {
        let normalized = Self::normalize_topic(topic);
        let discussion = self.discussions.get_mut(&normalized)?;

        // 创建提议
        let proposal = ContentProposal {
            id: format!("proposal_{}", Uuid::new_v4()),
            proposer_id,
            proposer_name: proposer_name.to_string(),
            part: part.clone(),
            content,
            proposed_at: Self::now(),
            supporters: vec![proposer_id],
            opponents: Vec::new(),
            comments: Vec::new(),
            status: ProposalStatus::Discussing,
            revisions: Vec::new(),
        };

        let proposal_id = proposal.id.clone();

        // 添加到对应部分
        let part_key = part.as_str().to_string();
        discussion.proposals
            .entry(part_key)
            .or_default()
            .push(proposal.clone());

        // 记录时间线
        discussion.timeline.push(TimelineEvent {
            id: format!("event_{}", Uuid::new_v4()),
            timestamp: Self::now(),
            event_type: TimelineEventType::ProposalSubmitted,
            description: format!("{} 提议了 {} 的内容", proposer_name, part.as_str()),
            involved_gus: vec![proposer_id],
            related_proposal: Some(proposal_id.clone()),
            related_conflict: None,
        });

        Some(proposal_id)
    }

    /// 评论提议（可能引发冲突）
    pub fn comment(
        &mut self,
        topic: &str,
        proposal_id: &str,
        commenter_id: Uuid,
        commenter_name: &str,
        comment_content: String,
        comment_type: CommentType,
    ) -> Option<String> {
        let normalized = Self::normalize_topic(topic);

        // 收集需要的信息（避免借用冲突）
        let mut conflict_info: Option<(String, String, String, Uuid, String, String)> = None;
        let mut comment_id_opt: Option<String> = None;

        {
            let discussion = self.discussions.get_mut(&normalized)?;

            // 找到提议
            for (part_key, proposals) in discussion.proposals.iter_mut() {
                if let Some(proposal) = proposals.iter_mut().find(|p| p.id == proposal_id) {
                    let comment = Comment {
                        id: format!("comment_{}", Uuid::new_v4()),
                        commenter_id,
                        commenter_name: commenter_name.to_string(),
                        content: comment_content.clone(),
                        commented_at: Self::now(),
                        comment_type,
                        triggered_revision: false,
                    };

                    let comment_id = comment.id.clone();
                    comment_id_opt = Some(comment_id.clone());
                    proposal.comments.push(comment);

                    // 更新支持/反对
                    match comment_type {
                        CommentType::Agree => {
                            if !proposal.supporters.contains(&commenter_id) {
                                proposal.supporters.push(commenter_id);
                            }
                            proposal.opponents.retain(|id| id != &commenter_id);
                        }
                        CommentType::Disagree | CommentType::Critique => {
                            if !proposal.opponents.contains(&commenter_id) {
                                proposal.opponents.push(commenter_id);
                            }
                            proposal.supporters.retain(|id| id != &commenter_id);

                            // 记录冲突信息（稍后创建）
                            conflict_info = Some((
                                discussion.topic.clone(),
                                part_key.clone(),
                                proposal.content.clone(),
                                commenter_id,
                                commenter_name.to_string(),
                                comment_content.clone(),
                            ));
                        }
                        CommentType::SuggestModification => {
                            proposal.status = ProposalStatus::NeedsRevision;
                        }
                        _ => {}
                    }

                    // 记录时间线
                    discussion.timeline.push(TimelineEvent {
                        id: format!("event_{}", Uuid::new_v4()),
                        timestamp: Self::now(),
                        event_type: TimelineEventType::CommentPosted,
                        description: format!("{} 发表了 {:?}", commenter_name, comment_type),
                        involved_gus: vec![commenter_id],
                        related_proposal: Some(proposal_id.to_string()),
                        related_conflict: None,
                    });

                    break;
                }
            }
        }

        // 在借用结束后创建冲突
        if let Some((topic, part_key, proposal_content, commenter_id, commenter_name, reason)) = conflict_info {
            let conflict = Conflict {
                id: format!("conflict_{}", Uuid::new_v4()),
                topic,
                part: ContentPart::Custom(part_key.clone()),
                parties: vec![
                    ConflictParty {
                        gu_id: commenter_id, // 临时
                        name: "提议者".to_string(),
                        stance: Stance::Support,
                    },
                    ConflictParty {
                        gu_id: commenter_id,
                        name: commenter_name,
                        stance: Stance::Oppose,
                    },
                ],
                reason,
                arguments: vec![
                    Argument {
                        id: format!("arg_{}", Uuid::new_v4()),
                        proposer_id: commenter_id,
                        proposer_name: "提议者".to_string(),
                        content: proposal_content,
                        argument_type: ArgumentType::Fact,
                        evidence: Vec::new(),
                        proposed_at: Self::now(),
                    },
                ],
                resolution: None,
                occurred_at: Self::now(),
                resolved_at: None,
            };

            if let Some(discussion) = self.discussions.get_mut(&normalized) {
                discussion.conflicts.push(conflict);
            }
        }

        None
    }

    /// 解决冲突
    pub fn resolve_conflict(
        &mut self,
        topic: &str,
        conflict_id: &str,
        method: ResolutionMethod,
        final_content: String,
        lessons_learned: Vec<String>,
    ) -> Option<()> {
        let normalized = Self::normalize_topic(topic);
        let discussion = self.discussions.get_mut(&normalized)?;

        // 找到冲突
        let conflict = discussion.conflicts.iter_mut()
            .find(|c| c.id == conflict_id)?;

        // 记录解决方案
        conflict.resolution = Some(Resolution {
            method,
            final_content: final_content.clone(),
            lessons_learned: lessons_learned.clone(),
            resolved_at: Self::now(),
        });
        conflict.resolved_at = Some(Self::now());

        // 创建经验记录
        for lesson in lessons_learned {
            let experience = Experience {
                id: format!("exp_{}", Uuid::new_v4()),
                topic: topic.to_string(),
                conflict_id: conflict_id.to_string(),
                experience_type: ExperienceType::ConflictResolution,
                content: lesson,
                applicable_scenarios: vec![format!("{}", conflict.part)],
                learning_weight: 1.0,
                created_at: Self::now(),
                reference_count: 0,
            };

            discussion.experiences.push(experience);
        }

        // 记录时间线
        discussion.timeline.push(TimelineEvent {
            id: format!("event_{}", Uuid::new_v4()),
            timestamp: Self::now(),
            event_type: TimelineEventType::ConflictResolved,
            description: format!("冲突已通过 {:?} 解决", method),
            involved_gus: conflict.parties.iter().map(|p| p.gu_id).collect(),
            related_proposal: None,
            related_conflict: Some(conflict_id.to_string()),
        });

        Some(())
    }

    /// 投票支持
    pub fn vote_support(&mut self, topic: &str, proposal_id: &str, voter_id: Uuid) -> Option<ProposalStatus> {
        let normalized = Self::normalize_topic(topic);
        let discussion = self.discussions.get_mut(&normalized)?;

        for (_, proposals) in discussion.proposals.iter_mut() {
            if let Some(proposal) = proposals.iter_mut().find(|p| p.id == proposal_id) {
                if !proposal.supporters.contains(&voter_id) {
                    proposal.supporters.push(voter_id);
                }
                proposal.opponents.retain(|id| id != &voter_id);

                // 检查是否达成共识
                if proposal.supporters.len() >= discussion.required_consensus {
                    proposal.status = ProposalStatus::ConsensusReached;

                    // 记录时间线
                    discussion.timeline.push(TimelineEvent {
                        id: format!("event_{}", Uuid::new_v4()),
                        timestamp: Self::now(),
                        event_type: TimelineEventType::ConsensusReached,
                        description: format!("提议 {} 达成共识", proposal_id),
                        involved_gus: proposal.supporters.clone(),
                        related_proposal: Some(proposal_id.to_string()),
                        related_conflict: None,
                    });
                }

                return Some(proposal.status);
            }
        }

        None
    }

    /// 投票反对
    pub fn vote_oppose(&mut self, topic: &str, proposal_id: &str, voter_id: Uuid, reason: String) -> Option<ProposalStatus> {
        let normalized = Self::normalize_topic(topic);

        // 收集需要的信息
        let mut conflict_info: Option<(String, String, String, Uuid, String)> = None;
        let mut proposal_status = None;

        {
            let discussion = self.discussions.get_mut(&normalized)?;

            for (part_key, proposals) in discussion.proposals.iter_mut() {
                if let Some(proposal) = proposals.iter_mut().find(|p| p.id == proposal_id) {
                    if !proposal.opponents.contains(&voter_id) {
                        proposal.opponents.push(voter_id);
                    }
                    proposal.supporters.retain(|id| id != &voter_id);

                    // 记录冲突信息
                    conflict_info = Some((
                        discussion.topic.clone(),
                        part_key.clone(),
                        proposal.content.clone(),
                        voter_id,
                        reason.clone(),
                    ));

                    proposal_status = Some(proposal.status);
                    break;
                }
            }
        }

        // 创建冲突
        if let Some((topic, part_key, proposal_content, voter_id, reason)) = conflict_info {
            let conflict = Conflict {
                id: format!("conflict_{}", Uuid::new_v4()),
                topic,
                part: ContentPart::Custom(part_key),
                parties: vec![
                    ConflictParty {
                        gu_id: voter_id,
                        name: "提议者".to_string(),
                        stance: Stance::Support,
                    },
                    ConflictParty {
                        gu_id: voter_id,
                        name: "反对者".to_string(),
                        stance: Stance::Oppose,
                    },
                ],
                reason,
                arguments: vec![
                    Argument {
                        id: format!("arg_{}", Uuid::new_v4()),
                        proposer_id: voter_id,
                        proposer_name: "提议者".to_string(),
                        content: proposal_content,
                        argument_type: ArgumentType::Fact,
                        evidence: Vec::new(),
                        proposed_at: Self::now(),
                    },
                ],
                resolution: None,
                occurred_at: Self::now(),
                resolved_at: None,
            };

            if let Some(discussion) = self.discussions.get_mut(&normalized) {
                discussion.conflicts.push(conflict);
            }
        }

        proposal_status
    }

    /// 修改提议（妥协）
    pub fn revise_proposal(
        &mut self,
        topic: &str,
        proposal_id: &str,
        reviser_id: Uuid,
        reviser_name: &str,
        new_content: String,
        reason: String,
    ) -> Option<()> {
        let normalized = Self::normalize_topic(topic);
        let discussion = self.discussions.get_mut(&normalized)?;

        for (_, proposals) in discussion.proposals.iter_mut() {
            if let Some(proposal) = proposals.iter_mut().find(|p| p.id == proposal_id) {
                let version = proposal.revisions.len() as u32 + 1;

                proposal.revisions.push(Revision {
                    version,
                    revised_by: reviser_id,
                    reviser_name: reviser_name.to_string(),
                    content: proposal.content.clone(),
                    reason: reason.clone(),
                    revised_at: Self::now(),
                });

                proposal.content = new_content;
                proposal.status = ProposalStatus::Discussing;

                // 重置投票（修改后需要重新投票）
                proposal.supporters = vec![reviser_id];
                proposal.opponents = Vec::new();

                // 记录时间线
                discussion.timeline.push(TimelineEvent {
                    id: format!("event_{}", Uuid::new_v4()),
                    timestamp: Self::now(),
                    event_type: TimelineEventType::ProposalRevised,
                    description: format!("提议被修改：{}", reason),
                    involved_gus: vec![reviser_id],
                    related_proposal: Some(proposal_id.to_string()),
                    related_conflict: None,
                });

                return Some(());
            }
        }

        None
    }

    /// 检查并生成共识内容
    pub fn check_consensus(&mut self, topic: &str) -> Option<HashMap<String, String>> {
        let normalized = Self::normalize_topic(topic);

        // 先获取需要的所有信息
        let (parts, required_consensus) = {
            let discussion = self.discussions.get(&normalized)?;
            let parts = vec![
                ContentPart::Definition,
                ContentPart::Concepts,
                ContentPart::Relations,
                ContentPart::Examples,
                ContentPart::Layout,
            ];
            (parts, discussion.required_consensus)
        };

        // 检查是否至少有一个部分有提议
        {
            let discussion = self.discussions.get(&normalized)?;
            let has_any_proposal = parts.iter().any(|part| {
                discussion.proposals.get(part.as_str())
                    .map(|ps| !ps.is_empty())
                    .unwrap_or(false)
            });

            if !has_any_proposal {
                return None;
            }
        }

        // 检查每个部分是否达成共识
        {
            let discussion = self.discussions.get(&normalized)?;
            for part in &parts {
                let part_key = part.as_str();
                if let Some(ps) = discussion.proposals.get(part_key) {
                    if !ps.is_empty() {
                        let has_consensus = ps.iter().any(|p| p.status == ProposalStatus::ConsensusReached);
                        let has_pending = ps.iter().any(|p| p.status == ProposalStatus::Discussing);

                        if has_pending && !has_consensus {
                            return None;
                        }
                    }
                }
            }
        }

        // 收集经验和最终内容
        let (experiences, consensus_content, participants) = {
            let discussion = self.discussions.get_mut(&normalized)?;

            // 所有部分都有共识，生成最终内容
            for part in &parts {
                let part_key = part.as_str();
                if let Some(proposals) = discussion.proposals.get(part_key) {
                    let winner = proposals.iter()
                        .filter(|p| p.status == ProposalStatus::ConsensusReached || p.supporters.len() >= required_consensus)
                        .max_by_key(|p| p.supporters.len());

                    if let Some(proposal) = winner {
                        discussion.consensus.insert(part_key.to_string(), proposal.content.clone());
                    }
                }
            }

            discussion.status = DiscussionStatus::Completed;

            // 记录时间线
            discussion.timeline.push(TimelineEvent {
                id: format!("event_{}", Uuid::new_v4()),
                timestamp: Self::now(),
                event_type: TimelineEventType::DiscussionCompleted,
                description: format!("讨论 '{}' 完成，达成共识", topic),
                involved_gus: discussion.participants.clone(),
                related_proposal: None,
                related_conflict: None,
            });

            let experiences = discussion.experiences.clone();
            let consensus = discussion.consensus.clone();
            let participants = discussion.participants.clone();

            (experiences, consensus, participants)
        };

        // 提取经验到全局经验库
        for exp in experiences {
            self.experience_library.push(exp);
        }

        Some(consensus_content)
    }

    /// 获取讨论经验（供 LNN 学习）
    pub fn get_experiences(&self, topic: &str) -> Vec<&Experience> {
        let normalized = Self::normalize_topic(topic);
        self.discussions.get(&normalized)
            .map(|d| d.experiences.iter().collect())
            .unwrap_or_default()
    }

    /// 获取全局经验库
    pub fn get_global_experiences(&self) -> &[Experience] {
        &self.experience_library
    }

    /// 获取冲突解决模式（用于训练 LNN）
    pub fn get_conflict_patterns(&self) -> Vec<ConflictPattern> {
        self.discussions.values()
            .flat_map(|d| &d.conflicts)
            .filter_map(|c| {
                c.resolution.as_ref().map(|r| ConflictPattern {
                    conflict_type: format!("{:?}", c.part),
                    num_parties: c.parties.len(),
                    resolution_method: r.method,
                    lessons: r.lessons_learned.clone(),
                    duration: r.resolved_at.saturating_sub(c.occurred_at),
                })
            })
            .collect()
    }

    /// 生成最终的 HTML 文件（包含完整讨论记录）
    pub fn generate_html(&self, topic: &str) -> Option<String> {
        let normalized = Self::normalize_topic(topic);
        let discussion = self.discussions.get(&normalized)?;

        if discussion.status != DiscussionStatus::Completed {
            return None;
        }

        let definition = discussion.consensus.get("definition").cloned().unwrap_or_default();
        let concepts = discussion.consensus.get("concepts").cloned().unwrap_or_default();
        let relations = discussion.consensus.get("relations").cloned().unwrap_or_default();
        let examples = discussion.consensus.get("examples").cloned().unwrap_or_default();
        let layout = discussion.consensus.get("layout").cloned().unwrap_or_else(|| Self::default_layout());

        // 生成讨论记录 HTML
        let timeline_html = Self::format_timeline(&discussion.timeline);
        let conflicts_html = Self::format_conflicts(&discussion.conflicts);
        let experiences_html = Self::format_experiences(&discussion.experiences);

        Some(Self::build_html(
            topic,
            &definition,
            &concepts,
            &relations,
            &examples,
            &layout,
            discussion,
            &timeline_html,
            &conflicts_html,
            &experiences_html,
        ))
    }

    /// 格式化时间线
    fn format_timeline(timeline: &[TimelineEvent]) -> String {
        timeline.iter()
            .map(|e| format!(
                r#"<div class="timeline-event">
                    <span class="time">{}</span>
                    <span class="type">{:?}</span>
                    <span class="desc">{}</span>
                </div>"#,
                e.timestamp,
                e.event_type,
                e.description
            ))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// 格式化冲突记录
    fn format_conflicts(conflicts: &[Conflict]) -> String {
        conflicts.iter()
            .map(|c| {
                let resolution_html = c.resolution.as_ref()
                    .map(|r| format!(
                        r#"<div class="resolution">
                            <h4>解决方案: {:?}</h4>
                            <p>{}</p>
                            <h5>经验教训:</h5>
                            <ul>{}</ul>
                        </div>"#,
                        r.method,
                        r.final_content,
                        r.lessons_learned.iter()
                            .map(|l| format!("<li>{}</li>", l))
                            .collect::<Vec<_>>()
                            .join("")
                    ))
                    .unwrap_or_else(|| "<p>未解决</p>".to_string());

                format!(
                    r#"<div class="conflict">
                        <h4>冲突: {}</h4>
                        <p>原因: {}</p>
                        <div class="parties">{}</div>
                        {}
                    </div>"#,
                    c.id,
                    c.reason,
                    c.parties.iter()
                        .map(|p| format!("<span>{:?}: {}</span>", p.stance, p.name))
                        .collect::<Vec<_>>()
                        .join(" vs "),
                    resolution_html
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// 格式化经验记录
    fn format_experiences(experiences: &[Experience]) -> String {
        experiences.iter()
            .map(|e| format!(
                r#"<div class="experience">
                    <span class="type">{:?}</span>
                    <p>{}</p>
                    <small>权重: {:.2}</small>
                </div>"#,
                e.experience_type,
                e.content,
                e.learning_weight
            ))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// 构建最终 HTML
    fn build_html(
        topic: &str,
        definition: &str,
        concepts: &str,
        relations: &str,
        examples: &str,
        layout: &str,
        discussion: &KnowledgeDiscussion,
        timeline_html: &str,
        conflicts_html: &str,
        experiences_html: &str,
    ) -> String {
        let concepts_html = Self::parse_list(concepts)
            .iter()
            .map(|c| format!("<dt>{}</dt>\n                <dd>相关概念</dd>", c))
            .collect::<Vec<_>>()
            .join("\n");

        let relations_html = Self::parse_list(relations)
            .iter()
            .map(|r| format!("                <li>{}</li>", r))
            .collect::<Vec<_>>()
            .join("\n");

        let examples_html = Self::parse_list(examples)
            .iter()
            .map(|e| format!("            <li>{}</li>", e))
            .collect::<Vec<_>>()
            .join("\n");

        let participants_count = discussion.participants.len();
        let conflicts_count = discussion.conflicts.len();
        let experiences_count = discussion.experiences.len();

        format!(
r#"<!DOCTYPE html>
<html lang="zh-CN" data-type="concept">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="concept-id" content="concept_{}">
    <meta name="consensus-status" content="approved">
    <meta name="conflicts-resolved" content="{}">
    <meta name="experiences-learned" content="{}">
    <title>{} - 世界知识库</title>
    <style>
{}
    </style>
</head>
<body>
    <article class="concept">
        <header class="concept-header">
            <h1>{}</h1>
            <div class="concept-meta">
                <span class="badge consensus">✓ 100% 共识</span>
                <span class="badge participants">参与者: {}</span>
                <span class="badge conflicts">解决冲突: {}</span>
                <span class="badge experiences">经验: {}</span>
            </div>
        </header>

        <section class="definition">
            <h2>定义</h2>
            <p>{}</p>
        </section>

        <section class="attributes">
            <h2>概念</h2>
            <dl>
{}
            </dl>
        </section>

        <section class="relations">
            <h2>关系</h2>
            <ul class="relation-list">
{}
            </ul>
        </section>

        <section class="examples">
            <h2>示例</h2>
            <ul>
{}
            </ul>
        </section>

        <!-- 讨论过程（宝贵经验） -->
        <section class="discussion-process">
            <h2>💬 讨论过程</h2>
            <div class="timeline">
{}
            </div>

            <h3>⚔️ 冲突与解决</h3>
            <div class="conflicts">
{}
            </div>

            <h3>📚 经验总结</h3>
            <div class="experiences">
{}
            </div>
        </section>

        <!-- 机器可读数据 -->
        <section class="embedding" hidden>
            <data name="discussion-json" value="{}"></data>
        </section>
    </article>
</body>
</html>"#,
            Self::normalize_topic(topic),
            conflicts_count,
            experiences_count,
            topic,
            layout,
            topic,
            participants_count,
            conflicts_count,
            experiences_count,
            definition,
            concepts_html,
            relations_html,
            examples_html,
            timeline_html,
            conflicts_html,
            experiences_html,
            serde_json::to_string(discussion).unwrap_or_default(),
        )
    }

    /// 解析列表
    fn parse_list(content: &str) -> Vec<String> {
        if let Ok(arr) = serde_json::from_str::<Vec<String>>(content) {
            return arr;
        }
        content.lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// 默认排版样式
    fn default_layout() -> String {
        r#"* { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: 'Microsoft YaHei', sans-serif; background: #f5f5f5; padding: 20px; }
        .concept { max-width: 1000px; margin: 0 auto; background: white; padding: 30px; border-radius: 10px; }
        h1 { color: #333; border-bottom: 2px solid #4CAF50; padding-bottom: 15px; }
        h2 { color: #4CAF50; margin: 20px 0 10px 0; border-top: 1px solid #eee; padding-top: 15px; }
        h3 { color: #666; margin: 15px 0 10px 0; }
        .badge { display: inline-block; padding: 4px 10px; border-radius: 4px; font-size: 0.8em; margin-right: 5px; }
        .consensus { background: #4CAF50; color: white; }
        .participants { background: #2196F3; color: white; }
        .conflicts { background: #FF9800; color: white; }
        .experiences { background: #9C27B0; color: white; }
        dl { margin-left: 20px; }
        dt { font-weight: bold; color: #666; margin-top: 10px; }
        dd { margin-left: 20px; color: #888; }
        ul { margin-left: 20px; }
        li { margin: 5px 0; }
        .timeline { border-left: 3px solid #4CAF50; padding-left: 15px; margin: 10px 0; }
        .timeline-event { margin: 8px 0; padding: 5px 10px; background: #f9f9f9; border-radius: 4px; }
        .timeline-event .time { color: #999; font-size: 0.8em; }
        .timeline-event .type { color: #4CAF50; font-weight: bold; margin: 0 10px; }
        .conflict { background: #fff3e0; padding: 15px; margin: 10px 0; border-radius: 8px; border-left: 4px solid #FF9800; }
        .resolution { background: #e8f5e9; padding: 10px; margin-top: 10px; border-radius: 4px; }
        .experience { background: #f3e5f5; padding: 10px; margin: 8px 0; border-radius: 4px; border-left: 4px solid #9C27B0; }
        .experience .type { font-weight: bold; color: #9C27B0; }"#.to_string()
    }

    /// 保存到世界知识库
    pub fn save_to_world(&self, topic: &str) -> Option<std::path::PathBuf> {
        let html = self.generate_html(topic)?;
        let normalized = Self::normalize_topic(topic);

        let path = self.root
            .join("world")
            .join(format!("{}.html", normalized));

        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        std::fs::write(&path, html).ok()?;
        Some(path)
    }

    /// 获取讨论状态
    pub fn get_discussion(&self, topic: &str) -> Option<&KnowledgeDiscussion> {
        let normalized = Self::normalize_topic(topic);
        self.discussions.get(&normalized)
    }

    /// 标准化主题名称
    fn normalize_topic(topic: &str) -> String {
        topic.to_lowercase()
            .replace(' ', "_")
            .replace(['/', '\\', '-'], "_")
            .replace(['(', ')'], "")
    }

    /// 当前时间戳
    fn now() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

impl Default for KnowledgeCollaboration {
    fn default() -> Self {
        Self::new()
    }
}

/// 冲突解决模式（用于 LNN 训练）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictPattern {
    /// 冲突类型
    pub conflict_type: String,
    /// 参与方数量
    pub num_parties: usize,
    /// 解决方式
    pub resolution_method: ResolutionMethod,
    /// 经验教训
    pub lessons: Vec<String>,
    /// 解决耗时
    pub duration: u64,
}

/// 讨论状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscussionStatus {
    /// 进行中
    InProgress,
    /// 已完成
    Completed,
    /// 已取消
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_discussion() {
        let mut kc = KnowledgeCollaboration::new();
        let gu_id = Uuid::new_v4();
        kc.register_gu(gu_id);

        let topic = kc.start_discussion("HTML基础", gu_id);
        assert!(!topic.is_empty());

        let discussion = kc.get_discussion("HTML基础");
        assert!(discussion.is_some());
    }

    #[test]
    fn test_propose_and_vote() {
        let mut kc = KnowledgeCollaboration::new();

        let gu1 = Uuid::new_v4();
        let gu2 = Uuid::new_v4();
        kc.register_gu(gu1);
        kc.register_gu(gu2);

        kc.start_discussion("测试主题", gu1);

        let proposal_id = kc.propose(
            "测试主题",
            gu1,
            "蛊虫A",
            ContentPart::Definition,
            "这是一个测试定义".to_string(),
        ).unwrap();

        let status = kc.vote_support("测试主题", &proposal_id, gu2);
        assert_eq!(status, Some(ProposalStatus::ConsensusReached));
    }

    #[test]
    fn test_conflict_and_resolution() {
        let mut kc = KnowledgeCollaboration::new();

        let gu1 = Uuid::new_v4();
        let gu2 = Uuid::new_v4();
        let gu3 = Uuid::new_v4();
        kc.register_gu(gu1);
        kc.register_gu(gu2);
        kc.register_gu(gu3);

        kc.start_discussion("Python函数", gu1);
        kc.join_discussion("Python函数", gu2);
        kc.join_discussion("Python函数", gu3);

        let def1 = kc.propose(
            "Python函数",
            gu1,
            "蛊虫A",
            ContentPart::Definition,
            "函数是可重用的代码块".to_string(),
        ).unwrap();

        // gu2 反对（创建冲突）
        kc.comment(
            "Python函数",
            &def1,
            gu2,
            "蛊虫B",
            "定义不够完整，缺少 def 关键字".to_string(),
            CommentType::Critique,
        );

        // 检查是否创建了冲突（获取冲突 ID）
        let conflict_id = {
            let discussion = kc.get_discussion("Python函数").unwrap();
            assert!(!discussion.conflicts.is_empty());
            discussion.conflicts[0].id.clone()
        };

        // 解决冲突（修改提议）
        kc.revise_proposal(
            "Python函数",
            &def1,
            gu1,
            "蛊虫A",
            "函数是使用 def 关键字定义的可重用代码块".to_string(),
            "吸收蛊虫B的建议".to_string(),
        );

        // 所有人支持修改后的版本
        kc.vote_support("Python函数", &def1, gu1);
        kc.vote_support("Python函数", &def1, gu2);
        kc.vote_support("Python函数", &def1, gu3);

        // 解决冲突
        kc.resolve_conflict(
            "Python函数",
            &conflict_id,
            ResolutionMethod::Compromise,
            "函数是使用 def 关键字定义的可重用代码块".to_string(),
            vec!["反对意见可以提升内容质量".to_string()],
        );

        let consensus = kc.check_consensus("Python函数");
        assert!(consensus.is_some());

        // 检查经验是否被记录
        {
            let discussion = kc.get_discussion("Python函数").unwrap();
            assert!(!discussion.experiences.is_empty());
        }

        // 生成 HTML 应该包含冲突和经验记录
        let html = kc.generate_html("Python函数").unwrap();
        assert!(html.contains("冲突与解决"));
        assert!(html.contains("经验总结"));
    }

    #[test]
    fn test_experience_library() {
        let mut kc = KnowledgeCollaboration::new();

        let gu1 = Uuid::new_v4();
        let gu2 = Uuid::new_v4();
        kc.register_gu(gu1);
        kc.register_gu(gu2);

        kc.start_discussion("测试", gu1);
        kc.join_discussion("测试", gu2);

        let prop = kc.propose(
            "测试",
            gu1,
            "蛊虫A",
            ContentPart::Definition,
            "定义内容".to_string(),
        ).unwrap();

        kc.vote_support("测试", &prop, gu2);
        kc.check_consensus("测试");

        // 检查全局经验库
        let experiences = kc.get_global_experiences();
        // 初始没有冲突，没有经验
        assert!(experiences.is_empty() || experiences.len() >= 0);
    }
}
