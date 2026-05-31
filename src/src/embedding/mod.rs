//! 词向量嵌入模块
//!
//! 提供词向量的加载、查询、相似度计算功能
//!
//! 支持的格式：
//! - fastText .vec 文本格式
//! - 自定义二进制格式（更快加载）
//! - 多模态嵌入（代码、图像、音频、视频）
//!
//! 核心公式：
//! Similarity = cos(v1, v2) = (v1 · v2) / (||v1|| × ||v2||)

pub mod config;
pub mod training;
pub mod features;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

// 从config模块重导出类型
pub use config::{EmbeddingConfig, EmbeddingSourceType, UnknownStrategy, TrainingConfig};
pub use config::{MultimodalConfig, ModalityType};
pub use training::{Corpus, EmbeddingTrainer, TrainingSample, TrainingResult, MultimodalTrainer};
pub use features::{FeatureVector, FeatureType, UnifiedFeatureExtractor};

// ============================================================================
// 词向量存储
// ============================================================================

/// 词向量存储
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordEmbedding {
    /// 配置
    config: EmbeddingConfig,
    /// 词汇表：词 -> 向量
    vectors: HashMap<String, Vec<f64>>,
    /// 词频统计
    frequencies: HashMap<String, usize>,
    /// 归一化向量缓存
    normalized: HashMap<String, Vec<f64>>,
    /// 平均向量（用于未知词）
    average_vector: Vec<f64>,
    /// 向量模长缓存
    norms: HashMap<String, f64>,
}

impl WordEmbedding {
    pub fn new(config: EmbeddingConfig) -> Self {
        Self {
            average_vector: vec![0.0; config.dimension],
            config,
            vectors: HashMap::new(),
            frequencies: HashMap::new(),
            normalized: HashMap::new(),
            norms: HashMap::new(),
        }
    }

    /// 从 fastText .vec 文件加载
    pub fn from_fasttext_vec<P: AsRef<Path>>(
        path: P,
        config: EmbeddingConfig,
    ) -> Result<Self, EmbeddingError> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);

        let mut embedding = Self::new(config.clone());
        let mut first_line = true;
        let mut loaded_count = 0;

        for line in reader.lines() {
            let line = line?;

            if first_line {
                // 第一行是词汇量和维度
                first_line = false;
                continue;
            }

            // 检查最大词汇量限制
            if config.max_vocabulary > 0 && loaded_count >= config.max_vocabulary {
                break;
            }

            // 解析行：词 vec[0] vec[1] ... vec[n]
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() <= config.dimension {
                continue;
            }

            let word = parts[0].to_string();
            let vector: Vec<f64> = parts[1..=config.dimension]
                .iter()
                .filter_map(|s| s.parse().ok())
                .collect();

            if vector.len() != config.dimension {
                continue;
            }

            embedding.add_vector(word, vector);
            loaded_count += 1;
        }

        // 计算平均向量
        embedding.compute_average_vector();

        // 归一化
        if config.normalize {
            embedding.compute_normalized();
        }

        println!("Loaded {} word vectors", loaded_count);
        Ok(embedding)
    }

    /// 添加词向量
    pub fn add_vector(&mut self, word: String, vector: Vec<f64>) {
        let norm = Self::compute_norm(&vector);
        self.norms.insert(word.clone(), norm);
        self.vectors.insert(word, vector);
    }

    /// 获取词向量
    pub fn get_vector(&self, word: &str) -> Vec<f64> {
        if let Some(vec) = self.vectors.get(word) {
            vec.clone()
        } else {
            match self.config.unknown_strategy {
                UnknownStrategy::Zero => vec![0.0; self.config.dimension],
                UnknownStrategy::Average => self.average_vector.clone(),
                UnknownStrategy::Random => {
                    // 生成随机小向量
                    (0..self.config.dimension)
                        .map(|_| (rand_simple() - 0.5) * 0.1)
                        .collect()
                }
                UnknownStrategy::Subword => {
                    // 子词分解（简化实现：生成基于词哈希的伪向量）
                    let mut vector = vec![0.0; self.config.dimension];
                    let hash = self.hash_word(word);
                    for (i, v) in vector.iter_mut().enumerate() {
                        *v = ((hash.wrapping_add(i as u64) >> 32) as f64 / u32::MAX as f64 - 0.5) * 0.1;
                    }
                    vector
                }
            }
        }
    }

    /// 简单的词哈希
    fn hash_word(&self, word: &str) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        word.hash(&mut hasher);
        hasher.finish()
    }

    /// 获取归一化向量
    pub fn get_normalized(&self, word: &str) -> Vec<f64> {
        if let Some(vec) = self.normalized.get(word) {
            vec.clone()
        } else {
            let vec = self.get_vector(word);
            Self::normalize(&vec)
        }
    }

    /// 计算余弦相似度
    pub fn similarity(&self, word1: &str, word2: &str) -> f64 {
        let v1 = self.get_vector(word1);
        let v2 = self.get_vector(word2);

        // cos(v1, v2) = (v1 · v2) / (||v1|| × ||v2||)
        let dot: f64 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
        let norm1 = self.norms.get(word1).copied().unwrap_or_else(|| Self::compute_norm(&v1));
        let norm2 = self.norms.get(word2).copied().unwrap_or_else(|| Self::compute_norm(&v2));

        if norm1 > 0.0 && norm2 > 0.0 {
            dot / (norm1 * norm2)
        } else {
            0.0
        }
    }

    /// 查找最相似的词
    pub fn most_similar(&self, word: &str, top_k: usize) -> Vec<(String, f64)> {
        let target = self.get_normalized(word);
        if target.iter().all(|&x| x == 0.0) {
            return Vec::new();
        }

        let mut scores: Vec<(String, f64)> = self
            .normalized
            .iter()
            .filter(|(w, _)| *w != word)
            .map(|(w, vec)| {
                let sim: f64 = target.iter().zip(vec.iter()).map(|(a, b)| a * b).sum();
                (w.clone(), sim)
            })
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores.truncate(top_k);
        scores
    }

    /// 类比推理：A之于B如同C之于？
    /// result = B - A + C
    pub fn analogy(&self, word_a: &str, word_b: &str, word_c: &str, top_k: usize) -> Vec<(String, f64)> {
        let va = self.get_vector(word_a);
        let vb = self.get_vector(word_b);
        let vc = self.get_vector(word_c);

        // 目标向量 = B - A + C
        let target: Vec<f64> = vb.iter()
            .zip(va.iter())
            .zip(vc.iter())
            .map(|((b, a), c)| b - a + c)
            .collect();

        let target_norm = Self::normalize(&target);

        let mut scores: Vec<(String, f64)> = self
            .normalized
            .iter()
            .filter(|(w, _)| *w != word_a && *w != word_b && *w != word_c)
            .map(|(w, vec)| {
                let sim: f64 = target_norm.iter().zip(vec.iter()).map(|(a, b)| a * b).sum();
                (w.clone(), sim)
            })
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores.truncate(top_k);
        scores
    }

    /// 词汇表大小
    pub fn vocabulary_size(&self) -> usize {
        self.vectors.len()
    }

    /// 向量维度
    pub fn dimension(&self) -> usize {
        self.config.dimension
    }

    /// 检查词是否存在
    pub fn contains(&self, word: &str) -> bool {
        self.vectors.contains_key(word)
    }

    /// 计算向量模长
    fn compute_norm(vec: &[f64]) -> f64 {
        vec.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    /// 归一化向量
    fn normalize(vec: &[f64]) -> Vec<f64> {
        let norm = Self::compute_norm(vec);
        if norm > 0.0 {
            vec.iter().map(|x| x / norm).collect()
        } else {
            vec.to_vec()
        }
    }

    /// 计算平均向量
    fn compute_average_vector(&mut self) {
        if self.vectors.is_empty() {
            return;
        }

        let dim = self.config.dimension;
        let mut sum = vec![0.0; dim];

        for vec in self.vectors.values() {
            for (i, &v) in vec.iter().enumerate() {
                sum[i] += v;
            }
        }

        let count = self.vectors.len() as f64;
        self.average_vector = sum.iter().map(|x| x / count).collect();
    }

    /// 计算归一化向量
    fn compute_normalized(&mut self) {
        self.normalized.clear();
        for (word, vec) in &self.vectors {
            self.normalized.insert(word.clone(), Self::normalize(vec));
        }
    }

    /// 保存为二进制格式
    pub fn save_binary<P: AsRef<Path>>(&self, path: P) -> Result<(), EmbeddingError> {
        let mut file = File::create(path)?;

        // 写入维度
        file.write_all(&(self.config.dimension as u32).to_le_bytes())?;
        // 写入词汇量
        file.write_all(&(self.vectors.len() as u32).to_le_bytes())?;

        // 写入每个词和向量
        for (word, vec) in &self.vectors {
            // 词长度 + 词
            let word_bytes = word.as_bytes();
            file.write_all(&(word_bytes.len() as u32).to_le_bytes())?;
            file.write_all(word_bytes)?;

            // 向量
            for &v in vec {
                file.write_all(&v.to_le_bytes())?;
            }
        }

        Ok(())
    }

    /// 从二进制格式加载
    pub fn load_binary<P: AsRef<Path>>(path: P, config: EmbeddingConfig) -> Result<Self, EmbeddingError> {
        let data = std::fs::read(path)?;
        let mut pos = 0;

        // 读取维度
        let dim = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        pos += 4;

        // 读取词汇量
        let vocab_size = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
        pos += 4;

        let mut embedding = Self::new(EmbeddingConfig {
            dimension: dim,
            ..config
        });

        for _ in 0..vocab_size {
            // 词长度
            let word_len = u32::from_le_bytes([
                data[pos], data[pos + 1], data[pos + 2], data[pos + 3],
            ]) as usize;
            pos += 4;

            // 词
            let word = String::from_utf8(data[pos..pos + word_len].to_vec())?;
            pos += word_len;

            // 向量
            let mut vector = Vec::with_capacity(dim);
            for _ in 0..dim {
                let bytes: [u8; 8] = data[pos..pos + 8].try_into()?;
                vector.push(f64::from_le_bytes(bytes));
                pos += 8;
            }

            embedding.add_vector(word, vector);
        }

        embedding.compute_average_vector();
        if embedding.config.normalize {
            embedding.compute_normalized();
        }

        Ok(embedding)
    }

    /// 从 Word2Vec 二进制格式加载（腾讯词向量格式）
    ///
    /// 格式：
    /// - 头部：词汇量(4字节) + 维度(4字节)
    /// - 每个词：词(空格结尾) + 向量(float32数组)
    pub fn from_word2vec_binary<P: AsRef<Path>>(
        path: P,
        config: EmbeddingConfig,
    ) -> Result<Self, EmbeddingError> {
        let data = std::fs::read(path)?;
        let mut pos = 0;

        // 读取头部
        let header_end = data.iter().position(|&b| b == b'\n').unwrap_or(data.len());
        let header = String::from_utf8_lossy(&data[..header_end]);
        let header_parts: Vec<&str> = header.split_whitespace().collect();

        if header_parts.len() < 2 {
            return Err(EmbeddingError::IoError(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid Word2Vec binary header",
            )));
        }

        let vocab_size: usize = header_parts[0].parse().unwrap_or(0);
        let dim: usize = header_parts[1].parse().unwrap_or(0);

        println!("加载腾讯词向量: {} 词, {} 维", vocab_size, dim);

        pos = header_end + 1;

        let actual_dim = if config.dimension > 0 { config.dimension } else { dim };
        let mut embedding = Self::new(EmbeddingConfig {
            dimension: actual_dim,
            ..config
        });

        let mut loaded_count = 0;
        let max_vocab = if config.max_vocabulary > 0 {
            config.max_vocabulary.min(vocab_size)
        } else {
            vocab_size
        };

        while pos < data.len() && loaded_count < max_vocab {
            // 读取词（以空格结尾）
            let word_start = pos;
            while pos < data.len() && data[pos] != b' ' {
                pos += 1;
            }

            if pos >= data.len() {
                break;
            }

            let word = String::from_utf8_lossy(&data[word_start..pos]).to_string();
            pos += 1; // 跳过空格

            // 读取向量（float32）
            let vec_bytes = dim * 4;
            if pos + vec_bytes > data.len() {
                break;
            }

            let mut vector = Vec::with_capacity(actual_dim);
            for i in 0..dim.min(actual_dim) {
                let start = pos + i * 4;
                let bytes: [u8; 4] = [data[start], data[start + 1], data[start + 2], data[start + 3]];
                vector.push(f32::from_le_bytes(bytes) as f64);
            }

            // 如果配置维度大于实际维度，用零填充
            while vector.len() < actual_dim {
                vector.push(0.0);
            }

            pos += vec_bytes;

            embedding.add_vector(word, vector);
            loaded_count += 1;

            if loaded_count % 100000 == 0 {
                println!("已加载 {} 词...", loaded_count);
            }
        }

        embedding.compute_average_vector();
        if embedding.config.normalize {
            embedding.compute_normalized();
        }

        println!("加载完成: {} 词", loaded_count);
        Ok(embedding)
    }
}

// ============================================================================
// 错误类型
// ============================================================================

#[derive(Debug)]
pub enum EmbeddingError {
    IoError(std::io::Error),
    Utf8Error(std::string::FromUtf8Error),
    TryFromSliceError(std::array::TryFromSliceError),
}

impl From<std::io::Error> for EmbeddingError {
    fn from(e: std::io::Error) -> Self {
        EmbeddingError::IoError(e)
    }
}

impl From<std::string::FromUtf8Error> for EmbeddingError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        EmbeddingError::Utf8Error(e)
    }
}

impl From<std::array::TryFromSliceError> for EmbeddingError {
    fn from(e: std::array::TryFromSliceError) -> Self {
        EmbeddingError::TryFromSliceError(e)
    }
}

// ============================================================================
// 简单随机数生成
// ============================================================================

fn rand_simple() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as f64 / u32::MAX as f64)
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_creation() {
        let config = EmbeddingConfig::default();
        let embedding = WordEmbedding::new(config);

        assert_eq!(embedding.dimension(), 300);
        assert_eq!(embedding.vocabulary_size(), 0);
    }

    #[test]
    fn test_add_vector() {
        let config = EmbeddingConfig { dimension: 3, ..Default::default() };
        let mut embedding = WordEmbedding::new(config);

        embedding.add_vector("hello".to_string(), vec![1.0, 0.0, 0.0]);
        embedding.add_vector("world".to_string(), vec![0.0, 1.0, 0.0]);

        assert_eq!(embedding.vocabulary_size(), 2);
        assert!(embedding.contains("hello"));
        assert!(embedding.contains("world"));
    }

    #[test]
    fn test_get_vector() {
        let config = EmbeddingConfig { dimension: 3, ..Default::default() };
        let mut embedding = WordEmbedding::new(config);

        embedding.add_vector("test".to_string(), vec![1.0, 2.0, 3.0]);

        let vec = embedding.get_vector("test");
        assert_eq!(vec, vec![1.0, 2.0, 3.0]);

        // 未知词返回零向量
        let unknown = embedding.get_vector("unknown");
        assert_eq!(unknown, vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_similarity() {
        let config = EmbeddingConfig { dimension: 3, ..Default::default() };
        let mut embedding = WordEmbedding::new(config);

        embedding.add_vector("a".to_string(), vec![1.0, 0.0, 0.0]);
        embedding.add_vector("b".to_string(), vec![1.0, 0.0, 0.0]);
        embedding.add_vector("c".to_string(), vec![0.0, 1.0, 0.0]);

        // 相同向量 = 相似度1
        let sim = embedding.similarity("a", "b");
        assert!((sim - 1.0).abs() < 0.001);

        // 正交向量 = 相似度0
        let sim = embedding.similarity("a", "c");
        assert!(sim.abs() < 0.001);
    }

    #[test]
    fn test_compute_norm() {
        let vec = vec![3.0, 4.0];
        let norm = WordEmbedding::compute_norm(&vec);
        assert!((norm - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_normalize() {
        let vec = vec![3.0, 4.0];
        let normalized = WordEmbedding::normalize(&vec);

        assert!((normalized[0] - 0.6).abs() < 0.001);
        assert!((normalized[1] - 0.8).abs() < 0.001);

        let norm = WordEmbedding::compute_norm(&normalized);
        assert!((norm - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_save_load_binary() {
        let config = EmbeddingConfig { dimension: 3, ..Default::default() };
        let mut embedding = WordEmbedding::new(config);

        embedding.add_vector("hello".to_string(), vec![1.0, 2.0, 3.0]);
        embedding.add_vector("world".to_string(), vec![4.0, 5.0, 6.0]);

        let temp_path = std::env::temp_dir().join("test_embedding.bin");

        // 保存
        embedding.save_binary(&temp_path).unwrap();

        // 加载
        let loaded = WordEmbedding::load_binary(&temp_path, EmbeddingConfig::default()).unwrap();

        assert_eq!(loaded.vocabulary_size(), 2);
        assert!(loaded.contains("hello"));
        assert!(loaded.contains("world"));

        let vec = loaded.get_vector("hello");
        assert_eq!(vec, vec![1.0, 2.0, 3.0]);

        // 清理
        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_most_similar() {
        let config = EmbeddingConfig { dimension: 3, ..Default::default() };
        let mut embedding = WordEmbedding::new(config);

        embedding.add_vector("king".to_string(), vec![1.0, 0.0, 0.0]);
        embedding.add_vector("queen".to_string(), vec![0.9, 0.1, 0.0]);
        embedding.add_vector("man".to_string(), vec![0.0, 1.0, 0.0]);

        embedding.compute_normalized();

        let similar = embedding.most_similar("king", 2);
        assert!(!similar.is_empty());
        assert_eq!(similar[0].0, "queen");  // queen 最相似
    }

    // ========================================================================
    // 中文词向量测试（需要先运行 python scripts/download_embeddings.py --model test）
    // ========================================================================

    #[test]
    fn test_load_chinese_test_embeddings() {
        // 加载测试词向量
        let config = EmbeddingConfig {
            dimension: 10,
            ..Default::default()
        };

        // 尝试加载，如果文件不存在则跳过
        let embedding = WordEmbedding::from_fasttext_vec(
            "data/embeddings/test.vec",
            config,
        );

        if embedding.is_err() {
            println!("跳过测试：请先运行 python scripts/download_embeddings.py --model test");
            return;
        }

        let embedding = embedding.unwrap();

        // 验证加载
        assert!(embedding.vocabulary_size() > 0);
        println!("词汇表大小: {}", embedding.vocabulary_size());

        // 测试词向量查询
        let vec = embedding.get_vector("猫");
        println!("'猫' 的向量维度: {}", vec.len());

        // 测试相似度计算
        let sim_cat_dog = embedding.similarity("猫", "狗");
        let sim_cat_bird = embedding.similarity("猫", "鸟");
        let sim_love_hate = embedding.similarity("爱", "恨");

        println!("相似度 猫-狗: {:.4}", sim_cat_dog);
        println!("相似度 猫-鸟: {:.4}", sim_cat_bird);
        println!("相似度 爱-恨: {:.4}", sim_love_hate);

        // 猫和狗应该更相似（向量方向更接近）
        assert!(sim_cat_dog > sim_cat_bird);
    }

    #[test]
    fn test_chinese_similar_words() {
        let config = EmbeddingConfig {
            dimension: 10,
            ..Default::default()
        };

        let embedding = WordEmbedding::from_fasttext_vec(
            "data/embeddings/test.vec",
            config,
        );

        if embedding.is_err() {
            println!("跳过测试：请先运行 python scripts/download_embeddings.py --model test");
            return;
        }

        let embedding = embedding.unwrap();

        // 查找最相似的词
        let similar_to_cat = embedding.most_similar("猫", 5);
        println!("与'猫'最相似的词:");
        for (word, score) in &similar_to_cat {
            println!("  {} : {:.4}", word, score);
        }

        let similar_to_world = embedding.most_similar("世界", 5);
        println!("与'世界'最相似的词:");
        for (word, score) in &similar_to_world {
            println!("  {} : {:.4}", word, score);
        }

        assert!(!similar_to_cat.is_empty());
    }

    #[test]
    fn test_word_categories() {
        let config = EmbeddingConfig {
            dimension: 10,
            ..Default::default()
        };

        let embedding = WordEmbedding::from_fasttext_vec(
            "data/embeddings/test.vec",
            config,
        );

        if embedding.is_err() {
            println!("跳过测试：请先运行 python scripts/download_embeddings.py --model test");
            return;
        }

        let embedding = embedding.unwrap();

        // 动物类词汇应该相似
        let animals = vec!["猫", "狗", "鸟", "鱼"];
        println!("\n动物类词汇相似度矩阵:");
        for w1 in &animals {
            for w2 in &animals {
                let sim = embedding.similarity(w1, w2);
                print!("{:.2} ", sim);
            }
            println!();
        }

        // 颜色类词汇应该相似
        let colors = vec!["红", "蓝", "绿"];
        println!("\n颜色类词汇相似度矩阵:");
        for w1 in &colors {
            for w2 in &colors {
                let sim = embedding.similarity(w1, w2);
                print!("{:.2} ", sim);
            }
            println!();
        }

        // 情感类词汇
        let emotions = vec!["爱", "恨", "喜", "怒"];
        println!("\n情感类词汇相似度矩阵:");
        for w1 in &emotions {
            for w2 in &emotions {
                let sim = embedding.similarity(w1, w2);
                print!("{:.2} ", sim);
            }
            println!();
        }
    }

    #[test]
    fn test_load_tencent_embeddings() {
        // 加载腾讯词向量
        let config = EmbeddingConfig {
            dimension: 200,
            max_vocabulary: 1000, // 限制加载量以加快测试
            ..Default::default()
        };

        let embedding = WordEmbedding::from_word2vec_binary(
            "data/embeddings/tencent_chinese.bin",
            config,
        );

        if embedding.is_err() {
            println!("跳过测试：请先下载腾讯词向量到 data/embeddings/tencent_chinese.bin");
            return;
        }

        let embedding = embedding.unwrap();

        // 验证加载
        assert!(embedding.vocabulary_size() > 0);
        println!("腾讯词向量加载成功: {} 词", embedding.vocabulary_size());

        // 测试一些词
        let test_words = vec!["中国", "北京", "人工智能", "机器学习", "神经网络"];
        for word in &test_words {
            if embedding.contains(word) {
                let vec = embedding.get_vector(word);
                println!("'{}' 的向量维度: {}", word, vec.len());
            }
        }

        // 测试相似度
        if embedding.contains("中国") && embedding.contains("北京") {
            let sim = embedding.similarity("中国", "北京");
            println!("相似度 中国-北京: {:.4}", sim);
        }
    }
}
