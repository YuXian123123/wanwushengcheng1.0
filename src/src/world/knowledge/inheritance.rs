//! 知识传承轨道
//!
//! 实现双轨制知识传承：
//! - 遗传轨：个体继承（师徒、同类型）
//! - 文化轨：世界知识库（抽象化存储）

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::{Knowledge, KnowledgeType, KnowledgePriority, KnowledgeConfig};

// ============================================================================
// 遗传轨
// ============================================================================

/// 遗传传承轨道
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticTrack {
    /// 继承者候选列表（蛊虫ID -> 优先级）
    pub heirs: HashMap<Uuid, f64>,
    /// 已传承的知识
    pub inherited_knowledge: HashMap<Uuid, Vec<Uuid>>, // heir_id -> knowledge_ids
    /// 最大继承者数量
    pub max_heirs: usize,
}

impl GeneticTrack {
    pub fn new(config: &KnowledgeConfig) -> Self {
        Self {
            heirs: HashMap::new(),
            inherited_knowledge: HashMap::new(),
            max_heirs: config.max_heirs,
        }
    }

    /// 添加继承者候选
    pub fn add_heir(&mut self, heir_id: Uuid, priority: f64) {
        self.heirs.insert(heir_id, priority);
    }

    /// 选择最佳继承者
    pub fn select_heirs(&self) -> Vec<Uuid> {
        let mut heirs: Vec<_> = self.heirs.iter().collect();
        heirs.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));

        heirs.into_iter()
            .take(self.max_heirs)
            .map(|(id, _)| *id)
            .collect()
    }

    /// 传承知识给继承者
    pub fn inherit(&mut self, heir_id: Uuid, knowledge_ids: Vec<Uuid>) {
        self.inherited_knowledge
            .entry(heir_id)
            .or_insert_with(Vec::new)
            .extend(knowledge_ids);
    }

    /// 计算传承效率
    pub fn calculate_efficiency(&self) -> f64 {
        if self.heirs.is_empty() {
            return 0.0;
        }

        let selected = self.select_heirs();
        if selected.is_empty() {
            return 0.0;
        }

        // 效率 = 选中的继承者平均优先级
        let total_priority: f64 = selected
            .iter()
            .filter_map(|id| self.heirs.get(id))
            .sum();

        total_priority / selected.len() as f64
    }
}

// ============================================================================
// 文化轨
// ============================================================================

/// 文化传承轨道
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalTrack {
    /// 世界知识库
    pub world_knowledge: HashMap<Uuid, Knowledge>,
    /// 知识分类索引
    pub category_index: HashMap<KnowledgeType, Vec<Uuid>>,
    /// 高优先级知识队列
    pub priority_queue: Vec<(Uuid, KnowledgePriority)>,
    /// 知识库容量
    pub capacity: usize,
}

impl CulturalTrack {
    pub fn new() -> Self {
        Self {
            world_knowledge: HashMap::new(),
            category_index: HashMap::new(),
            priority_queue: Vec::new(),
            capacity: 10000,
        }
    }

    /// 添加知识到世界知识库
    pub fn add_knowledge(&mut self, knowledge: Knowledge, priority: KnowledgePriority) {
        let id = knowledge.id;
        let ktype = knowledge.knowledge_type;

        // 添加到知识库
        self.world_knowledge.insert(id, knowledge);

        // 更新分类索引
        self.category_index
            .entry(ktype)
            .or_insert_with(Vec::new)
            .push(id);

        // 添加到优先级队列
        self.priority_queue.push((id, priority));
        self.priority_queue.sort_by(|a, b| b.1.cmp(&a.1));

        // 检查容量
        self.enforce_capacity();
    }

    /// 获取知识
    pub fn get_knowledge(&self, id: &Uuid) -> Option<&Knowledge> {
        self.world_knowledge.get(id)
    }

    /// 按类型获取知识
    pub fn get_by_type(&self, ktype: KnowledgeType) -> Vec<&Knowledge> {
        self.category_index
            .get(&ktype)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| self.world_knowledge.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 获取高优先级知识
    pub fn get_high_priority(&self, limit: usize) -> Vec<&Knowledge> {
        self.priority_queue
            .iter()
            .take(limit)
            .filter_map(|(id, _)| self.world_knowledge.get(id))
            .collect()
    }

    /// 强制容量限制
    fn enforce_capacity(&mut self) {
        while self.world_knowledge.len() > self.capacity {
            // 移除优先级最低的知识
            if let Some((id, _)) = self.priority_queue.pop() {
                self.world_knowledge.remove(&id);

                // 更新分类索引
                for ids in self.category_index.values_mut() {
                    ids.retain(|k| k != &id);
                }
            }
        }
    }

    /// 计算知识多样性
    pub fn calculate_diversity(&self) -> f64 {
        if self.world_knowledge.is_empty() {
            return 0.0;
        }

        // 使用类型分布的熵来衡量多样性
        let total = self.world_knowledge.len() as f64;
        let mut entropy = 0.0;

        for ids in self.category_index.values() {
            let p = ids.len() as f64 / total;
            if p > 0.0 {
                entropy -= p * p.ln();
            }
        }

        entropy
    }
}

// ============================================================================
// 知识传承系统
// ============================================================================

/// 知识传承系统
#[derive(Debug, Clone)]
pub struct KnowledgeInheritance {
    /// 配置
    config: KnowledgeConfig,
    /// 遗传轨
    pub genetic: GeneticTrack,
    /// 文化轨
    pub cultural: CulturalTrack,
    /// 传承历史
    pub history: Vec<InheritanceRecord>,
}

/// 传承记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InheritanceRecord {
    /// 记录ID
    pub id: Uuid,
    /// 源蛊虫ID
    pub source_gu: Uuid,
    /// 知识ID列表
    pub knowledge_ids: Vec<Uuid>,
    /// 继承者ID列表
    pub heir_ids: Vec<Uuid>,
    /// 是否进入文化轨
    pub to_cultural: bool,
    /// 时间戳
    pub timestamp: u64,
}

impl KnowledgeInheritance {
    pub fn new(config: KnowledgeConfig) -> Self {
        Self {
            genetic: GeneticTrack::new(&config),
            cultural: CulturalTrack::new(),
            config,
            history: Vec::new(),
        }
    }

    /// 处理蛊虫死亡时的知识传承
    pub fn process_gu_death(
        &mut self,
        dying_gu: Uuid,
        knowledge_list: Vec<Knowledge>,
        potential_heirs: &[(Uuid, f64)], // (heir_id, priority)
    ) -> InheritanceRecord {
        // 1. 添加继承者候选
        for (heir_id, priority) in potential_heirs {
            self.genetic.add_heir(*heir_id, *priority);
        }

        // 2. 选择最佳继承者
        let heirs = self.genetic.select_heirs();

        // 3. 分配知识
        let knowledge_ids: Vec<Uuid> = knowledge_list.iter().map(|k| k.id).collect();

        // 遗传轨传承
        for heir_id in &heirs {
            self.genetic.inherit(*heir_id, knowledge_ids.clone());
        }

        // 4. 同时添加到文化轨（高优先级知识）
        let mut to_cultural = false;
        for knowledge in knowledge_list {
            let value = knowledge.calculate_value();
            let priority = if value > 0.8 {
                KnowledgePriority::Critical
            } else if value > 0.6 {
                KnowledgePriority::High
            } else if value > 0.4 {
                KnowledgePriority::Medium
            } else {
                KnowledgePriority::Low
            };

            if priority >= KnowledgePriority::Medium {
                self.cultural.add_knowledge(knowledge, priority);
                to_cultural = true;
            }
        }

        // 5. 记录传承历史
        let record = InheritanceRecord {
            id: Uuid::new_v4(),
            source_gu: dying_gu,
            knowledge_ids: knowledge_ids.clone(),
            heir_ids: heirs,
            to_cultural,
            timestamp: current_timestamp(),
        };

        self.history.push(record.clone());
        record
    }

    /// 计算传承效率
    pub fn calculate_total_efficiency(&self) -> f64 {
        let genetic_eff = self.genetic.calculate_efficiency();
        let cultural_diversity = self.cultural.calculate_diversity();

        // 综合效率 = 遗传效率 × 0.6 + 文化多样性 × 0.4
        genetic_eff * 0.6 + cultural_diversity * 0.4
    }

    /// 获取统计信息
    pub fn get_stats(&self) -> InheritanceStats {
        InheritanceStats {
            total_inheritances: self.history.len(),
            genetic_heirs: self.genetic.heirs.len(),
            cultural_knowledge: self.cultural.world_knowledge.len(),
            efficiency: self.calculate_total_efficiency(),
        }
    }
}

/// 传承统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InheritanceStats {
    pub total_inheritances: usize,
    pub genetic_heirs: usize,
    pub cultural_knowledge: usize,
    pub efficiency: f64,
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genetic_track_creation() {
        let config = KnowledgeConfig::default();
        let track = GeneticTrack::new(&config);
        assert!(track.heirs.is_empty());
    }

    #[test]
    fn test_genetic_track_add_heir() {
        let config = KnowledgeConfig::default();
        let mut track = GeneticTrack::new(&config);

        track.add_heir(Uuid::new_v4(), 0.8);
        track.add_heir(Uuid::new_v4(), 0.6);

        assert_eq!(track.heirs.len(), 2);
    }

    #[test]
    fn test_genetic_track_select_heirs() {
        let config = KnowledgeConfig { max_heirs: 2, ..Default::default() };
        let mut track = GeneticTrack::new(&config);

        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let id3 = Uuid::new_v4();

        track.add_heir(id1, 0.5);
        track.add_heir(id2, 0.9);
        track.add_heir(id3, 0.7);

        let selected = track.select_heirs();
        assert_eq!(selected.len(), 2);
        assert_eq!(selected[0], id2); // 最高优先级
    }

    #[test]
    fn test_cultural_track_creation() {
        let track = CulturalTrack::new();
        assert!(track.world_knowledge.is_empty());
    }

    #[test]
    fn test_cultural_track_add_knowledge() {
        let mut track = CulturalTrack::new();

        let knowledge = Knowledge::new(
            "test".to_string(),
            KnowledgeType::Skill,
            Uuid::new_v4(),
        );

        track.add_knowledge(knowledge, KnowledgePriority::High);

        assert_eq!(track.world_knowledge.len(), 1);
    }

    #[test]
    fn test_cultural_track_get_by_type() {
        let mut track = CulturalTrack::new();

        let k1 = Knowledge::new("skill1".to_string(), KnowledgeType::Skill, Uuid::new_v4());
        let k2 = Knowledge::new("skill2".to_string(), KnowledgeType::Skill, Uuid::new_v4());
        let k3 = Knowledge::new("fact1".to_string(), KnowledgeType::Fact, Uuid::new_v4());

        track.add_knowledge(k1, KnowledgePriority::Medium);
        track.add_knowledge(k2, KnowledgePriority::Medium);
        track.add_knowledge(k3, KnowledgePriority::Low);

        let skills = track.get_by_type(KnowledgeType::Skill);
        assert_eq!(skills.len(), 2);
    }

    #[test]
    fn test_knowledge_inheritance_creation() {
        let config = KnowledgeConfig::default();
        let system = KnowledgeInheritance::new(config);
        assert!(system.history.is_empty());
    }

    #[test]
    fn test_process_gu_death() {
        let config = KnowledgeConfig::default();
        let mut system = KnowledgeInheritance::new(config);

        let dying_gu = Uuid::new_v4();
        let heir1 = Uuid::new_v4();
        let heir2 = Uuid::new_v4();

        let k1 = Knowledge::new("skill1".to_string(), KnowledgeType::Skill, dying_gu);
        let k2 = Knowledge::new("skill2".to_string(), KnowledgeType::Skill, dying_gu);

        let knowledge_list = vec![k1, k2];
        let potential_heirs = vec![(heir1, 0.9), (heir2, 0.7)];

        let record = system.process_gu_death(dying_gu, knowledge_list, &potential_heirs);

        assert!(!record.heir_ids.is_empty());
        assert_eq!(system.history.len(), 1);
    }

    #[test]
    fn test_inheritance_efficiency() {
        let config = KnowledgeConfig::default();
        let mut system = KnowledgeInheritance::new(config);

        // 添加一些知识到文化轨
        let k = Knowledge::new("test".to_string(), KnowledgeType::Skill, Uuid::new_v4());
        system.cultural.add_knowledge(k, KnowledgePriority::High);

        let efficiency = system.calculate_total_efficiency();
        assert!(efficiency >= 0.0);
    }
}
