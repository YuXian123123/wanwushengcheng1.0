//! 生存压力系统 - 黑塔创新架构设计
//!
//! 实现生存压力传导、欲望驱动进化机制

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::config::SurvivalConfig;

/// 生存状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalState {
    /// 实体ID
    pub id: Uuid,
    /// 当前生命值
    pub lifespan: f64,
    /// 最大生命值
    pub max_lifespan: f64,
    /// 当前货币
    pub coins: f64,
    /// 延寿次数
    pub extensions: u32,
    /// 生存压力
    pub pressure: f64,
    /// 进化驱动力
    pub evolution_drive: f64,
}

impl SurvivalState {
    pub fn new(id: Uuid, config: &SurvivalConfig) -> Self {
        Self {
            id,
            lifespan: config.base_lifespan,
            max_lifespan: config.base_lifespan * 2.0,
            coins: 0.0,
            extensions: 0,
            pressure: config.base_pressure,
            evolution_drive: 0.0,
        }
    }

    /// 计算生存压力: P(t) = P₀ × e^(k·t) × (1 - coins/c₀)
    /// t 为已消耗的生命比例
    pub fn calculate_pressure(&self, config: &SurvivalConfig) -> f64 {
        let time_ratio = 1.0 - self.lifespan / config.base_lifespan;
        let time_factor = (config.pressure_growth_rate * time_ratio).exp();
        let coin_factor = 1.0 - (self.coins / config.coin_baseline).min(1.0);
        config.base_pressure * time_factor * coin_factor
    }

    /// 计算欲望函数: D = (coins/c₀)^α × (capabilities/cₐ)^β
    /// 简化版本：D = coins / c₀
    pub fn calculate_desire(&self, config: &SurvivalConfig) -> f64 {
        (self.coins / config.coin_baseline).min(1.0)
    }

    /// 计算进化驱动力: Drive = ω·P + φ·D + ψ·C
    /// 简化版本：Drive = P + D
    pub fn calculate_evolution_drive(&self, config: &SurvivalConfig) -> f64 {
        let pressure = self.calculate_pressure(config);
        let desire = self.calculate_desire(config);
        pressure + desire
    }

    /// 进化概率: E_prob = 1 - e^(-γ·Drive)
    pub fn evolution_probability(&self, config: &SurvivalConfig, gamma: f64) -> f64 {
        let drive = self.calculate_evolution_drive(config);
        1.0 - (-gamma * drive).exp()
    }

    /// 更新生存状态（不可变）
    pub fn update(&self, config: &SurvivalConfig) -> Self {
        let mut new_state = self.clone();
        new_state.pressure = self.calculate_pressure(config);
        new_state.evolution_drive = self.calculate_evolution_drive(config);
        new_state
    }

    /// 消耗生命（不可变）
    pub fn consume_lifespan(&self, amount: f64) -> Option<Self> {
        if self.lifespan - amount < 0.0 {
            return None;
        }
        let mut new_state = self.clone();
        new_state.lifespan -= amount;
        Some(new_state)
    }

    /// 延寿决策: extend = coins > threshold ? 1 : 0
    pub fn can_extend(&self, config: &SurvivalConfig) -> bool {
        self.coins > config.extension_threshold
    }

    /// 执行延寿（不可变）
    pub fn extend_lifespan(&self, config: &SurvivalConfig) -> Option<Self> {
        if !self.can_extend(config) {
            return None;
        }
        if self.lifespan + config.extension_amount > self.max_lifespan {
            return None;
        }
        let mut new_state = self.clone();
        new_state.lifespan += config.extension_amount;
        new_state.coins -= config.extension_threshold;
        new_state.extensions += 1;
        Some(new_state)
    }

    /// 获得货币（不可变）
    pub fn earn_coins(&self, amount: f64) -> Self {
        let mut new_state = self.clone();
        new_state.coins += amount;
        new_state
    }
}

/// 生存系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalSystem {
    /// 配置
    config: SurvivalConfig,
    /// 实体状态
    entities: HashMap<Uuid, SurvivalState>,
}

impl SurvivalSystem {
    pub fn new(config: SurvivalConfig) -> Self {
        Self {
            config,
            entities: HashMap::new(),
        }
    }

    /// 注册实体
    pub fn register(&self, id: Uuid) -> Self {
        let mut new_system = self.clone();
        let state = SurvivalState::new(id, &self.config);
        new_system.entities.insert(id, state);
        new_system
    }

    /// 更新所有实体
    pub fn update_all(&self) -> Self {
        let mut new_system = self.clone();
        for (id, state) in &self.entities {
            let new_state = state.update(&self.config);
            new_system.entities.insert(*id, new_state);
        }
        new_system
    }

    /// 获取实体状态
    pub fn get(&self, id: &Uuid) -> Option<&SurvivalState> {
        self.entities.get(id)
    }

    /// 获取所有实体的平均压力
    pub fn average_pressure(&self) -> f64 {
        if self.entities.is_empty() {
            return 0.0;
        }
        let total: f64 = self.entities.values().map(|s| s.pressure).sum();
        total / self.entities.len() as f64
    }

    /// 自然选择：淘汰压力过高的实体
    pub fn natural_selection(&self, pressure_threshold: f64) -> Self {
        let mut new_system = self.clone();
        new_system.entities.retain(|_, s| s.pressure < pressure_threshold);
        new_system
    }
}

// 需要导入 HashMap
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_survival_state_creation() {
        let config = SurvivalConfig::default();
        let id = Uuid::new_v4();
        let state = SurvivalState::new(id, &config);

        assert_eq!(state.lifespan, config.base_lifespan);
        assert_eq!(state.coins, 0.0);
    }

    #[test]
    fn test_pressure_calculation() {
        let config = SurvivalConfig::default();
        let id = Uuid::new_v4();
        let state = SurvivalState::new(id, &config);

        // 初始压力应该接近基础压力
        let pressure = state.calculate_pressure(&config);
        assert!(pressure > 0.0);
    }

    #[test]
    fn test_lifespan_extension() {
        let config = SurvivalConfig::default();
        let id = Uuid::new_v4();
        let state = SurvivalState::new(id, &config);

        // 没有足够的货币，无法延寿
        assert!(!state.can_extend(&config));

        // 获得足够的货币
        let state = state.earn_coins(config.extension_threshold + 10.0);
        assert!(state.can_extend(&config));

        // 执行延寿
        let extended = state.extend_lifespan(&config).unwrap();
        assert!(extended.lifespan > state.lifespan);
        assert_eq!(extended.extensions, 1);
    }

    #[test]
    fn test_natural_selection() {
        let config = SurvivalConfig::default();
        let system = SurvivalSystem::new(config);

        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        let system = system.register(id1);
        let system = system.register(id2);

        assert_eq!(system.entities.len(), 2);

        // 自然选择不会淘汰压力正常的实体
        let selected = system.natural_selection(100.0);
        assert_eq!(selected.entities.len(), 2);
    }
}
