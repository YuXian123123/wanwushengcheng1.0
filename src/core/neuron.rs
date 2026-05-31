//! 神经元实现
//!
//! 液体神经网络的核心计算单元

use crate::core::NeuronType;
use serde::{Deserialize, Serialize};

/// 获取当前时间戳（毫秒）
fn current_time_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// 神经元状态（可序列化）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuronState {
    /// 唯一标识
    pub id: String,
    /// 神经元类型
    pub neuron_type: NeuronType,
    /// 当前状态值 [-1, 1]
    pub state: f64,
    /// 时间常数 τ [0.1, 10.0]
    pub tau: f64,
    /// 偏置 [-1, 1]
    pub bias: f64,
    /// 活跃度 [0, 1]
    pub activity: f64,
    /// 重要度 [0, 1]
    pub importance: f64,
    /// 最后活跃时间戳（毫秒）
    pub last_active_ms: Option<u64>,
    /// 创建时间戳（毫秒）
    pub created_at_ms: u64,
}

/// 神经元
#[derive(Debug, Clone)]
pub struct Neuron {
    /// 内部状态
    inner: NeuronState,
}

impl Neuron {
    /// 创建新神经元
    pub fn new(id: String, neuron_type: NeuronType) -> Self {
        Self {
            inner: NeuronState {
                id,
                neuron_type,
                state: 0.0,
                tau: 1.0,
                bias: 0.0,
                activity: 0.0,
                importance: 0.0,
                last_active_ms: None,
                created_at_ms: current_time_ms(),
            },
        }
    }

    /// 获取神经元ID
    pub fn id(&self) -> &str {
        &self.inner.id
    }

    /// 获取神经元类型
    pub fn neuron_type(&self) -> NeuronType {
        self.inner.neuron_type
    }

    /// 获取当前状态
    pub fn state(&self) -> f64 {
        self.inner.state
    }

    /// 获取活跃度
    pub fn activity(&self) -> f64 {
        self.inner.activity
    }

    /// 获取重要度
    pub fn importance(&self) -> f64 {
        self.inner.importance
    }

    /// 设置时间常数
    pub fn set_tau(&mut self, tau: f64) {
        self.inner.tau = tau.clamp(0.1, 10.0);
    }

    /// 设置偏置
    pub fn set_bias(&mut self, bias: f64) {
        self.inner.bias = bias.clamp(-1.0, 1.0);
    }

    /// 连续时间状态更新
    ///
    /// 状态方程: τ·dx/dt = -x + input + bias
    /// 使用欧拉法求解: x(t+dt) = x(t) + dt/τ * (-x + input + bias)
    ///
    /// # Arguments
    /// * `input` - 输入信号（来自其他神经元的加权和）
    /// * `dt` - 时间步长
    ///
    /// # Returns
    /// 更新后的状态值
    pub fn update(&mut self, input: f64, dt: f64) -> f64 {
        // 欧拉法求解微分方程
        let dx = (dt / self.inner.tau) * (-self.inner.state + input + self.inner.bias);

        // 状态归一化到 [-1, 1]
        self.inner.state = (self.inner.state + dx).clamp(-1.0, 1.0);

        // 活跃度更新（指数移动平均）
        self.inner.activity = self.inner.activity * 0.99 + self.inner.state.abs() * 0.01;

        // 记录活跃时间
        self.inner.last_active_ms = Some(current_time_ms());

        self.inner.state
    }

    /// 重置神经元状态
    pub fn reset(&mut self) {
        self.inner.state = 0.0;
        self.inner.activity = 0.0;
        self.inner.last_active_ms = None;
    }

    /// 检查神经元是否活跃
    pub fn is_active(&self, threshold: f64) -> bool {
        self.inner.activity > threshold
    }

    /// 检查状态是否异常
    pub fn is_abnormal(&self) -> bool {
        !self.inner.state.is_finite() || self.inner.state.abs() > 0.95
    }

    /// 获取状态快照（用于序列化）
    pub fn to_state(&self) -> NeuronState {
        self.inner.clone()
    }

    /// 从状态恢复（用于反序列化）
    pub fn from_state(state: NeuronState) -> Self {
        Self { inner: state }
    }
}

impl PartialEq for Neuron {
    fn eq(&self, other: &Self) -> bool {
        self.inner.id == other.inner.id
    }
}

impl Eq for Neuron {}

impl std::hash::Hash for Neuron {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuron_creation() {
        let neuron = Neuron::new("test_1".to_string(), NeuronType::Cognitive);
        assert_eq!(neuron.id(), "test_1");
        assert_eq!(neuron.state(), 0.0);
    }

    #[test]
    fn test_state_update_convergence() {
        let mut neuron = Neuron::new("test".to_string(), NeuronType::Cognitive);
        neuron.set_tau(1.0);

        // 恒定输入，状态应收敛到稳态
        let input = 0.5;
        for _ in 0..1000 {
            neuron.update(input, 0.01);
        }

        // 稳态时 dx/dt = 0, 所以 x = input + bias = 0.5
        // 但由于归一化到 [-1, 1]，状态应该在 0.5 附近
        assert!((neuron.state() - 0.5).abs() < 0.1);
    }

    #[test]
    fn test_state_normalization() {
        let mut neuron = Neuron::new("test".to_string(), NeuronType::Cognitive);

        // 极大输入，状态应被归一化
        for _ in 0..100 {
            neuron.update(1000.0, 0.01);
        }

        assert!(neuron.state().abs() <= 1.0);
    }
}
