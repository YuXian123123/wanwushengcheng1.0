//! 知识层次模块 - 优雅设计
//!
//! 分层知识组织：事实层、概念层、原理层、元知识层

use std::collections::{HashMap, HashSet};

/// 知识层级
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum KnowledgeLevel {
    /// L0: 事实层 - 具体数据和观察
    Fact = 0,
    /// L1: 概念层 - 抽象概念和分类
    Concept = 1,
    /// L2: 原理层 - 规则和定律
    Principle = 2,
    /// L3: 元知识层 - 关于知识的知识
    MetaKnowledge = 3,
}

/// 知识节点
#[derive(Debug, Clone)]
pub struct KnowledgeNode {
    /// 节点ID
    pub id: String,
    /// 节点名称
    pub name: String,
    /// 层级
    pub level: KnowledgeLevel,
    /// 内容
    pub content: String,
    /// 来源（学习或推理）
    pub source: KnowledgeSource,
    /// 置信度
    pub confidence: f64,
    /// 父节点（更高层级）
    pub parents: HashSet<String>,
    /// 子节点（更低层级）
    pub children: HashSet<String>,
}

/// 知识来源
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KnowledgeSource {
    /// 从文档学习
    Learned(String),
    /// 通过推理得出
    Inferred(String),
    /// 用户输入
    UserProvided,
    /// 共识知识
    Consensus,
}

/// 知识层次结构
#[derive(Debug)]
pub struct KnowledgeHierarchy {
    /// 所有节点
    nodes: HashMap<String, KnowledgeNode>,
    /// 按层级索引
    level_index: HashMap<KnowledgeLevel, HashSet<String>>,
}

impl KnowledgeHierarchy {
    /// 创建空的知识层次
    pub fn new() -> Self {
        let mut level_index = HashMap::new();
        level_index.insert(KnowledgeLevel::Fact, HashSet::new());
        level_index.insert(KnowledgeLevel::Concept, HashSet::new());
        level_index.insert(KnowledgeLevel::Principle, HashSet::new());
        level_index.insert(KnowledgeLevel::MetaKnowledge, HashSet::new());

        Self {
            nodes: HashMap::new(),
            level_index,
        }
    }

    /// 添加知识节点
    pub fn add_node(&mut self, node: KnowledgeNode) {
        let level = node.level;
        let id = node.id.clone();

        // 添加到层级索引
        self.level_index.get_mut(&level).unwrap().insert(id.clone());

        // 添加到节点集合
        self.nodes.insert(id, node);
    }

    /// 建立父子关系
    pub fn link(&mut self, parent_id: &str, child_id: &str) -> bool {
        let parent_exists = self.nodes.contains_key(parent_id);
        let child_exists = self.nodes.contains_key(child_id);

        if !parent_exists || !child_exists {
            return false;
        }

        // 检查层级关系（父节点层级应大于子节点）
        let parent_level = self.nodes.get(parent_id).unwrap().level;
        let child_level = self.nodes.get(child_id).unwrap().level;

        if parent_level <= child_level {
            return false; // 父节点层级必须更高
        }

        // 建立双向链接
        self.nodes.get_mut(parent_id).unwrap().children.insert(child_id.to_string());
        self.nodes.get_mut(child_id).unwrap().parents.insert(parent_id.to_string());

        true
    }

    /// 获取节点
    pub fn get(&self, id: &str) -> Option<&KnowledgeNode> {
        self.nodes.get(id)
    }

    /// 获取某层级的所有节点
    pub fn get_by_level(&self, level: KnowledgeLevel) -> Vec<&KnowledgeNode> {
        self.level_index
            .get(&level)
            .map(|ids| ids.iter().filter_map(|id| self.nodes.get(id)).collect())
            .unwrap_or_default()
    }

    /// 从事实归纳概念
    pub fn induct_concept(&mut self, fact_ids: &[String], concept_name: String) -> Option<String> {
        // 验证所有事实存在
        for fid in fact_ids {
            if !self.nodes.contains_key(fid) {
                return None;
            }
        }

        // 创建新概念
        let concept_id = format!("concept_{}", concept_name);
        let mut concept = KnowledgeNode {
            id: concept_id.clone(),
            name: concept_name,
            level: KnowledgeLevel::Concept,
            content: format!("从{}个事实归纳", fact_ids.len()),
            source: KnowledgeSource::Inferred("induction".to_string()),
            confidence: 0.8, // 归纳置信度
            parents: HashSet::new(),
            children: HashSet::new(),
        };

        // 建立父子关系
        for fid in fact_ids {
            concept.children.insert(fid.clone());
        }

        self.add_node(concept);

        // 更新子节点的父引用
        for fid in fact_ids {
            if let Some(fact) = self.nodes.get_mut(fid) {
                fact.parents.insert(concept_id.clone());
            }
        }

        Some(concept_id)
    }

    /// 统计信息
    pub fn stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        stats.insert("facts".to_string(), self.level_index.get(&KnowledgeLevel::Fact).map(|s| s.len()).unwrap_or(0));
        stats.insert("concepts".to_string(), self.level_index.get(&KnowledgeLevel::Concept).map(|s| s.len()).unwrap_or(0));
        stats.insert("principles".to_string(), self.level_index.get(&KnowledgeLevel::Principle).map(|s| s.len()).unwrap_or(0));
        stats.insert("meta".to_string(), self.level_index.get(&KnowledgeLevel::MetaKnowledge).map(|s| s.len()).unwrap_or(0));
        stats
    }

    /// 节点总数
    pub fn total_nodes(&self) -> usize {
        self.nodes.len()
    }
}

impl Default for KnowledgeHierarchy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchy_creation() {
        let h = KnowledgeHierarchy::new();
        assert_eq!(h.total_nodes(), 0);
    }

    #[test]
    fn test_add_fact() {
        let mut h = KnowledgeHierarchy::new();
        h.add_node(KnowledgeNode {
            id: "fact_1".to_string(),
            name: "太阳升起".to_string(),
            level: KnowledgeLevel::Fact,
            content: "太阳从东方升起".to_string(),
            source: KnowledgeSource::Learned("observation".to_string()),
            confidence: 1.0,
            parents: HashSet::new(),
            children: HashSet::new(),
        });

        assert_eq!(h.total_nodes(), 1);
        assert_eq!(h.get_by_level(KnowledgeLevel::Fact).len(), 1);
    }

    #[test]
    fn test_link_nodes() {
        let mut h = KnowledgeHierarchy::new();

        h.add_node(KnowledgeNode {
            id: "fact_1".to_string(),
            name: "鸟会飞".to_string(),
            level: KnowledgeLevel::Fact,
            content: "鸟会飞".to_string(),
            source: KnowledgeSource::Learned("doc".to_string()),
            confidence: 1.0,
            parents: HashSet::new(),
            children: HashSet::new(),
        });

        h.add_node(KnowledgeNode {
            id: "concept_1".to_string(),
            name: "飞行能力".to_string(),
            level: KnowledgeLevel::Concept,
            content: "飞行是一种能力".to_string(),
            source: KnowledgeSource::Inferred("abstraction".to_string()),
            confidence: 0.9,
            parents: HashSet::new(),
            children: HashSet::new(),
        });

        assert!(h.link("concept_1", "fact_1"));
        assert!(!h.link("fact_1", "concept_1")); // 不能反向链接
    }

    #[test]
    fn test_induction() {
        let mut h = KnowledgeHierarchy::new();

        // 添加多个事实
        h.add_node(KnowledgeNode {
            id: "fact_1".to_string(),
            name: "麻雀会飞".to_string(),
            level: KnowledgeLevel::Fact,
            content: "".to_string(),
            source: KnowledgeSource::Learned("doc".to_string()),
            confidence: 1.0,
            parents: HashSet::new(),
            children: HashSet::new(),
        });

        h.add_node(KnowledgeNode {
            id: "fact_2".to_string(),
            name: "老鹰会飞".to_string(),
            level: KnowledgeLevel::Fact,
            content: "".to_string(),
            source: KnowledgeSource::Learned("doc".to_string()),
            confidence: 1.0,
            parents: HashSet::new(),
            children: HashSet::new(),
        });

        let concept_id = h.induct_concept(&["fact_1".to_string(), "fact_2".to_string()], "鸟类飞行".to_string());
        assert!(concept_id.is_some());
        assert_eq!(h.get_by_level(KnowledgeLevel::Concept).len(), 1);
    }
}
