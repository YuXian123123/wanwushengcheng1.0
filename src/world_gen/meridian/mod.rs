//! 信息脉络模块
//!
//! 脉络是连接文本与3D世界的桥梁，类似于人体的经络系统。
//! 定义信息如何"展开"成空间结构。

pub mod types;
pub mod generator;

pub use types::{
    MeridianId, ChannelId, WorldNodeId, WorldId,
    MeridianNodeType, ChannelType, GeometryHint,
    MeridianNode, MeridianChannel,
    MeridianTemplate, MeridianHierarchy,
    InformationMeridian,
    World3D, WorldNode, BoundingBox,
    // 导出模板参数类型
    BioTemplateParams, ArchTemplateParams, NaturalTemplateParams, AbstractTemplateParams,
};
pub use generator::{MeridianGenerator, GenerateError, TemplateLibrary};
