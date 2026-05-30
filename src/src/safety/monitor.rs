//! 安全监控器

use crate::safety::FuseState;

/// 安全监控器
#[derive(Debug, Clone)]
pub struct SafetyMonitor {
    /// 当前熔断状态
    state: FuseState,
    /// 熔断原因
    fuse_reason: Option<String>,
    /// 熔断时间
    fuse_time: Option<f64>,
}

impl SafetyMonitor {
    /// 创建新的安全监控器
    pub fn new() -> Self {
        Self {
            state: FuseState::Normal,
            fuse_reason: None,
            fuse_time: None,
        }
    }

    /// 检查是否已熔断
    pub fn is_fused(&self) -> bool {
        self.state == FuseState::Fused
    }

    /// 获取熔断状态
    pub fn state(&self) -> FuseState {
        self.state
    }

    /// 获取熔断原因
    pub fn reason(&self) -> Option<&str> {
        self.fuse_reason.as_deref()
    }

    /// 触发熔断
    pub fn trigger_fuse(&mut self, reason: String) {
        self.state = FuseState::Fused;
        self.fuse_reason = Some(reason);
        self.fuse_time = Some(0.0); // 实际使用时应该传入当前时间

        log::error!("[LNN FUSE] Network frozen: {:?}", self.fuse_reason);
    }

    /// 重置熔断状态（谨慎使用！）
    ///
    /// # Safety
    /// 只有在确认问题已修复后才应调用此方法
    pub fn reset(&mut self) {
        log::warn!("[LNN FUSE] Resetting fuse state");
        self.state = FuseState::Normal;
        self.fuse_reason = None;
        self.fuse_time = None;
    }
}

impl Default for SafetyMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let monitor = SafetyMonitor::new();
        assert!(!monitor.is_fused());
        assert_eq!(monitor.state(), FuseState::Normal);
    }

    #[test]
    fn test_trigger_fuse() {
        let mut monitor = SafetyMonitor::new();
        monitor.trigger_fuse("Test fuse".to_string());

        assert!(monitor.is_fused());
        assert_eq!(monitor.state(), FuseState::Fused);
        assert_eq!(monitor.reason(), Some("Test fuse"));
    }

    #[test]
    fn test_reset() {
        let mut monitor = SafetyMonitor::new();
        monitor.trigger_fuse("Test".to_string());
        monitor.reset();

        assert!(!monitor.is_fused());
        assert!(monitor.reason().is_none());
    }
}
