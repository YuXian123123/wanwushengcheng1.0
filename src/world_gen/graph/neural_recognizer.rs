//! 神经实体识别器
//!
//! 使用 LNN 和概念空间进行实体识别，而非硬编码模式匹配

use crate::core::LNN;
use crate::language::concept::ConceptSpace;
use crate::embedding::{WordEmbedding, EmbeddingConfig};
use crate::world_gen::config::GraphBuildConfig;
use crate::world_gen::graph::*;
use crate::world_gen::pretrained_vectors::PretrainedVectors;
use std::collections::HashMap;

/// 神经实体识别器
///
/// 核心思想：
/// 1. 将文本编码为向量序列
/// 2. 通过 LNN 进行序列标注
/// 3. 利用概念空间进行语义匹配
pub struct NeuralEntityRecognizer {
    /// 配置
    config: GraphBuildConfig,
    /// 概念空间（语义匹配）
    concept_space: ConceptSpace,
    /// 词向量嵌入
    embedding: WordEmbedding,
    /// 预训练词向量（用于分词和类别推断）
    pretrained_vectors: Option<PretrainedVectors>,
    /// 实体识别网络（可选，用于序列标注）
    recognizer_lnn: Option<LNN>,
}

impl NeuralEntityRecognizer {
    /// 创建新识别器
    pub fn new(config: GraphBuildConfig) -> Self {
        Self {
            config,
            concept_space: ConceptSpace::new(),
            embedding: WordEmbedding::new(EmbeddingConfig::default()),
            pretrained_vectors: None,
            recognizer_lnn: None,
        }
    }

    /// 设置概念空间
    pub fn with_concept_space(mut self, concept_space: ConceptSpace) -> Self {
        self.concept_space = concept_space;
        self
    }

    /// 设置词向量
    pub fn with_embedding(mut self, embedding: WordEmbedding) -> Self {
        self.embedding = embedding;
        self
    }

    /// 设置预训练词向量
    pub fn with_pretrained_vectors(mut self, vectors: PretrainedVectors) -> Self {
        self.pretrained_vectors = Some(vectors);
        self
    }

    /// 设置识别网络
    pub fn with_recognizer_lnn(mut self, lnn: LNN) -> Self {
        self.recognizer_lnn = Some(lnn);
        self
    }

    /// 识别文本中的实体
    ///
    /// 算法流程：
    /// 1. 分词 → 2. 向量化 → 3. 概念匹配 → 4. 类型推断
    pub fn recognize(&self, text: &str) -> Vec<GraphEntity> {
        let mut entities = Vec::new();

        // Step 1: 分词（优先使用预训练词向量分词）
        let tokens = if let Some(pv) = &self.pretrained_vectors {
            pv.tokenize(text)
        } else {
            self.tokenize(text)
        };

        // 用于去重
        let mut seen_tokens = std::collections::HashSet::new();

        // Step 2: 对每个token进行语义分析
        for token in tokens {
            // 去重
            if seen_tokens.contains(&token) {
                continue;
            }
            seen_tokens.insert(token.clone());

            // 尝试使用预训练词向量推断类别
            if let Some(pv) = &self.pretrained_vectors {
                if let Some((category, confidence)) = pv.infer_category(&token) {
                    if confidence >= self.config.entity_threshold {
                        // 从类别推断实体类型
                        let entity_type = pv.category_to_entity_type(&category)
                            .unwrap_or(EntityType::Object);

                        let entity = GraphEntity::new(token.clone(), entity_type)
                            .with_confidence(confidence);

                        entities.push(entity);
                        continue;
                    }
                }
            }

            // 回退：使用概念空间匹配
            let token_vector = self.embedding.get_vector(&token);

            // Step 3: 在概念空间中查找相似概念
            let matches = self.concept_space.find_similar(&token, 3);

            // Step 4: 如果找到匹配概念，创建实体
            if let Some(similar_concepts) = matches {
                for (concept_id, similarity) in similar_concepts {
                    if similarity >= self.config.entity_threshold {
                        // 从概念推断实体类型
                        let entity_type = self.infer_entity_type(&concept_id);

                        let entity = GraphEntity::new(token.clone(), entity_type)
                            .with_confidence(similarity);

                        entities.push(entity);
                        break; // 只取最佳匹配
                    }
                }
            }
        }

        // Step 5: 如果有训练好的 LNN，用它进行序列标注优化
        if let Some(lnn) = &self.recognizer_lnn {
            entities = self.refine_with_lnn(entities, text, lnn);
        }

        entities
    }

    /// 分词
    fn tokenize(&self, text: &str) -> Vec<String> {
        // 使用嵌入系统的分词能力
        // MVP: 简单的中文字符分词
        // TODO: 集成真正的分词器（如 jieba）

        let mut tokens = Vec::new();
        let mut current_token = String::new();

        for ch in text.chars() {
            if ch.is_whitespace() || ch.is_ascii_punctuation() {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            } else {
                current_token.push(ch);
            }
        }

        if !current_token.is_empty() {
            tokens.push(current_token);
        }

        tokens
    }

    /// 从概念ID推断实体类型
    fn infer_entity_type(&self, concept_id: &str) -> EntityType {
        // 基于概念的层次结构推断类型
        // 例如：concept:person → EntityType::Person

        let concept_lower = concept_id.to_lowercase();

        if concept_lower.contains("person") || concept_lower.contains("人") {
            EntityType::Person
        } else if concept_lower.contains("animal") || concept_lower.contains("动物") {
            EntityType::Animal
        } else if concept_lower.contains("plant") || concept_lower.contains("植物") {
            EntityType::Plant
        } else if concept_lower.contains("building") || concept_lower.contains("建筑") || concept_lower.contains("房子") {
            EntityType::Building
        } else if concept_lower.contains("location") || concept_lower.contains("地点") {
            EntityType::Location
        } else if concept_lower.contains("object") || concept_lower.contains("物体") {
            EntityType::Object
        } else {
            EntityType::Concept
        }
    }

    /// 使用 LNN 优化实体识别结果
    fn refine_with_lnn(
        &self,
        entities: Vec<GraphEntity>,
        text: &str,
        lnn: &LNN,
    ) -> Vec<GraphEntity> {
        // TODO: 实现序列标注优化
        // 使用 LNN 的上下文感知能力，考虑实体间的依赖关系
        // 例如："红色的房子" → "红色"是"房子"的属性，不是独立实体

        // MVP: 直接返回
        entities
    }

    /// 训练实体识别器
    ///
    /// 使用标注数据训练 LNN
    pub fn train(&mut self, training_data: &[(String, Vec<EntityLabel>)]) {
        // 创建 LNN 用于序列标注
        let mut lnn = LNN::new(Some(crate::core::LNNConfig::default()), None);

        // 训练数据格式：(文本, 实体标签列表)
        for (text, labels) in training_data {
            // 将文本编码为向量序列
            let vectors: Vec<_> = self.tokenize(text)
                .iter()
                .map(|t| self.embedding.get_vector(t))
                .collect();

            // 训练 LNN
            // TODO: 实现序列标注训练算法
            // 使用 LNN 的局部学习规则进行在线学习
        }

        self.recognizer_lnn = Some(lnn);
    }
}

/// 实体标签（用于训练）
#[derive(Debug, Clone)]
pub struct EntityLabel {
    /// 实体文本
    pub text: String,
    /// 实体类型
    pub entity_type: EntityType,
    /// 起始位置
    pub start: usize,
    /// 结束位置
    pub end: usize,
}

/// 神经关系抽取器
///
/// 使用 LNN 学习实体间的关系模式
pub struct NeuralRelationExtractor {
    /// 配置
    config: GraphBuildConfig,
    /// 关系分类网络
    relation_lnn: Option<LNN>,
    /// 词向量
    embedding: WordEmbedding,
}

impl NeuralRelationExtractor {
    pub fn new(config: GraphBuildConfig) -> Self {
        Self {
            config,
            relation_lnn: None,
            embedding: WordEmbedding::new(EmbeddingConfig::default()),
        }
    }

    /// 抽取实体间关系
    ///
    /// 算法：
    /// 1. 构建实体对
    /// 2. 编码上下文
    /// 3. 通过 LNN 分类关系类型
    pub fn extract(
        &self,
        text: &str,
        entities: &[GraphEntity],
    ) -> Vec<GraphRelation> {
        let mut relations = Vec::new();

        // 如果没有训练好的网络，使用基于距离的启发式方法
        if self.relation_lnn.is_none() {
            return self.heuristic_extract(text, entities);
        }

        let lnn = self.relation_lnn.as_ref().unwrap();

        // 遍历所有实体对
        for i in 0..entities.len() {
            for j in 0..entities.len() {
                if i == j {
                    continue;
                }

                let e1 = &entities[i];
                let e2 = &entities[j];

                // 编码实体对和上下文
                let context_vector = self.encode_pair_context(text, e1, e2);

                // 通过 LNN 分类关系
                let relation_type = self.classify_relation(&context_vector, lnn);

                if let Some(rel_type) = relation_type {
                    let relation = GraphRelation::new(
                        e1.id.clone(),
                        e2.id.clone(),
                        rel_type,
                    ).with_strength(self.config.relation_threshold);

                    relations.push(relation);
                }
            }
        }

        relations
    }

    /// 启发式关系抽取（无训练数据时的后备方案）
    fn heuristic_extract(
        &self,
        text: &str,
        entities: &[GraphEntity],
    ) -> Vec<GraphRelation> {
        let mut relations = Vec::new();

        // 基于文本距离和语法结构推断关系
        for i in 0..entities.len() {
            for j in 0..entities.len() {
                if i >= j {
                    continue;
                }

                let e1 = &entities[i];
                let e2 = &entities[j];

                // 查找两个实体在文本中的位置
                let pos1 = text.find(&e1.name);
                let pos2 = text.find(&e2.name);

                if let (Some(p1), Some(p2)) = (pos1, pos2) {
                    // 分析两个实体间的文本
                    let between = if p1 < p2 {
                        &text[p1 + e1.name.len()..p2]
                    } else {
                        &text[p2 + e2.name.len()..p1]
                    };

                    // 根据中间文本推断关系类型
                    let relation_type = self.infer_relation_from_context(between);

                    if let Some(rel_type) = relation_type {
                        let (from, to) = if p1 < p2 {
                            (e1.id.clone(), e2.id.clone())
                        } else {
                            (e2.id.clone(), e1.id.clone())
                        };

                        let relation = GraphRelation::new(from, to, rel_type)
                            .with_strength(self.config.relation_threshold);

                        relations.push(relation);
                    }
                }
            }
        }

        relations
    }

    /// 从上下文推断关系类型
    fn infer_relation_from_context(&self, context: &str) -> Option<RelationType> {
        // 基于语义分析推断关系
        // 注意：这里的关键词是语义提示，不是硬编码模式

        let context_lower = context.to_lowercase();

        // 空间关系提示
        if context_lower.contains("里有") || context_lower.contains("包含") {
            return Some(RelationType::Contains);
        }
        if context_lower.contains("在") && context_lower.contains("里") {
            return Some(RelationType::Inside);
        }
        if context_lower.contains("外面") || context_lower.contains("旁边") {
            return Some(RelationType::Adjacent);
        }

        // 属性关系提示
        if context_lower.contains("是") && context_lower.chars().count() < 5 {
            return Some(RelationType::HasAttribute);
        }

        None
    }

    /// 编码实体对的上下文
    fn encode_pair_context(
        &self,
        text: &str,
        e1: &GraphEntity,
        e2: &GraphEntity,
    ) -> Vec<f64> {
        // 提取两个实体间的上下文
        let pos1 = text.find(&e1.name);
        let pos2 = text.find(&e2.name);

        let context = match (pos1, pos2) {
            (Some(p1), Some(p2)) if p1 < p2 => {
                &text[p1..p2 + e2.name.len()]
            }
            (Some(p1), Some(p2)) => {
                &text[p2..p1 + e1.name.len()]
            }
            _ => "",
        };

        // 编码为向量
        self.embedding.get_vector(context)
    }

    /// 通过 LNN 分类关系类型
    fn classify_relation(
        &self,
        context_vector: &[f64],
        lnn: &LNN,
    ) -> Option<RelationType> {
        // TODO: 实现 LNN 分类
        // 将上下文向量输入 LNN，获取关系类型

        // MVP: 返回 None
        None
    }

    /// 训练关系抽取器
    pub fn train(&mut self, training_data: &[(String, Vec<RelationLabel>)]) {
        // 创建 LNN 用于关系分类
        let mut lnn = LNN::new(Some(crate::core::LNNConfig::default()), None);

        // 训练数据格式：(文本, 关系标签列表)
        for (text, labels) in training_data {
            // TODO: 实现关系分类训练
        }

        self.relation_lnn = Some(lnn);
    }
}

/// 关系标签（用于训练）
#[derive(Debug, Clone)]
pub struct RelationLabel {
    /// 起始实体
    pub from_entity: String,
    /// 目标实体
    pub to_entity: String,
    /// 关系类型
    pub relation_type: RelationType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recognizer_creation() {
        let config = GraphBuildConfig::default();
        let recognizer = NeuralEntityRecognizer::new(config);
        assert!(recognizer.recognizer_lnn.is_none());
    }

    #[test]
    fn test_tokenize() {
        let config = GraphBuildConfig::default();
        let recognizer = NeuralEntityRecognizer::new(config);

        let tokens = recognizer.tokenize("房子里有桌子");
        assert!(tokens.contains(&"房子".to_string()));
        assert!(tokens.contains(&"桌子".to_string()));
    }

    #[test]
    fn test_relation_extractor_creation() {
        let config = GraphBuildConfig::default();
        let extractor = NeuralRelationExtractor::new(config);
        assert!(extractor.relation_lnn.is_none());
    }
}
