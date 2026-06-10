//! 知识信号编码器 - 将认知粒子转换为神经信号
//!
//! # 设计理念
//!
//! - 黑塔：知识 → 神经激活模式的映射
//! - 螺丝咕姆：信号强度归一化，防止过载
//! - 拉蒂奥：优雅的编码公式，最大化信息传递效率
//!
//! # 核心思想
//!
//! 每种认知粒子对应特定的神经激活模式：
//! - Entity → Perceive + Cognitive (感知并认知实体)
//! - Attribute → Cognitive (属性认知)
//! - Relation → Cognitive + Comm (关系涉及通信)
//!
//! 信号强度由粒子的重要性决定

use crate::core::NeuronType;
use crate::world::cognis::{CogniParticle, EntityType, RelationType, ParseResult};

/// 知识信号编码器
///
/// 将认知粒子转换为 LNN 可接受的神经信号
#[derive(Debug, Clone)]
pub struct KnowledgeEncoder {
    /// 实体类型权重映射
    entity_weights: std::collections::HashMap<EntityType, f64>,
    /// 关系类型权重映射
    relation_weights: std::collections::HashMap<RelationType, f64>,
    /// 最大信号强度（归一化用）
    max_signal: f64,
}

impl KnowledgeEncoder {
    /// 创建新的编码器
    pub fn new() -> Self {
        let mut entity_weights = std::collections::HashMap::new();
        entity_weights.insert(EntityType::CodeLanguage, 1.0);   // 代码语言最重要
        entity_weights.insert(EntityType::TechTerm, 0.8);       // 技术术语次之
        entity_weights.insert(EntityType::Concept, 0.6);        // 概念
        entity_weights.insert(EntityType::Keyword, 0.4);        // 关键词
        entity_weights.insert(EntityType::Other, 0.2);          // 其他

        let mut relation_weights = std::collections::HashMap::new();
        relation_weights.insert(RelationType::Contains, 0.7);
        relation_weights.insert(RelationType::BelongsTo, 0.6);
        relation_weights.insert(RelationType::DependsOn, 0.8);  // 依赖关系重要
        relation_weights.insert(RelationType::SimilarTo, 0.5);
        relation_weights.insert(RelationType::RelatedTo, 0.3);

        Self {
            entity_weights,
            relation_weights,
            max_signal: 0.5, // 单次最大信号强度
        }
    }

    /// 将解析结果编码为神经信号向量
    ///
    /// 返回：[(NeuronType, signal_strength), ...]
    ///
    /// 设计原则：
    /// 1. 每个粒子产生一组神经信号
    /// 2. 信号强度与粒子重要性相关
    /// 3. 总信号强度归一化，防止过载
    pub fn encode(&self, parse_result: &ParseResult) -> Vec<NeuralSignal> {
        let mut signals = Vec::new();
        let total_particles = parse_result.particles.len().max(1);

        for particle in &parse_result.particles {
            let particle_signals = self.encode_particle(particle);
            signals.extend(particle_signals);
        }

        // 归一化：总信号强度不超过 max_signal
        let total_strength: f64 = signals.iter().map(|s| s.strength.abs()).sum();
        if total_strength > self.max_signal {
            let scale = self.max_signal / total_strength;
            for signal in &mut signals {
                signal.strength *= scale;
            }
        }

        signals
    }

    /// 编码单个认知粒子
    fn encode_particle(&self, particle: &CogniParticle) -> Vec<NeuralSignal> {
        match particle {
            CogniParticle::Entity { entity_type, .. } => {
                let weight = self.entity_weights.get(entity_type).copied().unwrap_or(0.2);
                vec![
                    NeuralSignal::new(NeuronType::Perception, weight * 0.3),  // 感知输入
                    NeuralSignal::new(NeuronType::Cognitive, weight * 0.5),   // 认知处理
                ]
            }
            CogniParticle::Attribute { .. } => {
                vec![
                    NeuralSignal::new(NeuronType::Cognitive, 0.4),  // 属性认知
                ]
            }
            CogniParticle::Relation { rel_type, .. } => {
                let weight = self.relation_weights.get(rel_type).copied().unwrap_or(0.3);
                vec![
                    NeuralSignal::new(NeuronType::Cognitive, weight * 0.4),  // 关系认知
                    NeuralSignal::new(NeuronType::Comm, weight * 0.3),       // 通信（关系连接）
                ]
            }
        }
    }

    /// 计算知识价值分数（用于三天才裁决）
    ///
    /// 基于解析结果计算知识的整体价值
    pub fn calculate_knowledge_value(&self, parse_result: &ParseResult) -> KnowledgeValue {
        let entity_count = parse_result.particles.iter()
            .filter(|p| matches!(p, CogniParticle::Entity { .. }))
            .count();

        let relation_count = parse_result.particles.iter()
            .filter(|p| matches!(p, CogniParticle::Relation { .. }))
            .count();

        let code_lang_count = parse_result.code_languages.len();
        let keyword_count = parse_result.keywords.len();

        // 有主题 = 更高价值
        let has_topic = parse_result.main_topic.is_some() as usize;

        // 综合评分
        let score = (entity_count as f64 * 0.3
            + relation_count as f64 * 0.2
            + code_lang_count as f64 * 0.25
            + keyword_count as f64 * 0.1
            + has_topic as f64 * 0.15)
            .min(1.0);

        KnowledgeValue {
            score,
            entity_count,
            relation_count,
            code_language_count: code_lang_count,
            keyword_count,
            has_topic: parse_result.main_topic.is_some(),
        }
    }
}

impl Default for KnowledgeEncoder {
    fn default() -> Self {
        Self::new()
    }
}

/// 神经信号
#[derive(Debug, Clone)]
pub struct NeuralSignal {
    /// 目标神经元类型
    pub neuron_type: NeuronType,
    /// 信号强度 [-1, 1]
    pub strength: f64,
}

impl NeuralSignal {
    pub fn new(neuron_type: NeuronType, strength: f64) -> Self {
        Self {
            neuron_type,
            strength: strength.clamp(-1.0, 1.0),
        }
    }
}

/// 知识价值评估
#[derive(Debug, Clone)]
pub struct KnowledgeValue {
    /// 综合评分 [0, 1]
    pub score: f64,
    /// 实体数量
    pub entity_count: usize,
    /// 关系数量
    pub relation_count: usize,
    /// 代码语言数量
    pub code_language_count: usize,
    /// 关键词数量
    pub keyword_count: usize,
    /// 是否有明确主题
    pub has_topic: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::cognis::CognisParser;

    #[test]
    fn test_encode_knowledge() {
        let content = r#"
# HTML Basics

```html
<div class="container">Hello</div>
```

HTML elements have attributes. Tags are building blocks.
        "#;

        let mut parser = CognisParser::new();
        let result = parser.parse(content);

        let encoder = KnowledgeEncoder::new();
        let signals = encoder.encode(&result);

        // 应该有信号
        assert!(!signals.is_empty());

        // 信号强度应该归一化
        let total: f64 = signals.iter().map(|s| s.strength.abs()).sum();
        assert!(total <= encoder.max_signal + 0.01);
    }

    #[test]
    fn test_knowledge_value() {
        let content = r#"
# JavaScript Guide

```javascript
const x = async () => await fetch('/api');
```

JavaScript has async functions and promises.
        "#;

        let mut parser = CognisParser::new();
        let result = parser.parse(content);

        let encoder = KnowledgeEncoder::new();
        let value = encoder.calculate_knowledge_value(&result);

        // 应该有代码语言
        assert!(value.code_language_count > 0);

        // 评分应该在合理范围
        assert!(value.score >= 0.0 && value.score <= 1.0);
    }

    #[test]
    fn test_entity_signal_strength() {
        let encoder = KnowledgeEncoder::new();

        // CodeLanguage 类型应该有更强的信号
        let lang_particle = CogniParticle::Entity {
            id: 1,
            name: "Rust".to_string(),
            entity_type: EntityType::CodeLanguage,
        };

        let other_particle = CogniParticle::Entity {
            id: 2,
            name: "Something".to_string(),
            entity_type: EntityType::Other,
        };

        let lang_signals = encoder.encode_particle(&lang_particle);
        let other_signals = encoder.encode_particle(&other_particle);

        // CodeLanguage 的信号应该更强
        let lang_total: f64 = lang_signals.iter().map(|s| s.strength).sum();
        let other_total: f64 = other_signals.iter().map(|s| s.strength).sum();

        assert!(lang_total > other_total);
    }
}
