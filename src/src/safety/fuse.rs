//! 熔断状态

use serde::{Deserialize, Serialize};

/// 熔断状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FuseState {
    /// 正常运行
    Normal,
    /// 已熔断（网络冻结）
    Fused,
}

impl Default for FuseState {
    fn default() -> Self {
        Self::Normal
    }
}
