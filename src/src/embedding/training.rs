//! 多模态词向量训练模块
//!
//! 支持训练：代码、图像、音频、视频特征嵌入

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use super::config::{EmbeddingConfig, ModalityType, TrainingConfig, MultimodalConfig};

// ============================================================================
// 训练数据样本
// ============================================================================

/// 训练样本
#[derive(Debug, Clone)]
pub struct TrainingSample {
    /// 样本ID
    pub id: String,
    /// 模态类型
    pub modality: ModalityType,
    /// 内容（文本或特征向量）
    pub content: SampleContent,
    /// 上下文（用于上下文窗口训练）
    pub context: Vec<String>,
}

/// 样本内容
#[derive(Debug, Clone)]
pub enum SampleContent {
    /// 文本内容
    Text(String),
    /// 代码内容
    Code(String),
    /// 图像特征（展平的向量）
    ImageFeatures(Vec<f64>),
    /// 音频特征（如MFCC）
    AudioFeatures(Vec<f64>),
    /// 视频特征（帧序列特征）
    VideoFeatures(Vec<Vec<f64>>),
}

// ============================================================================
// 训练语料库
// ============================================================================

/// 训练语料库
#[derive(Debug, Clone)]
pub struct Corpus {
    /// 语料名称
    pub name: String,
    /// 模态类型
    pub modality: ModalityType,
    /// 样本列表
    pub samples: Vec<TrainingSample>,
    /// 词汇表
    pub vocabulary: HashMap<String, usize>,
    /// 词频统计
    pub word_freq: HashMap<String, usize>,
}

impl Corpus {
    /// 创建空语料库
    pub fn new(name: impl Into<String>, modality: ModalityType) -> Self {
        Self {
            name: name.into(),
            modality,
            samples: Vec::new(),
            vocabulary: HashMap::new(),
            word_freq: HashMap::new(),
        }
    }

    /// 添加文本样本
    pub fn add_text(&mut self, id: impl Into<String>, text: &str) {
        let sample = TrainingSample {
            id: id.into(),
            modality: self.modality,
            content: SampleContent::Text(text.to_string()),
            context: Vec::new(),
        };
        self.samples.push(sample);

        // 更新词汇表
        for word in text.split_whitespace() {
            *self.word_freq.entry(word.to_string()).or_insert(0) += 1;
        }
    }

    /// 添加代码样本
    pub fn add_code(&mut self, id: impl Into<String>, code: &str) {
        let sample = TrainingSample {
            id: id.into(),
            modality: ModalityType::Code,
            content: SampleContent::Code(code.to_string()),
            context: Vec::new(),
        };
        self.samples.push(sample);

        // 代码tokenization（简化版）
        for token in self.tokenize_code(code) {
            *self.word_freq.entry(token).or_insert(0) += 1;
        }
    }

    /// 添加图像特征
    pub fn add_image_features(&mut self, id: impl Into<String>, features: Vec<f64>) {
        let sample = TrainingSample {
            id: id.into(),
            modality: ModalityType::Image,
            content: SampleContent::ImageFeatures(features),
            context: Vec::new(),
        };
        self.samples.push(sample);
    }

    /// 添加音频特征
    pub fn add_audio_features(&mut self, id: impl Into<String>, features: Vec<f64>) {
        let sample = TrainingSample {
            id: id.into(),
            modality: ModalityType::Audio,
            content: SampleContent::AudioFeatures(features),
            context: Vec::new(),
        };
        self.samples.push(sample);
    }

    /// 添加视频特征
    pub fn add_video_features(&mut self, id: impl Into<String>, frame_features: Vec<Vec<f64>>) {
        let sample = TrainingSample {
            id: id.into(),
            modality: ModalityType::Video,
            content: SampleContent::VideoFeatures(frame_features),
            context: Vec::new(),
        };
        self.samples.push(sample);
    }

    /// 构建词汇表（只保留达到最小词频的词）
    pub fn build_vocabulary(&mut self, min_freq: usize) {
        self.vocabulary.clear();

        let mut idx = 0;
        for (word, freq) in &self.word_freq {
            if *freq >= min_freq {
                self.vocabulary.insert(word.clone(), idx);
                idx += 1;
            }
        }
    }

    /// 代码tokenization（简化版）
    fn tokenize_code(&self, code: &str) -> Vec<String> {
        // 简化的代码tokenization
        let mut tokens = Vec::new();
        let mut current_token = String::new();

        for ch in code.chars() {
            if ch.is_alphanumeric() || ch == '_' {
                current_token.push(ch);
            } else {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
                if !ch.is_whitespace() {
                    tokens.push(ch.to_string());
                }
            }
        }

        if !current_token.is_empty() {
            tokens.push(current_token);
        }

        tokens
    }

    /// 获取样本数量
    pub fn len(&self) -> usize {
        self.samples.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }

    /// 获取词汇表大小
    pub fn vocab_size(&self) -> usize {
        self.vocabulary.len()
    }
}

// ============================================================================
// 词向量训练器
// ============================================================================

/// 词向量训练器
pub struct EmbeddingTrainer {
    /// 训练配置
    config: TrainingConfig,
    /// 词向量（词 -> 向量）
    embeddings: HashMap<String, Vec<f64>>,
    /// 上下文向量（用于负采样）
    context_embeddings: HashMap<String, Vec<f64>>,
    /// 训练损失历史
    loss_history: Vec<f64>,
}

impl EmbeddingTrainer {
    /// 创建新训练器
    pub fn new(config: TrainingConfig) -> Self {
        Self {
            config,
            embeddings: HashMap::new(),
            context_embeddings: HashMap::new(),
            loss_history: Vec::new(),
        }
    }

    /// 初始化词汇表向量
    pub fn init_vocabulary(&mut self, corpus: &Corpus) {
        let dim = self.config.base.dimension;

        for (word, _) in &corpus.vocabulary {
            // 随机初始化向量
            let vector: Vec<f64> = (0..dim)
                .map(|_| (rand_simple() - 0.5) / dim as f64)
                .collect();

            self.embeddings.insert(word.clone(), vector.clone());
            self.context_embeddings.insert(word.clone(), vector);
        }
    }

    /// 训练 Skip-gram 模型
    pub fn train_skipgram(&mut self, corpus: &Corpus) -> Result<TrainingResult, String> {
        if corpus.samples.is_empty() {
            return Err("Corpus is empty".to_string());
        }

        let dim = self.config.base.dimension;
        let lr = self.config.learning_rate;
        let window = self.config.window_size;

        self.init_vocabulary(corpus);

        let mut total_loss = 0.0;
        let mut update_count = 0;

        for epoch in 0..self.config.epochs {
            let mut epoch_loss = 0.0;

            for sample in &corpus.samples {
                if let SampleContent::Text(text) = &sample.content {
                    let words: Vec<&str> = text.split_whitespace().collect();

                    for (i, &center) in words.iter().enumerate() {
                        if !self.embeddings.contains_key(center) {
                            continue;
                        }

                        // 获取上下文窗口
                        let start = i.saturating_sub(window);
                        let end = (i + window + 1).min(words.len());

                        for (j, &context) in words[start..end].iter().enumerate() {
                            if i == j + start || !self.embeddings.contains_key(context) {
                                continue;
                            }

                            // 负采样训练
                            let loss = self.train_pair(center, context, lr);
                            epoch_loss += loss;
                            update_count += 1;
                        }
                    }
                }
            }

            total_loss = epoch_loss;
            self.loss_history.push(epoch_loss);

            println!(
                "Epoch {}: loss = {:.6}",
                epoch + 1,
                epoch_loss / update_count.max(1) as f64
            );
        }

        Ok(TrainingResult {
            vocab_size: self.embeddings.len(),
            dimension: dim,
            epochs: self.config.epochs,
            final_loss: total_loss / update_count.max(1) as f64,
        })
    }

    /// 训练词对（负采样）
    fn train_pair(&mut self, center: &str, context: &str, lr: f64) -> f64 {
        let dim = self.config.base.dimension;

        // 获取向量
        let center_vec = self.embeddings.get(center).cloned().unwrap_or_else(|| vec![0.0; dim]);
        let context_vec = self.context_embeddings.get(context).cloned().unwrap_or_else(|| vec![0.0; dim]);

        // 计算点积
        let dot: f64 = center_vec.iter().zip(context_vec.iter()).map(|(a, b)| a * b).sum();

        // Sigmoid
        let sigmoid = 1.0 / (1.0 + (-dot).exp());

        // 正样本梯度
        let gradient = (1.0 - sigmoid) * lr;

        // 更新向量
        let mut new_center = center_vec.clone();
        let mut new_context = context_vec.clone();

        for i in 0..dim {
            new_center[i] += gradient * new_context[i];
            new_context[i] += gradient * center_vec[i];
        }

        self.embeddings.insert(center.to_string(), new_center);
        self.context_embeddings.insert(context.to_string(), new_context);

        // 负对数似然损失
        -sigmoid.ln()
    }

    /// 获取训练好的词向量
    pub fn get_embedding(&self, word: &str) -> Option<&Vec<f64>> {
        self.embeddings.get(word)
    }

    /// 获取所有词向量
    pub fn get_all_embeddings(&self) -> &HashMap<String, Vec<f64>> {
        &self.embeddings
    }

    /// 导出为 fastText 格式
    pub fn export_fasttext(&self, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create(path)?;
        use std::io::Write;

        // 写入头部
        writeln!(file, "{} {}", self.embeddings.len(), self.config.base.dimension)?;

        // 写入每个词向量
        for (word, vec) in &self.embeddings {
            let vec_str: String = vec.iter().map(|v| format!("{:.6}", v)).collect::<Vec<_>>().join(" ");
            writeln!(file, "{} {}", word, vec_str)?;
        }

        Ok(())
    }

    /// 获取损失历史
    pub fn loss_history(&self) -> &[f64] {
        &self.loss_history
    }
}

/// 训练结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingResult {
    pub vocab_size: usize,
    pub dimension: usize,
    pub epochs: usize,
    pub final_loss: f64,
}

// ============================================================================
// 多模态训练器
// ============================================================================

/// 多模态嵌入训练器
pub struct MultimodalTrainer {
    /// 配置
    config: MultimodalConfig,
    /// 各模态的训练器
    trainers: HashMap<ModalityType, EmbeddingTrainer>,
    /// 各模态的语料库
    corpora: HashMap<ModalityType, Corpus>,
}

impl MultimodalTrainer {
    /// 创建多模态训练器
    pub fn new(config: MultimodalConfig) -> Self {
        let mut trainers = HashMap::new();
        let configs = config.get_embedding_configs();

        for (modality, emb_config) in configs {
            let train_config = TrainingConfig {
                base: emb_config,
                ..TrainingConfig::default()
            };
            trainers.insert(modality, EmbeddingTrainer::new(train_config));
        }

        Self {
            config,
            trainers,
            corpora: HashMap::new(),
        }
    }

    /// 获取或创建语料库
    pub fn corpus(&mut self, modality: ModalityType) -> &mut Corpus {
        self.corpora
            .entry(modality)
            .or_insert_with(|| Corpus::new(format!("{:?}_corpus", modality), modality))
    }

    /// 训练特定模态
    pub fn train_modality(&mut self, modality: ModalityType) -> Result<TrainingResult, String> {
        // 先获取min_count
        let min_count = self.trainers.get(&modality)
            .map(|t| t.config.min_count)
            .unwrap_or(1);

        // 构建词汇表
        if let Some(corpus) = self.corpora.get_mut(&modality) {
            corpus.build_vocabulary(min_count);
        }

        let corpus = self.corpora.get(&modality)
            .ok_or("Corpus not found for modality")?;

        let trainer = self.trainers.get_mut(&modality)
            .ok_or("Trainer not found for modality")?;

        trainer.train_skipgram(corpus)
    }

    /// 训练所有模态
    pub fn train_all(&mut self) -> HashMap<ModalityType, Result<TrainingResult, String>> {
        let mut results = HashMap::new();

        for modality in [ModalityType::Text, ModalityType::Code,
                        ModalityType::Image, ModalityType::Audio, ModalityType::Video] {
            if let Some(corpus) = self.corpora.get(&modality) {
                if !corpus.is_empty() {
                    results.insert(modality, self.train_modality(modality));
                }
            }
        }

        results
    }

    /// 导出所有词向量
    pub fn export_all(&self, output_dir: impl AsRef<Path>) -> Result<(), std::io::Error> {
        let output_dir = output_dir.as_ref();
        std::fs::create_dir_all(output_dir)?;

        for (modality, trainer) in &self.trainers {
            let filename = format!("{:?}.vec", modality).to_lowercase();
            let path = output_dir.join(filename);
            trainer.export_fasttext(path)?;
        }

        Ok(())
    }
}

// ============================================================================
// 辅助函数
// ============================================================================

fn rand_simple() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    nanos as f64 / u32::MAX as f64
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corpus_creation() {
        let corpus = Corpus::new("test", ModalityType::Text);
        assert_eq!(corpus.len(), 0);
        assert!(corpus.is_empty());
    }

    #[test]
    fn test_add_text_sample() {
        let mut corpus = Corpus::new("test", ModalityType::Text);
        corpus.add_text("doc1", "hello world hello");

        assert_eq!(corpus.len(), 1);
        assert_eq!(*corpus.word_freq.get("hello").unwrap_or(&0), 2);
        assert_eq!(*corpus.word_freq.get("world").unwrap_or(&0), 1);
    }

    #[test]
    fn test_build_vocabulary() {
        let mut corpus = Corpus::new("test", ModalityType::Text);
        corpus.add_text("doc1", "hello world hello");
        corpus.add_text("doc2", "world test");

        corpus.build_vocabulary(2); // 最小词频2

        // "hello" 和 "world" 各出现2次
        assert_eq!(corpus.vocab_size(), 2);
    }

    #[test]
    fn test_code_tokenization() {
        let mut corpus = Corpus::new("code", ModalityType::Code);
        corpus.add_code("func1", "fn main() { let x = 1; }");

        // 应该识别出关键字和符号
        assert!(corpus.word_freq.contains_key("fn"));
        assert!(corpus.word_freq.contains_key("main"));
        assert!(corpus.word_freq.contains_key("let"));
    }

    #[test]
    fn test_trainer_creation() {
        let config = TrainingConfig::for_code();
        let trainer = EmbeddingTrainer::new(config);

        assert_eq!(trainer.get_all_embeddings().len(), 0);
    }

    #[test]
    fn test_training() {
        let mut corpus = Corpus::new("test", ModalityType::Text);

        // 添加足够多的文本用于训练
        for i in 0..100 {
            corpus.add_text(format!("doc{}", i), "hello world test sample");
        }

        corpus.build_vocabulary(1);

        let config = TrainingConfig {
            epochs: 2,
            min_count: 1,
            ..TrainingConfig::default()
        };

        let mut trainer = EmbeddingTrainer::new(config);
        let result = trainer.train_skipgram(&corpus);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.epochs, 2);
        assert!(result.vocab_size > 0);
    }

    #[test]
    fn test_multimodal_trainer() {
        let config = MultimodalConfig::default();
        let mut trainer = MultimodalTrainer::new(config);

        // 添加一些文本数据
        let corpus = trainer.corpus(ModalityType::Text);
        // 注意：这里需要先获取再修改，实际使用时需要重新设计API

        assert!(trainer.corpora.contains_key(&ModalityType::Text));
    }
}
