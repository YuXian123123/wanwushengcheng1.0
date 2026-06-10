//! 预训练词向量加载器
//!
//! 从 JSON 文件加载预训练的词向量，用于改进实体识别

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use super::graph::EntityType;

/// 预训练词向量数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PretrainedVectors {
    /// 向量维度
    pub dimension: usize,
    /// 词到向量的映射
    pub words: HashMap<String, Vec<f64>>,
    /// 词到类别的映射（可选）
    #[serde(default)]
    pub categories: HashMap<String, Vec<String>>,
}

impl Default for PretrainedVectors {
    fn default() -> Self {
        Self {
            dimension: 200,
            words: HashMap::new(),
            categories: HashMap::new(),
        }
    }
}

impl PretrainedVectors {
    /// 从 JSON 文件加载
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("无法打开文件: {}", e))?;
        let reader = BufReader::new(file);
        let vectors: Self = serde_json::from_reader(reader)
            .map_err(|e| format!("解析JSON失败: {}", e))?;
        Ok(vectors)
    }

    /// 获取词向量
    pub fn get_vector(&self, word: &str) -> Option<&Vec<f64>> {
        self.words.get(word)
    }

    /// 计算两个词的余弦相似度
    pub fn similarity(&self, word1: &str, word2: &str) -> Option<f64> {
        let v1 = self.words.get(word1)?;
        let v2 = self.words.get(word2)?;
        Some(cosine_similarity(v1, v2))
    }

    /// 找到最相似的词
    pub fn most_similar(&self, word: &str, top_k: usize) -> Vec<(String, f64)> {
        let query_vec = match self.words.get(word) {
            Some(v) => v,
            None => return Vec::new(),
        };

        let mut similarities: Vec<(String, f64)> = self.words
            .iter()
            .filter(|(w, _)| *w != word)
            .map(|(w, v)| (w.clone(), cosine_similarity(query_vec, v)))
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        similarities.truncate(top_k);
        similarities
    }

    /// 推断词的类别（通过找最相似的类别关键词）
    pub fn infer_category(&self, word: &str) -> Option<(String, f64)> {
        let word_vec = self.words.get(word)?;

        let mut best_category = String::new();
        let mut best_score = -1.0;

        for (category, keywords) in &self.categories {
            for keyword in keywords {
                if let Some(keyword_vec) = self.words.get(keyword) {
                    let score = cosine_similarity(word_vec, keyword_vec);
                    if score > best_score {
                        best_score = score;
                        best_category = category.clone();
                    }
                }
            }
        }

        if best_score > 0.3 {
            Some((best_category, best_score))
        } else {
            None
        }
    }

    /// 词表大小
    pub fn vocab_size(&self) -> usize {
        self.words.len()
    }

    /// 检查词是否在词表中
    pub fn contains(&self, word: &str) -> bool {
        self.words.contains_key(word)
    }

    /// 使用词表进行最大匹配分词
    ///
    /// 算法：正向最大匹配（Forward Maximum Matching）
    /// 优先匹配词表中最长的词
    pub fn tokenize(&self, text: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let mut pos = 0;

        // 最大词长（用于限制搜索范围）
        let max_word_len = self.words.keys()
            .map(|w| w.chars().count())
            .max()
            .unwrap_or(4);

        while pos < chars.len() {
            let mut matched = false;

            // 从最长开始尝试匹配
            for len in (1..=std::cmp::min(max_word_len, chars.len() - pos)).rev() {
                let word: String = chars[pos..pos + len].iter().collect();

                // 检查是否在词表中
                if self.words.contains_key(&word) {
                    tokens.push(word.clone());
                    pos += len;
                    matched = true;
                    break;
                }
            }

            // 如果没有匹配到，单字作为未知词
            if !matched {
                let word: String = chars[pos..pos + 1].iter().collect();
                // 跳过空白和标点
                if !word.chars().next().unwrap().is_whitespace()
                   && !word.chars().next().unwrap().is_ascii_punctuation() {
                    // 单字不加入tokens，除非它在词表中
                    if self.words.contains_key(&word) {
                        tokens.push(word);
                    }
                }
                pos += 1;
            }
        }

        tokens
    }

    /// 从类别推断实体类型
    pub fn category_to_entity_type(&self, category: &str) -> Option<EntityType> {
        match category {
            "building" => Some(EntityType::Building),
            "furniture" => Some(EntityType::Object),
            "plant" => Some(EntityType::Plant),
            "person" => Some(EntityType::Person),
            "location" => Some(EntityType::Location),
            "color" => Some(EntityType::Concept), // 颜色是属性
            "material" => Some(EntityType::Concept), // 材料是属性
            _ => None,
        }
    }
}

/// 计算余弦相似度
fn cosine_similarity(v1: &[f64], v2: &[f64]) -> f64 {
    if v1.len() != v2.len() || v1.is_empty() {
        return 0.0;
    }

    let dot: f64 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
    let norm1: f64 = v1.iter().map(|a| a * a).sum::<f64>().sqrt();
    let norm2: f64 = v2.iter().map(|a| a * a).sum::<f64>().sqrt();

    if norm1 == 0.0 || norm2 == 0.0 {
        return 0.0;
    }

    dot / (norm1 * norm2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let v1 = vec![1.0, 0.0, 0.0];
        let v2 = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&v1, &v2) - 1.0).abs() < 0.0001);

        let v3 = vec![0.0, 1.0, 0.0];
        assert!((cosine_similarity(&v1, &v3) - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_load_vectors() {
        // 尝试加载测试数据
        if let Ok(vectors) = PretrainedVectors::load("data/word_vectors_test.json") {
            println!("加载词向量: {} 个词, {} 维", vectors.vocab_size(), vectors.dimension);

            // 测试相似度
            if let Some(sim) = vectors.similarity("房子", "建筑") {
                println!("房子 <-> 建筑: {}", sim);
            }

            // 测试类别推断
            if let Some((cat, score)) = vectors.infer_category("桌子") {
                println!("桌子 类别: {} (置信度: {})", cat, score);
            }
        }
    }
}
