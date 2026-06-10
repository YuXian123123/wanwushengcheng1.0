//! 文本关系图谱模块
//!
//! 从文本中提取的结构化知识图谱，包含实体、关系和层次结构

pub mod types;
pub mod builder;
pub mod neural_recognizer;

pub use types::{
    EntityId, RelationId, GraphId,
    EntityType, RelationType, RelationDirection,
    AttributeValue, TextReference,
    GraphEntity, GraphRelation, GraphHierarchy,
    TextRelationGraph,
};
pub use builder::{GraphBuilder, BuildError};
pub use neural_recognizer::{
    NeuralEntityRecognizer, NeuralRelationExtractor,
    EntityLabel, RelationLabel,
};
