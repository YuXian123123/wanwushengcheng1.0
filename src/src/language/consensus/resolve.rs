//! 冲突解决模块
//!
//! 处理概念冲突，通过上下文分离、概念细化、版本化等方式解决

use crate::language::concept::{ConceptId, ConceptLevel};
use std::collections::HashMap;

/// 冲突类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictType {
    /// 语义冲突 - 同一概念有不同含义
    SemanticConflict,
    /// 向量冲突 - 向量过于相似但概念不同
    VectorConflict,
    /// 层级冲突 - 父子关系矛盾
    HierarchyConflict,
    /// 共识冲突 - 不同蛊虫对同一概念有不同理解
    ConsensusConflict,
}

/// 冲突记录
#[derive(Debug, Clone)]
pub struct Conflict {
    /// 冲突ID
    pub id: String,
    /// 冲突类型
    pub conflict_type: ConflictType,
    /// 涉及的概念
    pub concepts: Vec<ConceptId>,
    /// 冲突描述
    pub description: String,
    /// 严重程度 [0, 1]
    pub severity: f64,
    /// 是否已解决
    pub resolved: bool,
}

/// 解决方案
#[derive(Debug, Clone)]
pub struct Resolution {
    /// 解决方案ID
    pub id: String,
    /// 冲突ID
    pub conflict_id: String,
    /// 解决方法
    pub method: ResolutionMethod,
    /// 结果描述
    pub result: String,
    /// 置信度
    pub confidence: f64,
}

/// 解决方法
#[derive(Debug, Clone)]
pub enum ResolutionMethod {
    /// 上下文分离 - 根据上下文区分概念
    ContextSeparation,
    /// 概念细化 - 创建更具体的子概念
    ConceptRefinement,
    /// 概念合并 - 合并相似概念
    ConceptMerge,
    /// 版本化 - 创建概念的不同版本
    Versioning,
    /// 投票决定 - 通过共识机制决定
    ConsensusVote,
    /// 人工介入 - 需要管理员决策
    ManualIntervention,
}

/// 冲突解决器
pub struct ConflictResolver {
    /// 已记录的冲突
    conflicts: HashMap<String, Conflict>,
    /// 已应用的解决方案
    resolutions: HashMap<String, Resolution>,
    /// 自动解决阈值
    auto_resolve_threshold: f64,
}

impl ConflictResolver {
    /// 创建新解决器
    pub fn new() -> Self {
        Self {
            conflicts: HashMap::new(),
            resolutions: HashMap::new(),
            auto_resolve_threshold: 0.7,
        }
    }

    /// 设置自动解决阈值
    pub fn with_auto_threshold(mut self, threshold: f64) -> Self {
        self.auto_resolve_threshold = threshold;
        self
    }

    /// 记录冲突
    pub fn record(
        &mut self,
        conflict_type: ConflictType,
        concepts: Vec<ConceptId>,
        description: String,
        severity: f64,
    ) -> String {
        let id = format!("conflict_{}", self.conflicts.len() + 1);

        let conflict = Conflict {
            id: id.clone(),
            conflict_type,
            concepts,
            description,
            severity,
            resolved: false,
        };

        self.conflicts.insert(id.clone(), conflict);
        id
    }

    /// 获取冲突
    pub fn get_conflict(&self, id: &str) -> Option<&Conflict> {
        self.conflicts.get(id)
    }

    /// 获取所有未解决的冲突
    pub fn get_unresolved(&self) -> Vec<&Conflict> {
        self.conflicts.values()
            .filter(|c| !c.resolved)
            .collect()
    }

    /// 尝试自动解决
    pub fn try_auto_resolve(&mut self, conflict_id: &str) -> Option<Resolution> {
        let conflict = self.conflicts.get(conflict_id)?;

        if conflict.resolved {
            return None;
        }

        // 根据冲突类型和严重程度决定解决方法
        let (method, confidence) = self.determine_resolution_method(conflict);

        if confidence >= self.auto_resolve_threshold {
            let resolution = self.apply_resolution(conflict_id, method.clone(), confidence);
            return Some(resolution);
        }

        None
    }

    /// 决定解决方法
    fn determine_resolution_method(&self, conflict: &Conflict) -> (ResolutionMethod, f64) {
        match conflict.conflict_type {
            ConflictType::SemanticConflict => {
                // 语义冲突：优先上下文分离
                (ResolutionMethod::ContextSeparation, 0.8)
            }
            ConflictType::VectorConflict => {
                // 向量冲突：优先概念细化
                (ResolutionMethod::ConceptRefinement, 0.75)
            }
            ConflictType::HierarchyConflict => {
                // 层级冲突：需要人工介入
                (ResolutionMethod::ManualIntervention, 0.5)
            }
            ConflictType::ConsensusConflict => {
                // 共识冲突：通过投票决定
                (ResolutionMethod::ConsensusVote, 0.85)
            }
        }
    }

    /// 应用解决方案
    pub fn apply_resolution(
        &mut self,
        conflict_id: &str,
        method: ResolutionMethod,
        confidence: f64,
    ) -> Resolution {
        let resolution_id = format!("resolution_{}", self.resolutions.len() + 1);

        let result = match &method {
            ResolutionMethod::ContextSeparation => "已根据上下文分离概念".to_string(),
            ResolutionMethod::ConceptRefinement => "已创建更具体的子概念".to_string(),
            ResolutionMethod::ConceptMerge => "已合并相似概念".to_string(),
            ResolutionMethod::Versioning => "已创建概念的新版本".to_string(),
            ResolutionMethod::ConsensusVote => "已通过投票解决冲突".to_string(),
            ResolutionMethod::ManualIntervention => "等待管理员决策".to_string(),
        };

        let resolution = Resolution {
            id: resolution_id.clone(),
            conflict_id: conflict_id.to_string(),
            method,
            result,
            confidence,
        };

        // 标记冲突为已解决
        if let Some(conflict) = self.conflicts.get_mut(conflict_id) {
            conflict.resolved = true;
        }

        self.resolutions.insert(resolution_id, resolution.clone());
        resolution
    }

    /// 获取解决方案
    pub fn get_resolution(&self, id: &str) -> Option<&Resolution> {
        self.resolutions.get(id)
    }

    /// 检查概念是否可用于解决冲突
    pub fn can_modify_for_resolution(&self, level: ConceptLevel) -> bool {
        // 只有非系统核心概念可以被修改
        level != ConceptLevel::SystemCore
    }

    /// 统计冲突
    pub fn stats(&self) -> ConflictStats {
        let total = self.conflicts.len();
        let resolved = self.conflicts.values().filter(|c| c.resolved).count();
        let unresolved = total - resolved;

        ConflictStats {
            total,
            resolved,
            unresolved,
        }
    }
}

/// 冲突统计
#[derive(Debug, Clone)]
pub struct ConflictStats {
    pub total: usize,
    pub resolved: usize,
    pub unresolved: usize,
}

impl Default for ConflictResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_conflict() {
        let mut resolver = ConflictResolver::new();

        let id = resolver.record(
            ConflictType::SemanticConflict,
            vec!["概念A".to_string()],
            "测试冲突".to_string(),
            0.5,
        );

        assert!(resolver.get_conflict(&id).is_some());
    }

    #[test]
    fn test_auto_resolve() {
        let mut resolver = ConflictResolver::new()
            .with_auto_threshold(0.5);

        let id = resolver.record(
            ConflictType::SemanticConflict,
            vec!["概念A".to_string()],
            "测试冲突".to_string(),
            0.5,
        );

        let resolution = resolver.try_auto_resolve(&id);
        assert!(resolution.is_some());

        let conflict = resolver.get_conflict(&id).unwrap();
        assert!(conflict.resolved);
    }

    #[test]
    fn test_get_unresolved() {
        let mut resolver = ConflictResolver::new();

        let id1 = resolver.record(
            ConflictType::SemanticConflict,
            vec!["A".to_string()],
            "冲突1".to_string(),
            0.5,
        );

        resolver.record(
            ConflictType::VectorConflict,
            vec!["B".to_string()],
            "冲突2".to_string(),
            0.5,
        );

        resolver.try_auto_resolve(&id1);

        let unresolved = resolver.get_unresolved();
        assert_eq!(unresolved.len(), 1);
    }

    #[test]
    fn test_conflict_stats() {
        let mut resolver = ConflictResolver::new();

        resolver.record(
            ConflictType::SemanticConflict,
            vec!["A".to_string()],
            "冲突1".to_string(),
            0.5,
        );

        let stats = resolver.stats();
        assert_eq!(stats.total, 1);
        assert_eq!(stats.unresolved, 1);
    }
}
