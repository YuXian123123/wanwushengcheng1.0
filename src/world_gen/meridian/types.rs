//! 脉络核心类型定义
//!
//! 定义信息脉络的所有核心数据结构

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::world_gen::graph::{EntityId, GraphId, RelationType};

// ============================================================================
// 类型别名
// ============================================================================

/// 脉络节点ID
pub type MeridianId = String;
/// 通道ID
pub type ChannelId = String;
/// 3D世界节点ID
pub type WorldNodeId = String;
/// 3D世界ID
pub type WorldId = String;

// ============================================================================
// 脉络节点类型
// ============================================================================

/// 脉络节点类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MeridianNodeType {
    /// 世界/场景根节点
    Root,
    /// 区域节点（房间、区域）
    Region,
    /// 具体实体节点
    Entity,
    /// 组件节点（实体的组成部分）
    Component,
    /// 属性节点（颜色、材质等）
    Attribute,
    /// 关系节点（连接两个实体）
    Relation,
}

impl MeridianNodeType {
    /// 获取类型名称
    pub fn name(&self) -> &str {
        match self {
            MeridianNodeType::Root => "世界",
            MeridianNodeType::Region => "区域",
            MeridianNodeType::Entity => "实体",
            MeridianNodeType::Component => "组件",
            MeridianNodeType::Attribute => "属性",
            MeridianNodeType::Relation => "关系",
        }
    }

    /// 是否可渲染
    pub fn is_renderable(&self) -> bool {
        matches!(
            self,
            MeridianNodeType::Entity | MeridianNodeType::Component | MeridianNodeType::Attribute
        )
    }
}

// ============================================================================
// 通道类型
// ============================================================================

/// 通道类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChannelType {
    /// 空间连接（物理相邻）
    Spatial,
    /// 功能连接（使用关系）
    Functional,
    /// 视觉连接（可见关系）
    Visual,
    /// 逻辑连接（依赖关系）
    Logical,
    /// 情感连接（关联关系）
    Emotional,
}

impl ChannelType {
    pub fn name(&self) -> &str {
        match self {
            ChannelType::Spatial => "空间",
            ChannelType::Functional => "功能",
            ChannelType::Visual => "视觉",
            ChannelType::Logical => "逻辑",
            ChannelType::Emotional => "情感",
        }
    }

    /// 从关系类型推断通道类型
    pub fn from_relation_type(relation_type: &RelationType) -> Self {
        match relation_type {
            RelationType::Contains | RelationType::Inside | RelationType::Adjacent => ChannelType::Spatial,
            RelationType::Uses | RelationType::Creates | RelationType::DependsOn => ChannelType::Functional,
            RelationType::HasAttribute => ChannelType::Logical,
            _ => ChannelType::Logical,
        }
    }
}

// ============================================================================
// 几何类型提示
// ============================================================================

/// 几何类型提示
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GeometryHint {
    // === 基础形状 ===
    /// 球体
    Sphere,
    /// 立方体
    Cube,
    /// 圆柱体
    Cylinder,
    /// 平面
    Plane,
    /// 圆锥体
    Cone,

    // === 生物形态 ===
    /// 人形
    Humanoid,
    /// 四足动物
    Quadruped,
    /// 植物
    Plant,

    // === 建筑形态 ===
    /// 建筑
    Building,
    /// 房间
    Room,
    /// 家具
    Furniture,

    // === 自然形态 ===
    /// 地形
    Terrain,
    /// 水体
    Water,
    /// 天空
    Sky,
    /// 树木
    Tree,

    // === 自定义 ===
    /// 自定义几何
    Custom(String),
}

impl GeometryHint {
    /// 从实体类型推断几何类型
    pub fn from_entity_type(entity_type: &crate::world_gen::graph::EntityType) -> Self {
        use crate::world_gen::graph::EntityType;

        match entity_type {
            EntityType::Person => GeometryHint::Humanoid,
            EntityType::Animal => GeometryHint::Quadruped,
            EntityType::Plant => GeometryHint::Plant,
            EntityType::Building => GeometryHint::Building,
            EntityType::Location => GeometryHint::Room,
            EntityType::Object => GeometryHint::Cube,
            _ => GeometryHint::Cube,
        }
    }

    /// 是否为程序化生成
    pub fn is_procedural(&self) -> bool {
        matches!(
            self,
            GeometryHint::Humanoid
                | GeometryHint::Quadruped
                | GeometryHint::Plant
                | GeometryHint::Tree
                | GeometryHint::Terrain
        )
    }
}

// ============================================================================
// 脉络节点
// ============================================================================

/// 脉络节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeridianNode {
    /// 唯一标识
    pub id: MeridianId,
    /// 对应的实体ID（如果有）
    pub entity_id: Option<EntityId>,
    /// 节点名称
    pub name: String,
    /// 节点类型
    pub node_type: MeridianNodeType,
    /// 相对位置（相对于父节点）
    pub position: [f64; 3],
    /// 缩放
    pub scale: [f64; 3],
    /// 旋转（四元数）
    pub rotation: [f64; 4],
    /// 几何类型提示
    pub geometry_hint: GeometryHint,
    /// 子节点ID列表
    pub children: Vec<MeridianId>,
    /// 父节点ID
    pub parent: Option<MeridianId>,
    /// 关联通道ID列表
    pub channels: Vec<ChannelId>,
    /// 概念向量
    pub concept_vector: Vec<f64>,
    /// 重要度 [0, 1]
    pub importance: f64,
    /// 元数据（颜色、材质等）
    pub metadata: HashMap<String, String>,
}

impl MeridianNode {
    /// 创建新节点
    pub fn new(name: String, node_type: MeridianNodeType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            entity_id: None,
            name,
            node_type,
            position: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            rotation: [0.0, 0.0, 0.0, 1.0], // 单位四元数
            geometry_hint: GeometryHint::Cube,
            children: Vec::new(),
            parent: None,
            channels: Vec::new(),
            concept_vector: Vec::new(),
            importance: 0.5,
            metadata: HashMap::new(),
        }
    }

    /// 设置位置
    pub fn with_position(mut self, x: f64, y: f64, z: f64) -> Self {
        self.position = [x, y, z];
        self
    }

    /// 设置缩放
    pub fn with_scale(mut self, x: f64, y: f64, z: f64) -> Self {
        self.scale = [x, y, z];
        self
    }

    /// 设置几何类型
    pub fn with_geometry(mut self, hint: GeometryHint) -> Self {
        self.geometry_hint = hint;
        self
    }

    /// 设置实体关联
    pub fn with_entity(mut self, entity_id: EntityId) -> Self {
        self.entity_id = Some(entity_id);
        self
    }

    /// 添加元数据
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// 添加子节点
    pub fn add_child(&mut self, child_id: MeridianId) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
        }
    }

    /// 世界坐标（递归计算）
    pub fn world_position(&self, meridian: &InformationMeridian) -> [f64; 3] {
        match &self.parent {
            Some(parent_id) => {
                if let Some(parent) = meridian.nodes.get(parent_id) {
                    let parent_pos = parent.world_position(meridian);
                    [
                        parent_pos[0] + self.position[0],
                        parent_pos[1] + self.position[1],
                        parent_pos[2] + self.position[2],
                    ]
                } else {
                    self.position
                }
            }
            None => self.position,
        }
    }
}

// ============================================================================
// 脉络通道
// ============================================================================

/// 脉络通道
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeridianChannel {
    /// 唯一标识
    pub id: ChannelId,
    /// 起始节点ID
    pub from: MeridianId,
    /// 目标节点ID
    pub to: MeridianId,
    /// 通道类型
    pub channel_type: ChannelType,
    /// 通道强度 [0, 1]
    pub strength: f64,
}

impl MeridianChannel {
    /// 创建新通道
    pub fn new(from: MeridianId, to: MeridianId, channel_type: ChannelType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            from,
            to,
            channel_type,
            strength: 1.0,
        }
    }

    /// 设置强度
    pub fn with_strength(mut self, strength: f64) -> Self {
        self.strength = strength.clamp(0.0, 1.0);
        self
    }
}

// ============================================================================
// 脉络模板
// ============================================================================

/// 脉络模板类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeridianTemplate {
    /// 生物模板（人、动物）
    Biological(BioTemplateParams),
    /// 建筑模板（房子、城市）
    Architectural(ArchTemplateParams),
    /// 自然模板（森林、山脉）
    Natural(NaturalTemplateParams),
    /// 抽象模板
    Abstract(AbstractTemplateParams),
    /// 自定义模板
    Custom(String),
}

/// 生物模板参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioTemplateParams {
    /// 基础高度
    pub base_height: f64,
    /// 比例因子
    pub scale_factor: [f64; 3],
    /// 是否有四肢
    pub has_limbs: bool,
    /// 肢体数量
    pub limb_count: u32,
}

impl Default for BioTemplateParams {
    fn default() -> Self {
        Self {
            base_height: 1.7,
            scale_factor: [1.0, 1.0, 1.0],
            has_limbs: true,
            limb_count: 4,
        }
    }
}

impl BioTemplateParams {
    /// 人形参数
    pub fn human() -> Self {
        Self {
            base_height: 1.7,
            scale_factor: [1.0, 1.0, 1.0],
            has_limbs: true,
            limb_count: 4,
        }
    }

    /// 四足动物参数
    pub fn quadruped() -> Self {
        Self {
            base_height: 0.5,
            scale_factor: [1.5, 1.0, 1.0],
            has_limbs: true,
            limb_count: 4,
        }
    }
}

/// 建筑模板参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchTemplateParams {
    /// 基础尺寸
    pub base_size: [f64; 3],
    /// 是否有多层
    pub multi_floor: bool,
    /// 楼层数
    pub floor_count: u32,
    /// 是否有屋顶
    pub has_roof: bool,
}

impl Default for ArchTemplateParams {
    fn default() -> Self {
        Self {
            base_size: [10.0, 5.0, 10.0],
            multi_floor: false,
            floor_count: 1,
            has_roof: true,
        }
    }
}

/// 自然模板参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaturalTemplateParams {
    /// 地形起伏
    pub terrain_height: f64,
    /// 植物密度
    pub vegetation_density: f64,
    /// 是否有水
    pub has_water: bool,
}

impl Default for NaturalTemplateParams {
    fn default() -> Self {
        Self {
            terrain_height: 10.0,
            vegetation_density: 0.5,
            has_water: false,
        }
    }
}

/// 抽象模板参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractTemplateParams {
    /// 默认几何
    pub default_geometry: GeometryHint,
    /// 间距
    pub spacing: f64,
}

impl Default for AbstractTemplateParams {
    fn default() -> Self {
        Self {
            default_geometry: GeometryHint::Cube,
            spacing: 2.0,
        }
    }
}

// ============================================================================
// 脉络层次结构
// ============================================================================

/// 脉络层次结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeridianHierarchy {
    /// 根节点
    pub root: MeridianId,
    /// 深度映射
    pub depth_map: HashMap<MeridianId, usize>,
    /// 层级节点列表（按深度组织）
    pub levels: Vec<Vec<MeridianId>>,
}

impl MeridianHierarchy {
    /// 创建新层次结构
    pub fn new(root: MeridianId) -> Self {
        let mut depth_map = HashMap::new();
        depth_map.insert(root.clone(), 0);

        let root_clone = root.clone();
        Self {
            root,
            depth_map,
            levels: vec![vec![root_clone]],
        }
    }

    /// 添加节点到指定深度
    pub fn add_node(&mut self, node_id: MeridianId, depth: usize) {
        self.depth_map.insert(node_id.clone(), depth);

        // 确保层级足够
        while self.levels.len() <= depth {
            self.levels.push(Vec::new());
        }

        self.levels[depth].push(node_id);
    }

    /// 获取节点深度
    pub fn depth(&self, node_id: &MeridianId) -> Option<usize> {
        self.depth_map.get(node_id).copied()
    }

    /// 广度优先遍历顺序
    pub fn bfs_order(&self) -> Vec<MeridianId> {
        self.levels.iter().flatten().cloned().collect()
    }

    /// 最大深度
    pub fn max_depth(&self) -> usize {
        self.levels.len().saturating_sub(1)
    }
}

// ============================================================================
// 信息脉络
// ============================================================================

/// 信息脉络（完整蓝图）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InformationMeridian {
    /// 脉络唯一标识
    pub id: MeridianId,
    /// 根节点ID
    pub root: MeridianId,
    /// 节点集合
    pub nodes: HashMap<MeridianId, MeridianNode>,
    /// 通道集合
    pub channels: HashMap<ChannelId, MeridianChannel>,
    /// 层次结构
    pub hierarchy: MeridianHierarchy,
    /// 模板
    pub template: MeridianTemplate,
    /// 来源图谱ID
    pub source_graph: GraphId,
}

impl InformationMeridian {
    /// 创建新脉络
    pub fn new(root: MeridianNode, source_graph: GraphId) -> Self {
        let root_id = root.id.clone();
        let hierarchy = MeridianHierarchy::new(root_id.clone());
        let mut nodes = HashMap::new();
        nodes.insert(root_id.clone(), root);

        Self {
            id: Uuid::new_v4().to_string(),
            root: root_id,
            nodes,
            channels: HashMap::new(),
            hierarchy,
            template: MeridianTemplate::Abstract(AbstractTemplateParams::default()),
            source_graph,
        }
    }

    /// 添加节点
    pub fn add_node(&mut self, parent_id: &MeridianId, mut node: MeridianNode) {
        // 设置父节点
        node.parent = Some(parent_id.clone());

        // 更新父节点的子列表
        if let Some(parent) = self.nodes.get_mut(parent_id) {
            parent.add_child(node.id.clone());
        }

        // 计算深度
        let parent_depth = self.hierarchy.depth(parent_id).unwrap_or(0);
        self.hierarchy.add_node(node.id.clone(), parent_depth + 1);

        // 添加节点
        self.nodes.insert(node.id.clone(), node);
    }

    /// 添加通道
    pub fn add_channel(&mut self, channel: MeridianChannel) {
        // 更新节点通道列表
        if let Some(from_node) = self.nodes.get_mut(&channel.from) {
            from_node.channels.push(channel.id.clone());
        }
        if let Some(to_node) = self.nodes.get_mut(&channel.to) {
            to_node.channels.push(channel.id.clone());
        }

        self.channels.insert(channel.id.clone(), channel);
    }

    /// 获取节点
    pub fn get_node(&self, id: &MeridianId) -> Option<&MeridianNode> {
        self.nodes.get(id)
    }

    /// 节点数量
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// 通道数量
    pub fn channel_count(&self) -> usize {
        self.channels.len()
    }

    /// 广度优先遍历顺序
    pub fn bfs_order(&self) -> Vec<MeridianId> {
        self.hierarchy.bfs_order()
    }
}

// ============================================================================
// 3D世界结构
// ============================================================================

/// 3D世界节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldNode {
    /// 唯一标识
    pub id: WorldNodeId,
    /// 对应的脉络节点ID
    pub meridian_id: MeridianId,
    /// 世界坐标
    pub position: [f64; 3],
    /// 旋转（四元数）
    pub rotation: [f64; 4],
    /// 缩放
    pub scale: [f64; 3],
    /// 几何类型
    pub geometry: GeometryHint,
    /// 颜色（RGBA）
    pub color: [f64; 4],
    /// 子节点ID列表
    pub children: Vec<WorldNodeId>,
    /// 父节点ID
    pub parent: Option<WorldNodeId>,
    /// 是否可见
    pub visible: bool,
    /// 版本号
    pub version: u64,
}

impl WorldNode {
    /// 创建新节点
    pub fn new(meridian_id: MeridianId) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            meridian_id,
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
            geometry: GeometryHint::Cube,
            color: [0.5, 0.5, 0.5, 1.0], // 默认灰色
            children: Vec::new(),
            parent: None,
            visible: true,
            version: 1,
        }
    }

    /// 设置位置
    pub fn with_position(mut self, x: f64, y: f64, z: f64) -> Self {
        self.position = [x, y, z];
        self
    }

    /// 设置颜色
    pub fn with_color(mut self, r: f64, g: f64, b: f64, a: f64) -> Self {
        self.color = [r, g, b, a];
        self
    }

    /// 设置几何
    pub fn with_geometry(mut self, geometry: GeometryHint) -> Self {
        self.geometry = geometry;
        self
    }
}

/// 3D世界结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World3D {
    /// 世界唯一标识
    pub id: WorldId,
    /// 根节点ID
    pub root: WorldNodeId,
    /// 节点集合
    pub nodes: HashMap<WorldNodeId, WorldNode>,
    /// 边界框
    pub bounding_box: BoundingBox,
    /// 来源脉络ID
    pub source_meridian: MeridianId,
}

/// 边界框
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoundingBox {
    /// 最小点
    pub min: [f64; 3],
    /// 最大点
    pub max: [f64; 3],
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            min: [f64::MAX, f64::MAX, f64::MAX],
            max: [f64::MIN, f64::MIN, f64::MIN],
        }
    }
}

impl BoundingBox {
    /// 创建新边界框
    pub fn new() -> Self {
        Self::default()
    }

    /// 扩展边界框以包含点
    pub fn expand(&mut self, point: [f64; 3]) {
        self.min[0] = self.min[0].min(point[0]);
        self.min[1] = self.min[1].min(point[1]);
        self.min[2] = self.min[2].min(point[2]);
        self.max[0] = self.max[0].max(point[0]);
        self.max[1] = self.max[1].max(point[1]);
        self.max[2] = self.max[2].max(point[2]);
    }

    /// 尺寸
    pub fn size(&self) -> [f64; 3] {
        [
            self.max[0] - self.min[0],
            self.max[1] - self.min[1],
            self.max[2] - self.min[2],
        ]
    }
}

impl World3D {
    /// 创建新世界
    pub fn new(source_meridian: MeridianId) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            root: String::new(),
            nodes: HashMap::new(),
            bounding_box: BoundingBox::new(),
            source_meridian,
        }
    }

    /// 设置根节点
    pub fn set_root(&mut self, node: WorldNode) {
        self.root = node.id.clone();
        self.add_node(node);
    }

    /// 添加节点
    pub fn add_node(&mut self, node: WorldNode) {
        // 更新边界框
        self.bounding_box.expand(node.position);

        // 添加节点
        self.nodes.insert(node.id.clone(), node);
    }

    /// 获取节点
    pub fn get_node(&self, id: &WorldNodeId) -> Option<&WorldNode> {
        self.nodes.get(id)
    }

    /// 节点数量
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meridian_node_creation() {
        let node = MeridianNode::new("房子".to_string(), MeridianNodeType::Entity)
            .with_position(0.0, 0.0, 0.0)
            .with_geometry(GeometryHint::Building);

        assert_eq!(node.name, "房子");
        assert_eq!(node.node_type, MeridianNodeType::Entity);
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_meridian_hierarchy() {
        let root = MeridianNode::new("世界".to_string(), MeridianNodeType::Root);
        let root_id = root.id.clone();

        let mut meridian = InformationMeridian::new(
            root,
            "graph_1".to_string(),
        );

        let child = MeridianNode::new("房子".to_string(), MeridianNodeType::Entity);
        meridian.add_node(&root_id, child);

        assert_eq!(meridian.node_count(), 2);
        assert_eq!(meridian.hierarchy.max_depth(), 1);
    }

    #[test]
    fn test_world_node() {
        let node = WorldNode::new("meridian_1".to_string())
            .with_position(1.0, 2.0, 3.0)
            .with_color(1.0, 0.0, 0.0, 1.0)
            .with_geometry(GeometryHint::Cube);

        assert_eq!(node.position, [1.0, 2.0, 3.0]);
        assert_eq!(node.color, [1.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_bounding_box() {
        let mut bb = BoundingBox::new();
        bb.expand([0.0, 0.0, 0.0]);
        bb.expand([1.0, 2.0, 3.0]);

        assert_eq!(bb.size(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_channel_type_from_relation() {
        assert_eq!(
            ChannelType::from_relation_type(&RelationType::Contains),
            ChannelType::Spatial
        );
        assert_eq!(
            ChannelType::from_relation_type(&RelationType::Uses),
            ChannelType::Functional
        );
    }
}
