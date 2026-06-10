//! 万物生成器配置
//!
//! 所有参数通过配置管理，遵循"禁止硬编码"原则

use serde::{Deserialize, Serialize};

/// 万物生成器总配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldGenConfig {
    /// 图谱构建配置
    pub graph: GraphBuildConfig,
    /// 脉络生成配置
    pub meridian: MeridianConfig,
}

impl Default for WorldGenConfig {
    fn default() -> Self {
        Self {
            graph: GraphBuildConfig::default(),
            meridian: MeridianConfig::default(),
        }
    }
}

impl WorldGenConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<(), String> {
        self.graph.validate()?;
        self.meridian.validate()?;
        Ok(())
    }
}

/// 图谱构建配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphBuildConfig {
    /// 实体识别置信度阈值
    pub entity_threshold: f64,
    /// 关系抽取置信度阈值
    pub relation_threshold: f64,
    /// 最大实体数量
    pub max_entities: usize,
    /// 最大关系数量
    pub max_relations: usize,
    /// 概念匹配阈值
    pub concept_match_threshold: f64,
    /// 实体合并阈值
    pub entity_merge_threshold: f64,
    /// 最大关系距离（字符数）
    pub max_relation_distance: usize,
}

impl Default for GraphBuildConfig {
    fn default() -> Self {
        Self {
            entity_threshold: 0.5,
            relation_threshold: 0.3,
            max_entities: 1000,
            max_relations: 5000,
            concept_match_threshold: 0.7,
            entity_merge_threshold: 0.9,
            max_relation_distance: 50,
        }
    }
}

impl GraphBuildConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.entity_threshold <= 0.0 || self.entity_threshold > 1.0 {
            return Err("entity_threshold must be in (0, 1]".to_string());
        }
        if self.relation_threshold <= 0.0 || self.relation_threshold > 1.0 {
            return Err("relation_threshold must be in (0, 1]".to_string());
        }
        if self.max_entities == 0 {
            return Err("max_entities must be positive".to_string());
        }
        if self.max_relations == 0 {
            return Err("max_relations must be positive".to_string());
        }
        if self.concept_match_threshold <= 0.0 || self.concept_match_threshold > 1.0 {
            return Err("concept_match_threshold must be in (0, 1]".to_string());
        }
        if self.entity_merge_threshold <= 0.0 || self.entity_merge_threshold > 1.0 {
            return Err("entity_merge_threshold must be in (0, 1]".to_string());
        }
        Ok(())
    }
}

/// 脉络生成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeridianConfig {
    /// 默认缩放
    pub default_scale: [f64; 3],
    /// 每节点最大通道数
    pub max_channels_per_node: usize,
    /// 布局算法
    pub layout_algorithm: LayoutAlgorithm,
    /// 默认展开深度
    pub default_expand_depth: usize,
    /// 位置随机扰动范围
    pub position_jitter: f64,
}

/// 布局算法类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutAlgorithm {
    /// 力导向布局
    ForceDirected,
    /// 层次布局
    Hierarchical,
    /// 圆形布局
    Circular,
    /// 网格布局
    Grid,
}

impl Default for MeridianConfig {
    fn default() -> Self {
        Self {
            default_scale: [1.0, 1.0, 1.0],
            max_channels_per_node: 10,
            layout_algorithm: LayoutAlgorithm::ForceDirected,
            default_expand_depth: 5,
            position_jitter: 0.1,
        }
    }
}

impl MeridianConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.default_scale.iter().any(|&s| s <= 0.0) {
            return Err("default_scale must be positive".to_string());
        }
        if self.max_channels_per_node == 0 {
            return Err("max_channels_per_node must be positive".to_string());
        }
        if self.default_expand_depth == 0 {
            return Err("default_expand_depth must be positive".to_string());
        }
        if self.position_jitter < 0.0 {
            return Err("position_jitter must be non-negative".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_valid() {
        let config = WorldGenConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_graph_config_validation() {
        let mut config = GraphBuildConfig::default();
        assert!(config.validate().is_ok());

        config.entity_threshold = 1.5;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_meridian_config_validation() {
        let mut config = MeridianConfig::default();
        assert!(config.validate().is_ok());

        config.default_scale = [0.0, 1.0, 1.0];
        assert!(config.validate().is_err());
    }
}
