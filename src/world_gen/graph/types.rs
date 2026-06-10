//! 图谱核心类型定义
//!
//! 定义文本关系图谱的所有核心数据结构

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ============================================================================
// 类型别名
// ============================================================================

/// 实体ID
pub type EntityId = String;
/// 关系ID
pub type RelationId = String;
/// 图谱ID
pub type GraphId = String;

// ============================================================================
// 实体类型
// ============================================================================

/// 实体类型（可扩展）
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    // === 生物类 ===
    /// 人
    Person,
    /// 动物
    Animal,
    /// 植物
    Plant,

    // === 物理对象 ===
    /// 建筑
    Building,
    /// 车辆
    Vehicle,
    /// 普通物体
    Object,

    // === 空间位置 ===
    /// 地点
    Location,
    /// 区域
    Region,

    // === 抽象概念 ===
    /// 概念
    Concept,
    /// 事件
    Event,
    /// 时间
    Time,

    // === 自定义 ===
    /// 自定义类型（通过配置扩展）
    Custom(String),
}

impl EntityType {
    /// 获取类型名称
    pub fn name(&self) -> &str {
        match self {
            EntityType::Person => "人",
            EntityType::Animal => "动物",
            EntityType::Plant => "植物",
            EntityType::Building => "建筑",
            EntityType::Vehicle => "车辆",
            EntityType::Object => "物体",
            EntityType::Location => "地点",
            EntityType::Region => "区域",
            EntityType::Concept => "概念",
            EntityType::Event => "事件",
            EntityType::Time => "时间",
            EntityType::Custom(name) => name,
        }
    }

    /// 是否为生物类型
    pub fn is_biological(&self) -> bool {
        matches!(self, EntityType::Person | EntityType::Animal | EntityType::Plant)
    }

    /// 是否为物理对象
    pub fn is_physical(&self) -> bool {
        matches!(self, EntityType::Building | EntityType::Vehicle | EntityType::Object)
    }

    /// 是否为空间位置
    pub fn is_spatial(&self) -> bool {
        matches!(self, EntityType::Location | EntityType::Region)
    }
}

// ============================================================================
// 关系类型
// ============================================================================

/// 关系类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationType {
    // === 空间关系 ===
    /// 包含关系 (A contains B)
    Contains,
    /// 相邻关系
    Adjacent,
    /// 上方关系
    Above,
    /// 下方关系
    Below,
    /// 内部关系
    Inside,

    // === 社会关系 ===
    /// 拥有关系
    Owns,
    /// 认识关系
    Knows,
    /// 亲属关系
    FamilyOf,

    // === 功能关系 ===
    /// 使用关系
    Uses,
    /// 创建关系
    Creates,
    /// 依赖关系
    DependsOn,

    // === 时序关系 ===
    /// 先于
    Before,
    /// 后于
    After,
    /// 期间
    During,

    // === 属性关系 ===
    /// 属性关系（颜色、大小等）
    HasAttribute,

    // === 自定义 ===
    Custom(String),
}

impl RelationType {
    /// 获取关系名称
    pub fn name(&self) -> &str {
        match self {
            RelationType::Contains => "包含",
            RelationType::Adjacent => "相邻",
            RelationType::Above => "在上方",
            RelationType::Below => "在下方",
            RelationType::Inside => "在内部",
            RelationType::Owns => "拥有",
            RelationType::Knows => "认识",
            RelationType::FamilyOf => "亲属",
            RelationType::Uses => "使用",
            RelationType::Creates => "创建",
            RelationType::DependsOn => "依赖",
            RelationType::Before => "先于",
            RelationType::After => "后于",
            RelationType::During => "期间",
            RelationType::HasAttribute => "具有属性",
            RelationType::Custom(name) => name,
        }
    }

    /// 是否为空间关系
    pub fn is_spatial(&self) -> bool {
        matches!(
            self,
            RelationType::Contains
                | RelationType::Adjacent
                | RelationType::Above
                | RelationType::Below
                | RelationType::Inside
        )
    }

    /// 是否为属性关系
    pub fn is_attribute(&self) -> bool {
        matches!(self, RelationType::HasAttribute)
    }
}

/// 关系方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationDirection {
    /// 正向 (from → to)
    Forward,
    /// 反向 (from ← to)
    Backward,
    /// 双向 (from ↔ to)
    Bidirectional,
}

// ============================================================================
// 属性值
// ============================================================================

/// 属性值（支持多种类型）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttributeValue {
    /// 字符串
    String(String),
    /// 整数
    Integer(i64),
    /// 浮点数
    Float(f64),
    /// 布尔值
    Boolean(bool),
    /// 向量
    Vector(Vec<f64>),
}

impl AttributeValue {
    /// 获取字符串值
    pub fn as_str(&self) -> Option<&str> {
        match self {
            AttributeValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// 获取浮点值
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            AttributeValue::Float(f) => Some(*f),
            AttributeValue::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }
}

// ============================================================================
// 文本引用
// ============================================================================

/// 文本中的位置引用
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextReference {
    /// 起始位置（字符索引）
    pub start: usize,
    /// 结束位置（字符索引）
    pub end: usize,
    /// 引用的文本
    pub text: String,
}

impl TextReference {
    pub fn new(start: usize, end: usize, text: String) -> Self {
        Self { start, end, text }
    }

    /// 长度
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// ============================================================================
// 图谱实体
// ============================================================================

/// 图谱实体节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEntity {
    /// 唯一标识
    pub id: EntityId,
    /// 实体名称
    pub name: String,
    /// 实体类型
    pub entity_type: EntityType,
    /// 属性集合
    pub attributes: HashMap<String, AttributeValue>,
    /// 原文引用
    pub source_refs: Vec<TextReference>,
    /// 概念向量（用于语义相似度计算）
    pub concept_vector: Vec<f64>,
    /// 置信度 [0, 1]
    pub confidence: f64,
}

impl GraphEntity {
    /// 创建新实体
    pub fn new(name: String, entity_type: EntityType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            entity_type,
            attributes: HashMap::new(),
            source_refs: Vec::new(),
            concept_vector: Vec::new(),
            confidence: 1.0,
        }
    }

    /// 设置属性
    pub fn with_attribute(mut self, key: String, value: AttributeValue) -> Self {
        self.attributes.insert(key, value);
        self
    }

    /// 设置置信度
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// 添加原文引用
    pub fn add_reference(&mut self, reference: TextReference) {
        self.source_refs.push(reference);
    }

    /// 获取属性值
    pub fn get_attribute(&self, key: &str) -> Option<&AttributeValue> {
        self.attributes.get(key)
    }
}

// ============================================================================
// 图谱关系
// ============================================================================

/// 图谱关系边
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphRelation {
    /// 唯一标识
    pub id: RelationId,
    /// 起始实体ID
    pub from: EntityId,
    /// 目标实体ID
    pub to: EntityId,
    /// 关系类型
    pub relation_type: RelationType,
    /// 关系强度 [0, 1]
    pub strength: f64,
    /// 关系方向
    pub direction: RelationDirection,
    /// 证据（原文引用）
    pub evidence: Vec<TextReference>,
}

impl GraphRelation {
    /// 创建新关系
    pub fn new(from: EntityId, to: EntityId, relation_type: RelationType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            from,
            to,
            relation_type,
            strength: 1.0,
            direction: RelationDirection::Forward,
            evidence: Vec::new(),
        }
    }

    /// 设置强度
    pub fn with_strength(mut self, strength: f64) -> Self {
        self.strength = strength.clamp(0.0, 1.0);
        self
    }

    /// 设置方向
    pub fn with_direction(mut self, direction: RelationDirection) -> Self {
        self.direction = direction;
        self
    }

    /// 添加证据
    pub fn add_evidence(&mut self, reference: TextReference) {
        self.evidence.push(reference);
    }
}

// ============================================================================
// 图谱层次结构
// ============================================================================

/// 图谱层次结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphHierarchy {
    /// 根节点列表（可能有多个根）
    pub roots: Vec<EntityId>,
    /// 父节点映射：子节点 → 父节点
    pub parent_map: HashMap<EntityId, EntityId>,
    /// 子节点映射：父节点 → 子节点列表
    pub children_map: HashMap<EntityId, Vec<EntityId>>,
    /// 深度映射：节点 → 深度（根节点深度为0）
    pub depth_map: HashMap<EntityId, usize>,
}

impl Default for GraphHierarchy {
    fn default() -> Self {
        Self {
            roots: Vec::new(),
            parent_map: HashMap::new(),
            children_map: HashMap::new(),
            depth_map: HashMap::new(),
        }
    }
}

impl GraphHierarchy {
    /// 创建新的层次结构
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加根节点
    pub fn add_root(&mut self, entity_id: EntityId) {
        if !self.roots.contains(&entity_id) {
            self.roots.push(entity_id.clone());
            self.depth_map.insert(entity_id, 0);
        }
    }

    /// 添加父子关系
    pub fn add_child(&mut self, parent: EntityId, child: EntityId) {
        // 更新父节点映射
        self.parent_map.insert(child.clone(), parent.clone());

        // 更新子节点映射
        self.children_map
            .entry(parent.clone())
            .or_insert_with(Vec::new)
            .push(child.clone());

        // 更新深度映射
        let parent_depth = self.depth_map.get(&parent).copied().unwrap_or(0);
        self.depth_map.insert(child, parent_depth + 1);
    }

    /// 获取节点的所有祖先
    pub fn ancestors(&self, entity_id: &EntityId) -> Vec<EntityId> {
        let mut ancestors = Vec::new();
        let mut current = self.parent_map.get(entity_id).cloned();

        while let Some(parent) = current {
            ancestors.push(parent.clone());
            current = self.parent_map.get(&parent).cloned();
        }

        ancestors
    }

    /// 获取节点的所有后代
    pub fn descendants(&self, entity_id: &EntityId) -> Vec<EntityId> {
        let mut descendants = Vec::new();
        let mut queue = self.children_map.get(entity_id).cloned().unwrap_or_default();

        while let Some(child) = queue.pop() {
            descendants.push(child.clone());
            if let Some(grandchildren) = self.children_map.get(&child) {
                queue.extend(grandchildren.clone());
            }
        }

        descendants
    }

    /// 广度优先遍历顺序
    pub fn bfs_order(&self) -> Vec<EntityId> {
        let mut order = Vec::new();
        let mut queue = self.roots.clone();

        while let Some(entity_id) = queue.first().cloned() {
            queue.remove(0);
            order.push(entity_id.clone());

            if let Some(children) = self.children_map.get(&entity_id) {
                queue.extend(children.clone());
            }
        }

        order
    }
}

// ============================================================================
// 文本关系图谱
// ============================================================================

/// 文本关系图谱
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRelationGraph {
    /// 图谱唯一标识
    pub id: GraphId,
    /// 原始文本
    pub source_text: String,
    /// 实体集合
    pub entities: HashMap<EntityId, GraphEntity>,
    /// 关系集合
    pub relations: Vec<GraphRelation>,
    /// 层次结构
    pub hierarchy: GraphHierarchy,
}

impl TextRelationGraph {
    /// 创建空图谱
    pub fn new(source_text: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            source_text,
            entities: HashMap::new(),
            relations: Vec::new(),
            hierarchy: GraphHierarchy::new(),
        }
    }

    /// 添加实体
    pub fn add_entity(&mut self, entity: GraphEntity) {
        self.entities.insert(entity.id.clone(), entity);
    }

    /// 添加关系
    pub fn add_relation(&mut self, relation: GraphRelation) {
        self.relations.push(relation);
    }

    /// 获取实体
    pub fn get_entity(&self, id: &EntityId) -> Option<&GraphEntity> {
        self.entities.get(id)
    }

    /// 获取实体的所有关系
    pub fn entity_relations(&self, entity_id: &EntityId) -> Vec<&GraphRelation> {
        self.relations
            .iter()
            .filter(|r| &r.from == entity_id || &r.to == entity_id)
            .collect()
    }

    /// 实体数量
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// 关系数量
    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_type_classification() {
        assert!(EntityType::Person.is_biological());
        assert!(EntityType::Building.is_physical());
        assert!(EntityType::Location.is_spatial());
    }

    #[test]
    fn test_entity_creation() {
        let entity = GraphEntity::new("房子".to_string(), EntityType::Building)
            .with_attribute("颜色".to_string(), AttributeValue::String("红色".to_string()))
            .with_confidence(0.9);

        assert_eq!(entity.name, "房子");
        assert_eq!(entity.entity_type, EntityType::Building);
        assert!(entity.get_attribute("颜色").is_some());
    }

    #[test]
    fn test_relation_creation() {
        let from = "entity_1".to_string();
        let to = "entity_2".to_string();
        let relation = GraphRelation::new(from.clone(), to.clone(), RelationType::Contains)
            .with_strength(0.8);

        assert_eq!(relation.from, from);
        assert_eq!(relation.to, to);
        assert_eq!(relation.relation_type, RelationType::Contains);
    }

    #[test]
    fn test_hierarchy() {
        let mut hierarchy = GraphHierarchy::new();
        let root = "root".to_string();
        let child1 = "child1".to_string();
        let child2 = "child2".to_string();

        hierarchy.add_root(root.clone());
        hierarchy.add_child(root.clone(), child1.clone());
        hierarchy.add_child(root.clone(), child2.clone());

        assert_eq!(hierarchy.roots.len(), 1);
        assert_eq!(hierarchy.depth_map.get(&child1), Some(&1));
        assert_eq!(hierarchy.bfs_order(), vec![root, child1, child2]);
    }

    #[test]
    fn test_graph() {
        let mut graph = TextRelationGraph::new("一个房子里有一张桌子".to_string());

        let house = GraphEntity::new("房子".to_string(), EntityType::Building);
        let table = GraphEntity::new("桌子".to_string(), EntityType::Object);

        let house_id = house.id.clone();
        let table_id = table.id.clone();

        graph.add_entity(house);
        graph.add_entity(table);

        graph.add_relation(GraphRelation::new(
            house_id.clone(),
            table_id.clone(),
            RelationType::Contains,
        ));

        assert_eq!(graph.entity_count(), 2);
        assert_eq!(graph.relation_count(), 1);
    }
}
