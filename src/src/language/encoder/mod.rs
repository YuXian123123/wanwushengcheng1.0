//! 编码器模块
//!
//! 将自然语言转换为内部表示
//!
//! # 编码流程
//!
//! ```text
//! 自然语言 → 分词 → 词嵌入 → 上下文消歧 → 概念对齐 → 内部表示
//! ```

mod tokenizer;
mod aligner;
mod embedding;
mod disambiguator;

pub use tokenizer::{Tokenizer, Token, TokenType};
pub use aligner::{Aligner, EncodedResult};
pub use embedding::Embedding;
pub use disambiguator::{Disambiguator, DisambiguationResult};

use crate::config::GlobalConfig;

/// 编码器
pub struct Encoder {
    /// 分词器
    tokenizer: Tokenizer,
    /// 概念对齐器
    aligner: Aligner,
    /// 词嵌入器
    embedding: Embedding,
    /// 消歧器
    disambiguator: Disambiguator,
}

impl Encoder {
    /// 创建新编码器
    pub fn new() -> Self {
        Self {
            tokenizer: Tokenizer::new(),
            aligner: Aligner::new(),
            embedding: Embedding::new(),
            disambiguator: Disambiguator::new(),
        }
    }

    /// 使用配置创建编码器
    pub fn with_config(config: &GlobalConfig) -> Self {
        Self {
            tokenizer: Tokenizer::new(),
            aligner: Aligner::with_config(config.concept.clone()),
            embedding: Embedding::with_config(config.concept.clone()),
            disambiguator: Disambiguator::new(),
        }
    }

    /// 编码文本（简化版）
    pub fn encode(&self, text: &str) -> EncodedResult {
        // 分词
        let tokens = self.tokenizer.tokenize(text);

        // 概念对齐
        let result = self.aligner.align(&tokens);

        result
    }

    /// 完整编码流程
    ///
    /// 包含：分词 → 嵌入 → 消歧 → 对齐
    pub fn encode_full(&mut self, text: &str) -> EncodedResult {
        // 1. 分词
        let tokens = self.tokenizer.tokenize(text);

        // 2. 词嵌入
        let words: Vec<String> = tokens.iter().map(|t| t.text.clone()).collect();
        let vectors = self.embedding.embed_batch(&words);

        // 3. 上下文消歧
        let _disambiguation = self.disambiguator.disambiguate(&tokens, &vectors);

        // 4. 概念对齐
        let result = self.aligner.align(&tokens);

        result
    }

    /// 获取分词器引用
    pub fn tokenizer(&self) -> &Tokenizer {
        &self.tokenizer
    }

    /// 获取嵌入器可变引用
    pub fn embedding(&mut self) -> &mut Embedding {
        &mut self.embedding
    }

    /// 获取消歧器引用
    pub fn disambiguator(&self) -> &Disambiguator {
        &self.disambiguator
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}
