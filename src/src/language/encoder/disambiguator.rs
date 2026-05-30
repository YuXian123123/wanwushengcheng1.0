//! 上下文消歧模块
//!
//! 处理多义词，根据上下文选择正确的概念

use crate::language::concept::ConceptVector;
use crate::language::encoder::tokenizer::Token;
use std::collections::HashMap;

/// 消歧结果
#[derive(Debug, Clone)]
pub struct DisambiguationResult {
    /// Token索引
    pub token_index: usize,
    /// 候选概念列表
    pub candidates: Vec<(String, f64)>, // (概念ID, 置信度)
    /// 选中的概念
    pub selected_concept: String,
    /// 置信度
    pub confidence: f64,
}

/// 上下文消歧器
pub struct Disambiguator {
    /// 多义词映射：词 -> 可能的概念列表
    polysemous: HashMap<String, Vec<String>>,
    /// 上下文窗口大小
    context_window: usize,
    /// 最小置信度阈值
    min_confidence: f64,
}

impl Disambiguator {
    /// 创建新消歧器
    pub fn new() -> Self {
        let mut polysemous = HashMap::new();

        // 注册常见多义词
        Self::register_common_polysemous(&mut polysemous);

        Self {
            polysemous,
            context_window: 5,
            min_confidence: 0.3,
        }
    }

    /// 设置上下文窗口大小
    pub fn with_context_window(mut self, window: usize) -> Self {
        self.context_window = window;
        self
    }

    /// 设置最小置信度
    pub fn with_min_confidence(mut self, confidence: f64) -> Self {
        self.min_confidence = confidence;
        self
    }

    /// 注册常见多义词
    fn register_common_polysemous(map: &mut HashMap<String, Vec<String>>) {
        // 中文多义词
        map.insert("打".to_string(), vec![
            "打_击打".to_string(),
            "打_打电话".to_string(),
            "打_打字".to_string(),
            "打_打球".to_string(),
        ]);

        map.insert("开".to_string(), vec![
            "开_打开".to_string(),
            "开_开始".to_string(),
            "开_开车".to_string(),
        ]);

        map.insert("看".to_string(), vec![
            "看_观看".to_string(),
            "看_看书".to_string(),
            "看_看见".to_string(),
        ]);

        // 英文多义词
        map.insert("bank".to_string(), vec![
            "bank_河岸".to_string(),
            "bank_银行".to_string(),
        ]);

        map.insert("run".to_string(), vec![
            "run_跑步".to_string(),
            "run_运行".to_string(),
            "run_经营".to_string(),
        ]);
    }

    /// 注册多义词
    pub fn register(&mut self, word: &str, concepts: Vec<String>) {
        self.polysemous.insert(word.to_string(), concepts);
    }

    /// 检查是否为多义词
    pub fn is_polysemous(&self, word: &str) -> bool {
        self.polysemous.contains_key(word)
    }

    /// 获取多义词的候选概念
    pub fn get_candidates(&self, word: &str) -> Option<&Vec<String>> {
        self.polysemous.get(word)
    }

    /// 消歧
    ///
    /// 对tokens中的多义词进行消歧
    pub fn disambiguate(&self, tokens: &[Token], vectors: &[ConceptVector]) -> Vec<DisambiguationResult> {
        let mut results = Vec::new();

        for (i, token) in tokens.iter().enumerate() {
            if let Some(candidates) = self.polysemous.get(&token.text) {
                // 构建上下文向量
                let context_vector = self.build_context_vector(i, vectors);

                // 选择最佳概念
                let (selected, confidence) = self.select_concept(
                    &token.text,
                    candidates,
                    &context_vector,
                );

                if confidence >= self.min_confidence {
                    results.push(DisambiguationResult {
                        token_index: i,
                        candidates: candidates.iter()
                            .map(|c| (c.clone(), 1.0 / candidates.len() as f64))
                            .collect(),
                        selected_concept: selected,
                        confidence,
                    });
                }
            }
        }

        results
    }

    /// 构建上下文向量
    fn build_context_vector(&self, center: usize, vectors: &[ConceptVector]) -> ConceptVector {
        let start = center.saturating_sub(self.context_window);
        let end = (center + self.context_window + 1).min(vectors.len());

        // 排除中心词
        let context_vectors: Vec<&ConceptVector> = (start..end)
            .filter(|&i| i != center)
            .map(|i| &vectors[i])
            .collect();

        if context_vectors.is_empty() {
            return ConceptVector::zero();
        }

        // 平均上下文向量
        let dim = context_vectors[0].data.len();
        let mut sum = vec![0.0; dim];

        for v in &context_vectors {
            for (i, &val) in v.data.iter().enumerate() {
                sum[i] += val;
            }
        }

        let count = context_vectors.len() as f64;
        for val in &mut sum {
            *val /= count;
        }

        ConceptVector::from_data(sum)
    }

    /// 选择最佳概念
    fn select_concept(
        &self,
        _word: &str,
        candidates: &[String],
        _context: &ConceptVector,
    ) -> (String, f64) {
        // 简化实现：选择第一个候选
        // 实际实现应该：
        // 1. 查询每个候选概念的概念向量
        // 2. 计算与上下文向量的相似度
        // 3. 选择最相似的

        if candidates.is_empty() {
            return (String::new(), 0.0);
        }

        // 使用均匀分布作为置信度
        (candidates[0].clone(), 1.0 / candidates.len() as f64)
    }

    /// 批量消歧并返回概念ID列表
    pub fn disambiguate_to_concepts(&self, tokens: &[Token], vectors: &[ConceptVector]) -> Vec<String> {
        let disambiguation = self.disambiguate(tokens, vectors);
        let mut concepts = Vec::new();

        for result in disambiguation {
            concepts.push(result.selected_concept);
        }

        concepts
    }
}

impl Default for Disambiguator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disambiguator_creation() {
        let disambiguator = Disambiguator::new();
        assert!(disambiguator.is_polysemous("打"));
        assert!(disambiguator.is_polysemous("bank"));
        assert!(!disambiguator.is_polysemous("hello"));
    }

    #[test]
    fn test_register_polysemous() {
        let mut disambiguator = Disambiguator::new();
        disambiguator.register("测试", vec!["测试_名词".to_string(), "测试_动词".to_string()]);

        assert!(disambiguator.is_polysemous("测试"));
        let candidates = disambiguator.get_candidates("测试").unwrap();
        assert_eq!(candidates.len(), 2);
    }

    #[test]
    fn test_disambiguate_empty() {
        let disambiguator = Disambiguator::new();
        let tokens = vec![];
        let vectors = vec![];

        let results = disambiguator.disambiguate(&tokens, &vectors);
        assert!(results.is_empty());
    }

    #[test]
    fn test_context_window() {
        let disambiguator = Disambiguator::new()
            .with_context_window(3);

        assert_eq!(disambiguator.context_window, 3);
    }
}
