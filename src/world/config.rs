//! 世界模型配置
//!
//! 世界智能体的所有参数通过配置管理

use serde::{Deserialize, Serialize};

/// 世界系统配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    /// 生存配置
    pub survival: WorldSurvivalConfig,
    /// 网络配置
    pub network: WorldNetworkConfig,
    /// 意识配置
    pub consciousness: ConsciousnessConfig,
    /// 监控配置
    pub monitor: MonitorConfig,
    /// 世界神经网络配置
    pub neural: WorldNeuralConfig,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            survival: WorldSurvivalConfig::default(),
            network: WorldNetworkConfig::default(),
            consciousness: ConsciousnessConfig::default(),
            monitor: MonitorConfig::default(),
            neural: WorldNeuralConfig::default(),
        }
    }
}

impl WorldConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<(), String> {
        self.survival.validate()?;
        self.network.validate()?;
        self.consciousness.validate()?;
        self.monitor.validate()?;
        self.neural.validate()?;
        Ok(())
    }
}

/// 世界生存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSurvivalConfig {
    /// 最小蛊虫数量
    pub min_population: u64,
    /// 生存阈值
    pub survival_threshold: f64,
    /// 警戒阈值
    pub warning_threshold: f64,
    /// 危险阈值
    pub danger_threshold: f64,
    /// 濒死阈值
    pub critical_threshold: f64,
    /// 心跳超时（秒）
    pub heartbeat_timeout: u64,
    /// 蛊虫出生携带金币
    pub gu_birth_coins: f64,
}

impl Default for WorldSurvivalConfig {
    fn default() -> Self {
        Self {
            min_population: 10,
            survival_threshold: 0.5,
            warning_threshold: 0.3,
            danger_threshold: 0.2,
            critical_threshold: 0.1,
            heartbeat_timeout: 30,
            gu_birth_coins: 500.0, // 每只蛊虫出生携带500金币
        }
    }
}

impl WorldSurvivalConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.min_population == 0 {
            return Err("min_population must be positive".to_string());
        }
        if self.survival_threshold <= 0.0 || self.survival_threshold > 1.0 {
            return Err("survival_threshold must be between 0 and 1".to_string());
        }
        if self.gu_birth_coins < 0.0 {
            return Err("gu_birth_coins cannot be negative".to_string());
        }
        Ok(())
    }
}

/// 世界网络配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldNetworkConfig {
    /// 每蛊虫接入点数量
    pub access_points_per_gu: usize,
    /// 最大连接数
    pub max_connections: usize,
    /// 信号衰减系数
    pub signal_decay_rate: f64,
    /// 基础延迟（毫秒）
    pub base_latency_ms: u64,
    /// 小世界系数目标
    pub small_world_target: f64,
    /// 世界记忆最大容量
    pub max_memory_size: usize,
    /// Perceive接入点默认权重
    pub perceive_weight: f64,
    /// Cognitive接入点默认权重
    pub cognitive_weight: f64,
    /// Behavior接入点默认权重
    pub behavior_weight: f64,
    /// Comm接入点默认权重
    pub comm_weight: f64,
    /// Survival接入点默认权重
    pub survival_weight: f64,
}

impl Default for WorldNetworkConfig {
    fn default() -> Self {
        Self {
            access_points_per_gu: 5, // 感知、认知、行为、通信、生存
            max_connections: 1000,
            signal_decay_rate: 0.1,
            base_latency_ms: 10,
            small_world_target: 2.0,
            max_memory_size: 10000,
            perceive_weight: 1.0,
            cognitive_weight: 2.0,
            behavior_weight: 1.5,
            comm_weight: 1.0,
            survival_weight: 0.5,
        }
    }
}

impl WorldNetworkConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.access_points_per_gu == 0 {
            return Err("access_points_per_gu must be positive".to_string());
        }
        Ok(())
    }
}

/// 意识配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessConfig {
    /// 共识阈值
    pub consensus_threshold: f64,
    /// 冲突检测阈值
    pub conflict_threshold: f64,
    /// 整合信息目标
    pub integration_target: f64,
    /// 决策超时（毫秒）
    pub decision_timeout_ms: u64,
}

impl Default for ConsciousnessConfig {
    fn default() -> Self {
        Self {
            consensus_threshold: 0.6,
            conflict_threshold: 0.3,
            integration_target: 0.5,
            decision_timeout_ms: 5000,
        }
    }
}

impl ConsciousnessConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.consensus_threshold <= 0.0 || self.consensus_threshold > 1.0 {
            return Err("consensus_threshold must be between 0 and 1".to_string());
        }
        Ok(())
    }
}

/// 监控配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfig {
    /// 心跳检测间隔（秒）
    pub heartbeat_interval: u64,
    /// 状态快照间隔（秒）
    pub snapshot_interval: u64,
    /// 异常检测阈值
    pub anomaly_threshold: f64,
    /// 最大历史记录
    pub max_history: usize,
    /// 最小种群警告阈值
    pub min_population_warning: u64,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval: 10,
            snapshot_interval: 60,
            anomaly_threshold: 3.0,
            max_history: 1000,
            min_population_warning: 5,
        }
    }
}

impl MonitorConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.heartbeat_interval == 0 {
            return Err("heartbeat_interval must be positive".to_string());
        }
        Ok(())
    }
}

/// 世界神经网络配置
///
/// 控制世界级神经网络的行为参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldNeuralConfig {
    /// 状态向量维度（5个接入点）
    pub state_vector_dim: usize,
    /// 意识涌现同步阈值
    pub emergence_sync_threshold: f64,
    /// 意识涌现因子阈值
    pub emergence_factor_threshold: f64,
    /// 跨蛊虫突触最大数量
    pub max_cross_gu_synapses: usize,
    /// 跨蛊虫信号衰减系数
    pub cross_gu_signal_decay: f64,
    /// 聚合权重平滑因子
    pub aggregation_smoothing: f64,
    /// 网络更新时间步长（毫秒）
    pub update_dt_ms: u64,
    /// 多样性计算采样数
    pub diversity_sample_size: usize,
}

impl Default for WorldNeuralConfig {
    fn default() -> Self {
        Self {
            state_vector_dim: 5,
            emergence_sync_threshold: 0.7,
            emergence_factor_threshold: 0.5,
            max_cross_gu_synapses: 100,
            cross_gu_signal_decay: 0.1,
            aggregation_smoothing: 0.1,
            update_dt_ms: 10,
            diversity_sample_size: 100,
        }
    }
}

impl WorldNeuralConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.state_vector_dim == 0 {
            return Err("state_vector_dim must be positive".to_string());
        }
        if self.emergence_sync_threshold <= 0.0 || self.emergence_sync_threshold > 1.0 {
            return Err("emergence_sync_threshold must be between 0 and 1".to_string());
        }
        if self.emergence_factor_threshold <= 0.0 || self.emergence_factor_threshold > 1.0 {
            return Err("emergence_factor_threshold must be between 0 and 1".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_valid() {
        let config = WorldConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_access_points_count() {
        let config = WorldNetworkConfig::default();
        assert_eq!(config.access_points_per_gu, 5);
    }
}
