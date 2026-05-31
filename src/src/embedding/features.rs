//! 特征提取模块
//!
//! 从图像、音频、视频中提取特征向量

use serde::{Deserialize, Serialize};
use std::path::Path;

// ============================================================================
// 特征类型
// ============================================================================

/// 特征向量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureVector {
    /// 特征ID
    pub id: String,
    /// 特征类型
    pub feature_type: FeatureType,
    /// 特征向量
    pub vector: Vec<f64>,
    /// 元数据
    pub metadata: FeatureMetadata,
}

/// 特征类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeatureType {
    /// 图像特征（颜色直方图）
    ImageColorHistogram,
    /// 图像特征（边缘检测）
    ImageEdge,
    /// 图像特征（纹理）
    ImageTexture,
    /// 图像特征（组合）
    ImageCombined,
    /// 音频特征（MFCC）
    AudioMFCC,
    /// 音频特征（频谱）
    AudioSpectrum,
    /// 音频特征（能量）
    AudioEnergy,
    /// 音频特征（组合）
    AudioCombined,
    /// 视频特征（帧特征）
    VideoFrameFeatures,
    /// 视频特征（运动向量）
    VideoMotion,
    /// 视频特征（场景变化）
    VideoSceneChange,
    /// 视频特征（组合）
    VideoCombined,
}

/// 特征元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureMetadata {
    /// 来源文件
    pub source: Option<String>,
    /// 时间戳
    pub timestamp: Option<f64>,
    /// 帧号（视频）
    pub frame_number: Option<usize>,
    /// 维度
    pub dimension: usize,
}

// ============================================================================
// 图像特征提取器
// ============================================================================

/// 图像特征提取器
pub struct ImageFeatureExtractor {
    /// 输出维度
    output_dim: usize,
}

impl ImageFeatureExtractor {
    /// 创建新提取器
    pub fn new(output_dim: usize) -> Self {
        Self { output_dim }
    }

    /// 从文件提取特征（简化版，实际需要图像处理库）
    pub fn extract_from_file(&self, path: &Path) -> Result<FeatureVector, String> {
        // 检查文件是否存在
        if !path.exists() {
            return Err(format!("File not found: {:?}", path));
        }

        // 简化实现：生成基于文件名的伪特征
        // 实际应用中应该使用图像处理库如 image-rs
        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let vector = self.compute_pseudo_features(filename);

        Ok(FeatureVector {
            id: format!("img_{}", filename),
            feature_type: FeatureType::ImageCombined,
            vector,
            metadata: FeatureMetadata {
                source: Some(path.to_string_lossy().to_string()),
                timestamp: None,
                frame_number: None,
                dimension: self.output_dim,
            },
        })
    }

    /// 从像素数据提取特征
    pub fn extract_from_pixels(&self, pixels: &[u8], width: usize, height: usize) -> FeatureVector {
        let mut vector = Vec::with_capacity(self.output_dim);

        // 颜色直方图（简化版）
        let mut hist_r = vec![0.0; 64];
        let mut hist_g = vec![0.0; 64];
        let mut hist_b = vec![0.0; 64];

        for chunk in pixels.chunks(3) {
            if chunk.len() >= 3 {
                let r = (chunk[0] as usize / 4).min(63);
                let g = (chunk[1] as usize / 4).min(63);
                let b = (chunk[2] as usize / 4).min(63);
                hist_r[r] += 1.0;
                hist_g[g] += 1.0;
                hist_b[b] += 1.0;
            }
        }

        // 归一化
        let total = (width * height) as f64;
        for h in &mut hist_r { *h /= total; }
        for h in &mut hist_g { *h /= total; }
        for h in &mut hist_b { *h /= total; }

        // 组合特征
        vector.extend(hist_r);
        vector.extend(hist_g);
        vector.extend(hist_b);

        // 填充或截断到目标维度
        while vector.len() < self.output_dim {
            vector.push(0.0);
        }
        vector.truncate(self.output_dim);

        FeatureVector {
            id: format!("img_{}x{}", width, height),
            feature_type: FeatureType::ImageColorHistogram,
            vector,
            metadata: FeatureMetadata {
                source: None,
                timestamp: None,
                frame_number: None,
                dimension: self.output_dim,
            },
        }
    }

    /// 计算伪特征（用于测试）
    fn compute_pseudo_features(&self, seed: &str) -> Vec<f64> {
        // 基于字符串哈希生成确定性的伪特征
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        seed.hash(&mut hasher);
        let hash = hasher.finish();

        let mut vector = Vec::with_capacity(self.output_dim);
        let mut state = hash;

        for _ in 0..self.output_dim {
            // 简单的伪随机数生成
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let value = ((state >> 32) as f64 / u32::MAX as f64 - 0.5) * 2.0;
            vector.push(value);
        }

        // 归一化
        let norm: f64 = vector.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 0.0 {
            for v in &mut vector {
                *v /= norm;
            }
        }

        vector
    }
}

// ============================================================================
// 音频特征提取器
// ============================================================================

/// 音频特征提取器
pub struct AudioFeatureExtractor {
    /// 输出维度
    output_dim: usize,
    /// 采样率
    sample_rate: u32,
    /// MFCC系数数量
    mfcc_count: usize,
}

impl AudioFeatureExtractor {
    /// 创建新提取器
    pub fn new(output_dim: usize) -> Self {
        Self {
            output_dim,
            sample_rate: 16000,
            mfcc_count: 13,
        }
    }

    /// 从文件提取特征
    pub fn extract_from_file(&self, path: &Path) -> Result<FeatureVector, String> {
        if !path.exists() {
            return Err(format!("File not found: {:?}", path));
        }

        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let vector = self.compute_pseudo_features(filename);

        Ok(FeatureVector {
            id: format!("audio_{}", filename),
            feature_type: FeatureType::AudioCombined,
            vector,
            metadata: FeatureMetadata {
                source: Some(path.to_string_lossy().to_string()),
                timestamp: None,
                frame_number: None,
                dimension: self.output_dim,
            },
        })
    }

    /// 从采样数据提取MFCC特征（简化版）
    pub fn extract_mfcc(&self, samples: &[f64]) -> Vec<Vec<f64>> {
        // 简化实现：计算帧的能量和过零率
        let frame_size = 512;
        let hop_size = 256;
        let mut features = Vec::new();

        for start in (0..samples.len()).step_by(hop_size) {
            let end = (start + frame_size).min(samples.len());
            let frame = &samples[start..end];

            // 能量
            let energy: f64 = frame.iter().map(|x| x * x).sum::<f64>().sqrt();

            // 过零率
            let mut crossings = 0;
            for i in 1..frame.len() {
                if (frame[i] >= 0.0) != (frame[i - 1] >= 0.0) {
                    crossings += 1;
                }
            }
            let zcr = crossings as f64 / frame.len() as f64;

            // 构建特征向量（重复填充到mfcc_count）
            let mut frame_features = vec![energy, zcr];
            while frame_features.len() < self.mfcc_count {
                frame_features.push(0.0);
            }
            features.push(frame_features);
        }

        features
    }

    /// 计算伪特征
    fn compute_pseudo_features(&self, seed: &str) -> Vec<f64> {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        seed.hash(&mut hasher);
        let hash = hasher.finish();

        let mut vector = Vec::with_capacity(self.output_dim);
        let mut state = hash;

        for _ in 0..self.output_dim {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let value = ((state >> 32) as f64 / u32::MAX as f64 - 0.5) * 2.0;
            vector.push(value);
        }

        // 归一化
        let norm: f64 = vector.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 0.0 {
            for v in &mut vector {
                *v /= norm;
            }
        }

        vector
    }
}

// ============================================================================
// 视频特征提取器
// ============================================================================

/// 视频特征提取器
pub struct VideoFeatureExtractor {
    /// 输出维度
    output_dim: usize,
    /// 图像特征提取器
    image_extractor: ImageFeatureExtractor,
    /// 帧采样间隔
    frame_interval: usize,
}

impl VideoFeatureExtractor {
    /// 创建新提取器
    pub fn new(output_dim: usize) -> Self {
        Self {
            output_dim,
            image_extractor: ImageFeatureExtractor::new(output_dim),
            frame_interval: 30, // 每30帧采样一次
        }
    }

    /// 从文件提取特征
    pub fn extract_from_file(&self, path: &Path) -> Result<Vec<FeatureVector>, String> {
        if !path.exists() {
            return Err(format!("File not found: {:?}", path));
        }

        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        // 简化实现：生成多帧特征
        let frame_count = 10; // 假设10帧
        let mut features = Vec::new();

        for i in 0..frame_count {
            let seed = format!("{}_frame{}", filename, i);
            let vector = self.compute_pseudo_features(&seed);

            features.push(FeatureVector {
                id: format!("video_{}_frame{}", filename, i),
                feature_type: FeatureType::VideoFrameFeatures,
                vector,
                metadata: FeatureMetadata {
                    source: Some(path.to_string_lossy().to_string()),
                    timestamp: Some(i as f64 * 0.1),
                    frame_number: Some(i),
                    dimension: self.output_dim,
                },
            });
        }

        Ok(features)
    }

    /// 计算伪特征
    fn compute_pseudo_features(&self, seed: &str) -> Vec<f64> {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        seed.hash(&mut hasher);
        let hash = hasher.finish();

        let mut vector = Vec::with_capacity(self.output_dim);
        let mut state = hash;

        for _ in 0..self.output_dim {
            state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let value = ((state >> 32) as f64 / u32::MAX as f64 - 0.5) * 2.0;
            vector.push(value);
        }

        let norm: f64 = vector.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 0.0 {
            for v in &mut vector {
                *v /= norm;
            }
        }

        vector
    }
}

// ============================================================================
// 统一特征提取接口
// ============================================================================

/// 统一特征提取器
pub struct UnifiedFeatureExtractor {
    /// 图像提取器
    pub image: ImageFeatureExtractor,
    /// 音频提取器
    pub audio: AudioFeatureExtractor,
    /// 视频提取器
    pub video: VideoFeatureExtractor,
}

impl UnifiedFeatureExtractor {
    /// 创建新提取器
    pub fn new(image_dim: usize, audio_dim: usize, video_dim: usize) -> Self {
        Self {
            image: ImageFeatureExtractor::new(image_dim),
            audio: AudioFeatureExtractor::new(audio_dim),
            video: VideoFeatureExtractor::new(video_dim),
        }
    }

    /// 从目录提取所有特征
    pub fn extract_from_directory(&self, dir: &Path) -> Result<Vec<FeatureVector>, String> {
        if !dir.is_dir() {
            return Err(format!("Not a directory: {:?}", dir));
        }

        let mut features = Vec::new();

        for entry in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                match ext.to_lowercase().as_str() {
                    "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" => {
                        if let Ok(f) = self.image.extract_from_file(&path) {
                            features.push(f);
                        }
                    }
                    "mp3" | "wav" | "flac" | "ogg" | "m4a" => {
                        if let Ok(f) = self.audio.extract_from_file(&path) {
                            features.push(f);
                        }
                    }
                    "mp4" | "avi" | "mkv" | "mov" | "webm" => {
                        if let Ok(frame_features) = self.video.extract_from_file(&path) {
                            features.extend(frame_features);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(features)
    }
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_extractor_creation() {
        let extractor = ImageFeatureExtractor::new(512);
        assert_eq!(extractor.output_dim, 512);
    }

    #[test]
    fn test_audio_extractor_creation() {
        let extractor = AudioFeatureExtractor::new(256);
        assert_eq!(extractor.output_dim, 256);
    }

    #[test]
    fn test_video_extractor_creation() {
        let extractor = VideoFeatureExtractor::new(512);
        assert_eq!(extractor.output_dim, 512);
    }

    #[test]
    fn test_pseudo_features_deterministic() {
        let extractor = ImageFeatureExtractor::new(128);
        let f1 = extractor.compute_pseudo_features("test");
        let f2 = extractor.compute_pseudo_features("test");

        assert_eq!(f1.len(), f2.len());
        for (a, b) in f1.iter().zip(f2.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }

    #[test]
    fn test_pseudo_features_normalized() {
        let extractor = ImageFeatureExtractor::new(128);
        let f = extractor.compute_pseudo_features("test");

        let norm: f64 = f.iter().map(|x| x * x).sum::<f64>().sqrt();
        assert!((norm - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_audio_mfcc() {
        let extractor = AudioFeatureExtractor::new(256);

        // 生成测试音频数据
        let samples: Vec<f64> = (0..16000).map(|i| (i as f64 / 100.0).sin()).collect();

        let mfcc = extractor.extract_mfcc(&samples);

        assert!(!mfcc.is_empty());
        assert_eq!(mfcc[0].len(), extractor.mfcc_count);
    }

    #[test]
    fn test_feature_metadata() {
        let feature = FeatureVector {
            id: "test".to_string(),
            feature_type: FeatureType::ImageColorHistogram,
            vector: vec![0.0; 512],
            metadata: FeatureMetadata {
                source: Some("test.png".to_string()),
                timestamp: None,
                frame_number: None,
                dimension: 512,
            },
        };

        assert_eq!(feature.metadata.dimension, 512);
        assert!(feature.metadata.source.is_some());
    }
}
