//! 概念对齐器

use super::tokenizer::Token;
use crate::language::concept::ConceptVector;
use crate::config::concept::ConceptConfig;

/// 编码结果
#[derive(Debug, Clone)]
pub struct EncodedResult {
    /// Token列表
    pub tokens: Vec<Token>,
    /// 向量序列
    pub vectors: Vec<ConceptVector>,
    /// 编码置信度
    pub confidence: f64,
    /// 是否成功
    pub success: bool,
}

/// 概念对齐器
pub struct Aligner {
    /// 概念配置
    config: ConceptConfig,
}

impl Aligner {
    /// 创建新对齐器
    pub fn new() -> Self {
        Self {
            config: ConceptConfig::new(),
        }
    }

    /// 使用配置创建对齐器
    pub fn with_config(config: ConceptConfig) -> Self {
        Self { config }
    }

    /// 将Token对齐到概念向量
    pub fn align(&self, tokens: &[Token]) -> EncodedResult {
        let vectors: Vec<ConceptVector> = tokens.iter()
            .map(|token| self.token_to_vector(token))
            .collect();

        // 计算置信度（简化：基于向量有效性）
        let valid_count = vectors.iter().filter(|v| v.is_valid()).count();
        let confidence = if tokens.is_empty() {
            0.0
        } else {
            valid_count as f64 / tokens.len() as f64
        };

        EncodedResult {
            tokens: tokens.to_vec(),
            vectors,
            confidence,
            success: confidence > 0.5,
        }
    }

    /// Token转向量（简化版）
    fn token_to_vector(&self, token: &Token) -> ConceptVector {
        // 简化实现：基于字符串哈希生成伪向量
        // 实际实现应该使用预训练的词嵌入或概念空间查找
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        token.text.hash(&mut hasher);
        let hash = hasher.finish();

        let vector_dim = self.config.vector_dim;
        let mut data = vec![0.0; vector_dim];
        for i in 0..vector_dim {
            let offset = ((hash >> (i % 64)) % 1000) as f64;
            data[i] = (offset / 500.0 - 1.0) * 0.1;
        }

        let mut vector = ConceptVector::from_data_unnormalized(data);
        // 归一化
        let norm = vector.norm();
        if norm > self.config.normalization_threshold {
            vector = vector.scale(1.0 / norm);
        }

        vector
    }
}

impl Default for Aligner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::language::encoder::tokenizer::Tokenizer;

    #[test]
    fn test_align_empty() {
        let aligner = Aligner::new();
        let result = aligner.align(&[]);
        assert_eq!(result.tokens.len(), 0);
        assert_eq!(result.vectors.len(), 0);
    }

    #[test]
    fn test_align_tokens() {
        let tokenizer = Tokenizer::new();
        let aligner = Aligner::new();

        let tokens = tokenizer.tokenize_no_whitespace("hello world");
        let result = aligner.align(&tokens);

        assert_eq!(result.vectors.len(), 2);
        assert!(result.confidence > 0.0);
        assert!(result.success);
    }

    #[test]
    fn test_vectors_normalized() {
        let tokenizer = Tokenizer::new();
        let aligner = Aligner::new();

        let tokens = tokenizer.tokenize_no_whitespace("测试");
        let result = aligner.align(&tokens);

        for vector in &result.vectors {
            assert!((vector.norm() - 1.0).abs() < 1e-6, "向量应归一化");
        }
    }
}
