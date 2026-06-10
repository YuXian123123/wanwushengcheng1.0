//! LNN核心配置
//!
//! 管理液体神经网络核心参数，避免硬编码

use serde::{Deserialize, Serialize};

/// LNN核心配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LNNCoreConfig {
    /// 突触权重上限
    ///
    /// 突触权重的最大绝对值
    /// - 1.0: 小型网络
    /// - 10.0: 标准网络（推荐）
    /// - 100.0: 大型网络
    pub max_weight: f64,

    /// 神经元活跃度衰减因子
    ///
    /// 活跃度更新: activity = activity * decay + state * (1 - decay)
    /// - 0.9: 快速衰减
    /// - 0.99: 标准衰减（推荐）
    /// - 0.999: 慢速衰减
    pub activity_decay: f64,

    /// 神经元状态归一化范围
    ///
    /// 神经元状态的绝对值上限
    pub state_normalization_limit: f64,

    /// 时间常数最小值
    pub tau_min: f64,

    /// 时间常数最大值
    pub tau_max: f64,

    /// 偏置范围最小值
    pub bias_min: f64,

    /// 偏置范围最大值
    pub bias_max: f64,
}

impl LNNCoreConfig {
    /// 创建默认配置
    pub fn new() -> Self {
        Self {
            max_weight: 10.0,
            activity_decay: 0.99,
            state_normalization_limit: 1.0,
            tau_min: 0.1,
            tau_max: 10.0,
            bias_min: -1.0,
            bias_max: 1.0,
        }
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), String> {
        if self.max_weight <= 0.0 {
            return Err("max_weight 必须大于0".to_string());
        }
        if self.activity_decay <= 0.0 || self.activity_decay >= 1.0 {
            return Err("activity_decay 必须在 (0, 1) 范围内".to_string());
        }
        if self.state_normalization_limit <= 0.0 {
            return Err("state_normalization_limit 必须大于0".to_string());
        }
        if self.tau_min >= self.tau_max {
            return Err("tau_min 必须小于 tau_max".to_string());
        }
        if self.tau_min <= 0.0 {
            return Err("tau_min 必须大于0".to_string());
        }
        if self.bias_min >= self.bias_max {
            return Err("bias_min 必须小于 bias_max".to_string());
        }
        Ok(())
    }

    /// 获取活跃度更新因子
    ///
    /// 返回 (decay_factor, growth_factor)
    pub fn activity_factors(&self) -> (f64, f64) {
        (self.activity_decay, 1.0 - self.activity_decay)
    }

    /// 归一化权重
    pub fn clamp_weight(&self, weight: f64) -> f64 {
        weight.clamp(-self.max_weight, self.max_weight)
    }

    /// 归一化状态
    pub fn clamp_state(&self, state: f64) -> f64 {
        state.clamp(-self.state_normalization_limit, self.state_normalization_limit)
    }

    /// 归一化时间常数
    pub fn clamp_tau(&self, tau: f64) -> f64 {
        tau.clamp(self.tau_min, self.tau_max)
    }

    /// 归一化偏置
    pub fn clamp_bias(&self, bias: f64) -> f64 {
        bias.clamp(self.bias_min, self.bias_max)
    }
}

impl Default for LNNCoreConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_valid() {
        let config = LNNCoreConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_weight_clamping() {
        let config = LNNCoreConfig::new();
        assert_eq!(config.clamp_weight(15.0), 10.0);
        assert_eq!(config.clamp_weight(-15.0), -10.0);
        assert_eq!(config.clamp_weight(5.0), 5.0);
    }

    #[test]
    fn test_activity_factors() {
        let config = LNNCoreConfig::new();
        let (decay, growth) = config.activity_factors();
        assert!((decay - 0.99).abs() < 1e-10);
        assert!((growth - 0.01).abs() < 1e-10);
        assert!((decay + growth - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_invalid_config() {
        let mut config = LNNCoreConfig::new();
        config.max_weight = -1.0;
        assert!(config.validate().is_err());

        config.max_weight = 10.0;
        config.activity_decay = 1.5;
        assert!(config.validate().is_err());
    }
}
