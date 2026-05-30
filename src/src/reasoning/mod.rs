//! 安全模块 - 螺丝咕姆设计
//!
//! 包含验证、漂移检测、协作协议三大安全机制

pub mod validation;
pub mod drift;
pub mod protocol;

// 重导出主要类型
pub use validation::{InferenceValidator, ValidationResult, ValidationLevel};
pub use drift::{VectorDriftDetector, DriftConfig, DriftResult};
pub use protocol::{
    CollaborationProtocol, CollaborationMessage, ConsensusRequest,
    GuProfile, GuId, AnomalyRecord, AnomalyType,
};
