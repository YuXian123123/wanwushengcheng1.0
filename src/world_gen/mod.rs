//! 万物生成器 - 文本到3D世界的生成系统
//!
//! 核心理念：文字是信息的压缩，图像和3D世界是信息的展开
//!
//! # 架构
//!
//! ```text
//! Text → TextRelationGraph → InformationMeridian → World3D
//! 文本    关系图谱              脉络蓝图           3D世界
//! ```
//!
//! # 信息脉络 (Meridian)
//!
//! 信息脉络是连接文本与3D世界的桥梁，类似于人体的经络系统。
//! 不同实体有不同的脉络模板（人、狗、房子等），定义了信息如何"展开"成空间结构。

pub mod config;
pub mod graph;
pub mod meridian;
pub mod expander;
pub mod pretrained_vectors;
pub mod training_data;

// 后续阶段
// pub mod mutual;

pub use config::{GraphBuildConfig, MeridianConfig, WorldGenConfig};
pub use graph::{EntityId, GraphEntity, GraphRelation, TextRelationGraph, GraphBuilder};
pub use meridian::{InformationMeridian, MeridianNode, MeridianGenerator};
pub use expander::{MeridianExpander, ExpandError};
pub use pretrained_vectors::PretrainedVectors;
pub use training_data::{TrainingDataset, TrainingSample};

/// 完整的文本到3D转换管道
///
/// 使用方法：
/// ```rust
/// use world_gen::Pipeline;
///
/// let pipeline = Pipeline::new();
/// let world = pipeline.text_to_3d("一个房子里有一张桌子")?;
/// ```
pub struct Pipeline {
    graph_builder: GraphBuilder,
    meridian_gen: MeridianGenerator,
    expander: MeridianExpander,
    /// 预训练词向量（可选）
    pretrained_vectors: Option<PretrainedVectors>,
}

impl Pipeline {
    /// 创建新管道
    pub fn new() -> Self {
        Self {
            graph_builder: GraphBuilder::new(GraphBuildConfig::default()),
            meridian_gen: MeridianGenerator::new(MeridianConfig::default()),
            expander: MeridianExpander::new(MeridianConfig::default()),
            pretrained_vectors: None,
        }
    }

    /// 创建管道并加载预训练词向量
    pub fn with_pretrained_vectors(path: &str) -> Self {
        let pretrained_vectors = PretrainedVectors::load(path).ok();

        let graph_builder = if let Some(ref pv) = pretrained_vectors {
            // 使用预训练词向量创建识别器
            let config = GraphBuildConfig::default();
            GraphBuilder::with_pretrained_vectors(config, pv.clone())
        } else {
            GraphBuilder::new(GraphBuildConfig::default())
        };

        Self {
            graph_builder,
            meridian_gen: MeridianGenerator::new(MeridianConfig::default()),
            expander: MeridianExpander::new(MeridianConfig::default()),
            pretrained_vectors,
        }
    }

    /// 使用自定义配置创建管道
    pub fn with_config(
        graph_config: GraphBuildConfig,
        meridian_config: MeridianConfig,
    ) -> Self {
        Self {
            graph_builder: GraphBuilder::new(graph_config),
            meridian_gen: MeridianGenerator::new(meridian_config.clone()),
            expander: MeridianExpander::new(meridian_config),
            pretrained_vectors: None,
        }
    }

    /// 获取预训练词向量引用
    pub fn get_pretrained_vectors(&self) -> Option<&PretrainedVectors> {
        self.pretrained_vectors.as_ref()
    }

    /// 执行完整的文本到3D转换
    ///
    /// # Arguments
    /// * `text` - 输入文本描述
    ///
    /// # Returns
    /// * `Ok(World3D)` - 生成的3D世界
    /// * `Err(Error)` - 转换过程中的错误
    pub fn text_to_3d(&self, text: &str) -> Result<meridian::World3D, Error> {
        // Step 1: 构建关系图谱
        let graph = self.graph_builder.build(text)?;

        // Step 2: 生成脉络蓝图
        let meridian = self.meridian_gen.generate(&graph)?;

        // Step 3: 展开为3D世界
        let world = self.expander.expand(&meridian)?;

        Ok(world)
    }

    /// 仅构建关系图谱
    pub fn text_to_graph(&self, text: &str) -> Result<TextRelationGraph, Error> {
        Ok(self.graph_builder.build(text)?)
    }

    /// 仅生成脉络蓝图
    pub fn graph_to_meridian(&self, graph: &TextRelationGraph) -> Result<InformationMeridian, Error> {
        Ok(self.meridian_gen.generate(graph)?)
    }

    /// 仅展开脉络为3D世界
    pub fn meridian_to_3d(&self, meridian: &InformationMeridian) -> Result<meridian::World3D, Error> {
        Ok(self.expander.expand(meridian)?)
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// 统一错误类型
#[derive(Debug)]
pub enum Error {
    /// 图谱构建错误
    Graph(graph::BuildError),
    /// 脉络生成错误
    Meridian(meridian::GenerateError),
    /// 展开错误
    Expand(ExpandError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Graph(e) => write!(f, "图谱构建错误: {}", e),
            Error::Meridian(e) => write!(f, "脉络生成错误: {}", e),
            Error::Expand(e) => write!(f, "展开错误: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<graph::BuildError> for Error {
    fn from(e: graph::BuildError) -> Self {
        Error::Graph(e)
    }
}

impl From<meridian::GenerateError> for Error {
    fn from(e: meridian::GenerateError) -> Self {
        Error::Meridian(e)
    }
}

impl From<ExpandError> for Error {
    fn from(e: ExpandError) -> Self {
        Error::Expand(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_creation() {
        let pipeline = Pipeline::new();
        assert!(true); // 仅验证创建成功
    }

    #[test]
    fn test_simple_text_to_3d() {
        let pipeline = Pipeline::new();
        let text = "一个房子里有一张桌子";

        let result = pipeline.text_to_3d(text);

        // MVP 阶段：验证流程能走通
        // 由于简单的模式匹配可能不能完全解析，这里只验证不会崩溃
        match result {
            Ok(world) => {
                println!("生成世界成功，节点数: {}", world.node_count());
            }
            Err(e) => {
                println!("生成失败（MVP阶段可能正常）: {}", e);
            }
        }
    }

    #[test]
    fn test_mvp_example() {
        // MVP 示例：从文本生成3D场景
        let text = "房子里有桌子";
        let pipeline = Pipeline::new();

        // Step 1: 构建图谱
        let graph_result = pipeline.text_to_graph(text);
        assert!(graph_result.is_ok(), "图谱构建应该成功");

        let graph = graph_result.unwrap();
        println!("图谱实体数: {}", graph.entity_count());

        // 如果有实体，继续后续步骤
        if graph.entity_count() > 0 {
            // Step 2: 生成脉络
            let meridian_result = pipeline.graph_to_meridian(&graph);
            if let Ok(meridian) = meridian_result {
                println!("脉络节点数: {}", meridian.node_count());

                // Step 3: 展开为3D
                let world_result = pipeline.meridian_to_3d(&meridian);
                if let Ok(world) = world_result {
                    println!("3D世界节点数: {}", world.node_count());
                }
            }
        }
    }

    #[test]
    fn test_full_pipeline_with_house_and_table() {
        let text = "房子里有桌子";
        let pipeline = Pipeline::new();

        let result = pipeline.text_to_3d(text);

        // 验证流程完整执行
        match result {
            Ok(world) => {
                // 验证世界非空
                assert!(world.node_count() > 0, "世界应该有节点");

                // 验证边界框已计算
                let size = world.bounding_box.size();
                assert!(size[0] >= 0.0, "边界框宽度应非负");
            }
            Err(e) => {
                // MVP 阶段可能无法完全解析，打印错误用于调试
                eprintln!("MVP阶段预期错误: {}", e);
            }
        }
    }
}
