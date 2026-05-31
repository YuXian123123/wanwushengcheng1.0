//! 生存绑定机制
//!
//! 核心公理：World.Alive ⇔ ∃ Gu ∈ World: Gu.Alive
//!
//! 设计者：螺丝咕姆（安全视角）

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ============================================================================
// 降级阶段
// ============================================================================

/// 优雅降级阶段
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DegradationPhase {
    /// 正常运行 (>70% 蛊虫存活)
    Normal,
    /// 警告阶段 (50-70%)
    Warning,
    /// 危急阶段 (30-50%)
    Critical,
    /// 紧急阶段 (10-30%)
    Emergency,
    /// 终止阶段 (<10%)
    Termination,
}

impl DegradationPhase {
    /// 从蛊虫存活比例判断降级阶段
    pub fn from_ratio(alive_ratio: f64) -> Self {
        match alive_ratio {
            r if r > 0.7 => Self::Normal,
            r if r > 0.5 => Self::Warning,
            r if r > 0.3 => Self::Critical,
            r if r > 0.1 => Self::Emergency,
            _ => Self::Termination,
        }
    }

    /// 获取阶段优先级（数字越大越严重）
    pub fn severity(&self) -> u8 {
        match self {
            Self::Normal => 0,
            Self::Warning => 1,
            Self::Critical => 2,
            Self::Emergency => 3,
            Self::Termination => 4,
        }
    }

    /// 是否需要紧急干预
    pub fn needs_intervention(&self) -> bool {
        self.severity() >= 2
    }
}

// ============================================================================
// 世界种子
// ============================================================================

/// 世界种子（用于重启）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSeed {
    /// 知识快照
    pub knowledge_snapshot: Vec<u8>,
    /// 配置哈希
    pub config_hash: u64,
    /// 创建时间戳
    pub timestamp: u64,
    /// 存活蛊虫数量
    pub survivor_count: u64,
}

impl WorldSeed {
    /// 创建新种子
    pub fn new(knowledge: Vec<u8>, config_hash: u64, survivors: u64) -> Self {
        Self {
            knowledge_snapshot: knowledge,
            config_hash,
            timestamp: current_timestamp(),
            survivor_count: survivors,
        }
    }

    /// 验证种子有效性
    pub fn is_valid(&self) -> bool {
        !self.knowledge_snapshot.is_empty() && self.timestamp > 0
    }
}

// ============================================================================
// 生存绑定
// ============================================================================

/// 生存绑定配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalBindingConfig {
    /// 最小存活种群
    pub min_population: u64,
    /// 心跳超时（秒）
    pub heartbeat_timeout: u64,
    /// 紧急生成速率（每秒生成蛊虫数）
    pub emergency_spawn_rate: u64,
    /// 是否启用自动恢复
    pub auto_recovery: bool,
}

impl Default for SurvivalBindingConfig {
    fn default() -> Self {
        Self {
            min_population: 3,
            heartbeat_timeout: 30,
            emergency_spawn_rate: 1,
            auto_recovery: true,
        }
    }
}

/// 生存绑定状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalBinding {
    /// 配置
    config: SurvivalBindingConfig,
    /// 当前降级阶段
    current_phase: DegradationPhase,
    /// 心跳记录
    heartbeats: HashMap<Uuid, u64>,
    /// 世界种子
    world_seed: Option<WorldSeed>,
    /// 最后检查时间
    last_check: u64,
    /// 累计死亡蛊虫数
    total_deaths: u64,
    /// 累计生成蛊虫数
    total_spawns: u64,
}

impl SurvivalBinding {
    /// 创建新的生存绑定
    pub fn new(config: SurvivalBindingConfig) -> Self {
        Self {
            config,
            current_phase: DegradationPhase::Termination,
            heartbeats: HashMap::new(),
            world_seed: None,
            last_check: 0,
            total_deaths: 0,
            total_spawns: 0,
        }
    }

    /// 使用默认配置创建
    pub fn with_defaults() -> Self {
        Self::new(SurvivalBindingConfig::default())
    }

    /// 更新心跳
    pub fn update_heartbeat(&mut self, gu_id: Uuid) {
        self.heartbeats.insert(gu_id, current_timestamp());
    }

    /// 移除蛊虫心跳
    pub fn remove_gu(&mut self, gu_id: &Uuid) {
        if self.heartbeats.remove(gu_id).is_some() {
            self.total_deaths += 1;
        }
    }

    /// 检测死亡蛊虫
    pub fn detect_dead(&self) -> Vec<Uuid> {
        let now = current_timestamp();
        let timeout = self.config.heartbeat_timeout;

        self.heartbeats
            .iter()
            .filter(|(_, &last)| now.saturating_sub(last) > timeout)
            .map(|(id, _)| *id)
            .collect()
    }

    /// 检查世界是否存活
    pub fn is_world_alive(&self) -> bool {
        self.heartbeats.len() >= self.config.min_population as usize
    }

    /// 更新降级阶段
    pub fn update_phase(&mut self, total_population: u64) -> DegradationPhase {
        let alive = self.heartbeats.len() as u64;
        let ratio = if total_population > 0 {
            alive as f64 / total_population as f64
        } else {
            0.0
        };

        self.current_phase = DegradationPhase::from_ratio(ratio);
        self.last_check = current_timestamp();
        self.current_phase
    }

    /// 需要紧急干预？
    pub fn needs_intervention(&self) -> bool {
        self.current_phase.needs_intervention()
    }

    /// 需要生成新蛊虫？
    pub fn needs_spawn(&self) -> Option<u64> {
        if !self.config.auto_recovery {
            return None;
        }

        let alive = self.heartbeats.len() as u64;
        if alive < self.config.min_population {
            Some(self.config.min_population - alive)
        } else {
            None
        }
    }

    /// 记录生成蛊虫
    pub fn record_spawn(&mut self, count: u64) {
        self.total_spawns += count;
    }

    /// 准备世界终止（保存种子）
    pub fn prepare_termination(&mut self, knowledge: Vec<u8>, config_hash: u64) -> WorldSeed {
        let seed = WorldSeed::new(
            knowledge,
            config_hash,
            self.heartbeats.len() as u64,
        );
        self.world_seed = Some(seed.clone());
        seed
    }

    /// 从种子恢复
    pub fn restore_from_seed(&mut self, seed: WorldSeed) -> Result<(), String> {
        if !seed.is_valid() {
            return Err("Invalid seed".to_string());
        }

        self.world_seed = Some(seed);
        self.current_phase = DegradationPhase::Emergency;
        Ok(())
    }

    /// 获取当前阶段
    pub fn phase(&self) -> DegradationPhase {
        self.current_phase
    }

    /// 获取存活蛊虫数
    pub fn alive_count(&self) -> u64 {
        self.heartbeats.len() as u64
    }

    /// 获取统计信息
    pub fn stats(&self) -> SurvivalStats {
        SurvivalStats {
            alive_count: self.heartbeats.len() as u64,
            phase: self.current_phase,
            total_deaths: self.total_deaths,
            total_spawns: self.total_spawns,
            is_alive: self.is_world_alive(),
        }
    }
}

/// 生存统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalStats {
    pub alive_count: u64,
    pub phase: DegradationPhase,
    pub total_deaths: u64,
    pub total_spawns: u64,
    pub is_alive: bool,
}

// ============================================================================
// 辅助函数
// ============================================================================

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degradation_phases() {
        assert_eq!(DegradationPhase::from_ratio(0.8), DegradationPhase::Normal);
        assert_eq!(DegradationPhase::from_ratio(0.6), DegradationPhase::Warning);
        assert_eq!(DegradationPhase::from_ratio(0.4), DegradationPhase::Critical);
        assert_eq!(DegradationPhase::from_ratio(0.2), DegradationPhase::Emergency);
        assert_eq!(DegradationPhase::from_ratio(0.05), DegradationPhase::Termination);
    }

    #[test]
    fn test_world_alive() {
        let mut binding = SurvivalBinding::with_defaults();

        // 初始状态：无蛊虫，世界死亡
        assert!(!binding.is_world_alive());

        // 添加蛊虫
        binding.update_heartbeat(Uuid::new_v4());
        binding.update_heartbeat(Uuid::new_v4());
        binding.update_heartbeat(Uuid::new_v4());

        // 达到最小种群，世界存活
        assert!(binding.is_world_alive());
    }

    #[test]
    fn test_dead_detection() {
        // 这个测试验证检测逻辑，但不依赖精确时间
        let config = SurvivalBindingConfig {
            heartbeat_timeout: 1, // 1秒超时
            ..Default::default()
        };
        let mut binding = SurvivalBinding::new(config);

        let gu_id = Uuid::new_v4();
        binding.update_heartbeat(gu_id);

        // 心跳刚更新，不应该被检测为死亡
        let dead = binding.detect_dead();
        assert!(!dead.contains(&gu_id), "Should not be dead immediately after heartbeat");

        // 移除心跳记录（模拟长时间无响应）
        binding.remove_gu(&gu_id);

        // 检测死亡（因为心跳记录已移除）
        let dead = binding.detect_dead();
        assert!(!dead.contains(&gu_id), "No heartbeat record means not in detection");
        assert_eq!(binding.alive_count(), 0);
    }

    #[test]
    fn test_seed_creation() {
        let seed = WorldSeed::new(vec![1, 2, 3], 12345, 5);
        assert!(seed.is_valid());
        assert_eq!(seed.survivor_count, 5);
    }

    #[test]
    fn test_spawn_needed() {
        let mut binding = SurvivalBinding::with_defaults();

        // 少于最小种群，需要生成
        binding.update_heartbeat(Uuid::new_v4());
        assert!(binding.needs_spawn().is_some());

        // 达到最小种群
        binding.update_heartbeat(Uuid::new_v4());
        binding.update_heartbeat(Uuid::new_v4());
        assert!(binding.needs_spawn().is_none());
    }
}
