//! 概念空间
//!
//! 管理所有概念，提供查询、创建、更新等操作

use super::types::{Concept, ConceptId, ConceptVector, ConceptLevel};
use super::learning::VectorLearningRules;
use std::collections::HashMap;

/// 概念空间
pub struct ConceptSpace {
    /// 概念存储
    concepts: HashMap<ConceptId, Concept>,
    /// 名称到ID的映射
    name_to_id: HashMap<String, ConceptId>,
    /// 默认学习率
    learning_rate: f64,
}

impl ConceptSpace {
    /// 创建空的概念空间
    pub fn new() -> Self {
        Self {
            concepts: HashMap::new(),
            name_to_id: HashMap::new(),
            learning_rate: 0.01,
        }
    }

    /// 设置学习率
    pub fn with_learning_rate(mut self, rate: f64) -> Self {
        self.learning_rate = rate;
        self
    }

    /// 添加概念
    pub fn add_concept(&mut self, concept: Concept) -> Result<(), String> {
        if self.concepts.contains_key(&concept.id) {
            return Err(format!("概念ID已存在: {}", concept.id));
        }

        if self.name_to_id.contains_key(&concept.name) {
            return Err(format!("概念名称已存在: {}", concept.name));
        }

        // 如果有父概念，验证父概念存在
        if let Some(ref parent_id) = concept.parent_id {
            if !self.concepts.contains_key(parent_id) {
                return Err(format!("父概念不存在: {}", parent_id));
            }
        }

        let id = concept.id.clone();
        let name = concept.name.clone();

        self.name_to_id.insert(name, id.clone());
        self.concepts.insert(id, concept);

        Ok(())
    }

    /// 创建新概念
    pub fn create_concept(
        &mut self,
        id: ConceptId,
        name: String,
        level: ConceptLevel,
    ) -> Result<&Concept, String> {
        let id_clone = id.clone();
        let concept = Concept::new(id, name, level);
        self.add_concept(concept)?;
        Ok(self.concepts.get(&id_clone).unwrap())
    }

    /// 创建子概念
    pub fn create_child_concept(
        &mut self,
        parent_id: &str,
        child_id: ConceptId,
        child_name: String,
    ) -> Result<&Concept, String> {
        let parent = self.concepts.get(parent_id)
            .ok_or_else(|| format!("父概念不存在: {}", parent_id))?;

        let child = parent.create_child(child_id.clone(), child_name);

        // 更新父概念的子列表
        let parent = self.concepts.get_mut(parent_id).unwrap();
        parent.children_ids.push(child_id.clone());

        self.add_concept(child)?;
        Ok(self.concepts.get(&child_id).unwrap())
    }

    /// 获取概念
    pub fn get_concept(&self, id: &str) -> Option<&Concept> {
        self.concepts.get(id)
    }

    /// 通过名称获取概念
    pub fn get_concept_by_name(&self, name: &str) -> Option<&Concept> {
        self.name_to_id.get(name).and_then(|id| self.concepts.get(id))
    }

    /// 获取概念向量
    pub fn get_vector(&self, id: &str) -> Option<&ConceptVector> {
        self.concepts.get(id).map(|c| &c.vector)
    }

    /// 计算两个概念的相似度
    pub fn similarity(&self, id_a: &str, id_b: &str) -> Option<f64> {
        let v_a = self.get_vector(id_a)?;
        let v_b = self.get_vector(id_b)?;
        Some(v_a.cosine_similarity(v_b))
    }

    /// 更新概念向量（关联学习）
    pub fn learn_association(&mut self, id_a: &str, id_b: &str, strength: f64) -> Result<(), String> {
        let v_a = self.get_vector(id_a)
            .ok_or_else(|| format!("概念不存在: {}", id_a))?.clone();
        let v_b = self.get_vector(id_b)
            .ok_or_else(|| format!("概念不存在: {}", id_b))?.clone();

        let (new_v_a, new_v_b) = VectorLearningRules::association(
            &v_a, &v_b, self.learning_rate, strength
        );

        // 更新概念
        if let Some(concept_a) = self.concepts.get_mut(id_a) {
            if concept_a.can_modify() {
                concept_a.vector = new_v_a;
            }
        }
        if let Some(concept_b) = self.concepts.get_mut(id_b) {
            if concept_b.can_modify() {
                concept_b.vector = new_v_b;
            }
        }

        Ok(())
    }

    /// 更新概念向量（区分学习）
    pub fn learn_differentiation(&mut self, id_a: &str, id_b: &str, strength: f64) -> Result<(), String> {
        let v_a = self.get_vector(id_a)
            .ok_or_else(|| format!("概念不存在: {}", id_a))?.clone();
        let v_b = self.get_vector(id_b)
            .ok_or_else(|| format!("概念不存在: {}", id_b))?.clone();

        let (new_v_a, new_v_b) = VectorLearningRules::differentiation(
            &v_a, &v_b, self.learning_rate, strength
        );

        // 更新概念
        if let Some(concept_a) = self.concepts.get_mut(id_a) {
            if concept_a.can_modify() {
                concept_a.vector = new_v_a;
            }
        }
        if let Some(concept_b) = self.concepts.get_mut(id_b) {
            if concept_b.can_modify() {
                concept_b.vector = new_v_b;
            }
        }

        Ok(())
    }

    /// 查找最相似的概念
    pub fn find_most_similar(&self, vector: &ConceptVector, top_k: usize) -> Vec<(ConceptId, f64)> {
        let mut similarities: Vec<(ConceptId, f64)> = self.concepts.iter()
            .map(|(id, concept)| (id.clone(), vector.cosine_similarity(&concept.vector)))
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        similarities.truncate(top_k);
        similarities
    }

    /// 查找概念的所有子概念
    pub fn get_children(&self, parent_id: &str) -> Vec<&Concept> {
        self.concepts.get(parent_id)
            .map(|parent| {
                parent.children_ids.iter()
                    .filter_map(|child_id| self.concepts.get(child_id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 获取概念数量
    pub fn len(&self) -> usize {
        self.concepts.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.concepts.is_empty()
    }

    /// 获取所有概念ID
    pub fn all_ids(&self) -> Vec<&ConceptId> {
        self.concepts.keys().collect()
    }

    /// 获取所有概念的迭代器
    pub fn all_concepts(&self) -> impl Iterator<Item = (&String, &Concept)> {
        self.concepts.iter()
    }

    /// 查找与指定概念最相似的概念
    pub fn find_similar(&self, concept_id: &str, top_k: usize) -> Option<Vec<(ConceptId, f64)>> {
        let vector = self.get_vector(concept_id)?;
        let mut similarities: Vec<(ConceptId, f64)> = self.concepts.iter()
            .filter(|(id, _)| *id != concept_id) // 排除自身
            .map(|(id, concept)| (id.clone(), vector.cosine_similarity(&concept.vector)))
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        similarities.truncate(top_k);
        Some(similarities)
    }

    /// 增加概念使用次数
    pub fn increment_usage(&mut self, id: &str) {
        if let Some(concept) = self.concepts.get_mut(id) {
            concept.increment_usage();
        }
    }
}

impl Default for ConceptSpace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concept_space_creation() {
        let space = ConceptSpace::new();
        assert!(space.is_empty());
    }

    #[test]
    fn test_add_concept() {
        let mut space = ConceptSpace::new();
        let result = space.create_concept(
            "fruit".to_string(),
            "水果".to_string(),
            ConceptLevel::Basic,
        );
        assert!(result.is_ok());
        assert_eq!(space.len(), 1);
    }

    #[test]
    fn test_create_child_concept() {
        let mut space = ConceptSpace::new();
        space.create_concept(
            "fruit".to_string(),
            "水果".to_string(),
            ConceptLevel::Basic,
        ).unwrap();

        let result = space.create_child_concept(
            "fruit",
            "apple".to_string(),
            "苹果".to_string(),
        );
        assert!(result.is_ok());
        assert_eq!(space.len(), 2);

        // 验证父子关系
        let apple = space.get_concept("apple").unwrap();
        assert_eq!(apple.parent_id, Some("fruit".to_string()));

        let fruit = space.get_concept("fruit").unwrap();
        assert!(fruit.children_ids.contains(&"apple".to_string()));
    }

    #[test]
    fn test_similarity() {
        let mut space = ConceptSpace::new();
        space.create_concept("a".to_string(), "概念A".to_string(), ConceptLevel::Common).unwrap();
        space.create_concept("b".to_string(), "概念B".to_string(), ConceptLevel::Common).unwrap();

        let sim = space.similarity("a", "b");
        assert!(sim.is_some());
        let sim_val = sim.unwrap();
        assert!(sim_val >= -1.0 && sim_val <= 1.0);
    }

    #[test]
    fn test_association_learning() {
        let mut space = ConceptSpace::new();
        space.create_concept("a".to_string(), "概念A".to_string(), ConceptLevel::Common).unwrap();
        space.create_concept("b".to_string(), "概念B".to_string(), ConceptLevel::Common).unwrap();

        let initial_sim = space.similarity("a", "b").unwrap();

        // 执行关联学习
        for _ in 0..100 {
            space.learn_association("a", "b", 1.0).unwrap();
        }

        let final_sim = space.similarity("a", "b").unwrap();
        assert!(final_sim > initial_sim, "关联学习应该增加相似度");
    }

    #[test]
    fn test_find_most_similar() {
        let mut space = ConceptSpace::new();
        space.create_concept("a".to_string(), "概念A".to_string(), ConceptLevel::Common).unwrap();
        space.create_concept("b".to_string(), "概念B".to_string(), ConceptLevel::Common).unwrap();
        space.create_concept("c".to_string(), "概念C".to_string(), ConceptLevel::Common).unwrap();

        let vector = space.get_vector("a").unwrap().clone();
        let similar = space.find_most_similar(&vector, 3);

        assert_eq!(similar.len(), 3);
        // 最相似的应该是自己
        assert_eq!(similar[0].0, "a");
        assert!(similar[0].1 > 0.99);
    }

    #[test]
    fn test_system_core_not_modifiable() {
        let mut space = ConceptSpace::new();
        space.create_concept(
            "core".to_string(),
            "核心概念".to_string(),
            ConceptLevel::SystemCore,
        ).unwrap();
        space.create_concept(
            "other".to_string(),
            "其他概念".to_string(),
            ConceptLevel::Common,
        ).unwrap();

        let initial_vector = space.get_vector("core").unwrap().clone();

        // 尝试对核心概念执行学习
        space.learn_association("core", "other", 1.0).unwrap();

        // 核心概念的向量不应该改变
        let final_vector = space.get_vector("core").unwrap();
        assert_eq!(initial_vector.data, final_vector.data);
    }
}
