//! 监控模块 - 世界智能体的内部监控与参数调整
//!
//! 实现心跳检测、异常检测、动态参数优化

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::config::MonitorConfig;

/// 监控系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMonitor {
    /// 配置
    config: MonitorConfig,
    /// 心跳记录 (gu_id -> last_heartbeat)
    pub heartbeats: HashMap<Uuid, u64>,
    /// 状态快照历史
    pub snapshots: Vec<WorldSnapshot>,
    /// 异常记录
    pub anomalies: Vec<AnomalyRecord>,
    /// 监控指标
    pub metrics: MonitorMetrics,
}

/// 世界快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub timestamp: u64,
    pub population: u64,
    pub health: f64,
    pub activity: f64,
    pub signal_count: u64,
    pub decision_count: u64,
}

/// 异常记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyRecord {
    pub timestamp: u64,
    pub anomaly_type: AnomalyType,
    pub severity: AnomalySeverity,
    pub description: String,
    pub affected_entities: Vec<Uuid>,
    pub resolved: bool,
}

/// 异常类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalyType {
    /// 心跳超时
    HeartbeatTimeout,
    /// 种群过低
    LowPopulation,
    /// 活跃度下降
    LowActivity,
    /// 网络断连
    NetworkDisconnection,
    /// 经济停滞
    EconomicStagnation,
    /// 决策超时
    DecisionTimeout,
}

/// 异常严重程度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// 监控指标
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MonitorMetrics {
    /// 平均心跳延迟
    pub avg_heartbeat_delay_ms: f64,
    /// 心跳成功率
    pub heartbeat_success_rate: f64,
    /// 异常检测次数
    pub anomaly_count: u64,
    /// 恢复次数
    pub recovery_count: u64,
    /// 平均种群数量
    pub avg_population: f64,
    /// 平均健康度
    pub avg_health: f64,
}

impl WorldMonitor {
    pub fn new(config: MonitorConfig) -> Self {
        Self {
            config,
            heartbeats: HashMap::new(),
            snapshots: Vec::new(),
            anomalies: Vec::new(),
            metrics: MonitorMetrics::default(),
        }
    }

    /// 记录心跳
    pub fn record_heartbeat(&self, gu_id: Uuid) -> Self {
        let mut new_monitor = self.clone();
        let now = current_timestamp();
        new_monitor.heartbeats.insert(gu_id, now);
        new_monitor
    }

    /// 检测心跳超时
    pub fn detect_heartbeat_timeouts(&self, registered_gus: &[Uuid]) -> Vec<Uuid> {
        let now = current_timestamp();
        let timeout = self.config.heartbeat_timeout();

        registered_gus
            .iter()
            .filter(|id| {
                self.heartbeats
                    .get(id)
                    .map(|&last| now - last > timeout)
                    .unwrap_or(true)
            })
            .cloned()
            .collect()
    }

    /// 创建快照
    pub fn create_snapshot(&self, population: u64, health: f64, activity: f64, signal_count: u64, decision_count: u64) -> Self {
        let mut new_monitor = self.clone();
        let snapshot = WorldSnapshot {
            timestamp: current_timestamp(),
            population,
            health,
            activity,
            signal_count,
            decision_count,
        };
        new_monitor.snapshots.push(snapshot);

        // 限制历史大小
        if new_monitor.snapshots.len() > self.config.max_history {
            new_monitor.snapshots.remove(0);
        }

        // 更新指标
        new_monitor.update_metrics();
        new_monitor
    }

    /// 检测异常: Anomaly_score = Σ(|Metric_i - Expected_i| / σ_i)
    pub fn detect_anomalies(&self, population: u64, health: f64, activity: f64) -> Vec<AnomalyType> {
        let mut anomalies = Vec::new();

        // 种群过低（使用配置阈值）
        if population < self.config.min_population_warning {
            anomalies.push(AnomalyType::LowPopulation);
        }

        // 健康度过低
        if health < 0.3 {
            anomalies.push(AnomalyType::LowActivity);
        }

        // 活跃度过低
        if activity < 0.1 {
            anomalies.push(AnomalyType::EconomicStagnation);
        }

        anomalies
    }

    /// 记录异常
    pub fn record_anomaly(&self, anomaly_type: AnomalyType, severity: AnomalySeverity, description: String, affected: Vec<Uuid>) -> Self {
        let mut new_monitor = self.clone();
        let record = AnomalyRecord {
            timestamp: current_timestamp(),
            anomaly_type,
            severity,
            description,
            affected_entities: affected,
            resolved: false,
        };
        new_monitor.anomalies.push(record);
        new_monitor.metrics.anomaly_count += 1;
        new_monitor
    }

    /// 解决异常
    pub fn resolve_anomaly(&self, index: usize) -> Self {
        let mut new_monitor = self.clone();
        if index < new_monitor.anomalies.len() {
            new_monitor.anomalies[index].resolved = true;
            new_monitor.metrics.recovery_count += 1;
        }
        new_monitor
    }

    /// 更新指标
    fn update_metrics(&mut self) {
        if self.snapshots.is_empty() {
            return;
        }

        let count = self.snapshots.len();
        self.metrics.avg_population = self.snapshots.iter().map(|s| s.population as f64).sum::<f64>() / count as f64;
        self.metrics.avg_health = self.snapshots.iter().map(|s| s.health).sum::<f64>() / count as f64;
    }

    /// 获取最新快照
    pub fn latest_snapshot(&self) -> Option<&WorldSnapshot> {
        self.snapshots.last()
    }

    /// 获取活跃异常
    pub fn active_anomalies(&self) -> Vec<&AnomalyRecord> {
        self.anomalies.iter().filter(|a| !a.resolved).collect()
    }

    /// 计算系统稳定性
    pub fn stability_score(&self) -> f64 {
        if self.snapshots.len() < 2 {
            return 1.0;
        }

        // 基于最近快照的波动计算稳定性
        let recent: Vec<_> = self.snapshots.iter().rev().take(10).collect();
        if recent.len() < 2 {
            return 1.0;
        }

        let health_variance = calculate_variance(&recent.iter().map(|s| s.health).collect::<Vec<_>>());
        let activity_variance = calculate_variance(&recent.iter().map(|s| s.activity).collect::<Vec<_>>());

        // 方差越小，稳定性越高
        1.0 / (1.0 + health_variance + activity_variance)
    }
}

/// 计算方差
fn calculate_variance(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

impl MonitorConfig {
    pub fn heartbeat_timeout(&self) -> u64 {
        self.heartbeat_interval * 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_creation() {
        let config = MonitorConfig::default();
        let monitor = WorldMonitor::new(config);
        assert!(monitor.heartbeats.is_empty());
    }

    #[test]
    fn test_heartbeat_recording() {
        let config = MonitorConfig::default();
        let monitor = WorldMonitor::new(config);
        let gu_id = Uuid::new_v4();

        let monitor = monitor.record_heartbeat(gu_id);
        assert!(monitor.heartbeats.contains_key(&gu_id));
    }

    #[test]
    fn test_snapshot_creation() {
        let config = MonitorConfig::default();
        let monitor = WorldMonitor::new(config);

        let monitor = monitor.create_snapshot(10, 0.8, 0.5, 100, 5);
        assert!(!monitor.snapshots.is_empty());
        assert!(monitor.latest_snapshot().is_some());
    }
}
