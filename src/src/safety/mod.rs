//! 安全机制模块

mod monitor;
mod fuse;

pub use monitor::SafetyMonitor;
pub use fuse::FuseState;
