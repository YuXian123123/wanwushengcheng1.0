//! 实体识别和关系抽取详细测试

use lnn::world_gen::{PretrainedVectors, Pipeline};

fn main() {
    println!("=== 实体识别和关系抽取详细测试 ===\n");

    // 创建带预训练词向量的 Pipeline
    println!("加载预训练词向量...");
    let pipeline = Pipeline::with_pretrained_vectors("data/word_vectors.json");

    // 检查是否成功加载
    if pipeline.get_pretrained_vectors().is_some() {
        println!("[OK] 预训练词向量加载成功");
    } else {
        println!("[WARN] 预训练词向量加载失败，将使用回退模式");
    }

    // 测试用例
    let test_cases = vec![
        ("房子里有桌子", vec!["房子", "桌子"]),
        ("村庄里有房子和树", vec!["村庄", "房子", "树"]),
        ("红色的房子有蓝色的屋顶", vec!["红色", "房子", "屋顶"]),
    ];

    for (text, expected_entities) in test_cases {
        println!("\n测试: {}", text);
        println!("{}", "-".repeat(50));

        // 构建图谱
        match pipeline.text_to_graph(text) {
            Ok(graph) => {
                println!("实体 ({} 个):", graph.entity_count());
                for (id, entity) in &graph.entities {
                    println!("  - {} [{:?}] (置信度: {:.2})",
                        entity.name, entity.entity_type, entity.confidence);
                }

                println!("\n关系 ({} 个):", graph.relations.len());
                for relation in &graph.relations {
                    println!("  - {} --{:?}--> {}",
                        relation.from, relation.relation_type, relation.to);
                }

                // 验证预期实体
                let found_entities: Vec<&str> = graph.entities.values()
                    .map(|e| e.name.as_str())
                    .collect();

                for expected in &expected_entities {
                    if found_entities.iter().any(|e| e.contains(expected)) {
                        println!("✅ 找到预期实体: {}", expected);
                    } else {
                        println!("❌ 缺少预期实体: {}", expected);
                    }
                }
            }
            Err(e) => {
                println!("[FAIL] {}", e);
            }
        }
    }

    println!("\n=== 测试完成 ===");
}
