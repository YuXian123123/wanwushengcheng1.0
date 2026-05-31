//! 学习规则模块
//!
//! 包含各种学习规则和机制

mod rules;
pub mod recursive;

pub use rules::LearningRules;
pub use recursive::{RecursiveLearner, KnowledgeMetadata, KnowledgeDocument, LearningResult};
