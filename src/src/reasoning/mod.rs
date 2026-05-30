//! 创新模块 - 黑塔设计
//!
//! 包含三大创新架构：DCWN, SRU, CMS

pub mod dcwn;
pub mod sru;
pub mod cms;

// 重导出主要类型
pub use dcwn::{DynamicConceptWeavingNetwork, ConceptNode, WeaveConnection, WeaveConfig};
pub use sru::{SemanticResonanceUnderstanding, ConceptWave, ResonanceResult, SruConfig};
pub use cms::{
    CognitiveMarketSystem, MarketConfig, KnowledgeProduct,
    InferenceTask, Transaction, TransactionType,
};
