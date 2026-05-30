//! 概念配置

use serde::{Deserialize, Serialize};
use std::ops::Range;

/// 概念向量配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptConfig {
    /// 向量维度
    ///
    /// 决定概念在高维空间中的表示精度
    /// - 128: 快速但精度较低
    /// - 256: 平衡性能和精度（推荐）
    /// - 512: 高精度但计算量大
    pub vector_dim: usize,

    /// 向量归一化阈值
    ///
    /// 当范数小于此值时跳过归一化，避免除零
    pub normalization_threshold: f64,

    /// 继承扰动范围
    ///
    /// 子概念从父概念继承时，向量元素的随机扰动范围
    pub inheritance_perturbation: Range<f64>,

    /// 随机初始化范围
    ///
    /// 新概念创建时，向量元素的随机初始化范围
    pub random_init_range: Range<f64>,

    /// 相似度计算阈值
    ///
    /// 当范数小于此值时，相似度返回0
    pub similarity_threshold: f64,
}

impl ConceptConfig {
    /// 创建默认配置
    pub fn new() -> Self {
        Self {
            vector_dim: 256,
            normalization_threshold: 1e-10,
            inheritance_perturbation: -0.05..0.05,
            random_init_range: -0.1..0.1,
            similarity_threshold: 1e-10,
        }
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), String> {
        if self.vector_dim == 0 {
            return Err("vector_dim 必须大于0".to_string());
        }
        if self.vector_dim > 1024 {
            return Err("vector_dim 不应超过1024（性能考虑）".to_string());
        }
        if self.normalization_threshold <= 0.0 {
            return Err("normalization_threshold 必须为正数".to_string());
        }
        if self.inheritance_perturbation.start >= self.inheritance_perturbation.end {
            return Err("inheritance_perturbation 范围无效".to_string());
        }
        if self.random_init_range.start >= self.random_init_range.end {
            return Err("random_init_range 范围无效".to_string());
        }
        Ok(())
    }

    /// 获取向量维度
    pub fn vector_dim(&self) -> usize {
        self.vector_dim
    }

    /// 检查是否应该归一化
    pub fn should_normalize(&self, norm: f64) -> bool {
        norm > self.normalization_threshold
    }
}

impl Default for ConceptConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_valid() {
        let config = ConceptConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_vector_dim() {
        let mut config = ConceptConfig::new();
        config.vector_dim = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_vector_dim_too_large() {
        let mut config = ConceptConfig::new();
        config.vector_dim = 2048;
        assert!(config.validate().is_err());
    }
}
