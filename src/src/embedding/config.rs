//! 词向量配置模块
//!
//! 可配置的词向量系统，支持多种源和训练方式

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ============================================================================
// 词向量源类型
// ============================================================================

/// 词向量源类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmbeddingSourceType {
    /// fastText 预训练向量
    FastText,
    /// Word2Vec 格式
    Word2Vec,
    /// GloVe 格式
    GloVe,
    /// 自定义二进制格式
    Binary,
    /// 多模态嵌入（代码、图像、音频、视频）
    Multimodal,
}

// ============================================================================
// 词向量配置
// ============================================================================

/// 词向量基础配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    /// 向量维度
    pub dimension: usize,
    /// 最大词汇量（0表示无限制）
    pub max_vocabulary: usize,
    /// 最小词频（加载时过滤）
    pub min_frequency: usize,
    /// 是否归一化向量
    pub normalize: bool,
    /// 未知词向量策略
    pub unknown_strategy: UnknownStrategy,
    /// 源类型
    pub source_type: EmbeddingSourceType,
    /// 数据路径
    pub data_path: Option<PathBuf>,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            dimension: 300,
            max_vocabulary: 0,
            min_frequency: 1,
            normalize: true,
            unknown_strategy: UnknownStrategy::Zero,
            source_type: EmbeddingSourceType::FastText,
            data_path: None,
        }
    }
}

impl EmbeddingConfig {
    /// 创建 fastText 配置
    pub fn fasttext(dimension: usize) -> Self {
        Self {
            dimension,
            source_type: EmbeddingSourceType::FastText,
            ..Default::default()
        }
    }

    /// 创建多模态配置
    pub fn multimodal(dimension: usize) -> Self {
        Self {
            dimension,
            source_type: EmbeddingSourceType::Multimodal,
            ..Default::default()
        }
    }

    /// 创建代码向量配置
    pub fn code(dimension: usize) -> Self {
        Self {
            dimension,
            source_type: EmbeddingSourceType::Multimodal,
            data_path: Some(PathBuf::from("data/embeddings/code.vec")),
            ..Default::default()
        }
    }

    /// 设置数据路径
    pub fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.data_path = Some(path.into());
        self
    }
}

// ============================================================================
// 未知词策略
// ============================================================================

/// 未知词处理策略
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnknownStrategy {
    /// 零向量
    Zero,
    /// 随机向量
    Random,
    /// 平均向量
    Average,
    /// 子词分解（fastText风格）
    Subword,
}

// ============================================================================
// 训练配置
// ============================================================================

/// 词向量训练配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    /// 基础配置
    pub base: EmbeddingConfig,
    /// 学习率
    pub learning_rate: f64,
    /// 训练轮数
    pub epochs: usize,
    /// 上下文窗口大小
    pub window_size: usize,
    /// 负采样数量
    pub negative_samples: usize,
    /// 最小词频
    pub min_count: usize,
    /// 是否使用子词
    pub use_subwords: bool,
    /// 子词最小长度
    pub subword_min_len: usize,
    /// 子词最大长度
    pub subword_max_len: usize,
    /// 是否使用分层softmax
    pub hierarchical_softmax: bool,
    /// 批处理大小
    pub batch_size: usize,
    /// 线程数
    pub threads: usize,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            base: EmbeddingConfig::default(),
            learning_rate: 0.025,
            epochs: 5,
            window_size: 5,
            negative_samples: 5,
            min_count: 5,
            use_subwords: true,
            subword_min_len: 3,
            subword_max_len: 6,
            hierarchical_softmax: false,
            batch_size: 1024,
            threads: 4,
        }
    }
}

impl TrainingConfig {
    /// 创建代码训练配置
    pub fn for_code() -> Self {
        Self {
            base: EmbeddingConfig::code(256),
            window_size: 10, // 代码需要更大的上下文
            min_count: 1,    // 代码token都需要
            use_subwords: false, // 代码token通常不拆分
            ..Default::default()
        }
    }

    /// 创建图像特征训练配置
    pub fn for_image() -> Self {
        Self {
            base: EmbeddingConfig::multimodal(512),
            window_size: 1, // 图像特征不需要上下文窗口
            ..Default::default()
        }
    }

    /// 创建音频特征训练配置
    pub fn for_audio() -> Self {
        Self {
            base: EmbeddingConfig::multimodal(256),
            window_size: 3, // 音频帧的时序上下文
            ..Default::default()
        }
    }

    /// 创建视频特征训练配置
    pub fn for_video() -> Self {
        Self {
            base: EmbeddingConfig::multimodal(512),
            window_size: 5, // 视频帧序列上下文
            ..Default::default()
        }
    }
}

// ============================================================================
// 多模态配置
// ============================================================================

/// 多模态嵌入配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimodalConfig {
    /// 文本维度
    pub text_dim: usize,
    /// 代码维度
    pub code_dim: usize,
    /// 图像维度
    pub image_dim: usize,
    /// 音频维度
    pub audio_dim: usize,
    /// 视频维度
    pub video_dim: usize,
    /// 统一嵌入维度（融合后）
    pub unified_dim: usize,
    /// 是否进行跨模态对齐
    pub cross_modal_alignment: bool,
    /// 对齐温度参数
    pub alignment_temperature: f64,
}

impl Default for MultimodalConfig {
    fn default() -> Self {
        Self {
            text_dim: 300,
            code_dim: 256,
            image_dim: 512,
            audio_dim: 256,
            video_dim: 512,
            unified_dim: 512,
            cross_modal_alignment: true,
            alignment_temperature: 0.07,
        }
    }
}

impl MultimodalConfig {
    /// 总维度
    pub fn total_dim(&self) -> usize {
        self.text_dim + self.code_dim + self.image_dim + self.audio_dim + self.video_dim
    }

    /// 获取各模态配置
    pub fn get_embedding_configs(&self) -> HashMap<ModalityType, EmbeddingConfig> {
        let mut configs = HashMap::new();

        configs.insert(
            ModalityType::Text,
            EmbeddingConfig::fasttext(self.text_dim),
        );
        configs.insert(
            ModalityType::Code,
            EmbeddingConfig::code(self.code_dim),
        );
        configs.insert(
            ModalityType::Image,
            EmbeddingConfig::multimodal(self.image_dim),
        );
        configs.insert(
            ModalityType::Audio,
            EmbeddingConfig::multimodal(self.audio_dim),
        );
        configs.insert(
            ModalityType::Video,
            EmbeddingConfig::multimodal(self.video_dim),
        );

        configs
    }
}

// ============================================================================
// 模态类型
// ============================================================================

/// 模态类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModalityType {
    /// 纯文本
    Text,
    /// 代码
    Code,
    /// 图像
    Image,
    /// 音频
    Audio,
    /// 视频
    Video,
}

impl ModalityType {
    /// 获取文件扩展名
    pub fn extensions(&self) -> &[&str] {
        match self {
            Self::Text => &["txt", "md", "json", "yaml", "toml"],
            Self::Code => &["rs", "py", "js", "ts", "java", "cpp", "c", "go", "rb"],
            Self::Image => &["png", "jpg", "jpeg", "gif", "bmp", "webp"],
            Self::Audio => &["mp3", "wav", "flac", "ogg", "m4a"],
            Self::Video => &["mp4", "avi", "mkv", "mov", "webm"],
        }
    }

    /// 从文件路径推断模态
    pub fn from_path(path: &std::path::Path) -> Option<Self> {
        let ext = path.extension()?.to_str()?.to_lowercase();

        for (i, modality) in [Self::Text, Self::Code, Self::Image, Self::Audio, Self::Video]
            .iter()
            .enumerate()
        {
            // 跳过 Text，最后检查
            if i == 0 {
                continue;
            }
            if modality.extensions().contains(&ext.as_str()) {
                return Some(*modality);
            }
        }

        // 默认为文本
        Some(Self::Text)
    }
}

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_config_default() {
        let config = EmbeddingConfig::default();
        assert_eq!(config.dimension, 300);
        assert!(config.normalize);
    }

    #[test]
    fn test_training_config_code() {
        let config = TrainingConfig::for_code();
        assert_eq!(config.base.dimension, 256);
        assert_eq!(config.window_size, 10);
    }

    #[test]
    fn test_multimodal_config() {
        let config = MultimodalConfig::default();
        assert_eq!(config.unified_dim, 512);
        assert!(config.cross_modal_alignment);
    }

    #[test]
    fn test_modality_from_path() {
        assert_eq!(
            ModalityType::from_path(std::path::Path::new("test.rs")),
            Some(ModalityType::Code)
        );
        assert_eq!(
            ModalityType::from_path(std::path::Path::new("test.png")),
            Some(ModalityType::Image)
        );
        assert_eq!(
            ModalityType::from_path(std::path::Path::new("test.mp3")),
            Some(ModalityType::Audio)
        );
    }
}
