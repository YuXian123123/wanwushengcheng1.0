//! 突触实现
//!
//! 神经元之间的连接，包含权重和学习规则

use crate::core::PlasticityRule;
use serde::{Deserialize, Serialize};

/// 突触权重上限
pub const MAX_WEIGHT: f64 = 10.0;

/// 获取当前时间戳（毫秒）
fn current_time_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// 突触状态（可序列化）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynapseState {
    /// 唯一标识
    pub id: String,
    /// 前神经元ID
    pub from_neuron_id: String,
    /// 后神经元ID
    pub to_neuron_id: String,
    /// 权重 [-MAX_WEIGHT, MAX_WEIGHT]
    pub weight: f64,
    /// 可塑性规则
    pub plasticity_rule: PlasticityRule,
    /// 最后活跃时间戳（毫秒）
    pub last_active_ms: Option<u64>,
}

/// 突触
#[derive(Debug, Clone)]
pub struct Synapse {
    inner: SynapseState,
}

impl Synapse {
    /// 创建新突触
    pub fn new(
        from_id: String,
        to_id: String,
        initial_weight: f64,
        rule: PlasticityRule,
    ) -> Self {
        let id = format!("{}->{}", from_id, to_id);
        let weight = initial_weight.clamp(-MAX_WEIGHT, MAX_WEIGHT);

        Self {
            inner: SynapseState {
                id,
                from_neuron_id: from_id,
                to_neuron_id: to_id,
                weight,
                plasticity_rule: rule,
                last_active_ms: None,
            },
        }
    }

    /// 获取突触ID
    pub fn id(&self) -> &str {
        &self.inner.id
    }

    /// 获取前神经元ID
    pub fn from(&self) -> &str {
        &self.inner.from_neuron_id
    }

    /// 获取后神经元ID
    pub fn to(&self) -> &str {
        &self.inner.to_neuron_id
    }

    /// 获取权重
    pub fn weight(&self) -> f64 {
        self.inner.weight
    }

    /// 设置权重（带边界约束）
    pub fn set_weight(&mut self, weight: f64) {
        self.inner.weight = weight.clamp(-MAX_WEIGHT, MAX_WEIGHT);
    }

    /// 获取可塑性规则
    pub fn plasticity_rule(&self) -> PlasticityRule {
        self.inner.plasticity_rule
    }

    /// 更新权重（局部学习规则）
    ///
    /// # Arguments
    /// * `pre_state` - 前神经元状态
    /// * `post_state` - 后神经元状态
    /// * `learning_rate` - 学习率
    pub fn update_weight(
        &mut self,
        pre_state: f64,
        post_state: f64,
        learning_rate: f64,
    ) {
        let delta_w = match self.inner.plasticity_rule {
            // 赫布学习: Δw = η · xᵢ · xⱼ
            PlasticityRule::Hebbian => {
                learning_rate * pre_state * post_state
            }

            // Oja规则: Δw = η · y · (x - y · w)
            // 防止权重爆炸
            PlasticityRule::Oja => {
                let y = self.inner.weight * pre_state;
                learning_rate * y * (pre_state - y * self.inner.weight)
            }

            // STDP（简化版，完整实现需要脉冲时序）
            PlasticityRule::Stdp => {
                // 简化：基于状态相关性
                learning_rate * pre_state * post_state * 0.5
            }
        };

        // 更新权重并限制范围
        self.inner.weight = (self.inner.weight + delta_w).clamp(-MAX_WEIGHT, MAX_WEIGHT);
        self.inner.last_active_ms = Some(current_time_ms());
    }

    /// 检查突触是否活跃（权重足够大）
    pub fn is_active(&self, threshold: f64) -> bool {
        self.inner.weight.abs() > threshold
    }

    /// 获取状态快照
    pub fn to_state(&self) -> SynapseState {
        self.inner.clone()
    }
}

impl PartialEq for Synapse {
    fn eq(&self, other: &Self) -> bool {
        self.inner.id == other.inner.id
    }
}

impl Eq for Synapse {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synapse_creation() {
        let synapse = Synapse::new(
            "n1".to_string(),
            "n2".to_string(),
            0.5,
            PlasticityRule::Hebbian,
        );

        assert_eq!(synapse.weight(), 0.5);
        assert_eq!(synapse.from(), "n1");
        assert_eq!(synapse.to(), "n2");
    }

    #[test]
    fn test_weight_clamping() {
        let synapse = Synapse::new(
            "n1".to_string(),
            "n2".to_string(),
            100.0, // 超过上限
            PlasticityRule::Hebbian,
        );

        assert_eq!(synapse.weight(), MAX_WEIGHT);
    }

    #[test]
    fn test_hebbian_learning() {
        let mut synapse = Synapse::new(
            "n1".to_string(),
            "n2".to_string(),
            0.1,
            PlasticityRule::Hebbian,
        );

        let initial_weight = synapse.weight();
        let lr = 0.01;

        // 两个神经元同时激活
        synapse.update_weight(1.0, 1.0, lr);

        // 权重应该增加
        assert!(synapse.weight() > initial_weight);
        assert!((synapse.weight() - initial_weight - lr).abs() < 1e-10);
    }

    #[test]
    fn test_oja_prevents_explosion() {
        let mut synapse = Synapse::new(
            "n1".to_string(),
            "n2".to_string(),
            0.1,
            PlasticityRule::Oja,
        );

        // 持续学习
        for _ in 0..10000 {
            synapse.update_weight(1.0, 1.0, 0.01);
        }

        // Oja规则应该防止权重爆炸
        assert!(synapse.weight().abs() <= MAX_WEIGHT);
        assert!(synapse.weight().abs() > 0.0); // 应该有学习效果
    }
}
