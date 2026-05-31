//! 概念类型定义
//!
//! 定义概念、概念向量、概念ID等核心类型
//!
//! # 配置驱动
//!
//! 所有参数通过配置管理，避免硬编码：
//!
//! ```
//! use lnn::config::GlobalConfig;
//! use lnn::language::concept::ConceptVector;
//!
//! let config = GlobalConfig::new();
//! let vector = ConceptVector::with_config(&config.concept);
//! ```

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::config::concept::ConceptConfig;

/// 概念ID类型
pub type ConceptId = String;

/// 向量维度（已废弃，请使用配置）
///
/// 此常量仅用于向后兼容，新代码应使用配置系统
#[deprecated(since = "0.2.0", note = "请使用 ConceptConfig::vector_dim()")]
pub const VECTOR_DIM: usize = 256;

/// 概念向量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptVector {
    /// 高维向量数据
    pub data: Vec<f64>,
    /// 配置引用（可选，用于动态参数）
    #[serde(skip)]
    config: Option<Arc<ConceptConfig>>,
}

impl ConceptVector {
    /// 从数据创建向量（公开接口）
    pub fn from_data(data: Vec<f64>) -> Self {
        let mut result = Self { data, config: None };
        Self::normalize_in_place(&mut result.data);
        result
    }

    /// 从数据创建向量（不归一化）
    pub fn from_data_unnormalized(data: Vec<f64>) -> Self {
        Self { data, config: None }
    }

    /// 创建零向量（使用默认配置）
    pub fn zero() -> Self {
        Self::with_config(&ConceptConfig::new())
    }

    /// 使用配置创建零向量
    pub fn with_config(config: &ConceptConfig) -> Self {
        Self {
            data: vec![0.0; config.vector_dim],
            config: None, // 配置不存储，避免序列化问题
        }
    }

    /// 创建随机向量（小扰动）
    pub fn random_small() -> Self {
        Self::random_small_with_config(&ConceptConfig::new())
    }

    /// 使用配置创建随机向量
    pub fn random_small_with_config(config: &ConceptConfig) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut data: Vec<f64> = (0..config.vector_dim)
            .map(|_| rng.gen_range(config.random_init_range.clone()))
            .collect();
        Self::normalize_in_place_with_threshold(&mut data, config.normalization_threshold);
        Self { data, config: None }
    }

    /// 从父概念继承
    pub fn inherit(parent: &ConceptVector) -> Self {
        Self::inherit_with_config(parent, &ConceptConfig::new())
    }

    /// 使用配置从父概念继承
    pub fn inherit_with_config(parent: &ConceptVector, config: &ConceptConfig) -> Self {
        let mut data = parent.data.clone();
        // 添加小扰动
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for v in &mut data {
            *v += rng.gen_range(config.inheritance_perturbation.clone());
        }
        Self::normalize_in_place_with_threshold(&mut data, config.normalization_threshold);
        Self { data, config: None }
    }

    /// 归一化（使 ||v|| = 1）- 公开方法供模块内使用
    pub(super) fn normalize_in_place(data: &mut [f64]) {
        Self::normalize_in_place_with_threshold(data, 1e-10);
    }

    /// 使用阈值归一化
    fn normalize_in_place_with_threshold(data: &mut [f64], threshold: f64) {
        let norm: f64 = data.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > threshold {
            for v in data.iter_mut() {
                *v /= norm;
            }
        }
    }

    /// 计算范数
    pub fn norm(&self) -> f64 {
        self.data.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    /// 计算余弦相似度
    pub fn cosine_similarity(&self, other: &ConceptVector) -> f64 {
        let dot: f64 = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .sum();

        let norm_a = self.norm();
        let norm_b = other.norm();

        let threshold = 1e-10; // 使用默认阈值
        if norm_a > threshold && norm_b > threshold {
            dot / (norm_a * norm_b)
        } else {
            0.0
        }
    }

    /// 向量加法
    pub fn add(&self, other: &ConceptVector) -> ConceptVector {
        let data: Vec<f64> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        let mut result = ConceptVector { data, config: None };
        Self::normalize_in_place(&mut result.data);
        result
    }

    /// 向量减法
    pub fn sub(&self, other: &ConceptVector) -> ConceptVector {
        let data: Vec<f64> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a - b)
            .collect();
        let mut result = ConceptVector { data, config: None };
        Self::normalize_in_place(&mut result.data);
        result
    }

    /// 标量乘法
    pub fn scale(&self, scalar: f64) -> ConceptVector {
        let mut data: Vec<f64> = self.data.iter().map(|x| x * scalar).collect();
        Self::normalize_in_place(&mut data);
        ConceptVector { data, config: None }
    }

    /// 检查是否有效（无NaN/Inf）
    pub fn is_valid(&self) -> bool {
        self.data.iter().all(|x| x.is_finite())
    }
}

impl Default for ConceptVector {
    fn default() -> Self {
        Self::random_small()
    }
}

/// 概念安全级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConceptLevel {
    /// Level 0: 系统核心概念（不可修改）
    SystemCore,
    /// Level 1: 基础概念（高共识要求 0.9+）
    Basic,
    /// Level 2: 普通概念（中等共识要求 0.7+）
    Common,
    /// Level 3: 领域概念（低共识要求 0.6+）
    Domain,
    /// Level 4: 临时概念（最低要求 0.5+）
    Temporary,
}

impl ConceptLevel {
    /// 获取修改所需的共识阈值（使用默认配置）
    pub fn consensus_threshold(&self) -> f64 {
        self.consensus_threshold_with_config(&crate::config::consensus::ConsensusConfig::new())
    }

    /// 使用配置获取共识阈值
    pub fn consensus_threshold_with_config(&self, config: &crate::config::consensus::ConsensusConfig) -> f64 {
        match self {
            Self::SystemCore => config.system_core_threshold,
            Self::Basic => config.basic_threshold,
            Self::Common => config.common_threshold,
            Self::Domain => config.domain_threshold,
            Self::Temporary => config.temporary_threshold,
        }
    }

    /// 是否可修改
    pub fn is_modifiable(&self) -> bool {
        self != &Self::SystemCore
    }
}

/// 概念关系
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptRelation {
    /// 相关概念ID
    pub concept_id: ConceptId,
    /// 关联强度 [0, 1]
    pub strength: f64,
}

/// 共识状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusStatus {
    /// 待审核
    Pending,
    /// 已批准
    Approved,
    /// 已废弃
    Deprecated,
}

/// 概念
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    /// 唯一标识
    pub id: ConceptId,
    /// 概念名称
    pub name: String,
    /// 概念向量
    pub vector: ConceptVector,
    /// 安全级别
    pub level: ConceptLevel,
    /// 父概念ID
    pub parent_id: Option<ConceptId>,
    /// 子概念ID列表
    pub children_ids: Vec<ConceptId>,
    /// 同义词
    pub synonyms: Vec<String>,
    /// 相关概念及关联强度
    pub related: Vec<ConceptRelation>,
    /// 使用次数
    pub usage_count: u64,
    /// 共识状态
    pub consensus: ConsensusStatus,
    /// 投票数
    pub vote_count: u32,
    /// 批准率
    pub approval_rate: f64,
}

impl Concept {
    /// 创建新概念
    pub fn new(id: ConceptId, name: String, level: ConceptLevel) -> Self {
        Self {
            id,
            name,
            vector: ConceptVector::random_small(),
            level,
            parent_id: None,
            children_ids: Vec::new(),
            synonyms: Vec::new(),
            related: Vec::new(),
            usage_count: 0,
            consensus: ConsensusStatus::Pending,
            vote_count: 0,
            approval_rate: 0.0,
        }
    }

    /// 创建子概念
    pub fn create_child(&self, id: ConceptId, name: String) -> Self {
        let vector = ConceptVector::inherit(&self.vector);
        Self {
            id,
            name,
            vector,
            level: self.level,
            parent_id: Some(self.id.clone()),
            children_ids: Vec::new(),
            synonyms: Vec::new(),
            related: vec![ConceptRelation {
                concept_id: self.id.clone(),
                strength: 0.8,
            }],
            usage_count: 0,
            consensus: ConsensusStatus::Pending,
            vote_count: 0,
            approval_rate: 0.0,
        }
    }

    /// 增加使用次数
    pub fn increment_usage(&mut self) {
        self.usage_count += 1;
    }

    /// 添加相关概念
    pub fn add_relation(&mut self, concept_id: ConceptId, strength: f64) {
        let strength = strength.clamp(0.0, 1.0);
        if let Some(existing) = self.related.iter_mut().find(|r| r.concept_id == concept_id) {
            existing.strength = strength;
        } else {
            self.related.push(ConceptRelation { concept_id, strength });
        }
    }

    /// 添加同义词
    pub fn add_synonym(&mut self, synonym: String) {
        if !self.synonyms.contains(&synonym) {
            self.synonyms.push(synonym);
        }
    }

    /// 检查是否可修改
    pub fn can_modify(&self) -> bool {
        self.level.is_modifiable()
    }

    /// 更新共识状态
    pub fn update_consensus(&mut self, vote_count: u32, approval_rate: f64) {
        self.vote_count = vote_count;
        self.approval_rate = approval_rate;

        let threshold = self.level.consensus_threshold();
        if approval_rate >= threshold {
            self.consensus = ConsensusStatus::Approved;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_normalization() {
        let v = ConceptVector::random_small();
        assert!((v.norm() - 1.0).abs() < 1e-6, "向量应该归一化");
    }

    #[test]
    fn test_vector_similarity() {
        let v1 = ConceptVector::random_small();
        let v2 = v1.clone();
        assert!((v1.cosine_similarity(&v2) - 1.0).abs() < 1e-6, "相同向量相似度应为1");
    }

    #[test]
    fn test_vector_inherit() {
        let parent = ConceptVector::random_small();
        let child = ConceptVector::inherit(&parent);
        let sim = parent.cosine_similarity(&child);
        assert!(sim > 0.7, "子概念应该与父概念相似，相似度: {}", sim);
    }

    #[test]
    fn test_concept_creation() {
        let concept = Concept::new("test_1".to_string(), "测试概念".to_string(), ConceptLevel::Common);
        assert_eq!(concept.id, "test_1");
        assert_eq!(concept.name, "测试概念");
        assert!(concept.vector.is_valid());
    }

    #[test]
    fn test_concept_level_threshold() {
        assert_eq!(ConceptLevel::SystemCore.consensus_threshold(), 1.0);
        assert_eq!(ConceptLevel::Basic.consensus_threshold(), 0.9);
        assert_eq!(ConceptLevel::Common.consensus_threshold(), 0.7);
    }

    #[test]
    fn test_concept_level_modifiable() {
        assert!(!ConceptLevel::SystemCore.is_modifiable());
        assert!(ConceptLevel::Basic.is_modifiable());
        assert!(ConceptLevel::Common.is_modifiable());
    }
}
