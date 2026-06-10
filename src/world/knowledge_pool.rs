//! 知识池 - 候选知识管理与共识机制
//!
//! # 设计理念
//!
//! - 黑塔：竞争涌现最佳知识
//! - 螺丝咕姆：共识验证确保准确性
//! - 拉蒂奥：统一版本避免重复
//!
//! # 核心思想
//!
//! 所有蛊虫学习同一主题的知识时，先提交候选版本，
//! 通过投票验证后，统一存入共享知识库。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use super::knowledge_storage::{SkillStorage, SkillDocument};

/// 知识池
pub struct KnowledgePool {
    /// 候选知识（按主题分组）
    candidates: HashMap<String, Vec<CandidateKnowledge>>,
    /// 共享知识存储
    storage: SkillStorage,
    /// 共识阈值（需要多少票通过）
    consensus_threshold: usize,
    /// 知识 ID 到主题的映射
    id_to_topic: HashMap<String, String>,
}

/// 候选知识
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateKnowledge {
    /// 候选 ID
    pub id: String,
    /// 提交者蛊虫 ID
    pub submitted_by: Uuid,
    /// 提交者名称
    pub submitter_name: String,
    /// 提交时间戳
    pub submitted_at: u64,
    /// 知识内容
    pub knowledge: SkillDocument,
    /// 投票记录
    pub votes: Vec<VoteRecord>,
    /// 状态
    pub status: CandidateStatus,
    /// 综合分数
    pub score: f64,
}

/// 投票记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteRecord {
    /// 投票者蛊虫 ID
    pub voter_id: Uuid,
    /// 投票者名称
    pub voter_name: String,
    /// 是否同意
    pub approve: bool,
    /// 置信度 [0, 1]
    pub confidence: f64,
    /// 投票时间戳
    pub voted_at: u64,
    /// 投票理由
    pub reason: String,
}

/// 候选状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CandidateStatus {
    /// 等待投票
    Pending,
    /// 达成共识，待提升
    ConsensusReached,
    /// 已提升到共享库
    Promoted,
    /// 被拒绝
    Rejected,
}

/// 学习结果
#[derive(Debug, Clone)]
pub enum LearningResult {
    /// 新知识已提交候选
    CandidateSubmitted {
        topic: String,
        candidate_id: String,
        needs_votes: usize,
    },
    /// 已存在共享知识，强化学习
    ExistingReinforced {
        topic: String,
        ref_count: u32,
    },
    /// 竞争学习（其他蛊虫已提交）
    Competing {
        topic: String,
        candidate_count: usize,
    },
    /// 共识达成，知识入库
    ConsensusReached {
        topic: String,
        skill_id: String,
    },
}

impl KnowledgePool {
    /// 创建新的知识池
    pub fn new() -> Self {
        Self {
            candidates: HashMap::new(),
            storage: SkillStorage::new(),
            consensus_threshold: 3, // 默认需要 3 票
            id_to_topic: HashMap::new(),
        }
    }

    /// 设置共识阈值
    pub fn with_threshold(mut self, threshold: usize) -> Self {
        self.consensus_threshold = threshold;
        self
    }

    /// 检查共享知识库是否已存在该主题
    pub fn shared_exists(&self, topic: &str) -> bool {
        // 标准化主题名称
        let normalized = Self::normalize_topic(topic);

        // 尝试读取共享技能
        let skill_id = format!("skill_{}", normalized.replace(' ', "_").to_lowercase());
        self.storage.read_skill(&skill_id).is_ok()
    }

    /// 增加共享知识的引用计数
    pub fn increment_ref(&mut self, topic: &str) -> u32 {
        let normalized = Self::normalize_topic(topic);
        let skill_id = format!("skill_{}", normalized.replace(' ', "_").to_lowercase());

        if let Ok(mut skill) = self.storage.read_skill(&skill_id) {
            skill.ref_count += 1;
            let _ = self.storage.save_shared_skill(&skill);
            skill.ref_count
        } else {
            0
        }
    }

    /// 提交候选知识
    pub fn submit_candidate(
        &mut self,
        gu_id: Uuid,
        gu_name: &str,
        knowledge: SkillDocument,
    ) -> LearningResult {
        let topic = knowledge.name.clone();
        let normalized = Self::normalize_topic(&topic);

        // 检查是否已存在共享版本
        if self.shared_exists(&topic) {
            let ref_count = self.increment_ref(&topic);
            return LearningResult::ExistingReinforced { topic, ref_count };
        }

        // 创建候选
        let candidate = CandidateKnowledge {
            id: format!("candidate_{}", Uuid::new_v4()),
            submitted_by: gu_id,
            submitter_name: gu_name.to_string(),
            submitted_at: Self::current_timestamp(),
            knowledge,
            votes: Vec::new(),
            status: CandidateStatus::Pending,
            score: 0.0,
        };

        let candidate_id = candidate.id.clone();

        // 添加到候选池
        let candidates = self.candidates.entry(normalized.clone()).or_default();
        let candidate_count = candidates.len() + 1;
        candidates.push(candidate);

        // 返回结果
        if candidate_count == 1 {
            LearningResult::CandidateSubmitted {
                topic,
                candidate_id,
                needs_votes: self.consensus_threshold,
            }
        } else {
            LearningResult::Competing {
                topic,
                candidate_count,
            }
        }
    }

    /// 投票
    pub fn vote(
        &mut self,
        topic: &str,
        voter_id: Uuid,
        voter_name: &str,
        candidate_id: &str,
        approve: bool,
        confidence: f64,
        reason: String,
    ) -> Option<CandidateStatus> {
        let normalized = Self::normalize_topic(topic);
        let candidates = self.candidates.get_mut(&normalized)?;

        // 找到候选
        let candidate = candidates.iter_mut()
            .find(|c| c.id == candidate_id)?;

        // 添加投票
        candidate.votes.push(VoteRecord {
            voter_id,
            voter_name: voter_name.to_string(),
            approve,
            confidence,
            voted_at: Self::current_timestamp(),
            reason,
        });

        // 更新分数
        if approve {
            candidate.score += confidence;
        }

        // 检查是否达成共识
        let approve_count = candidate.votes.iter().filter(|v| v.approve).count();
        if approve_count >= self.consensus_threshold {
            candidate.status = CandidateStatus::ConsensusReached;
        }

        Some(candidate.status)
    }

    /// 检查并提升达成共识的知识
    pub fn check_and_promote(&mut self, topic: &str) -> Option<String> {
        let normalized = Self::normalize_topic(topic);
        let candidates = self.candidates.get_mut(&normalized)?;

        // 找到达成共识的候选（分数最高的）
        let winner_idx = candidates.iter()
            .enumerate()
            .filter(|(_, c)| c.status == CandidateStatus::ConsensusReached)
            .max_by(|(_, a), (_, b)| a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)?;

        // 提升到共享库
        let mut winner = candidates.remove(winner_idx);
        winner.status = CandidateStatus::Promoted;

        let skill_id = winner.knowledge.id.clone();
        let _ = self.storage.save_shared_skill(&winner.knowledge);

        // 清理其他候选
        candidates.clear();
        self.candidates.remove(&normalized);

        Some(skill_id)
    }

    /// 获取候选列表
    pub fn get_candidates(&self, topic: &str) -> Option<&Vec<CandidateKnowledge>> {
        let normalized = Self::normalize_topic(topic);
        self.candidates.get(&normalized)
    }

    /// 获取所有待投票的主题
    pub fn get_pending_topics(&self) -> Vec<String> {
        self.candidates.iter()
            .filter(|(_, candidates)| {
                candidates.iter().any(|c| c.status == CandidateStatus::Pending)
            })
            .map(|(topic, _)| topic.clone())
            .collect()
    }

    /// 标准化主题名称
    fn normalize_topic(topic: &str) -> String {
        topic.to_lowercase()
            .replace(' ', "_")
            .replace(['/', '\\'], "_")
    }

    /// 获取当前时间戳
    fn current_timestamp() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

impl Default for KnowledgePool {
    fn default() -> Self {
        Self::new()
    }
}

/// 知识投票评估器（三天才裁决）
pub struct KnowledgeEvaluator;

impl KnowledgeEvaluator {
    /// 评估候选知识（三天才裁决）
    pub fn evaluate(candidate: &CandidateKnowledge) -> VoteRecord {
        let knowledge = &candidate.knowledge;

        // 黑塔：创新性评估（知识粒子数量、代码语言数量）
        let innovation_score = Self::evaluate_innovation(knowledge);

        // 螺丝咕姆：准确性评估（概念一致性、定义完整度）
        let accuracy_score = Self::evaluate_accuracy(knowledge);

        // 拉蒂奥：优雅度评估（结构清晰度、关键词质量）
        let elegance_score = Self::evaluate_elegance(knowledge);

        // 综合裁决（准确性权重最高）
        let total = innovation_score * 0.25
                  + accuracy_score * 0.50
                  + elegance_score * 0.25;

        VoteRecord {
            voter_id: Uuid::nil(), // 系统评估
            voter_name: "三天才裁决".to_string(),
            approve: total > 0.5,
            confidence: total,
            voted_at: KnowledgePool::new().storage.read_skill(&knowledge.id).map(|_| 0).unwrap_or(0),
            reason: format!(
                "创新:{:.2} 准确:{:.2} 优雅:{:.2} = 总分:{:.2}",
                innovation_score, accuracy_score, elegance_score, total
            ),
        }
    }

    /// 评估创新性
    fn evaluate_innovation(knowledge: &SkillDocument) -> f64 {
        let mut score = 0.0_f64;

        // 有概念数量
        score += (knowledge.concepts.len() as f64 / 10.0).min(0.3_f64);

        // 有定义
        if !knowledge.definition.is_empty() {
            score += 0.3_f64;
        }

        // 有知识粒子
        if knowledge.particles.is_some() {
            score += 0.4_f64;
        }

        score.min(1.0_f64)
    }

    /// 评估准确性
    fn evaluate_accuracy(knowledge: &SkillDocument) -> f64 {
        let mut score = 0.0_f64;

        // 定义词数（合理的长度）
        let def_len = knowledge.definition.len();
        if def_len > 50 && def_len < 5000 {
            score += 0.3_f64;
        } else if def_len > 0 {
            score += 0.1_f64;
        }

        // 概念数量合理
        if knowledge.concepts.len() >= 3 && knowledge.concepts.len() <= 10 {
            score += 0.3_f64;
        } else if !knowledge.concepts.is_empty() {
            score += 0.1_f64;
        }

        // 有共识状态
        if knowledge.consensus_status == "approved" {
            score += 0.4_f64;
        } else if knowledge.consensus_status == "pending" {
            score += 0.2_f64;
        }

        score.min(1.0_f64)
    }

    /// 评估优雅度
    fn evaluate_elegance(knowledge: &SkillDocument) -> f64 {
        let mut score = 0.0_f64;

        // 名称简洁
        if knowledge.name.len() <= 30 {
            score += 0.3_f64;
        }

        // 有关系链接
        if !knowledge.relations.is_empty() {
            score += 0.3_f64;
        }

        // 有学习者
        if !knowledge.learners.is_empty() {
            score += 0.4_f64;
        }

        score.min(1.0_f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submit_candidate() {
        let mut pool = KnowledgePool::new();
        let gu_id = Uuid::new_v4();

        let skill = SkillDocument::new("test_skill", "测试技能", gu_id);

        let result = pool.submit_candidate(gu_id, "测试蛊虫", skill);

        match result {
            LearningResult::CandidateSubmitted { topic, needs_votes, .. } => {
                assert_eq!(topic, "测试技能");
                assert_eq!(needs_votes, 3);
            }
            _ => panic!("应该是 CandidateSubmitted"),
        }
    }

    #[test]
    fn test_vote_and_consensus() {
        let mut pool = KnowledgePool::new().with_threshold(2);
        let gu_id = Uuid::new_v4();

        let skill = SkillDocument::new("html", "HTML", gu_id);
        let result = pool.submit_candidate(gu_id, "蛊虫A", skill);

        let candidate_id = match result {
            LearningResult::CandidateSubmitted { candidate_id, .. } => candidate_id,
            _ => panic!("应该是 CandidateSubmitted"),
        };

        // 投票 1
        let status = pool.vote(
            "html",
            Uuid::new_v4(),
            "蛊虫B",
            &candidate_id,
            true,
            0.8,
            "同意".to_string(),
        );
        assert_eq!(status, Some(CandidateStatus::Pending));

        // 投票 2（达到阈值）
        let status = pool.vote(
            "html",
            Uuid::new_v4(),
            "蛊虫C",
            &candidate_id,
            true,
            0.9,
            "同意".to_string(),
        );
        assert_eq!(status, Some(CandidateStatus::ConsensusReached));

        // 提升
        let skill_id = pool.check_and_promote("html");
        assert!(skill_id.is_some());
    }

    #[test]
    fn test_existing_reinforce() {
        let mut pool = KnowledgePool::new();
        let gu_id = Uuid::new_v4();

        // 先创建并保存一个共享技能（ID 需要匹配 shared_exists 的计算方式）
        // shared_exists 会计算: skill_id = "skill_{normalized_topic}"
        // 所以我们创建一个 ID 为 "skill_test_shared" 的技能
        let mut skill = SkillDocument::new("skill_test_shared", "Test Shared", gu_id);
        let _ = pool.storage.save_shared_skill(&skill);

        // 现在用相同主题检查是否已存在
        let exists = pool.shared_exists("test shared");
        assert!(exists, "共享知识应该存在");
    }
}
