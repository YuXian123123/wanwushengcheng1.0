//! LNN - Liquid Neural Network
//!
//! 液体神经网络实现，用于蛊虫智能系统
//!
//! # 核心特性
//! - 连续时间状态更新
//! - 局部学习规则（非梯度下降）
//! - 动态拓扑管理
//! - 安全熔断机制
//! - 自然语言对齐
//!
//! # 配置驱动
//!
//! 所有系统参数通过配置管理，避免硬编码：
//!
//! ```
//! use lnn::config::GlobalConfig;
//!
//! let config = GlobalConfig::new();
//! let vector_dim = config.concept.vector_dim;
//! let learning_rate = config.learning.base_learning_rate;
//! ```
//!
//! # Example
//!
//! ```
//! use lnn::{LNN, NeuronType, PlasticityRule};
//!
//! let mut lnn = LNN::new(None, None);
//!
//! // 添加神经元
//! let n1 = lnn.add_neuron(NeuronType::Perception).unwrap();
//! let n2 = lnn.add_neuron(NeuronType::Cognitive).unwrap();
//!
//! // 添加突触
//! lnn.add_synapse(&n1, &n2, 0.5, PlasticityRule::Hebbian).unwrap();
//!
//! // 运行更新
//! lnn.update(0.01).unwrap();
//! ```

pub mod config;
pub mod core;
pub mod learning;
pub mod safety;
pub mod language;
pub mod reasoning;

// 重导出配置类型
pub use config::GlobalConfig;

// 重导出常用类型
pub use core::{LNN, Neuron, Synapse, NeuronType, PlasticityRule};
pub use core::{LNNConfig, TopologyDynamics};
pub use learning::LearningRules;
pub use learning::RecursiveLearner;
pub use safety::{SafetyMonitor, FuseState};

// 重导出语言对齐类型
pub use language::{ConceptSpace, ConceptLevel, Encoder, Decoder};

// 重导出推理类型
pub use reasoning::{GuReasoningCore, InferenceResult, InferenceType};
