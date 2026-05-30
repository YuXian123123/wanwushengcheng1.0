//! 概念空间模块
//!
//! 定义概念、概念向量、概念空间等核心数据结构
//! 使用局部学习规则（非梯度下降）进行向量更新

mod types;
mod space;
mod learning;

pub use types::{Concept, ConceptId, ConceptVector, ConceptLevel, ConceptRelation, ConsensusStatus, VECTOR_DIM};
pub use space::ConceptSpace;
pub use learning::VectorLearningRules;
