//! 蛊虫实体模块 - 定义完整的蛊虫数据结构

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

use crate::creature::{
    Ability, Lifecycle, MetaCognition, ResourcePool, ResourceType, ResourceCost,
};

/// 特性结构体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trait {
    /// 特性ID
    pub id: Uuid,
    /// 特性名称
    pub name: String,
    /// 特性描述
    pub description: String,
    /// 特性效果
    pub effects: Vec<TraitEffect>,
}

/// 特性效果
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TraitEffect {
    /// 属性加成
    AttributeBonus(AttributeType, f32),
    /// 抗性
    Resistance(DamageType, f32),
    /// 能力增强
    AbilityEnhancement(Uuid, f32),
    /// 新能力
    NewAbility(Uuid),
}

/// 属性类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributeType {
    Strength,
    Agility,
    Intelligence,
    Constitution,
    Wisdom,
}

/// 伤害类型（与ability模块中的定义保持一致）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    Physical,
    Fire,
    Ice,
    Lightning,
    Poison,
    Psychic,
}

/// 蛊虫实体结构体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GuCreature {
    /// 唯一标识符
    pub id: Uuid,
    /// 名称
    pub name: String,
    /// 生命周期
    pub lifecycle: Lifecycle,
    /// 能力集合
    pub abilities: HashSet<Ability>,
    /// 元认知
    pub cognition: MetaCognition,
    /// 资源池
    pub resources: ResourcePool,
    /// 特性集合
    pub traits: HashSet<Trait>,
}

impl GuCreature {
    /// 创建新的蛊虫
    pub fn new(
        id: Uuid,
        name: String,
        lifecycle: Lifecycle,
        abilities: Vec<Ability>,
        cognition: MetaCognition,
        resources: ResourcePool,
        traits: Vec<Trait>,
    ) -> Self {
        Self {
            id,
            name,
            lifecycle,
            abilities: abilities.into_iter().collect(),
            cognition,
            resources,
            traits: traits.into_iter().collect(),
        }
    }

    /// 获取蛊虫ID
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// 获取蛊虫名称
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 进化蛊虫
    pub fn evolve(&self) -> Option<Self> {
        if let Some(evolved_lifecycle) = self.lifecycle.evolve() {
            let mut evolved_creature = self.clone();
            evolved_creature.lifecycle = evolved_lifecycle;
            Some(evolved_creature)
        } else {
            None
        }
    }

    /// 学习新能力
    pub fn learn_ability(&self, ability: Ability) -> Self {
        let mut new_self = self.clone();
        
        // 检查是否有足够的资源学习能力
        if !new_self.resources.can_afford(&ability.cost) {
            return new_self; // 资源不足，无法学习
        }
        
        // 消耗资源
        if let Some(new_resources) = new_self.resources.consume(&ability.cost) {
            new_self.resources = new_resources;
            
            // 添加能力到集合
            new_self.abilities.insert(ability.clone());
            
            // 更新认知状态
            new_self.cognition = new_self.cognition.learn_ability(ability.id());
        }
        
        new_self
    }

    /// 使用能力
    pub fn use_ability(&self, ability_id: &Uuid) -> Option<(Self, EffectResult)> {
        // 查找能力
        let ability = self.abilities.iter().find(|a| a.id() == *ability_id)?;
        
        // 检查是否有足够的资源使用能力
        if !self.resources.can_afford(&ability.cost) {
            return None;
        }
        
        // 消耗资源
        let new_resources = self.resources.consume(&ability.cost)?;
        
        // 创建新状态
        let mut new_self = self.clone();
        new_self.resources = new_resources;
        
        // 应用能力效果（简化处理）
        let result = EffectResult::Success;
        
        Some((new_self, result))
    }

    /// 添加特性
    pub fn add_trait(&self, trait_item: Trait) -> Self {
        let mut new_self = self.clone();
        new_self.traits.insert(trait_item);
        new_self
    }

    /// 获取特定类型的能力
    pub fn abilities_by_type(&self, damage_type: DamageType) -> Vec<&Ability> {
        self.abilities
            .iter()
            .filter(|ability| match ability.effect() {
                crate::creature::ability::Effect::Damage(dt, _) => *dt == damage_type,
                _ => false,
            })
            .collect()
    }

    /// 计算总属性值（基于特性和基础值）
    pub fn total_attribute(&self, attribute: AttributeType) -> f32 {
        let base_value = 10.0; // 基础值
        let trait_bonus: f32 = self
            .traits
            .iter()
            .map(|t| {
                t.effects
                    .iter()
                    .filter_map(|effect| match effect {
                        TraitEffect::AttributeBonus(attr, bonus) if *attr == attribute => {
                            Some(bonus)
                        }
                        _ => None,
                    })
                    .sum::<f32>()
            })
            .sum();
        
        base_value + trait_bonus
    }
}

/// 能力效果结果
#[derive(Debug, Clone, PartialEq)]
pub enum EffectResult {
    Success,
    Failure(String),
    PartialSuccess { success_rate: f32 },
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_creature_creation() {
        let creature = GuCreature::new(
            Uuid::new_v4(),
            "火灵虫".to_string(),
            Lifecycle::new(crate::creature::lifecycle::LifeStage::Egg),
            vec![],
            MetaCognition::default(),
            ResourcePool::new(),
            vec![],
        );

        assert_eq!(creature.name(), "火灵虫");
        assert_eq!(creature.lifecycle.stage, crate::creature::lifecycle::LifeStage::Egg);
    }

    #[test]
    fn test_creature_evolution() {
        let creature = GuCreature::new(
            Uuid::new_v4(),
            "火灵虫".to_string(),
            Lifecycle::new(crate::creature::lifecycle::LifeStage::Egg),
            vec![],
            MetaCognition::default(),
            ResourcePool::new(),
            vec![],
        );

        let evolved_creature = creature.evolve().unwrap();
        assert_eq!(evolved_creature.lifecycle.stage, crate::creature::lifecycle::LifeStage::Larva);
    }

    #[test]
    fn test_ability_learning() {
        let creature = GuCreature::new(
            Uuid::new_v4(),
            "火灵虫".to_string(),
            Lifecycle::new(crate::creature::lifecycle::LifeStage::Egg),
            vec![],
            MetaCognition::default(),
            ResourcePool::with_values(100, 100, 100, 0),
            vec![],
        );

        let ability = Ability::new(
            Uuid::new_v4(),
            "火焰喷射".to_string(),
            "喷射高温火焰攻击敌人".to_string(),
            crate::creature::ability::Effect::Damage(crate::creature::ability::DamageType::Fire, 50),
            ResourceCost::new(ResourceType::Energy, 30),
            Duration::from_secs(10),
        );

        let learned_creature = creature.learn_ability(ability.clone());
        
        // 检查能力是否已学习
        assert!(learned_creature.abilities.contains(&ability));
        
        // 检查资源是否已消耗
        assert_eq!(learned_creature.resources.energy(), 70);
        
        // 检查认知状态是否更新
        assert!(learned_creature.cognition.learning.has_learned(&ability.id()));
    }
}
