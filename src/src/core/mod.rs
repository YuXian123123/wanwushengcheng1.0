//! 核心数据结构
//!
//! 定义神经元、突触、LNN网络的核心结构

mod neuron;
mod synapse;
mod lnn;
mod types;

pub use neuron::Neuron;
pub use neuron::NeuronState;
pub use synapse::Synapse;
pub use synapse::SynapseState;
pub use lnn::LNN;
pub use lnn::LNNState;
pub use lnn::LNNSnapshot;
pub use lnn::NetworkStatistics;
pub use lnn::AuditEntry;
pub use types::{NeuronType, PlasticityRule, LNNConfig, TopologyDynamics};
