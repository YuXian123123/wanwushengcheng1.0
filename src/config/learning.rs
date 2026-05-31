//! 学习配置

use serde::{Deserialize, Serialize};

/// 学习配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConfig {
    /// 基础学习率
    ///
    /// 控制向量更新的步长
    /// - 0.001: 非常保守，变化缓慢
    /// - 0.01: 标准学习率（推荐）
    /// - 0.1: 激进，变化快速但不稳定
    pub base_learning_rate: f64,

    /// 关联学习强度
    ///
    /// 当两个概念同时出现时，相互靠近的强度
    pub association_strength: f64,

    /// 区分学习强度
    ///
    /// 当需要区分两个概念时，相互远离的强度
    pub differentiation_strength: f64,

    /// STDP时间窗口
    ///
    /// 突触可塑性的时间窗口（毫秒）
    pub stdp_time_window_ms: f64,

    /// STDP长时程增强系数
    ///
    /// A+ 参数：突触前先于突触后发放时的增强
    pub stdp_a_plus: f64,

    /// STDP长时程抑制系数
    ///
    /// A- 参数：突触后先于突触前发放时的抑制
    pub stdp_a_minus: f64,

    /// STDP时间常数 τ+
    ///
    /// LTP时间衰减常数（毫秒）
    pub stdp_tau_plus: f64,

    /// STDP时间常数 τ-
    ///
    /// LTD时间衰减常数（毫秒）
    pub stdp_tau_minus: f64,

    /// 最小学习率
    pub min_learning_rate: f64,

    /// 最大学习率
    pub max_learning_rate: f64,

    /// 情绪调节 - 唤醒基础因子
    pub emotion_arousal_base: f64,

    /// 情绪调节 - 效价影响系数
    pub emotion_valence_factor: f64,
}

impl LearningConfig {
    /// 创建默认配置
    pub fn new() -> Self {
        Self {
            base_learning_rate: 0.01,
            association_strength: 1.0,
            differentiation_strength: 1.0,
            stdp_time_window_ms: 20.0,
            stdp_a_plus: 0.1,
            stdp_a_minus: 0.12,
            stdp_tau_plus: 20.0,
            stdp_tau_minus: 20.0,
            min_learning_rate: 0.0001,
            max_learning_rate: 0.5,
            emotion_arousal_base: 0.5,
            emotion_valence_factor: 0.2,
        }
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), String> {
        if self.base_learning_rate <= 0.0 || self.base_learning_rate > 1.0 {
            return Err("base_learning_rate 必须在 (0, 1] 范围内".to_string());
        }
        if self.association_strength <= 0.0 || self.association_strength > 2.0 {
            return Err("association_strength 必须在 (0, 2] 范围内".to_string());
        }
        if self.differentiation_strength <= 0.0 || self.differentiation_strength > 2.0 {
            return Err("differentiation_strength 必须在 (0, 2] 范围内".to_string());
        }
        if self.stdp_a_plus <= 0.0 || self.stdp_a_plus > 1.0 {
            return Err("stdp_a_plus 必须在 (0, 1] 范围内".to_string());
        }
        if self.stdp_a_minus <= 0.0 || self.stdp_a_minus > 1.0 {
            return Err("stdp_a_minus 必须在 (0, 1] 范围内".to_string());
        }
        if self.stdp_tau_plus <= 0.0 {
            return Err("stdp_tau_plus 必须大于0".to_string());
        }
        if self.stdp_tau_minus <= 0.0 {
            return Err("stdp_tau_minus 必须大于0".to_string());
        }
        if self.min_learning_rate >= self.max_learning_rate {
            return Err("min_learning_rate 必须小于 max_learning_rate".to_string());
        }
        Ok(())
    }

    /// 获取当前学习率（支持动态调整）
    pub fn learning_rate(&self, progress: f64) -> f64 {
        // 简单的线性衰减
        let decay = 1.0 - progress * 0.5;
        (self.base_learning_rate * decay)
            .max(self.min_learning_rate)
            .min(self.max_learning_rate)
    }

    /// 获取STDP权重变化
    pub fn stdp_weight_change(&self, time_diff_ms: f64) -> f64 {
        if time_diff_ms > 0.0 {
            // 突触前先于突触后：LTP
            self.stdp_a_plus * (-time_diff_ms / self.stdp_tau_plus).exp()
        } else {
            // 突触后先于突触前：LTD
            -self.stdp_a_minus * (time_diff_ms / self.stdp_tau_minus).exp()
        }
    }

    /// 情绪调节学习率
    pub fn emotion_modulated_rate(&self, base_rate: f64, arousal: f64, valence: f64) -> f64 {
        let arousal_factor = self.emotion_arousal_base + arousal;
        let valence_factor = 1.0 + valence * self.emotion_valence_factor;
        base_rate * arousal_factor * valence_factor
    }
}

impl Default for LearningConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_valid() {
        let config = LearningConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_learning_rate_decay() {
        let config = LearningConfig::new();
        let lr_0 = config.learning_rate(0.0);
        let lr_1 = config.learning_rate(1.0);

        assert!(lr_0 > lr_1, "学习率应该随进度衰减");
        assert!(lr_1 >= config.min_learning_rate);
    }

    #[test]
    fn test_stdp_ltp() {
        let config = LearningConfig::new();
        // 突触前先于突触后（正时间差）
        let weight_change = config.stdp_weight_change(10.0);
        assert!(weight_change > 0.0, "LTP应该产生正权重变化");
    }

    #[test]
    fn test_stdp_ltd() {
        let config = LearningConfig::new();
        // 突触后先于突触前（负时间差）
        let weight_change = config.stdp_weight_change(-10.0);
        assert!(weight_change < 0.0, "LTD应该产生负权重变化");
    }
}
