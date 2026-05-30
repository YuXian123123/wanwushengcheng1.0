//! 蛊虫模块 - 提供生命与能力的数据结构实现
//!
//! 本模块遵循不可变性、数学形式化和类型安全的原则，为蛊虫系统提供优雅的数据结构表示。

pub mod ability;
pub mod lifecycle;
pub mod cognition;
pub mod creature;

// 重新导出主要类型以便于使用
pub use ability::Ability;
pub use lifecycle::Lifecycle;
pub use cognition::MetaCognition;
pub use creature::GuCreature;

/// 资源类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ResourceType {
    Energy,
    Health,
    Mana,
    Experience,
}

/// 资源消耗结构
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ResourceCost {
    pub resource_type: ResourceType,
    pub amount: u32,
}

impl ResourceCost {
    pub fn new(resource_type: ResourceType, amount: u32) -> Self {
        Self {
            resource_type,
            amount,
        }
    }
}

/// 资源池
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResourcePool {
    energy: u32,
    health: u32,
    mana: u32,
    experience: u32,
}

impl ResourcePool {
    pub fn new() -> Self {
        Self {
            energy: 100,
            health: 100,
            mana: 100,
            experience: 0,
        }
    }

    pub fn with_values(energy: u32, health: u32, mana: u32, experience: u32) -> Self {
        Self {
            energy,
            health,
            mana,
            experience,
        }
    }

    pub fn energy(&self) -> u32 {
        self.energy
    }

    pub fn health(&self) -> u32 {
        self.health
    }

    pub fn mana(&self) -> u32 {
        self.mana
    }

    pub fn experience(&self) -> u32 {
        self.experience
    }

    pub fn can_afford(&self, cost: &ResourceCost) -> bool {
        match cost.resource_type {
            ResourceType::Energy => self.energy >= cost.amount,
            ResourceType::Health => self.health >= cost.amount,
            ResourceType::Mana => self.mana >= cost.amount,
            ResourceType::Experience => self.experience >= cost.amount,
        }
    }

    pub fn consume(&self, cost: &ResourceCost) -> Option<Self> {
        if !self.can_afford(cost) {
            return None;
        }

        let mut new_pool = self.clone();
        match cost.resource_type {
            ResourceType::Energy => new_pool.energy -= cost.amount,
            ResourceType::Health => new_pool.health -= cost.amount,
            ResourceType::Mana => new_pool.mana -= cost.amount,
            ResourceType::Experience => new_pool.experience -= cost.amount,
        }

        Some(new_pool)
    }
}

impl Default for ResourcePool {
    fn default() -> Self {
        Self::new()
    }
}
