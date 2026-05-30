//! 生命周期模块 - 定义蛊虫生命周期的数据结构

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use uuid::Uuid;

/// 生命阶段枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LifeStage {
    Egg,
    Larva,
    Pupa,
    Adult,
    Elder,
    Dead,
}

impl LifeStage {
    /// 获取下一阶段
    pub fn next_stage(&self) -> Option<LifeStage> {
        match self {
            LifeStage::Egg => Some(LifeStage::Larva),
            LifeStage::Larva => Some(LifeStage::Pupa),
            LifeStage::Pupa => Some(LifeStage::Adult),
            LifeStage::Adult => Some(LifeStage::Elder),
            LifeStage::Elder => Some(LifeStage::Dead),
            LifeStage::Dead => None,
        }
    }

    /// 获取阶段名称
    pub fn name(&self) -> &'static str {
        match self {
            LifeStage::Egg => "卵期",
            LifeStage::Larva => "幼虫期",
            LifeStage::Pupa => "蛹期",
            LifeStage::Adult => "成虫期",
            LifeStage::Elder => "老年期",
            LifeStage::Dead => "死亡期",
        }
    }
}

/// 阶段转换
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Transition {
    /// 转换条件
    pub condition: TransitionCondition,
    /// 目标阶段
    pub target_stage: LifeStage,
}

/// 转换条件
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransitionCondition {
    TimeBased(Duration),
    ResourceBased(ResourceRequirement),
    EventBased(String),
}

/// 资源需求
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceRequirement {
    pub resource_type: String,
    pub amount: u32,
}

/// 触发器类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Trigger {
    TimeBased(Duration),
    ResourceThreshold(String, u32),
    ExternalEvent(String),
}

/// 动作类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Evolve,
    LearnAbility(Uuid),
    ModifyTrait(Uuid),
    ReleaseEvent(String),
}

/// 生命周期结构体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lifecycle {
    /// 当前生命阶段
    pub stage: LifeStage,
    /// 可能的阶段转换
    pub transitions: HashSet<Transition>,
    /// 当前阶段持续时间
    pub duration: Duration,
    /// 触发器映射
    pub triggers: HashMap<Trigger, Action>,
}

impl Lifecycle {
    /// 创建新的生命周期
    pub fn new(stage: LifeStage) -> Self {
        Self {
            stage,
            transitions: HashSet::new(),
            duration: Duration::new(0, 0),
            triggers: HashMap::new(),
        }
    }

    /// 添加阶段转换
    pub fn add_transition(&self, transition: Transition) -> Self {
        let mut new_self = self.clone();
        new_self.transitions.insert(transition);
        new_self
    }

    /// 添加触发器
    pub fn add_trigger(&self, trigger: Trigger, action: Action) -> Self {
        let mut new_self = self.clone();
        new_self.triggers.insert(trigger, action);
        new_self
    }

    /// 增加持续时间
    pub fn add_duration(&self, duration: Duration) -> Self {
        let mut new_self = self.clone();
        new_self.duration += duration;
        new_self
    }

    /// 进化到下一阶段
    pub fn evolve(&self) -> Option<Self> {
        if let Some(next_stage) = self.stage.next_stage() {
            let mut new_self = self.clone();
            new_self.stage = next_stage;
            new_self.duration = Duration::new(0, 0);
            Some(new_self)
        } else {
            None
        }
    }

    /// 检查是否可以进化
    pub fn can_evolve(&self) -> bool {
        self.stage.next_stage().is_some()
    }
}

impl Default for Lifecycle {
    fn default() -> Self {
        Self::new(LifeStage::Egg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_life_stage_evolution() {
        assert_eq!(LifeStage::Egg.next_stage(), Some(LifeStage::Larva));
        assert_eq!(LifeStage::Larva.next_stage(), Some(LifeStage::Pupa));
        assert_eq!(LifeStage::Pupa.next_stage(), Some(LifeStage::Adult));
        assert_eq!(LifeStage::Adult.next_stage(), Some(LifeStage::Elder));
        assert_eq!(LifeStage::Elder.next_stage(), Some(LifeStage::Dead));
        assert_eq!(LifeStage::Dead.next_stage(), None);
    }

    #[test]
    fn test_lifecycle_evolution() {
        let lifecycle = Lifecycle::new(LifeStage::Egg);
        let evolved = lifecycle.evolve().unwrap();
        
        assert_eq!(evolved.stage, LifeStage::Larva);
        assert_eq!(evolved.duration, Duration::new(0, 0));
    }

    #[test]
    fn test_lifecycle_cannot_evolve() {
        let lifecycle = Lifecycle::new(LifeStage::Dead);
        assert_eq!(lifecycle.evolve(), None);
    }
}
