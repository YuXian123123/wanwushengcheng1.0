//! 局部学习规则实现
//!
//! 注意：这些是局部学习规则，不是梯度下降！
//! LNN使用生物可解释的学习规则
//!
//! # 配置驱动
//!
//! 所有参数通过 LearningConfig 管理，避免硬编码

use crate::core::PlasticityRule;
use crate::config::learning::LearningConfig;

/// 学习规则
pub struct LearningRules;

impl LearningRules {
    /// 获取默认配置
    fn config() -> LearningConfig {
        LearningConfig::new()
    }

    /// 赫布学习: Δw = η · xᵢ · xⱼ
    ///
    /// "一起激发的神经元连接在一起"
    ///
    /// # Arguments
    /// * `pre_state` - 前神经元状态
    /// * `post_state` - 后神经元状态
    /// * `learning_rate` - 学习率
    pub fn hebbian(pre_state: f64, post_state: f64, learning_rate: f64) -> f64 {
        learning_rate * pre_state * post_state
    }

    /// Oja规则: Δw = η · y · (x - y · w)
    ///
    /// 赫布学习的变体，防止权重爆炸
    ///
    /// # Arguments
    /// * `pre_state` - 前神经元状态 x
    /// * `post_state` - 后神经元状态（未使用，保持接口一致）
    /// * `weight` - 当前权重 w
    /// * `learning_rate` - 学习率 η
    pub fn oja(
        pre_state: f64,
        _post_state: f64,
        weight: f64,
        learning_rate: f64,
    ) -> f64 {
        let output = weight * pre_state; // y = w * x
        learning_rate * output * (pre_state - output * weight)
    }

    /// STDP: 脉冲时序依赖可塑性（使用配置）
    ///
    /// Δw = A₊·exp(-Δt/τ₊) 如果 post 在 pre 之后 (LTP)
    /// Δw = -A₋·exp(Δt/τ₋) 如果 post 在 pre 之前 (LTD)
    ///
    /// # Arguments
    /// * `delta_time` - t_post - t_pre（毫秒）
    /// * `learning_rate` - 学习率
    pub fn stdp(delta_time: f64, learning_rate: f64) -> f64 {
        Self::stdp_with_config(delta_time, learning_rate, &Self::config())
    }

    /// STDP使用指定配置
    pub fn stdp_with_config(delta_time: f64, learning_rate: f64, config: &LearningConfig) -> f64 {
        if delta_time > 0.0 {
            // post在pre之后激活 → LTP（长时程增强）
            config.stdp_a_plus * (-delta_time / config.stdp_tau_plus).exp() * learning_rate
        } else {
            // post在pre之前激活 → LTD（长时程抑制）
            -config.stdp_a_minus * (delta_time / config.stdp_tau_minus).exp() * learning_rate
        }
    }

    /// 情绪调节学习率（使用配置）
    ///
    /// 高唤醒加速学习，低唤醒放缓
    /// 正效价略微加速，负效价可能导致快速适应
    ///
    /// # Arguments
    /// * `base_rate` - 基础学习率
    /// * `arousal` - 唤醒度 [0, 1]
    /// * `valence` - 效价 [-1, 1]
    pub fn emotion_modulated(
        base_rate: f64,
        arousal: f64,
        valence: f64,
    ) -> f64 {
        Self::emotion_modulated_with_config(base_rate, arousal, valence, &Self::config())
    }

    /// 情绪调节使用指定配置
    pub fn emotion_modulated_with_config(
        base_rate: f64,
        arousal: f64,
        valence: f64,
        config: &LearningConfig,
    ) -> f64 {
        config.emotion_modulated_rate(base_rate, arousal, valence)
    }

    /// 计算权重变化
    ///
    /// 根据规则类型选择合适的学习规则
    pub fn compute_delta(
        rule: PlasticityRule,
        pre_state: f64,
        post_state: f64,
        weight: f64,
        learning_rate: f64,
    ) -> f64 {
        match rule {
            PlasticityRule::Hebbian => Self::hebbian(pre_state, post_state, learning_rate),
            PlasticityRule::Oja => Self::oja(pre_state, post_state, weight, learning_rate),
            PlasticityRule::Stdp => {
                // STDP需要时间信息，这里使用状态相关性简化
                Self::hebbian(pre_state, post_state, learning_rate) * 0.5
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hebbian_both_active() {
        // 两个神经元同时激活
        let delta = LearningRules::hebbian(1.0, 1.0, 0.01);
        assert!((delta - 0.01).abs() < 1e-10);
    }

    #[test]
    fn test_hebbian_opposite() {
        // 一个激活，一个抑制
        let delta = LearningRules::hebbian(1.0, -1.0, 0.01);
        assert!((delta + 0.01).abs() < 1e-10); // 负值
    }

    #[test]
    fn test_oja_stability() {
        // Oja规则应该使权重稳定
        let mut weight = 0.1;
        let lr = 0.01;

        for _ in 0..1000 {
            let delta = LearningRules::oja(1.0, 1.0, weight, lr);
            weight += delta;
        }

        // 权重应该收敛到一个稳定值
        assert!(weight.is_finite());
        assert!(weight.abs() < 10.0);
    }

    #[test]
    fn test_stdp_ltp() {
        // post在pre之后激活 → 应该增强
        let delta = LearningRules::stdp(10.0, 0.01);
        assert!(delta > 0.0);
    }

    #[test]
    fn test_stdp_ltd() {
        // post在pre之前激活 → 应该减弱
        let delta = LearningRules::stdp(-10.0, 0.01);
        assert!(delta < 0.0);
    }

    #[test]
    fn test_emotion_modulated() {
        // 高唤醒应该增加学习率
        let high_arousal = LearningRules::emotion_modulated(0.01, 1.0, 0.0);
        let low_arousal = LearningRules::emotion_modulated(0.01, 0.0, 0.0);

        assert!(high_arousal > low_arousal);
    }
}
