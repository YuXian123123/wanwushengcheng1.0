//! 知识抽象层
//!
//! 从原始经验提炼通用原则
//!
//! 抽象层次：
//! Level 0: 原始数据 D（高熵）
//! Level 1: 信息 I = D + Context
//! Level 2: 知识 K = I + Structure
//! Level 3: 智慧 W = K + Principles
//! Level 4: 道理 T = W + Values

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::{Knowledge, KnowledgeType};

// ============================================================================
// 抽象层次
// ============================================================================

/// 知识抽象层次
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AbstractionLevel {
    /// Level 0: 原始数据
    Data = 0,
    /// Level 1: 信息
    Information = 1,
    /// Level 2: 知识
    Knowledge = 2,
    /// Level 3: 智慧
    Wisdom = 3,
    /// Level 4: 道理/原则
    Principle = 4,
}

impl AbstractionLevel {
    /// 从数值创建
    pub fn from_level(level: u8) -> Self {
        match level {
            0 => AbstractionLevel::Data,
            1 => AbstractionLevel::Information,
            2 => AbstractionLevel::Knowledge,
            3 => AbstractionLevel::Wisdom,
            4 | _ => AbstractionLevel::Principle,
        }
    }

    /// 获取层次名称
    pub fn name(&self) -> &'static str {
        match self {
            AbstractionLevel::Data => "原始数据",
            AbstractionLevel::Information => "信息",
            AbstractionLevel::Knowledge => "知识",
            AbstractionLevel::Wisdom => "智慧",
            AbstractionLevel::Principle => "道理",
        }
    }
}

// ============================================================================
// 抽象化知识
// ============================================================================

/// 抽象化后的知识
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractedKnowledge {
    /// 知识ID
    pub id: Uuid,
    /// 原始知识ID列表
    pub source_ids: Vec<Uuid>,
    /// 抽象层次
    pub level: AbstractionLevel,
    /// 抽象内容
    pub content: String,
    /// 模式描述
    pub pattern: String,
    /// 适用范围
    pub scope: Vec<String>,
    /// 置信度（0-1）
    pub confidence: f64,
    /// 支持样本数
    pub support_count: usize,
    /// 创建时间戳
    pub created_at: u64,
}

impl AbstractedKnowledge {
    pub fn new(
        source_ids: Vec<Uuid>,
        level: AbstractionLevel,
        content: String,
        pattern: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_ids,
            level,
            content,
            pattern,
            scope: Vec::new(),
            confidence: 0.5,
            support_count: 1,
            created_at: current_timestamp(),
        }
    }

    /// 增加支持
    pub fn add_support(&mut self, confidence_delta: f64) {
        self.support_count += 1;
        // 置信度逐渐收敛
        self.confidence = self.confidence * 0.9 + confidence_delta * 0.1;
    }

    /// 计算可靠性
    pub fn calculate_reliability(&self) -> f64 {
        // 可靠性 = 置信度 × log(支持数 + 1)
        self.confidence * (self.support_count as f64 + 1.0).ln().max(0.0).min(1.0)
    }
}

// ============================================================================
// 知识抽象器
// ============================================================================

/// 知识抽象器
#[derive(Debug, Clone)]
pub struct KnowledgeAbstraction {
    /// 已抽象的知识
    pub abstracted: HashMap<Uuid, AbstractedKnowledge>,
    /// 按层次索引
    pub level_index: HashMap<AbstractionLevel, Vec<Uuid>>,
    /// 模式库
    pub pattern_library: HashMap<String, Vec<Uuid>>,
    /// 最小支持数（抽象需要的最小样本）
    pub min_support: usize,
    /// 抽象阈值
    pub abstraction_threshold: f64,
}

impl KnowledgeAbstraction {
    pub fn new() -> Self {
        Self {
            abstracted: HashMap::new(),
            level_index: HashMap::new(),
            pattern_library: HashMap::new(),
            min_support: 3,
            abstraction_threshold: 0.6,
        }
    }

    /// 抽象知识
    pub fn abstract_knowledge(&mut self, knowledge_list: &[Knowledge]) -> Option<AbstractedKnowledge> {
        if knowledge_list.len() < self.min_support {
            return None;
        }

        // 1. 提取共同模式
        let pattern = self.extract_pattern(knowledge_list)?;

        // 2. 确定抽象层次
        let level = self.determine_level(knowledge_list);

        // 3. 生成抽象内容
        let content = self.generate_abstract_content(&pattern, knowledge_list);

        // 4. 创建抽象知识
        let source_ids: Vec<Uuid> = knowledge_list.iter().map(|k| k.id).collect();
        let mut abstracted = AbstractedKnowledge::new(
            source_ids,
            level,
            content,
            pattern.clone(),
        );

        // 5. 计算置信度
        abstracted.confidence = self.calculate_confidence(knowledge_list);
        abstracted.support_count = knowledge_list.len();

        // 6. 存储并索引
        let id = abstracted.id;
        self.abstracted.insert(id, abstracted.clone());
        self.level_index
            .entry(level)
            .or_insert_with(Vec::new)
            .push(id);
        self.pattern_library
            .entry(pattern.clone())
            .or_insert_with(Vec::new)
            .push(id);

        Some(abstracted)
    }

    /// 提取共同模式
    fn extract_pattern(&self, knowledge_list: &[Knowledge]) -> Option<String> {
        if knowledge_list.is_empty() {
            return None;
        }

        // 简化实现：寻找共同关键词
        let first = &knowledge_list[0].content;
        let words: Vec<&str> = first.split_whitespace().collect();

        // 寻找在所有知识中都出现的词
        let mut common_words = Vec::new();
        for word in &words {
            let is_common = knowledge_list.iter().all(|k| {
                k.content.split_whitespace().any(|w| w == *word)
            });
            if is_common && word.len() > 1 {
                common_words.push(*word);
            }
        }

        if common_words.is_empty() {
            None
        } else {
            Some(common_words.join(" "))
        }
    }

    /// 确定抽象层次
    fn determine_level(&self, knowledge_list: &[Knowledge]) -> AbstractionLevel {
        // 根据知识类型和支持数确定层次
        let avg_value: f64 = knowledge_list
            .iter()
            .map(|k| k.calculate_value())
            .sum::<f64>() / knowledge_list.len() as f64;

        let support_factor = (knowledge_list.len() as f64 / 10.0).min(1.0);
        let combined = avg_value * 0.5 + support_factor * 0.5;

        match combined {
            x if x > 0.8 => AbstractionLevel::Principle,
            x if x > 0.6 => AbstractionLevel::Wisdom,
            x if x > 0.4 => AbstractionLevel::Knowledge,
            x if x > 0.2 => AbstractionLevel::Information,
            _ => AbstractionLevel::Data,
        }
    }

    /// 生成抽象内容
    fn generate_abstract_content(&self, pattern: &str, knowledge_list: &[Knowledge]) -> String {
        // 提取关键信息
        let types: Vec<&str> = knowledge_list
            .iter()
            .map(|k| match k.knowledge_type {
                KnowledgeType::Skill => "技能",
                KnowledgeType::Fact => "事实",
                KnowledgeType::Experience => "经验",
                KnowledgeType::Strategy => "策略",
                KnowledgeType::Meta => "元知识",
            })
            .collect();

        let unique_types: Vec<&str> = types.into_iter().collect();

        format!(
            "抽象模式: {} [类型: {}] [样本数: {}]",
            pattern,
            unique_types.join(", "),
            knowledge_list.len()
        )
    }

    /// 计算置信度
    fn calculate_confidence(&self, knowledge_list: &[Knowledge]) -> f64 {
        if knowledge_list.is_empty() {
            return 0.0;
        }

        // 基于知识的一致性和可靠性
        let avg_trust: f64 = knowledge_list
            .iter()
            .map(|k| k.trust_score)
            .sum::<f64>() / knowledge_list.len() as f64;

        let success_rates: Vec<f64> = knowledge_list
            .iter()
            .filter_map(|k| {
                if k.usage_count > 0 {
                    Some(k.success_count as f64 / k.usage_count as f64)
                } else {
                    None
                }
            })
            .collect();

        let avg_success = if success_rates.is_empty() {
            0.5
        } else {
            success_rates.iter().sum::<f64>() / success_rates.len() as f64
        };

        avg_trust * 0.5 + avg_success * 0.5
    }

    /// 获取指定层次的抽象知识
    pub fn get_by_level(&self, level: AbstractionLevel) -> Vec<&AbstractedKnowledge> {
        self.level_index
            .get(&level)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.abstracted.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 根据模式查找
    pub fn find_by_pattern(&self, pattern: &str) -> Vec<&AbstractedKnowledge> {
        self.pattern_library
            .get(pattern)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.abstracted.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 提升抽象层次
    pub fn promote(&mut self, id: Uuid) -> Option<AbstractionLevel> {
        let abstracted = self.abstracted.get_mut(&id)?;
        let current_level = abstracted.level;

        if current_level >= AbstractionLevel::Principle {
            return None;
        }

        let new_level = AbstractionLevel::from_level(current_level as u8 + 1);

        // 更新层次索引
        if let Some(ids) = self.level_index.get_mut(&current_level) {
            ids.retain(|&i| i != id);
        }
        self.level_index
            .entry(new_level)
            .or_insert_with(Vec::new)
            .push(id);

        abstracted.level = new_level;
        Some(new_level)
    }

    /// 获取统计信息
    pub fn get_stats(&self) -> AbstractionStats {
        let mut level_counts = HashMap::new();
        for (level, ids) in &self.level_index {
            level_counts.insert(*level, ids.len());
        }

        AbstractionStats {
            total_abstracted: self.abstracted.len(),
            level_counts,
            pattern_count: self.pattern_library.len(),
        }
    }
}

/// 抽象统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractionStats {
    pub total_abstracted: usize,
    pub level_counts: HashMap<AbstractionLevel, usize>,
    pub pattern_count: usize,
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abstraction_level_ordering() {
        assert!(AbstractionLevel::Principle > AbstractionLevel::Wisdom);
        assert!(AbstractionLevel::Wisdom > AbstractionLevel::Knowledge);
        assert!(AbstractionLevel::Knowledge > AbstractionLevel::Information);
        assert!(AbstractionLevel::Information > AbstractionLevel::Data);
    }

    #[test]
    fn test_abstraction_level_from_level() {
        assert_eq!(AbstractionLevel::from_level(0), AbstractionLevel::Data);
        assert_eq!(AbstractionLevel::from_level(3), AbstractionLevel::Wisdom);
        assert_eq!(AbstractionLevel::from_level(10), AbstractionLevel::Principle);
    }

    #[test]
    fn test_abstracted_knowledge_creation() {
        let ak = AbstractedKnowledge::new(
            vec![Uuid::new_v4()],
            AbstractionLevel::Knowledge,
            "test content".to_string(),
            "test pattern".to_string(),
        );

        assert_eq!(ak.level, AbstractionLevel::Knowledge);
        assert_eq!(ak.support_count, 1);
    }

    #[test]
    fn test_abstracted_knowledge_add_support() {
        let mut ak = AbstractedKnowledge::new(
            vec![],
            AbstractionLevel::Knowledge,
            "test".to_string(),
            "pattern".to_string(),
        );

        ak.add_support(0.8);
        ak.add_support(0.9);

        assert_eq!(ak.support_count, 3);
        // 置信度应该收敛
        assert!(ak.confidence > 0.0 && ak.confidence < 1.0);
    }

    #[test]
    fn test_knowledge_abstraction_creation() {
        let abstracter = KnowledgeAbstraction::new();
        assert!(abstracter.abstracted.is_empty());
    }

    #[test]
    fn test_extract_pattern() {
        let abstracter = KnowledgeAbstraction::new();

        let k1 = Knowledge::new("如何识别 危险 情况".to_string(), KnowledgeType::Skill, Uuid::new_v4());
        let k2 = Knowledge::new("识别 危险 是重要技能".to_string(), KnowledgeType::Skill, Uuid::new_v4());
        let k3 = Knowledge::new("学会 识别 危险".to_string(), KnowledgeType::Skill, Uuid::new_v4());

        let pattern = abstracter.extract_pattern(&[k1, k2, k3]);
        assert!(pattern.is_some());
        let pattern_str = pattern.unwrap();
        assert!(pattern_str.contains("识别") || pattern_str.contains("危险"));
    }

    #[test]
    fn test_abstract_knowledge() {
        let mut abstracter = KnowledgeAbstraction::new();
        abstracter.min_support = 2;

        // 使用有共同模式的测试数据
        let k1 = Knowledge::new("识别危险 是重要技能".to_string(), KnowledgeType::Skill, Uuid::new_v4());
        let k2 = Knowledge::new("识别危险 需要练习".to_string(), KnowledgeType::Skill, Uuid::new_v4());
        let k3 = Knowledge::new("识别危险 很关键".to_string(), KnowledgeType::Skill, Uuid::new_v4());

        let result = abstracter.abstract_knowledge(&[k1, k2, k3]);
        assert!(result.is_some());

        let ak = result.unwrap();
        assert!(!ak.source_ids.is_empty());
        assert!(ak.confidence > 0.0);
    }

    #[test]
    fn test_get_by_level() {
        let mut abstracter = KnowledgeAbstraction::new();
        abstracter.min_support = 2;

        let k1 = Knowledge::new("test one two".to_string(), KnowledgeType::Skill, Uuid::new_v4());
        let k2 = Knowledge::new("test one two".to_string(), KnowledgeType::Skill, Uuid::new_v4());
        let k3 = Knowledge::new("test one two".to_string(), KnowledgeType::Skill, Uuid::new_v4());

        abstracter.abstract_knowledge(&[k1, k2, k3]);

        // 应该有抽象知识
        assert!(!abstracter.abstracted.is_empty());
    }

    #[test]
    fn test_promote() {
        let mut abstracter = KnowledgeAbstraction::new();
        abstracter.min_support = 2;

        let k1 = Knowledge::new("test pattern here".to_string(), KnowledgeType::Skill, Uuid::new_v4());
        let k2 = Knowledge::new("test pattern here".to_string(), KnowledgeType::Skill, Uuid::new_v4());
        let k3 = Knowledge::new("test pattern here".to_string(), KnowledgeType::Skill, Uuid::new_v4());

        if let Some(ak) = abstracter.abstract_knowledge(&[k1, k2, k3]) {
            let id = ak.id;
            let result = abstracter.promote(id);
            assert!(result.is_some());
        }
    }
}
