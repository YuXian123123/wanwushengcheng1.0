//! 解码器模块
//!
//! 将内部表示转换为自然语言
//!
//! # 解码流程
//!
//! ```text
//! 内部表示 → 意图解析 → 概念规划 → 语言生成 → 自然语言
//! ```

mod intent;
mod generator;
mod planner;

pub use intent::{Intent, IntentType};
pub use generator::{Generator, DecodedResult, Style};
pub use planner::{Planner, PlanResult};

use crate::language::concept::ConceptVector;
use crate::config::concept::ConceptConfig;

/// 解码器
pub struct Decoder {
    /// 语言生成器
    generator: Generator,
    /// 概念规划器
    planner: Planner,
    /// 概念配置
    config: ConceptConfig,
}

impl Decoder {
    /// 创建新解码器
    pub fn new() -> Self {
        Self {
            generator: Generator::new(),
            planner: Planner::new(),
            config: ConceptConfig::new(),
        }
    }

    /// 使用配置创建解码器
    pub fn with_config(config: ConceptConfig) -> Self {
        Self {
            generator: Generator::new(),
            planner: Planner::new(),
            config,
        }
    }

    /// 设置风格
    pub fn with_style(style: Style) -> Self {
        Self {
            generator: Generator::new().with_style(style),
            planner: Planner::new(),
            config: ConceptConfig::new(),
        }
    }

    /// 解码向量序列
    pub fn decode(&self, vectors: &[ConceptVector]) -> DecodedResult {
        // 简化实现：将向量序列转换为单个向量后解析意图
        let aggregated = self.aggregate_vectors(vectors);
        let intent = Intent::from_vector(&aggregated);

        self.generator.generate(&intent)
    }

    /// 聚合向量
    fn aggregate_vectors(&self, vectors: &[ConceptVector]) -> Vec<f64> {
        let dim = self.config.vector_dim;
        if vectors.is_empty() {
            return vec![0.0; dim];
        }

        let mut result = vec![0.0; dim];

        for vector in vectors {
            for (i, &v) in vector.data.iter().enumerate() {
                if i < dim {
                    result[i] += v;
                }
            }
        }

        // 平均
        let len = vectors.len() as f64;
        for v in &mut result {
            *v /= len;
        }

        result
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}
