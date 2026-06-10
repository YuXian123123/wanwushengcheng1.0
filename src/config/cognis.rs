//! 认知素分解器配置
//!
//! 管理 CognisParser 和 KnowledgeEncoder 的所有参数

use serde::{Deserialize, Serialize};

/// 认知素分解器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognisConfig {
    /// 最大提取的高频词数量
    pub max_frequent_words: usize,
    /// 代码语言标识符最大长度
    pub max_lang_identifier_length: usize,
    /// 最大提取的代码语言数量
    pub max_code_languages: usize,
    /// 扫描标题的最大行数
    pub max_title_scan_lines: usize,
    /// 单词最小长度
    pub min_word_length: usize,
    /// 单词最大长度
    pub max_word_length: usize,
    /// 非大写单词最大长度
    pub max_non_uppercase_word_length: usize,
    /// 最大标题词数量
    pub max_title_words: usize,
    /// 内容预览最大字符数
    pub content_preview_chars: usize,
    /// 高频词阈值
    pub frequent_word_threshold: usize,
}

impl Default for CognisConfig {
    fn default() -> Self {
        Self {
            max_frequent_words: 10,
            max_lang_identifier_length: 20,
            max_code_languages: 3,
            max_title_scan_lines: 50,
            min_word_length: 2,
            max_word_length: 30,
            max_non_uppercase_word_length: 10,
            max_title_words: 5,
            content_preview_chars: 3000,
            frequent_word_threshold: 2,
        }
    }
}

impl CognisConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.max_frequent_words == 0 {
            return Err("max_frequent_words 必须大于0".to_string());
        }
        if self.min_word_length == 0 {
            return Err("min_word_length 必须大于0".to_string());
        }
        if self.max_word_length < self.min_word_length {
            return Err("max_word_length 必须大于等于 min_word_length".to_string());
        }
        Ok(())
    }
}

/// 知识编码器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEncoderConfig {
    /// 最大信号强度
    pub max_signal_strength: f64,

    // 实体类型权重
    /// 代码语言权重
    pub code_language_weight: f64,
    /// 技术术语权重
    pub tech_term_weight: f64,
    /// 概念权重
    pub concept_weight: f64,
    /// 关键词权重
    pub keyword_weight: f64,
    /// 其他实体权重
    pub other_entity_weight: f64,

    // 关系类型权重
    /// 包含关系权重
    pub contains_relation_weight: f64,
    /// 属于关系权重
    pub belongs_to_relation_weight: f64,
    /// 依赖关系权重
    pub depends_on_relation_weight: f64,
    /// 相似关系权重
    pub similar_to_relation_weight: f64,
    /// 关联关系权重
    pub related_to_relation_weight: f64,

    // 神经信号分配
    /// 实体感知信号比例
    pub entity_perception_ratio: f64,
    /// 实体认知信号比例
    pub entity_cognitive_ratio: f64,
    /// 属性认知信号
    pub attribute_cognitive_signal: f64,
    /// 关系认知信号比例
    pub relation_cognitive_ratio: f64,
    /// 关系通信信号比例
    pub relation_comm_ratio: f64,

    // 知识价值计算权重
    /// 实体数量权重
    pub entity_count_weight: f64,
    /// 关系数量权重
    pub relation_count_weight: f64,
    /// 代码语言数量权重
    pub code_language_count_weight: f64,
    /// 关键词数量权重
    pub keyword_count_weight: f64,
    /// 有主题权重
    pub has_topic_weight: f64,
}

impl Default for KnowledgeEncoderConfig {
    fn default() -> Self {
        Self {
            max_signal_strength: 0.5,

            code_language_weight: 1.0,
            tech_term_weight: 0.8,
            concept_weight: 0.6,
            keyword_weight: 0.4,
            other_entity_weight: 0.2,

            contains_relation_weight: 0.7,
            belongs_to_relation_weight: 0.6,
            depends_on_relation_weight: 0.8,
            similar_to_relation_weight: 0.5,
            related_to_relation_weight: 0.3,

            entity_perception_ratio: 0.3,
            entity_cognitive_ratio: 0.5,
            attribute_cognitive_signal: 0.4,
            relation_cognitive_ratio: 0.4,
            relation_comm_ratio: 0.3,

            entity_count_weight: 0.3,
            relation_count_weight: 0.2,
            code_language_count_weight: 0.25,
            keyword_count_weight: 0.1,
            has_topic_weight: 0.15,
        }
    }
}

impl KnowledgeEncoderConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.max_signal_strength <= 0.0 || self.max_signal_strength > 1.0 {
            return Err("max_signal_strength 必须在 (0, 1] 范围内".to_string());
        }

        let weights_sum = self.entity_count_weight
            + self.relation_count_weight
            + self.code_language_count_weight
            + self.keyword_count_weight
            + self.has_topic_weight;

        if (weights_sum - 1.0).abs() > 0.01 {
            return Err(format!("知识价值权重之和应该为1.0，当前为 {}", weights_sum));
        }

        Ok(())
    }
}

/// 知识消耗流程配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeConsumptionConfig {
    /// 学习信号乘数
    pub learning_signal_multiplier: f64,
    /// 最大赫布更新迭代次数
    pub max_hebbian_iterations: usize,
    /// LNN 更新时间步长
    pub lnn_dt: f64,
    /// 信任奖励乘数
    pub trust_reward_multiplier: f64,
}

impl Default for KnowledgeConsumptionConfig {
    fn default() -> Self {
        Self {
            learning_signal_multiplier: 0.3,
            max_hebbian_iterations: 10,
            lnn_dt: 0.01,
            trust_reward_multiplier: 0.05,
        }
    }
}

impl KnowledgeConsumptionConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.lnn_dt <= 0.0 {
            return Err("lnn_dt 必须大于0".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cognis_config_validation() {
        let config = CognisConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_encoder_config_validation() {
        let config = KnowledgeEncoderConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_consumption_config_validation() {
        let config = KnowledgeConsumptionConfig::new();
        assert!(config.validate().is_ok());
    }
}
