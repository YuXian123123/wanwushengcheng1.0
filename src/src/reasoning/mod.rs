//! 推理模块 - 天才理事会设计
//!
//! 包含三种视角的推理实现：
//! - 黑塔：创新架构（DCWN, SRU, CMS）
//! - 螺丝咕姆：安全验证
//! - 拉蒂奥：优雅数据结构

pub mod core;
pub mod chain;
pub mod hierarchy;

// 重导出主要类型
pub use core::{GuReasoningCore, InferenceResult, InferenceType, ReasoningConfig};
pub use chain::{ReasoningChain, ReasoningStep, ReasoningRule, ChainBuilder};
pub use hierarchy::{KnowledgeHierarchy, KnowledgeNode, KnowledgeLevel, KnowledgeSource};
