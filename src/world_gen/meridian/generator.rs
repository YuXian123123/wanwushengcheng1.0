//! 脉络生成器
//!
//! 从关系图谱生成信息脉络（结构化蓝图）

use crate::world_gen::config::MeridianConfig;
use crate::world_gen::graph::*;
use crate::world_gen::meridian::*;
use std::collections::HashMap;

/// 生成错误
#[derive(Debug, Clone)]
pub enum GenerateError {
    /// 图谱为空
    EmptyGraph,
    /// 无根节点
    NoRootNode,
    /// 模板不匹配
    TemplateMismatch(String),
}

impl std::fmt::Display for GenerateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerateError::EmptyGraph => write!(f, "图谱为空"),
            GenerateError::NoRootNode => write!(f, "无根节点"),
            GenerateError::TemplateMismatch(s) => write!(f, "模板不匹配: {}", s),
        }
    }
}

impl std::error::Error for GenerateError {}

/// 脉络生成器
pub struct MeridianGenerator {
    /// 配置
    config: MeridianConfig,
    /// 模板库
    templates: TemplateLibrary,
}

impl MeridianGenerator {
    /// 创建新生成器
    pub fn new(config: MeridianConfig) -> Self {
        Self {
            config,
            templates: TemplateLibrary::new(),
        }
    }

    /// 从关系图谱生成脉络
    pub fn generate(&self, graph: &TextRelationGraph) -> Result<InformationMeridian, GenerateError> {
        // 检查图谱是否为空
        if graph.entities.is_empty() {
            return Err(GenerateError::EmptyGraph);
        }

        // 检查是否有根节点
        if graph.hierarchy.roots.is_empty() {
            return Err(GenerateError::NoRootNode);
        }

        // 1. 检测模板
        let template = self.detect_template(graph);

        // 2. 确定根节点
        let root_entity_id = &graph.hierarchy.roots[0];
        let root_entity = graph.entities.get(root_entity_id).ok_or(GenerateError::NoRootNode)?;

        // 3. 创建根脉络节点
        let root_node = self.create_meridian_node(root_entity, MeridianNodeType::Root, None);

        // 4. 创建脉络
        let mut meridian = InformationMeridian::new(root_node, graph.id.clone());
        meridian.template = template;

        // 5. 按层次添加节点
        self.add_nodes_from_graph(&mut meridian, graph);

        // 6. 创建通道
        self.create_channels(&mut meridian, graph);

        Ok(meridian)
    }

    /// 检测适合的模板
    fn detect_template(&self, graph: &TextRelationGraph) -> MeridianTemplate {
        // 统计实体类型
        let mut type_counts = HashMap::new();
        for entity in graph.entities.values() {
            let type_name = entity.entity_type.name();
            *type_counts.entry(type_name).or_insert(0) += 1;
        }

        // 根据主要实体类型选择模板
        if type_counts.get("人").copied().unwrap_or(0) > 0 {
            MeridianTemplate::Biological(BioTemplateParams::human())
        } else if type_counts.get("建筑").copied().unwrap_or(0) > 0 {
            MeridianTemplate::Architectural(ArchTemplateParams::default())
        } else if type_counts.get("植物").copied().unwrap_or(0) > 0 {
            MeridianTemplate::Natural(NaturalTemplateParams::default())
        } else {
            MeridianTemplate::Abstract(AbstractTemplateParams::default())
        }
    }

    /// 从图谱添加节点
    fn add_nodes_from_graph(&self, meridian: &mut InformationMeridian, graph: &TextRelationGraph) {
        // 获取BFS顺序
        let bfs_order = graph.hierarchy.bfs_order();

        // 跟踪已创建的脉络节点
        let mut entity_to_meridian: HashMap<EntityId, MeridianId> = HashMap::new();

        // 根节点已创建
        if let Some(root_entity_id) = graph.hierarchy.roots.first() {
            if let Some(root_meridian) = meridian.nodes.values().find(|n| n.entity_id.as_ref() == Some(root_entity_id)) {
                entity_to_meridian.insert(root_entity_id.clone(), root_meridian.id.clone());
            }
        }

        // 按BFS顺序添加其他节点
        for entity_id in bfs_order {
            if entity_to_meridian.contains_key(&entity_id) {
                continue; // 已处理
            }

            if let Some(entity) = graph.entities.get(&entity_id) {
                // 确定父节点
                let parent_meridian_id = graph.hierarchy.parent_map
                    .get(&entity_id)
                    .and_then(|pid| entity_to_meridian.get(pid))
                    .cloned()
                    .unwrap_or_else(|| meridian.root.clone());

                // 确定节点类型
                let node_type = if entity.entity_type.is_spatial() {
                    MeridianNodeType::Region
                } else {
                    MeridianNodeType::Entity
                };

                // 创建脉络节点
                let meridian_node = self.create_meridian_node(
                    entity,
                    node_type,
                    Some(parent_meridian_id.clone()),
                );

                let node_id = meridian_node.id.clone();
                meridian.add_node(&parent_meridian_id, meridian_node);
                entity_to_meridian.insert(entity_id.clone(), node_id);
            }
        }
    }

    /// 创建脉络节点
    fn create_meridian_node(
        &self,
        entity: &GraphEntity,
        node_type: MeridianNodeType,
        parent_id: Option<MeridianId>,
    ) -> MeridianNode {
        // 计算相对位置
        let position = self.compute_position(node_type, parent_id.is_some());

        // 推断几何类型
        let geometry_hint = GeometryHint::from_entity_type(&entity.entity_type);

        // 创建节点
        let mut node = MeridianNode::new(entity.name.clone(), node_type)
            .with_entity(entity.id.clone())
            .with_position(position[0], position[1], position[2])
            .with_geometry(geometry_hint)
            .with_scale(
                self.config.default_scale[0],
                self.config.default_scale[1],
                self.config.default_scale[2],
            );

        // 复制属性到元数据
        for (key, value) in &entity.attributes {
            if let Some(s) = value.as_str() {
                node = node.with_metadata(key.clone(), s.to_string());
            }
        }

        node
    }

    /// 计算相对位置
    fn compute_position(&self, node_type: MeridianNodeType, has_parent: bool) -> [f64; 3] {
        if !has_parent {
            return [0.0, 0.0, 0.0]; // 根节点在原点
        }

        // 根据节点类型确定位置
        match node_type {
            MeridianNodeType::Root => [0.0, 0.0, 0.0],
            MeridianNodeType::Region => [0.0, 0.0, 0.0], // 区域与父节点同位置
            MeridianNodeType::Entity => {
                // 实体在父节点周围随机分布
                // MVP: 使用固定偏移
                [2.0, 0.0, 2.0]
            }
            MeridianNodeType::Component => [0.5, 0.0, 0.5],
            MeridianNodeType::Attribute => [0.0, 0.1, 0.0],
            MeridianNodeType::Relation => [1.0, 0.0, 0.0],
        }
    }

    /// 创建通道
    fn create_channels(&self, meridian: &mut InformationMeridian, graph: &TextRelationGraph) {
        // 为每个关系创建通道
        for relation in &graph.relations {
            // 查找对应的脉络节点
            let from_meridian = meridian.nodes.values()
                .find(|n| n.entity_id.as_ref() == Some(&relation.from));

            let to_meridian = meridian.nodes.values()
                .find(|n| n.entity_id.as_ref() == Some(&relation.to));

            if let (Some(from), Some(to)) = (from_meridian, to_meridian) {
                let channel_type = ChannelType::from_relation_type(&relation.relation_type);
                let channel = MeridianChannel::new(from.id.clone(), to.id.clone(), channel_type)
                    .with_strength(relation.strength);

                meridian.add_channel(channel);
            }
        }
    }
}

/// 模板库
pub struct TemplateLibrary {
    templates: HashMap<String, MeridianTemplate>,
}

impl TemplateLibrary {
    pub fn new() -> Self {
        let mut templates = HashMap::new();

        templates.insert(
            "humanoid".to_string(),
            MeridianTemplate::Biological(BioTemplateParams::human()),
        );
        templates.insert(
            "quadruped".to_string(),
            MeridianTemplate::Biological(BioTemplateParams::quadruped()),
        );
        templates.insert(
            "building".to_string(),
            MeridianTemplate::Architectural(ArchTemplateParams::default()),
        );
        templates.insert(
            "nature".to_string(),
            MeridianTemplate::Natural(NaturalTemplateParams::default()),
        );
        templates.insert(
            "abstract".to_string(),
            MeridianTemplate::Abstract(AbstractTemplateParams::default()),
        );

        Self { templates }
    }

    pub fn get(&self, name: &str) -> Option<&MeridianTemplate> {
        self.templates.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_creation() {
        let config = MeridianConfig::default();
        let generator = MeridianGenerator::new(config);
        assert!(generator.templates.get("humanoid").is_some());
    }

    #[test]
    fn test_empty_graph() {
        let config = MeridianConfig::default();
        let generator = MeridianGenerator::new(config);

        let graph = TextRelationGraph::new("".to_string());
        let result = generator.generate(&graph);

        assert!(matches!(result, Err(GenerateError::EmptyGraph)));
    }

    #[test]
    fn test_template_detection() {
        let config = MeridianConfig::default();
        let generator = MeridianGenerator::new(config);

        // 创建包含建筑的图谱
        let mut graph = TextRelationGraph::new("房子".to_string());
        let house = GraphEntity::new("房子".to_string(), EntityType::Building);
        graph.add_entity(house);
        graph.hierarchy.add_root("test_id".to_string());

        let result = generator.generate(&graph);
        assert!(result.is_ok());

        let meridian = result.unwrap();
        assert!(matches!(meridian.template, MeridianTemplate::Architectural(_)));
    }
}
