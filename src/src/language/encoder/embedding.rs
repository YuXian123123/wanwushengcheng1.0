//! 词嵌入模块
//!
//! 将词映射到概念向量空间

use crate::language::concept::ConceptVector;
use crate::config::concept::ConceptConfig;
use std::collections::HashMap;

/// 词嵌入器
pub struct Embedding {
    /// 词到向量的映射
    embeddings: HashMap<String, ConceptVector>,
    /// 概念配置
    config: ConceptConfig,
    /// 位置编码是否启用
    use_position_encoding: bool,
}

impl Embedding {
    /// 创建新嵌入器
    pub fn new() -> Self {
        Self {
            embeddings: HashMap::new(),
            config: ConceptConfig::new(),
            use_position_encoding: true,
        }
    }

    /// 使用配置创建嵌入器
    pub fn with_config(config: ConceptConfig) -> Self {
        Self {
            embeddings: HashMap::new(),
            config,
            use_position_encoding: true,
        }
    }

    /// 获取或创建词向量
    ///
    /// 如果词已存在，返回已有向量
    /// 如果词不存在，创建新向量并缓存
    pub fn embed(&mut self, word: &str) -> ConceptVector {
        if let Some(vector) = self.embeddings.get(word) {
            return vector.clone();
        }

        // 创建新向量
        let vector = self.create_embedding(word);
        self.embeddings.insert(word.to_string(), vector.clone());
        vector
    }

    /// 批量嵌入
    pub fn embed_batch(&mut self, words: &[String]) -> Vec<ConceptVector> {
        words.iter().map(|w| self.embed(w)).collect()
    }

    /// 带位置编码的嵌入
    ///
    /// position: 词在序列中的位置
    pub fn embed_with_position(&mut self, word: &str, position: usize) -> ConceptVector {
        let base_vector = self.embed(word);

        if !self.use_position_encoding {
            return base_vector;
        }

        // 添加位置编码
        self.add_position_encoding(&base_vector, position)
    }

    /// 创建词嵌入
    fn create_embedding(&self, word: &str) -> ConceptVector {
        // 简化实现：基于字符串特征生成向量
        // 实际实现应使用预训练模型（如Word2Vec、FastText等）
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        word.hash(&mut hasher);
        let hash = hasher.finish();

        // 基于字符特征
        let char_count = word.chars().count() as f64;
        let byte_count = word.len() as f64;
        let has_chinese = word.chars().any(|c| ('\u{4E00}'..='\u{9FFF}').contains(&c));

        let dim = self.config.vector_dim;
        let mut data = vec![0.0; dim];

        for i in 0..dim {
            let base = ((hash >> (i % 64)) % 1000) as f64 / 1000.0;
            let char_factor = (char_count / 10.0).min(1.0) * 0.1;
            let chinese_factor = if has_chinese { 0.05 } else { 0.0 };

            data[i] = (base - 0.5) * 0.1 + char_factor + chinese_factor;
        }

        ConceptVector::from_data(data)
    }

    /// 添加位置编码（正弦/余弦编码）
    fn add_position_encoding(&self, vector: &ConceptVector, position: usize) -> ConceptVector {
        let dim = vector.data.len();
        let mut data = vector.data.clone();

        // 使用正弦/余弦位置编码
        let pos = position as f64;
        for i in (0..dim).step_by(2) {
            let div = 10000_f64.powf(i as f64 / dim as f64);
            data[i] += (pos / div).sin() * 0.01;

            if i + 1 < dim {
                data[i + 1] += (pos / div).cos() * 0.01;
            }
        }

        ConceptVector::from_data(data)
    }

    /// 获取词汇表大小
    pub fn vocab_size(&self) -> usize {
        self.embeddings.len()
    }

    /// 检查词是否存在
    pub fn contains(&self, word: &str) -> bool {
        self.embeddings.contains_key(word)
    }

    /// 更新词向量（用于在线学习）
    pub fn update(&mut self, word: &str, new_vector: ConceptVector) {
        self.embeddings.insert(word.to_string(), new_vector);
    }

    /// 移除词
    pub fn remove(&mut self, word: &str) -> bool {
        self.embeddings.remove(word).is_some()
    }

    /// 清空词汇表
    pub fn clear(&mut self) {
        self.embeddings.clear();
    }
}

impl Default for Embedding {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embed_creates_vector() {
        let mut embedding = Embedding::new();
        let vector = embedding.embed("测试");
        assert!(vector.is_valid());
        assert_eq!(embedding.vocab_size(), 1);
    }

    #[test]
    fn test_embed_caches_vector() {
        let mut embedding = Embedding::new();
        let v1 = embedding.embed("hello");
        let v2 = embedding.embed("hello");

        // 应该返回相同的向量
        assert_eq!(v1.data, v2.data);
        assert_eq!(embedding.vocab_size(), 1);
    }

    #[test]
    fn test_position_encoding_changes_vector() {
        let mut embedding = Embedding::new();
        let v1 = embedding.embed_with_position("test", 0);
        let v2 = embedding.embed_with_position("test", 10);

        // 不同位置应该产生不同的向量
        assert_ne!(v1.data, v2.data);
    }

    #[test]
    fn test_batch_embedding() {
        let mut embedding = Embedding::new();
        let words = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let vectors = embedding.embed_batch(&words);

        assert_eq!(vectors.len(), 3);
        assert_eq!(embedding.vocab_size(), 3);
    }

    #[test]
    fn test_update_and_remove() {
        let mut embedding = Embedding::new();
        embedding.embed("test");

        let new_vector = ConceptVector::random_small();
        embedding.update("test", new_vector.clone());

        let retrieved = embedding.embed("test");
        assert_eq!(retrieved.data, new_vector.data);

        assert!(embedding.remove("test"));
        assert_eq!(embedding.vocab_size(), 0);
    }
}
