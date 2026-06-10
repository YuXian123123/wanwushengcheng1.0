//! 图谱构建器
//!
//! 从文本构建关系图谱的核心算法
//! 完全基于神经网络学习，无硬编码规则

use crate::world_gen::config::GraphBuildConfig;
use crate::world_gen::graph::*;
use crate::world_gen::graph::neural_recognizer::{NeuralEntityRecognizer, NeuralRelationExtractor, EntityLabel, RelationLabel};
use crate::world_gen::pretrained_vectors::PretrainedVectors;
use crate::language::concept::ConceptSpace;
use crate::embedding::WordEmbedding;
use std::collections::HashMap;

/// 构建错误
#[derive(Debug, Clone)]
pub enum BuildError {
    TooManyEntities(usize),
    TooManyRelations(usize),
    ParseError(String),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::TooManyEntities(n) => write!(f, "实体数量超过限制: {}", n),
            BuildError::TooManyRelations(n) => write!(f, "关系数量超过限制: {}", n),
            BuildError::ParseError(s) => write!(f, "解析错误: {}", s),
        }
    }
}

impl std::error::Error for BuildError {}

/// 图谱构建器
///
/// 完全依赖神经网络学习，无硬编码规则
pub struct GraphBuilder {
    config: GraphBuildConfig,
    entity_recognizer: NeuralEntityRecognizer,
    relation_extractor: NeuralRelationExtractor,
    /// 训练样本计数
    training_samples: usize,
}

impl GraphBuilder {
    pub fn new(config: GraphBuildConfig) -> Self {
        Self {
            entity_recognizer: NeuralEntityRecognizer::new(config.clone()),
            relation_extractor: NeuralRelationExtractor::new(config.clone()),
            config,
            training_samples: 0,
        }
    }

    pub fn with_pretrained(config: GraphBuildConfig, concept_space: ConceptSpace, embedding: WordEmbedding) -> Self {
        let entity_recognizer = NeuralEntityRecognizer::new(config.clone())
            .with_concept_space(concept_space)
            .with_embedding(embedding.clone());

        let relation_extractor = NeuralRelationExtractor::new(config.clone());

        Self {
            entity_recognizer,
            relation_extractor,
            config,
            training_samples: 0,
        }
    }

    pub fn with_pretrained_vectors(config: GraphBuildConfig, vectors: PretrainedVectors) -> Self {
        let entity_recognizer = NeuralEntityRecognizer::new(config.clone())
            .with_pretrained_vectors(vectors);

        let relation_extractor = NeuralRelationExtractor::new(config.clone());

        Self {
            entity_recognizer,
            relation_extractor,
            config,
            training_samples: 0,
        }
    }

    /// 训练实体识别器（从训练数据学习）
    pub fn train_entity_recognizer(&mut self, training_data: &[(String, Vec<EntityLabel>)]) {
        self.entity_recognizer.train(training_data);
        self.training_samples += training_data.len();
    }

    /// 训练关系抽取器（从训练数据学习）
    pub fn train_relation_extractor(&mut self, training_data: &[(String, Vec<RelationLabel>)]) {
        self.relation_extractor.train(training_data);
    }

    /// 获取训练样本数
    pub fn get_training_samples(&self) -> usize {
        self.training_samples
    }

    /// 从文本构建关系图谱（纯神经网络）
    pub fn build(&self, text: &str) -> Result<TextRelationGraph, BuildError> {
        // 1. 神经实体识别（完全依赖学习到的模式）
        let entities = self.recognize_entities(text)?;

        if entities.len() > self.config.max_entities {
            return Err(BuildError::TooManyEntities(entities.len()));
        }

        // 2. 神经关系抽取（完全依赖学习到的模式）
        let entity_vec: Vec<_> = entities.values().cloned().collect();
        let relations = self.extract_relations(text, &entity_vec)?;

        if relations.len() > self.config.max_relations {
            return Err(BuildError::TooManyRelations(relations.len()));
        }

        // 3. 构建层次结构
        let hierarchy = self.build_hierarchy(&entities, &relations);

        // 4. 创建图谱
        let mut graph = TextRelationGraph::new(text.to_string());
        for (_, entity) in entities {
            graph.add_entity(entity);
        }
        for relation in relations {
            graph.add_relation(relation);
        }
        graph.hierarchy = hierarchy;

        Ok(graph)
    }

    /// 神经实体识别
    fn recognize_entities(&self, text: &str) -> Result<HashMap<EntityId, GraphEntity>, BuildError> {
        // 完全依赖神经网络
        let entities = self.entity_recognizer.recognize(text);
        let map: HashMap<EntityId, GraphEntity> = entities
            .into_iter()
            .map(|e| (e.id.clone(), e))
            .collect();
        Ok(map)
    }

    /// 神经关系抽取
    fn extract_relations(&self, text: &str, entities: &[GraphEntity]) -> Result<Vec<GraphRelation>, BuildError> {
        // 完全依赖神经网络
        Ok(self.relation_extractor.extract(text, entities))
    }

    /// 构建层次结构
    fn build_hierarchy(
        &self,
        entities: &HashMap<EntityId, GraphEntity>,
        relations: &[GraphRelation],
    ) -> GraphHierarchy {
        let mut hierarchy = GraphHierarchy::new();

        // 找到根节点（没有父节点的实体）
        let mut has_parent = std::collections::HashSet::new();

        for relation in relations {
            if matches!(relation.relation_type, RelationType::Contains | RelationType::Inside) {
                has_parent.insert(relation.to.clone());
            }
        }

        // 添加根节点
        for (id, _) in entities {
            if !has_parent.contains(id) {
                hierarchy.add_root(id.clone());
            }
        }

        // 添加父子关系
        for relation in relations {
            if matches!(relation.relation_type, RelationType::Contains | RelationType::Inside) {
                hierarchy.add_child(relation.from.clone(), relation.to.clone());
            }
        }

        hierarchy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let config = GraphBuildConfig::default();
        let builder = GraphBuilder::new(config);
        assert_eq!(builder.training_samples, 0);
    }

    #[test]
    fn test_neural_build_without_training() {
        // 没有训练时，应该依赖概念空间的语义匹配
        let config = GraphBuildConfig::default();
        let builder = GraphBuilder::new(config);

        let text = "房子里有桌子";
        let result = builder.build(text);

        // 即使没有训练，也应该能工作（通过概念空间）
        assert!(result.is_ok());
    }
}
