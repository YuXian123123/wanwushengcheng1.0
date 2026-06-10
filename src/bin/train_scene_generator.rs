//! 3D场景生成训练程序
//!
//! 使用训练数据集训练场景生成模型

use lnn::world_gen::{
    TrainingDataset, TrainingSample, Pipeline,
    PretrainedVectors, GraphBuildConfig,
    graph::{EntityType, RelationType, GraphEntity, GraphRelation},
};
use std::collections::HashMap;

fn main() {
    println!("=== 3D场景生成训练 ===\n");

    // Step 1: 加载训练数据
    println!("Step 1: 加载训练数据");
    println!("{}", "-".repeat(50));

    let dataset = match TrainingDataset::load("../data/training/scenes_combined.json") {
        Ok(d) => {
            println!("[OK] 加载成功!");
            println!("   样本数: {}", d.len());
            println!("   实体类型: {:?}", d.entity_type_stats());
            println!("   关系类型: {:?}", d.relation_type_stats());
            d
        }
        Err(e) => {
            println!("[FAIL] {}", e);
            return;
        }
    };

    // Step 2: 加载预训练词向量
    println!("\nStep 2: 加载预训练词向量");
    println!("{}", "-".repeat(50));

    let vectors = match PretrainedVectors::load("../data/word_vectors.json") {
        Ok(v) => {
            println!("[OK] 词表大小: {}", v.vocab_size());
            v
        }
        Err(e) => {
            println!("[WARN] {}", e);
            println!("   将使用回退模式");
            PretrainedVectors::default()
        }
    };

    // Step 3: 构建几何模板库
    println!("\nStep 3: 构建几何模板库");
    println!("{}", "-".repeat(50));

    let mut geometry_templates = GeometryTemplateLibrary::new();
    geometry_templates.learn_from_dataset(&dataset);
    println!("[OK] 学习了 {} 种实体几何模板", geometry_templates.len());

    for (entity_type, template) in geometry_templates.templates.iter() {
        println!("   {} -> {:?} (缩放: {:?})",
            entity_type, template.geometry, template.default_scale);
    }

    // Step 4: 构建空间规则库
    println!("\nStep 4: 构建空间规则库");
    println!("{}", "-".repeat(50));

    let mut spatial_rules = SpatialRuleLibrary::new();
    spatial_rules.learn_from_dataset(&dataset);
    println!("[OK] 学习了 {} 条空间规则", spatial_rules.len());

    for (relation_type, rule) in spatial_rules.rules.iter() {
        println!("   {} -> 相对位置: {:?}", relation_type, rule.default_offset);
    }

    // Step 5: 验证训练效果
    println!("\nStep 5: 验证训练效果");
    println!("{}", "-".repeat(50));

    let pipeline = Pipeline::with_pretrained_vectors("../data/word_vectors.json");

    for sample in dataset.data.iter().take(5) {
        print!("输入: {:25} -> ", sample.text);

        match pipeline.text_to_3d(&sample.text) {
            Ok(world) => {
                let expected_count = sample.layout_3d.nodes.len();
                let actual_count = world.node_count();

                if actual_count == expected_count {
                    println!("[OK] {} 节点 (预期 {})", actual_count, expected_count);
                } else {
                    println!("[WARN] {} 节点 (预期 {})", actual_count, expected_count);
                }
            }
            Err(e) => {
                println!("[FAIL] {}", e);
            }
        }
    }

    // Step 6: 保存训练结果
    println!("\nStep 6: 保存训练结果");
    println!("{}", "-".repeat(50));

    match geometry_templates.save("../data/training/geometry_templates.json") {
        Ok(_) => println!("[OK] 几何模板已保存"),
        Err(e) => println!("[FAIL] {}", e),
    }

    match spatial_rules.save("../data/training/spatial_rules.json") {
        Ok(_) => println!("[OK] 空间规则已保存"),
        Err(e) => println!("[FAIL] {}", e),
    }

    println!("\n[OK] 训练完成!");
}

/// 几何模板
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeometryTemplate {
    /// 默认缩放
    pub default_scale: [f64; 3],
    /// 几何类型
    pub geometry: String,
    /// 观察次数
    pub count: usize,
}

/// 几何模板库
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeometryTemplateLibrary {
    pub templates: HashMap<String, GeometryTemplate>,
}

impl GeometryTemplateLibrary {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
        }
    }

    pub fn learn_from_dataset(&mut self, dataset: &TrainingDataset) {
        for sample in &dataset.data {
            for node in &sample.layout_3d.nodes {
                // 找到对应实体
                if let Some(entity) = sample.entities.iter().find(|e| e.id == node.entity_id) {
                    let entry = self.templates.entry(entity.entity_type.clone()).or_insert(
                        GeometryTemplate {
                            default_scale: node.scale,
                            geometry: node.geometry.clone(),
                            count: 0,
                        }
                    );

                    // 更新平均值
                    if entry.count == 0 {
                        entry.default_scale = node.scale;
                    } else {
                        // 加权平均
                        let n = entry.count as f64;
                        let new_n = (entry.count + 1) as f64;
                        for i in 0..3 {
                            entry.default_scale[i] =
                                (entry.default_scale[i] * n + node.scale[i]) / new_n;
                        }
                    }
                    entry.count += 1;
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.templates.len()
    }

    pub fn save(&self, path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("序列化失败: {}", e))?;
        std::fs::write(path, json)
            .map_err(|e| format!("写入文件失败: {}", e))?;
        Ok(())
    }
}

/// 空间规则
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpatialRule {
    /// 默认偏移位置
    pub default_offset: [f64; 3],
    /// 观察次数
    pub count: usize,
}

/// 空间规则库
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SpatialRuleLibrary {
    pub rules: HashMap<String, SpatialRule>,
}

impl SpatialRuleLibrary {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn learn_from_dataset(&mut self, dataset: &TrainingDataset) {
        for sample in &dataset.data {
            // 构建实体ID到位置的映射
            let mut positions: HashMap<String, [f64; 3]> = HashMap::new();
            for node in &sample.layout_3d.nodes {
                positions.insert(node.entity_id.clone(), node.position);
            }

            // 分析关系对应的位置偏移
            for relation in &sample.relations {
                if let (Some(&from_pos), Some(&to_pos)) =
                    (positions.get(&relation.from), positions.get(&relation.to))
                {
                    let offset = [
                        to_pos[0] - from_pos[0],
                        to_pos[1] - from_pos[1],
                        to_pos[2] - from_pos[2],
                    ];

                    let entry = self.rules.entry(relation.relation_type.clone()).or_insert(
                        SpatialRule {
                            default_offset: offset,
                            count: 0,
                        }
                    );

                    // 更新平均值
                    if entry.count == 0 {
                        entry.default_offset = offset;
                    } else {
                        let n = entry.count as f64;
                        let new_n = (entry.count + 1) as f64;
                        for i in 0..3 {
                            entry.default_offset[i] =
                                (entry.default_offset[i] * n + offset[i]) / new_n;
                        }
                    }
                    entry.count += 1;
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.rules.len()
    }

    pub fn save(&self, path: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("序列化失败: {}", e))?;
        std::fs::write(path, json)
            .map_err(|e| format!("写入文件失败: {}", e))?;
        Ok(())
    }
}
