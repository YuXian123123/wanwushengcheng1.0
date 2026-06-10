//! 训练数据加载器
//!
//! 加载场景训练数据，用于训练3D生成模型

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// 训练数据集
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingDataset {
    /// 版本
    pub version: String,
    /// 描述
    pub description: String,
    /// 数据条目
    pub data: Vec<TrainingSample>,
}

/// 训练样本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSample {
    /// 唯一标识
    pub id: String,
    /// 自然语言描述
    pub text: String,
    /// 语言
    #[serde(default = "default_language")]
    pub language: String,
    /// 实体列表
    pub entities: Vec<TrainingEntity>,
    /// 关系列表
    pub relations: Vec<TrainingRelation>,
    /// 3D布局
    pub layout_3d: TrainingLayout,
}

fn default_language() -> String {
    "zh-CN".to_string()
}

/// 训练实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingEntity {
    /// 实体ID
    pub id: String,
    /// 实体名称
    pub name: String,
    /// 实体类型
    #[serde(rename = "type")]
    pub entity_type: String,
    /// 属性（可选）
    #[serde(default)]
    pub attributes: HashMap<String, String>,
}

/// 训练关系
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingRelation {
    /// 源实体ID
    pub from: String,
    /// 目标实体ID
    pub to: String,
    /// 关系类型
    #[serde(rename = "type")]
    pub relation_type: String,
}

/// 3D布局
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingLayout {
    /// 根实体ID
    pub root: String,
    /// 布局节点列表
    pub nodes: Vec<LayoutNode>,
}

/// 布局节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutNode {
    /// 对应实体ID
    pub entity_id: String,
    /// 位置 [x, y, z]
    pub position: [f64; 3],
    /// 旋转 [qx, qy, qz, qw]（可选）
    #[serde(default = "default_rotation")]
    pub rotation: [f64; 4],
    /// 缩放 [sx, sy, sz]
    pub scale: [f64; 3],
    /// 几何类型
    pub geometry: String,
}

fn default_rotation() -> [f64; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

impl TrainingDataset {
    /// 从JSON文件加载
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| format!("无法打开文件: {}", e))?;
        let reader = BufReader::new(file);
        let dataset: Self = serde_json::from_reader(reader)
            .map_err(|e| format!("解析JSON失败: {}", e))?;
        Ok(dataset)
    }

    /// 获取样本数量
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 获取所有文本
    pub fn get_texts(&self) -> Vec<&str> {
        self.data.iter().map(|s| s.text.as_str()).collect()
    }

    /// 统计实体类型
    pub fn entity_type_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        for sample in &self.data {
            for entity in &sample.entities {
                *stats.entry(entity.entity_type.clone()).or_insert(0) += 1;
            }
        }
        stats
    }

    /// 统计关系类型
    pub fn relation_type_stats(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        for sample in &self.data {
            for relation in &sample.relations {
                *stats.entry(relation.relation_type.clone()).or_insert(0) += 1;
            }
        }
        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_dataset() {
        if let Ok(dataset) = TrainingDataset::load("data/training/scenes_basic.json") {
            println!("加载训练数据: {} 个样本", dataset.len());
            println!("实体类型统计: {:?}", dataset.entity_type_stats());
            println!("关系类型统计: {:?}", dataset.relation_type_stats());
        }
    }
}
