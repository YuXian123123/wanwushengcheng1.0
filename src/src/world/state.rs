//! 世界状态模块 - 世界智能体的状态管理

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::access_point::{AccessPoint, Signal};
use super::config::WorldConfig;

/// 世界健康状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorldHealthStatus {
    /// 健康
    Healthy,
    /// 正常
    Normal,
    /// 警戒
    Warning,
    /// 危险
    Danger,
    /// 濒死
    Critical,
    /// 死亡
    Dead,
}

/// 世界状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    /// 世界ID
    pub id: Uuid,
    /// 健康度 (0-1)
    pub health: f64,
    /// 健康状态
    pub health_status: WorldHealthStatus,
    /// 蛊虫数量
    pub population: u64,
    /// 平均蛊虫生命值
    pub avg_gu_health: f64,
    /// 多样性指数
    pub diversity: f64,
    /// 活跃度
    pub activity: f64,
    /// 接入点网络
    pub access_points: HashMap<Uuid, AccessPoint>,
    /// 连接列表 (from, to, weight)
    pub connections: Vec<(Uuid, Uuid, f64)>,
    /// 当前意图
    pub current_intention: Option<Intention>,
    /// 待处理决策
    pub pending_decisions: Vec<Decision>,
    /// 知识库
    pub knowledge_base: HashMap<Uuid, Knowledge>,
    /// 世界记忆（备份）
    pub world_memory: Vec<Knowledge>,
    /// 监控指标
    pub metrics: WorldMetrics,
}

impl WorldState {
    pub fn new(config: &WorldConfig) -> Self {
        Self {
            id: Uuid::new_v4(),
            health: 1.0,
            health_status: WorldHealthStatus::Healthy,
            population: 0,
            avg_gu_health: 1.0,
            diversity: 1.0,
            activity: 0.0,
            access_points: HashMap::new(),
            connections: Vec::new(),
            current_intention: None,
            pending_decisions: Vec::new(),
            knowledge_base: HashMap::new(),
            world_memory: Vec::new(),
            metrics: WorldMetrics::default(),
        }
    }

    /// 计算世界健康度: W_health = α × Population + β × Diversity + γ × Activity
    pub fn calculate_health(&self, config: &WorldConfig) -> f64 {
        let alpha = 0.4;
        let beta = 0.3;
        let gamma = 0.3;

        let pop_factor = (self.population as f64 / config.survival.min_population as f64).min(2.0) / 2.0;
        let act_factor = self.activity;

        alpha * pop_factor + beta * self.diversity + gamma * act_factor
    }

    /// 更新健康状态
    pub fn update_health_status(&self, config: &WorldConfig) -> Self {
        let health = self.calculate_health(config);
        let status = if health >= config.survival.survival_threshold {
            WorldHealthStatus::Healthy
        } else if health >= config.survival.warning_threshold {
            WorldHealthStatus::Warning
        } else if health >= config.survival.danger_threshold {
            WorldHealthStatus::Danger
        } else if health >= config.survival.critical_threshold {
            WorldHealthStatus::Critical
        } else if health > 0.0 {
            WorldHealthStatus::Dead
        } else {
            WorldHealthStatus::Dead
        };

        let mut new_state = self.clone();
        new_state.health = health;
        new_state.health_status = status;
        new_state
    }

    /// 注册蛊虫（添加接入点）
    pub fn register_gu(&self, gu_id: Uuid, access_points: Vec<AccessPoint>) -> Self {
        let mut new_state = self.clone();
        for point in access_points {
            new_state.access_points.insert(point.id, point);
        }
        new_state.population += 1;
        new_state
    }

    /// 注销蛊虫
    pub fn unregister_gu(&self, gu_id: &Uuid, access_point_ids: &[Uuid]) -> Self {
        let mut new_state = self.clone();
        for id in access_point_ids {
            new_state.access_points.remove(id);
        }
        new_state.population = new_state.population.saturating_sub(1);
        new_state
    }

    /// 检查生存条件: N_Gu ≥ N_min
    pub fn check_survival(&self, config: &WorldConfig) -> bool {
        self.population >= config.survival.min_population
    }

    /// 添加连接
    pub fn add_connection(&self, from: Uuid, to: Uuid, weight: f64) -> Self {
        let mut new_state = self.clone();
        new_state.connections.push((from, to, weight));
        new_state
    }

    /// 备份知识到世界记忆
    pub fn backup_knowledge(&self, knowledge: Knowledge, max_memory_size: usize) -> Self {
        let mut new_state = self.clone();
        new_state.world_memory.push(knowledge);
        // 限制记忆大小
        while new_state.world_memory.len() > max_memory_size {
            new_state.world_memory.remove(0);
        }
        new_state
    }
}

/// 意图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intention {
    pub id: Uuid,
    pub description: String,
    pub priority: f64,
    pub supporters: Vec<Uuid>,
    pub confidence: f64,
}

/// 决策
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub id: Uuid,
    pub options: Vec<DecisionOption>,
    pub votes: HashMap<Uuid, usize>, // gu_id -> option_index
    pub deadline: u64,
    pub status: DecisionStatus,
}

/// 决策选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionOption {
    pub description: String,
    pub expected_outcome: String,
}

/// 决策状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecisionStatus {
    Pending,
    InProgress,
    Resolved,
    Expired,
}

/// 知识
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knowledge {
    pub id: Uuid,
    pub content: String,
    pub proposer: Uuid,
    pub supporters: Vec<Uuid>,
    pub confidence: f64,
    pub scope: KnowledgeScope,
    pub created: u64,
    pub last_used: u64,
    pub usage_count: u64,
}

/// 知识范围
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnowledgeScope {
    Local,
    Group,
    Global,
}

/// 世界监控指标
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldMetrics {
    /// 总信号数
    pub total_signals: u64,
    /// 总决策数
    pub total_decisions: u64,
    /// 成功决策数
    pub successful_decisions: u64,
    /// 平均响应时间
    pub avg_response_time_ms: f64,
    /// 网络连通性
    pub connectivity: f64,
    /// 小世界系数
    pub small_world_coefficient: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_state_creation() {
        let config = WorldConfig::default();
        let state = WorldState::new(&config);

        assert_eq!(state.health, 1.0);
        assert_eq!(state.population, 0);
    }

    #[test]
    fn test_health_calculation() {
        let config = WorldConfig::default();
        let state = WorldState::new(&config);

        let health = state.calculate_health(&config);
        assert!(health >= 0.0 && health <= 1.0);
    }

    #[test]
    fn test_survival_check() {
        let config = WorldConfig::default();
        let state = WorldState::new(&config);

        // 初始状态不满足生存条件
        assert!(!state.check_survival(&config));
    }
}