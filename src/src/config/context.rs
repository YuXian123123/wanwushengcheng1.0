//! 上下文配置

use serde::{Deserialize, Serialize};

/// 上下文配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConfig {
    /// 最大历史记录数
    ///
    /// 保留的最近N条交互记录
    pub max_history: usize,

    /// 会话超时时间（秒）
    ///
    /// 会话空闲多久后过期
    pub session_timeout_secs: u64,

    /// 任务优先级衰减率
    ///
    /// 任务优先级随时间衰减的速度
    pub task_priority_decay: f64,

    /// 关系强度更新率
    ///
    /// 每次交互后关系强度的更新速度
    pub relationship_update_rate: f64,

    /// 最小关系强度
    pub min_relationship_strength: f64,

    /// 最大关系强度
    pub max_relationship_strength: f64,

    /// 上下文向量维度
    ///
    /// 用于表示上下文的向量维度
    pub context_vector_dim: usize,
}

impl ContextConfig {
    /// 创建默认配置
    pub fn new() -> Self {
        Self {
            max_history: 100,
            session_timeout_secs: 1800, // 30分钟
            task_priority_decay: 0.1,
            relationship_update_rate: 0.05,
            min_relationship_strength: 0.0,
            max_relationship_strength: 1.0,
            context_vector_dim: 64,
        }
    }

    /// 验证配置
    pub fn validate(&self) -> Result<(), String> {
        if self.max_history == 0 {
            return Err("max_history 必须大于0".to_string());
        }
        if self.session_timeout_secs == 0 {
            return Err("session_timeout_secs 必须大于0".to_string());
        }
        if self.task_priority_decay <= 0.0 || self.task_priority_decay > 1.0 {
            return Err("task_priority_decay 必须在 (0, 1] 范围内".to_string());
        }
        if self.relationship_update_rate <= 0.0 || self.relationship_update_rate > 1.0 {
            return Err("relationship_update_rate 必须在 (0, 1] 范围内".to_string());
        }
        if self.min_relationship_strength >= self.max_relationship_strength {
            return Err("min_relationship_strength 必须小于 max_relationship_strength".to_string());
        }
        if self.context_vector_dim == 0 {
            return Err("context_vector_dim 必须大于0".to_string());
        }
        Ok(())
    }

    /// 计算衰减后的优先级
    pub fn decay_priority(&self, priority: f64, time_elapsed: f64) -> f64 {
        let decay = (-self.task_priority_decay * time_elapsed).exp();
        priority * decay
    }

    /// 更新关系强度
    pub fn update_relationship(&self, current: f64, delta: f64) -> f64 {
        let new_strength = current + delta * self.relationship_update_rate;
        new_strength.clamp(self.min_relationship_strength, self.max_relationship_strength)
    }
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_valid() {
        let config = ContextConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_priority_decay() {
        let config = ContextConfig::new();
        let initial = 1.0;
        let decayed = config.decay_priority(initial, 10.0);
        assert!(decayed < initial, "优先级应该衰减");
        assert!(decayed > 0.0, "优先级应该保持正值");
    }

    #[test]
    fn test_relationship_update() {
        let config = ContextConfig::new();
        let current = 0.5;
        let updated = config.update_relationship(current, 0.5);
        assert!(updated > current, "正向变化应该增加关系强度");
        assert!(updated <= config.max_relationship_strength);
    }
}
