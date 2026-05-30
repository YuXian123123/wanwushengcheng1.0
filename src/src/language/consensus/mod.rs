//! 共识机制模块
//!
//! 管理群体概念共识，包括投票、冲突解决等
//!
//! # 共识流程
//!
//! ```text
//! 提案 → 讨论 → 投票 → 决议
//! ```

mod resolve;

use serde::{Deserialize, Serialize};
use crate::language::concept::{ConceptId, ConceptLevel};
use crate::config::consensus::ConsensusConfig;

// 重导出
pub use resolve::{ConflictResolver, Conflict, ConflictType, Resolution, ResolutionMethod};

/// 投票选项
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Vote {
    /// 赞成
    Approve,
    /// 反对
    Reject,
    /// 弃权
    Abstain,
}

/// 共识状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusState {
    /// 待审核
    Pending,
    /// 讨论中
    Discussing,
    /// 投票中
    Voting,
    /// 已批准
    Approved,
    /// 已拒绝
    Rejected,
}

/// 投票记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteRecord {
    /// 投票者ID
    pub voter_id: String,
    /// 投票选项
    pub vote: Vote,
    /// 权重
    pub weight: f64,
    /// 理由
    pub reason: Option<String>,
}

/// 共识提案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    /// 提案ID
    pub id: String,
    /// 概念ID
    pub concept_id: ConceptId,
    /// 提案类型
    pub proposal_type: ProposalType,
    /// 提案内容
    pub content: String,
    /// 投票记录
    pub votes: Vec<VoteRecord>,
    /// 当前状态
    pub status: ConsensusState,
    /// 需要的阈值
    pub threshold: f64,
}

/// 提案类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalType {
    /// 新增概念
    CreateConcept,
    /// 修改概念
    UpdateConcept,
    /// 废弃概念
    DeprecateConcept,
}

impl Proposal {
    /// 创建新提案
    pub fn new(id: String, concept_id: ConceptId, proposal_type: ProposalType, content: String, level: ConceptLevel, config: &ConsensusConfig) -> Self {
        let threshold = level.consensus_threshold_with_config(config);
        Self {
            id,
            concept_id,
            proposal_type,
            content,
            votes: Vec::new(),
            status: ConsensusState::Pending,
            threshold,
        }
    }

    /// 添加投票
    pub fn add_vote(&mut self, voter_id: String, vote: Vote, weight: f64, reason: Option<String>) {
        self.votes.push(VoteRecord {
            voter_id,
            vote,
            weight,
            reason,
        });
    }

    /// 计算批准率
    pub fn calculate_approval_rate(&self) -> f64 {
        if self.votes.is_empty() {
            return 0.0;
        }

        let total_weight: f64 = self.votes.iter().map(|v| v.weight).sum();
        if total_weight <= 0.0 {
            return 0.0;
        }

        let approve_weight: f64 = self.votes.iter()
            .filter(|v| v.vote == Vote::Approve)
            .map(|v| v.weight)
            .sum();

        approve_weight / total_weight
    }

    /// 检查是否达成共识
    pub fn check_consensus(&mut self) -> bool {
        let approval_rate = self.calculate_approval_rate();

        if approval_rate >= self.threshold {
            self.status = ConsensusState::Approved;
            true
        } else {
            false
        }
    }
}

/// 共识管理器
pub struct ConsensusManager {
    /// 活跃提案
    proposals: Vec<Proposal>,
    /// 配置
    config: ConsensusConfig,
    /// 冲突解决器
    conflict_resolver: ConflictResolver,
}

impl ConsensusManager {
    /// 创建新管理器
    pub fn new() -> Self {
        Self {
            proposals: Vec::new(),
            config: ConsensusConfig::new(),
            conflict_resolver: ConflictResolver::new(),
        }
    }

    /// 使用配置创建管理器
    pub fn with_config(config: ConsensusConfig) -> Self {
        Self {
            proposals: Vec::new(),
            config,
            conflict_resolver: ConflictResolver::new(),
        }
    }

    /// 创建提案
    pub fn create_proposal(&mut self, concept_id: ConceptId, proposal_type: ProposalType, content: String, level: ConceptLevel) -> String {
        let id = format!("proposal_{}", chrono::Utc::now().timestamp_millis());
        let proposal = Proposal::new(id.clone(), concept_id, proposal_type, content, level, &self.config);
        self.proposals.push(proposal);
        id
    }

    /// 投票
    pub fn vote(&mut self, proposal_id: &str, voter_id: String, vote: Vote, weight: f64, reason: Option<String>) -> Result<(), String> {
        let proposal = self.proposals.iter_mut()
            .find(|p| p.id == proposal_id)
            .ok_or_else(|| format!("提案不存在: {}", proposal_id))?;

        proposal.add_vote(voter_id, vote, weight, reason);
        Ok(())
    }

    /// 检查提案状态
    pub fn check_proposal(&mut self, proposal_id: &str) -> Result<ConsensusState, String> {
        let proposal = self.proposals.iter_mut()
            .find(|p| p.id == proposal_id)
            .ok_or_else(|| format!("提案不存在: {}", proposal_id))?;

        if proposal.votes.len() >= self.config.min_votes as usize {
            proposal.check_consensus();
        }

        Ok(proposal.status)
    }

    /// 获取提案
    pub fn get_proposal(&self, proposal_id: &str) -> Option<&Proposal> {
        self.proposals.iter().find(|p| p.id == proposal_id)
    }

    /// 获取冲突解决器
    pub fn conflict_resolver(&self) -> &ConflictResolver {
        &self.conflict_resolver
    }

    /// 获取冲突解决器（可变）
    pub fn conflict_resolver_mut(&mut self) -> &mut ConflictResolver {
        &mut self.conflict_resolver
    }

    /// 获取所有活跃提案
    pub fn active_proposals(&self) -> Vec<&Proposal> {
        self.proposals.iter()
            .filter(|p| p.status == ConsensusState::Pending || p.status == ConsensusState::Voting)
            .collect()
    }

    /// 获取配置
    pub fn config(&self) -> &ConsensusConfig {
        &self.config
    }
}

impl Default for ConsensusManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_proposal() {
        let mut manager = ConsensusManager::new();
        let id = manager.create_proposal(
            "test_concept".to_string(),
            ProposalType::CreateConcept,
            "创建测试概念".to_string(),
            ConceptLevel::Common,
        );

        assert!(!id.is_empty());
        assert!(manager.get_proposal(&id).is_some());
    }

    #[test]
    fn test_vote_and_consensus() {
        let mut manager = ConsensusManager::new();
        let id = manager.create_proposal(
            "test".to_string(),
            ProposalType::CreateConcept,
            "测试".to_string(),
            ConceptLevel::Common,
        );

        // 添加足够多的赞成票
        for i in 0..5 {
            manager.vote(&id, format!("voter_{}", i), Vote::Approve, 1.0, None).unwrap();
        }

        let status = manager.check_proposal(&id).unwrap();
        assert_eq!(status, ConsensusState::Approved);
    }

    #[test]
    fn test_conflict_resolver_access() {
        let mut manager = ConsensusManager::new();

        let conflict_id = manager.conflict_resolver_mut().record(
            ConflictType::SemanticConflict,
            vec!["A".to_string()],
            "测试冲突".to_string(),
            0.5,
        );

        let conflict = manager.conflict_resolver().get_conflict(&conflict_id);
        assert!(conflict.is_some());
    }
}
