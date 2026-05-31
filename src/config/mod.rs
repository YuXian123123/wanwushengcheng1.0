//! 配置系统
//!
//! 统一管理所有模块参数，避免硬编码
//!
//! # 设计原则
//! - 所有数值参数通过配置管理
//! - 支持运行时动态调整
//! - 参数验证和边界检查

pub mod concept;
pub mod learning;
pub mod consensus;
pub mod context;
pub mod tokenizer;

use serde::{Deserialize, Serialize};

/// 全局配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// 概念配置
    pub concept: concept::ConceptConfig,
    /// 学习配置
    pub learning: learning::LearningConfig,
    /// 共识配置
    pub consensus: consensus::ConsensusConfig,
    /// 上下文配置
    pub context: context::ContextConfig,
    /// 分词器配置
    pub tokenizer: tokenizer::TokenizerConfig,
}

impl GlobalConfig {
    /// 创建默认配置
    pub fn new() -> Self {
        Self {
            concept: concept::ConceptConfig::new(),
            learning: learning::LearningConfig::new(),
            consensus: consensus::ConsensusConfig::new(),
            context: context::ContextConfig::new(),
            tokenizer: tokenizer::TokenizerConfig::new(),
        }
    }

    /// 从文件加载配置
    pub fn from_file(path: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("读取配置文件失败: {}", e))?;

        let config: Self = toml::from_str(&content)
            .map_err(|e| format!("解析配置文件失败: {}", e))?;

        config.validate()?;
        Ok(config)
    }

    /// 保存配置到文件
    pub fn to_file(&self, path: &str) -> Result<(), String> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("序列化配置失败: {}", e))?;

        std::fs::write(path, content)
            .map_err(|e| format!("写入配置文件失败: {}", e))
    }

    /// 验证配置有效性
    pub fn validate(&self) -> Result<(), String> {
        self.concept.validate()?;
        self.learning.validate()?;
        self.consensus.validate()?;
        self.context.validate()?;
        self.tokenizer.validate()?;
        Ok(())
    }
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_valid() {
        let config = GlobalConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_serialization() {
        let config = GlobalConfig::new();
        let toml = toml::to_string(&config).unwrap();
        let parsed: GlobalConfig = toml::from_str(&toml).unwrap();
        assert!(parsed.validate().is_ok());
    }
}
