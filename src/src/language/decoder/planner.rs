//! 概念规划模块
//!
//! 选择要表达的概念序列

use crate::language::concept::ConceptVector;
use std::collections::HashMap;

/// 概念规划结果
#[derive(Debug, Clone)]
pub struct PlanResult {
    /// 概念序列
    pub concepts: Vec<String>,
    /// 每个概念的权重
    pub weights: Vec<f64>,
    /// 规划置信度
    pub confidence: f64,
}

/// 概念规划器
pub struct Planner {
    /// 概念频率统计
    concept_frequency: HashMap<String, u32>,
    /// 概念关联矩阵
    concept_relations: HashMap<String, Vec<(String, f64)>>,
    /// 最大概念数
    max_concepts: usize,
    /// 最小权重阈值
    min_weight: f64,
}

impl Planner {
    /// 创建新规划器
    pub fn new() -> Self {
        Self {
            concept_frequency: HashMap::new(),
            concept_relations: HashMap::new(),
            max_concepts: 10,
            min_weight: 0.1,
        }
    }

    /// 设置最大概念数
    pub fn with_max_concepts(mut self, max: usize) -> Self {
        self.max_concepts = max;
        self
    }

    /// 设置最小权重
    pub fn with_min_weight(mut self, weight: f64) -> Self {
        self.min_weight = weight;
        self
    }

    /// 从向量规划概念
    ///
    /// 根据向量特征选择最相关的概念
    pub fn plan(&self, vector: &ConceptVector) -> PlanResult {
        // 简化实现：基于向量特征生成概念
        let concepts = self.extract_concepts(vector);
        let weights = self.compute_weights(&concepts, vector);
        let confidence = self.compute_confidence(&weights);

        PlanResult {
            concepts,
            weights,
            confidence,
        }
    }

    /// 从向量序列规划概念
    pub fn plan_sequence(&self, vectors: &[ConceptVector]) -> PlanResult {
        // 聚合向量
        let aggregated = self.aggregate_vectors(vectors);

        // 规划概念
        self.plan(&aggregated)
    }

    /// 提取概念
    fn extract_concepts(&self, vector: &ConceptVector) -> Vec<String> {
        // 简化实现：基于向量统计特征
        let mean = vector.data.iter().sum::<f64>() / vector.data.len() as f64;
        let variance = vector.data.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / vector.data.len() as f64;

        let mut concepts = Vec::new();

        // 根据统计特征推断概念类型
        if mean > 0.0 {
            concepts.push("positive_concept".to_string());
        } else if mean < 0.0 {
            concepts.push("negative_concept".to_string());
        }

        if variance > 0.01 {
            concepts.push("complex_concept".to_string());
        } else {
            concepts.push("simple_concept".to_string());
        }

        // 添加高频概念
        let mut frequent: Vec<_> = self.concept_frequency.iter()
            .filter(|(_, &count)| count > 5)
            .map(|(k, _)| k.clone())
            .take(3)
            .collect();

        concepts.append(&mut frequent);

        // 限制数量
        concepts.truncate(self.max_concepts);

        concepts
    }

    /// 计算概念权重
    fn compute_weights(&self, concepts: &[String], vector: &ConceptVector) -> Vec<f64> {
        let norm = vector.norm();

        concepts.iter()
            .map(|concept| {
                // 基于概念频率和向量范数计算权重
                let freq_weight = self.concept_frequency
                    .get(concept)
                    .map(|&f| 1.0 / (1.0 + f as f64))
                    .unwrap_or(0.5);

                let norm_weight = (norm / 2.0).min(1.0);

                freq_weight * norm_weight
            })
            .filter(|&w| w >= self.min_weight)
            .collect()
    }

    /// 计算置信度
    fn compute_confidence(&self, weights: &[f64]) -> f64 {
        if weights.is_empty() {
            return 0.0;
        }

        let sum: f64 = weights.iter().sum();
        sum / weights.len() as f64
    }

    /// 聚合向量
    fn aggregate_vectors(&self, vectors: &[ConceptVector]) -> ConceptVector {
        if vectors.is_empty() {
            return ConceptVector::zero();
        }

        let dim = vectors[0].data.len();
        let mut sum = vec![0.0; dim];

        for v in vectors {
            for (i, &val) in v.data.iter().enumerate() {
                sum[i] += val;
            }
        }

        ConceptVector::from_data(sum)
    }

    /// 更新概念频率
    pub fn update_frequency(&mut self, concept: &str) {
        *self.concept_frequency.entry(concept.to_string()).or_insert(0) += 1;
    }

    /// 添加概念关联
    pub fn add_relation(&mut self, from: &str, to: &str, strength: f64) {
        self.concept_relations
            .entry(from.to_string())
            .or_insert_with(Vec::new)
            .push((to.to_string(), strength));
    }

    /// 获取相关概念
    pub fn get_related(&self, concept: &str) -> Option<&Vec<(String, f64)>> {
        self.concept_relations.get(concept)
    }

    /// 清空统计数据
    pub fn clear(&mut self) {
        self.concept_frequency.clear();
        self.concept_relations.clear();
    }
}

impl Default for Planner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan_creates_concepts() {
        let planner = Planner::new();
        let vector = ConceptVector::random_small();

        let result = planner.plan(&vector);

        assert!(!result.concepts.is_empty());
        assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
    }

    #[test]
    fn test_plan_sequence() {
        let planner = Planner::new();
        let vectors = vec![
            ConceptVector::random_small(),
            ConceptVector::random_small(),
        ];

        let result = planner.plan_sequence(&vectors);

        assert!(!result.concepts.is_empty());
    }

    #[test]
    fn test_frequency_update() {
        let mut planner = Planner::new();

        planner.update_frequency("概念A");
        planner.update_frequency("概念A");
        planner.update_frequency("概念B");

        assert_eq!(planner.concept_frequency.get("概念A"), Some(&2));
        assert_eq!(planner.concept_frequency.get("概念B"), Some(&1));
    }

    #[test]
    fn test_relations() {
        let mut planner = Planner::new();

        planner.add_relation("动物", "猫", 0.8);
        planner.add_relation("动物", "狗", 0.8);

        let related = planner.get_related("动物").unwrap();
        assert_eq!(related.len(), 2);
    }

    #[test]
    fn test_max_concepts_limit() {
        let planner = Planner::new().with_max_concepts(2);
        let vector = ConceptVector::random_small();

        let result = planner.plan(&vector);

        assert!(result.concepts.len() <= 2);
    }
}
