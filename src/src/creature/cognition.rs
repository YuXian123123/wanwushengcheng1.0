//! 元认知模块 - 定义蛊虫认知状态的数据结构

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 感知级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AwarenessLevel {
    None,
    Low,
    Medium,
    High,
    Full,
}

impl AwarenessLevel {
    /// 获取感知级别的数值表示
    pub fn value(&self) -> u8 {
        match self {
            AwarenessLevel::None => 0,
            AwarenessLevel::Low => 1,
            AwarenessLevel::Medium => 2,
            AwarenessLevel::High => 3,
            AwarenessLevel::Full => 4,
        }
    }
}

/// 学习状态
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LearningState {
    /// 学习速率
    pub rate: f32,
    /// 学习效率
    pub efficiency: f32,
    /// 已学习的能力ID集合
    pub learned_abilities: Vec<Uuid>,
}

impl LearningState {
    pub fn new(rate: f32, efficiency: f32) -> Self {
        Self {
            rate,
            efficiency,
            learned_abilities: Vec::new(),
        }
    }

    /// 添加已学习的能力
    pub fn add_learned_ability(&self, ability_id: Uuid) -> Self {
        let mut new_self = self.clone();
        if !new_self.learned_abilities.contains(&ability_id) {
            new_self.learned_abilities.push(ability_id);
        }
        new_self
    }

    /// 检查是否已学习某个能力
    pub fn has_learned(&self, ability_id: &Uuid) -> bool {
        self.learned_abilities.contains(ability_id)
    }
}

impl Default for LearningState {
    fn default() -> Self {
        Self::new(1.0, 1.0)
    }
}

/// 适应能力
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdaptationCapability {
    /// 适应性
    pub adaptability: f32,
    /// 环境适应集合
    pub environment_adaptations: HashMap<String, f32>,
}

impl AdaptationCapability {
    pub fn new(adaptability: f32) -> Self {
        Self {
            adaptability,
            environment_adaptations: HashMap::new(),
        }
    }

    /// 添加环境适应性
    pub fn add_environment_adaptation(&self, environment: String, adaptation: f32) -> Self {
        let mut new_self = self.clone();
        new_self.environment_adaptations.insert(environment, adaptation);
        new_self
    }

    /// 获取环境适应性
    pub fn get_environment_adaptation(&self, environment: &str) -> Option<f32> {
        self.environment_adaptations.get(environment).copied()
    }
}

impl Default for AdaptationCapability {
    fn default() -> Self {
        Self::new(1.0)
    }
}

/// 记忆条目
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// 记忆内容
    pub content: String,
    /// 重要性
    pub importance: f32,
    /// 时间戳
    pub timestamp: u64,
}

/// 记忆存储
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryBank {
    /// 记忆条目集合
    pub entries: Vec<MemoryEntry>,
    /// 最大容量
    pub capacity: usize,
}

impl MemoryBank {
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            capacity,
        }
    }

    /// 添加记忆
    pub fn add_memory(&self, entry: MemoryEntry) -> Self {
        let mut new_self = self.clone();
        
        // 如果已达到容量上限，移除最不重要的记忆
        if new_self.entries.len() >= new_self.capacity {
            if let Some(min_index) = new_self.entries
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.importance.partial_cmp(&b.importance).unwrap())
                .map(|(index, _)| index) {
                new_self.entries.remove(min_index);
            }
        }
        
        new_self.entries.push(entry);
        new_self
    }

    /// 根据重要性获取记忆
    pub fn get_important_memories(&self, min_importance: f32) -> Vec<&MemoryEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.importance >= min_importance)
            .collect()
    }
}

impl Default for MemoryBank {
    fn default() -> Self {
        Self::new(100)
    }
}

/// 元认知结构体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetaCognition {
    /// 环境感知级别
    pub awareness: AwarenessLevel,
    /// 学习状态
    pub learning: LearningState,
    /// 适应能力
    pub adaptation: AdaptationCapability,
    /// 记忆存储
    pub memory: MemoryBank,
}

impl MetaCognition {
    /// 创建新的元认知状态
    pub fn new(
        awareness: AwarenessLevel,
        learning: LearningState,
        adaptation: AdaptationCapability,
        memory: MemoryBank,
    ) -> Self {
        Self {
            awareness,
            learning,
            adaptation,
            memory,
        }
    }

    /// 提高感知级别
    pub fn increase_awareness(&self) -> Self {
        let new_awareness = match self.awareness {
            AwarenessLevel::None => AwarenessLevel::Low,
            AwarenessLevel::Low => AwarenessLevel::Medium,
            AwarenessLevel::Medium => AwarenessLevel::High,
            AwarenessLevel::High => AwarenessLevel::Full,
            AwarenessLevel::Full => AwarenessLevel::Full,
        };

        let mut new_self = self.clone();
        new_self.awareness = new_awareness;
        new_self
    }

    /// 添加记忆
    pub fn add_memory(&self, entry: MemoryEntry) -> Self {
        let mut new_self = self.clone();
        new_self.memory = new_self.memory.add_memory(entry);
        new_self
    }

    /// 学习新能力
    pub fn learn_ability(&self, ability_id: Uuid) -> Self {
        let mut new_self = self.clone();
        new_self.learning = new_self.learning.add_learned_ability(ability_id);
        new_self
    }
}

impl Default for MetaCognition {
    fn default() -> Self {
        Self::new(
            AwarenessLevel::None,
            LearningState::default(),
            AdaptationCapability::default(),
            MemoryBank::default(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_awareness_level_values() {
        assert_eq!(AwarenessLevel::None.value(), 0);
        assert_eq!(AwarenessLevel::Low.value(), 1);
        assert_eq!(AwarenessLevel::Medium.value(), 2);
        assert_eq!(AwarenessLevel::High.value(), 3);
        assert_eq!(AwarenessLevel::Full.value(), 4);
    }

    #[test]
    fn test_learning_state() {
        let mut learning = LearningState::default();
        let ability_id = Uuid::new_v4();
        
        learning = learning.add_learned_ability(ability_id);
        assert!(learning.has_learned(&ability_id));
    }

    #[test]
    fn test_memory_bank_capacity() {
        let memory_bank = MemoryBank::new(2);
        let entry1 = MemoryEntry {
            content: "记忆1".to_string(),
            importance: 0.5,
            timestamp: 1000,
        };
        
        let entry2 = MemoryEntry {
            content: "记忆2".to_string(),
            importance: 0.8,
            timestamp: 1001,
        };
        
        let entry3 = MemoryEntry {
            content: "记忆3".to_string(),
            importance: 0.3,
            timestamp: 1002,
        };
        
        let memory_bank = memory_bank.add_memory(entry1);
        let memory_bank = memory_bank.add_memory(entry2);
        let memory_bank = memory_bank.add_memory(entry3);
        
        // 应该只有2个记忆，因为容量限制
        assert_eq!(memory_bank.entries.len(), 2);

        // 重要性最低的 entry1 (0.5) 被移除，entry3 (0.3) 被添加
        // 最终剩下 entry2 (0.8) 和 entry3 (0.3)
        let contents: Vec<&str> = memory_bank.entries.iter().map(|e| e.content.as_str()).collect();
        assert!(!contents.contains(&"记忆1")); // entry1 被移除
        assert!(contents.contains(&"记忆2")); // entry2 保留
        assert!(contents.contains(&"记忆3")); // entry3 被添加
    }
}
