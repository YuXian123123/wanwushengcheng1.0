//! 内容质量检测 - 螺丝咕姆第一层防护

use serde::{Deserialize, Serialize};

/// 质量检测配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityConfig {
    /// 质量阈值
    pub threshold: f64,
    /// 内容长度阈值 - 低
    pub length_threshold_low: usize,
    /// 内容长度阈值 - 中
    pub length_threshold_medium: usize,
    /// 内容长度阈值 - 高
    pub length_threshold_high: usize,
}

impl Default for QualityConfig {
    fn default() -> Self {
        Self {
            threshold: 0.5,
            length_threshold_low: 10,
            length_threshold_medium: 50,
            length_threshold_high: 100,
        }
    }
}

/// 内容质量评分
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityScore {
    /// 总评分
    pub score: f64,
    /// 结构化程度 (0-1)
    pub structure: f64,
    /// 原创性 (0-1)
    pub originality: f64,
    /// 价值程度 (0-1)
    pub value: f64,
    /// 完整性 (0-1)
    pub completeness: f64,
}

impl QualityScore {
    pub fn new(structure: f64, originality: f64, value: f64, completeness: f64) -> Self {
        // Q = w₁·Structure + w₂·Originality + w₃·Value + w₄·Completeness
        let score = 0.25 * structure + 0.25 * originality + 0.30 * value + 0.20 * completeness;
        Self {
            score,
            structure,
            originality,
            value,
            completeness,
        }
    }
}

/// 质量检测器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityDetector {
    /// 配置
    config: QualityConfig,
}

impl QualityDetector {
    pub fn new(config: QualityConfig) -> Self {
        Self { config }
    }

    /// 使用默认配置创建
    pub fn with_defaults() -> Self {
        Self::new(QualityConfig::default())
    }

    /// 检测内容质量
    pub fn detect(&self, content: &str) -> QualityScore {
        let structure = self.detect_structure(content);
        let originality = self.detect_originality(content);
        let value = self.detect_value(content);
        let completeness = self.detect_completeness(content);

        QualityScore::new(structure, originality, value, completeness)
    }

    /// 结构化检测: S = ValidFormat(content) ? 1 : 0
    fn detect_structure(&self, content: &str) -> f64 {
        // 检测是否有结构化格式（列表、段落等）
        let has_structure = content.contains('\n')
            || content.contains(':')
            || content.contains('-')
            || content.len() > 20;

        if has_structure { 0.6 } else { 0.2 }
    }

    /// 原创性检测（简化版本）
    fn detect_originality(&self, content: &str) -> f64 {
        // 检测是否有独特词汇组合
        let unique_words = content.split_whitespace().collect::<Vec<_>>();
        let unique_ratio = if unique_words.is_empty() {
            0.0
        } else {
            unique_words.iter().filter(|w| w.len() > 2).count() as f64 / unique_words.len() as f64
        };

        unique_ratio.min(1.0)
    }

    /// 价值检测（使用配置阈值）
    fn detect_value(&self, content: &str) -> f64 {
        // 检测内容长度和丰富度
        let len = content.len();
        if len < self.config.length_threshold_low {
            0.1
        } else if len < self.config.length_threshold_medium {
            0.3
        } else if len < self.config.length_threshold_high {
            0.5
        } else {
            0.8
        }
    }

    /// 完整性检测
    fn detect_completeness(&self, content: &str) -> f64 {
        // 检测是否有完整的句子或段落
        let has_end = content.ends_with('.') || content.ends_with('。') || content.ends_with('!');
        let has_start = content.trim().len() > 0;

        if has_start && has_end {
            0.8
        } else if has_start {
            0.5
        } else {
            0.0
        }
    }

    /// 判断是否通过质量检测
    pub fn is_valid(&self, score: &QualityScore) -> bool {
        score.score >= self.config.threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_detection() {
        let detector = QualityDetector::with_defaults();
        let content = "这是一条有意义的知识分享内容。包含完整的句子结构。";

        let score = detector.detect(content);
        assert!(score.score > 0.0);
        assert!(detector.is_valid(&score));
    }

    #[test]
    fn test_low_quality_detection() {
        let config = QualityConfig {
            threshold: 0.5,
            ..Default::default()
        };
        let detector = QualityDetector::new(config.clone());
        let content = "短";

        let score = detector.detect(content);
        // 短内容应该是低质量
        assert!(!detector.is_valid(&score), "Short content should be low quality");
    }
}