//! 脉络展开器
//!
//! 将信息脉络展开为3D世界结构

use crate::world_gen::config::MeridianConfig;
use crate::world_gen::meridian::*;
use std::collections::HashMap;

/// 展开错误
#[derive(Debug, Clone)]
pub enum ExpandError {
    /// 脉络为空
    EmptyMeridian,
    /// 无根节点
    NoRootNode,
    /// 节点过多
    TooManyNodes(usize),
}

impl std::fmt::Display for ExpandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpandError::EmptyMeridian => write!(f, "脉络为空"),
            ExpandError::NoRootNode => write!(f, "无根节点"),
            ExpandError::TooManyNodes(n) => write!(f, "节点数量超过限制: {}", n),
        }
    }
}

impl std::error::Error for ExpandError {}

/// 脉络展开器
pub struct MeridianExpander {
    /// 配置
    config: MeridianConfig,
}

impl MeridianExpander {
    /// 创建新展开器
    pub fn new(config: MeridianConfig) -> Self {
        Self { config }
    }

    /// 展开脉络为3D世界
    pub fn expand(&self, meridian: &InformationMeridian) -> Result<World3D, ExpandError> {
        // 检查脉络是否为空
        if meridian.nodes.is_empty() {
            return Err(ExpandError::EmptyMeridian);
        }

        // 创建世界
        let mut world = World3D::new(meridian.id.clone());

        // 获取BFS顺序（保证父节点先处理）
        let bfs_order = meridian.bfs_order();

        // 跟踪脉络节点到世界节点的映射
        let mut meridian_to_world: HashMap<MeridianId, WorldNodeId> = HashMap::new();

        // 按BFS顺序创建世界节点
        for meridian_id in bfs_order {
            if let Some(meridian_node) = meridian.get_node(&meridian_id) {
                // 计算世界坐标
                let world_position = meridian_node.world_position(meridian);

                // 确定颜色
                let color = self.determine_color(meridian_node);

                // 创建世界节点
                let mut world_node = WorldNode::new(meridian_id.clone())
                    .with_position(world_position[0], world_position[1], world_position[2])
                    .with_geometry(meridian_node.geometry_hint.clone())
                    .with_color(color[0], color[1], color[2], color[3]);

                // 设置缩放
                world_node.scale = meridian_node.scale;

                // 设置父节点
                if let Some(parent_meridian_id) = &meridian_node.parent {
                    if let Some(parent_world_id) = meridian_to_world.get(parent_meridian_id) {
                        world_node.parent = Some(parent_world_id.clone());
                    }
                }

                // 记录映射
                let world_id = world_node.id.clone();
                meridian_to_world.insert(meridian_id.clone(), world_id.clone());

                // 添加到世界
                if world.root.is_empty() {
                    world.set_root(world_node);
                } else {
                    world.add_node(world_node);
                }
            }
        }

        // 更新子节点关系
        self.update_children_relations(&mut world, &meridian_to_world, meridian);

        Ok(world)
    }

    /// 确定节点颜色
    fn determine_color(&self, node: &MeridianNode) -> [f64; 4] {
        // 从元数据中查找颜色
        if let Some(color_name) = node.metadata.get("颜色") {
            return self.color_from_name(color_name);
        }

        // 根据几何类型返回默认颜色
        match &node.geometry_hint {
            GeometryHint::Building => [0.8, 0.7, 0.6, 1.0], // 米色
            GeometryHint::Furniture => [0.6, 0.5, 0.4, 1.0], // 棕色
            GeometryHint::Tree => [0.2, 0.6, 0.2, 1.0],      // 绿色
            GeometryHint::Plant => [0.3, 0.7, 0.3, 1.0],     // 浅绿色
            GeometryHint::Humanoid => [0.9, 0.8, 0.7, 1.0],  // 肤色
            GeometryHint::Quadruped => [0.6, 0.4, 0.2, 1.0], // 棕色
            _ => [0.5, 0.5, 0.5, 1.0],                       // 灰色
        }
    }

    /// 从颜色名称获取RGB值
    fn color_from_name(&self, name: &str) -> [f64; 4] {
        match name {
            "红" | "红色" => [1.0, 0.0, 0.0, 1.0],
            "蓝" | "蓝色" => [0.0, 0.0, 1.0, 1.0],
            "绿" | "绿色" => [0.0, 1.0, 0.0, 1.0],
            "黄" | "黄色" => [1.0, 1.0, 0.0, 1.0],
            "白" | "白色" => [1.0, 1.0, 1.0, 1.0],
            "黑" | "黑色" => [0.0, 0.0, 0.0, 1.0],
            "橙" | "橙色" => [1.0, 0.5, 0.0, 1.0],
            "紫" | "紫色" => [0.5, 0.0, 0.5, 1.0],
            "粉" | "粉色" => [1.0, 0.75, 0.8, 1.0],
            "棕" | "棕色" => [0.6, 0.3, 0.1, 1.0],
            "灰" | "灰色" => [0.5, 0.5, 0.5, 1.0],
            _ => [0.5, 0.5, 0.5, 1.0], // 默认灰色
        }
    }

    /// 更新子节点关系
    fn update_children_relations(
        &self,
        world: &mut World3D,
        meridian_to_world: &HashMap<MeridianId, WorldNodeId>,
        meridian: &InformationMeridian,
    ) {
        // 遍历所有世界节点
        for (meridian_id, world_id) in meridian_to_world {
            // 获取脉络节点的子节点
            if let Some(meridian_node) = meridian.get_node(meridian_id) {
                // 找到对应的世界节点
                if let Some(world_node) = world.nodes.get_mut(world_id) {
                    // 设置子节点ID列表
                    for child_meridian_id in &meridian_node.children {
                        if let Some(child_world_id) = meridian_to_world.get(child_meridian_id) {
                            if !world_node.children.contains(child_world_id) {
                                world_node.children.push(child_world_id.clone());
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world_gen::graph::*;

    #[test]
    fn test_expander_creation() {
        let config = MeridianConfig::default();
        let expander = MeridianExpander::new(config);
        // 仅验证创建成功
        assert!(true);
    }

    #[test]
    fn test_empty_meridian() {
        let config = MeridianConfig::default();
        let expander = MeridianExpander::new(config);

        // 创建空脉络
        let root = MeridianNode::new("世界".to_string(), MeridianNodeType::Root);
        let meridian = InformationMeridian::new(root, "graph_1".to_string());

        // 删除根节点使其为空
        let mut empty_meridian = meridian.clone();
        empty_meridian.nodes.clear();

        let result = expander.expand(&empty_meridian);
        assert!(matches!(result, Err(ExpandError::EmptyMeridian)));
    }

    #[test]
    fn test_simple_expand() {
        let config = MeridianConfig::default();
        let expander = MeridianExpander::new(config);

        // 创建简单脉络
        let root = MeridianNode::new("房子".to_string(), MeridianNodeType::Entity)
            .with_geometry(GeometryHint::Building);
        let mut meridian = InformationMeridian::new(root, "graph_1".to_string());

        let child = MeridianNode::new("桌子".to_string(), MeridianNodeType::Entity)
            .with_geometry(GeometryHint::Furniture)
            .with_metadata("颜色".to_string(), "红色".to_string());
        meridian.add_node(&meridian.root.clone(), child);

        let result = expander.expand(&meridian);
        assert!(result.is_ok());

        let world = result.unwrap();
        assert_eq!(world.node_count(), 2);
    }

    #[test]
    fn test_color_from_name() {
        let config = MeridianConfig::default();
        let expander = MeridianExpander::new(config);

        assert_eq!(expander.color_from_name("红色"), [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(expander.color_from_name("蓝色"), [0.0, 0.0, 1.0, 1.0]);
        assert_eq!(expander.color_from_name("绿色"), [0.0, 1.0, 0.0, 1.0]);
    }
}
