//! 能力模块 - 定义蛊虫能力的数据结构

use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

use crate::creature::{ResourceCost, ResourceType};

/// 能力效果类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Effect {
    Damage(DamageType, u32),
    Heal(u32),
    Buff(BuffType, u32),
    Debuff(DebuffType, u32),
    Transform(TransformType),
}

/// 伤害类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    Physical,
    Fire,
    Ice,
    Lightning,
    Poison,
    Psychic,
}

/// 增益类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuffType {
    Strength,
    Agility,
    Intelligence,
    Defense,
    Speed,
}

/// 减益类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DebuffType {
    Weakness,
    Slow,
    Confusion,
    Poison,
    Silence,
}

/// 转化类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransformType {
    Size(f32),      // 大小变化比例
    Shape(String),  // 形状变化
    Element(String), // 元素属性变化
}

/// 能力结构体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ability {
    /// 全局唯一标识符
    pub id: Uuid,
    /// 能力名称
    pub name: String,
    /// 能力描述
    pub description: String,
    /// 能力效果
    pub effect: Effect,
    /// 资源消耗
    pub cost: ResourceCost,
    /// 冷却时间
    pub cooldown: Duration,
}

impl Ability {
    /// 创建新的能力
    pub fn new(
        id: Uuid,
        name: String,
        description: String,
        effect: Effect,
        cost: ResourceCost,
        cooldown: Duration,
    ) -> Self {
        Self {
            id,
            name,
            description,
            effect,
            cost,
            cooldown,
        }
    }

    /// 获取能力的唯一标识符
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// 获取能力名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 获取能力描述
    pub fn description(&self) -> &str {
        &self.description
    }

    /// 获取能力效果
    pub fn effect(&self) -> &Effect {
        &self.effect
    }

    /// 获取资源消耗
    pub fn cost(&self) -> &ResourceCost {
        &self.cost
    }

    /// 获取冷却时间
    pub fn cooldown(&self) -> Duration {
        self.cooldown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ability_creation() {
        let ability = Ability::new(
            Uuid::new_v4(),
            "火焰喷射".to_string(),
            "喷射高温火焰攻击敌人".to_string(),
            Effect::Damage(DamageType::Fire, 50),
            ResourceCost::new(ResourceType::Energy, 30),
            Duration::from_secs(10),
        );

        assert_eq!(ability.name(), "火焰喷射");
        assert_eq!(ability.description(), "喷射高温火焰攻击敌人");
        assert_eq!(ability.effect(), &Effect::Damage(DamageType::Fire, 50));
        assert_eq!(ability.cost().resource_type, ResourceType::Energy);
        assert_eq!(ability.cost().amount, 30);
        assert_eq!(ability.cooldown(), Duration::from_secs(10));
    }
}
