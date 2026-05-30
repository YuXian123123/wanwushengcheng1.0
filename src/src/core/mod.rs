//! 核心数据结构
//!
//! 定义神经元、突触、LNN网络的核心结构

mod neuron;
mod synapse;
mod lnn;
mod types;

pub use neuron::Neuron;
pub use synapse::Synapse;
pub use lnn::LNN;
pub use lnn::LNNState;
pub use types::{NeuronType, PlasticityRule, LNNConfig, TopologyDynamics};
