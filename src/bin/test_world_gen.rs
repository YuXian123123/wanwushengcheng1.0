//! world_gen 模块测试示例
//!
//! 验证神经学习方法的改进效果

use lnn::world_gen::{
    GraphBuildConfig, MeridianConfig, Pipeline,
    GraphBuilder, TextRelationGraph,
};

fn main() {
    println!("=== 万物生成器测试 ===\n");

    // 测试1：启发式模式（无训练数据）
    test_heuristic_mode();

    // 测试2：完整管道
    test_full_pipeline();

    // 测试3：对比不同输入
    test_various_inputs();
}

fn test_heuristic_mode() {
    println!("【测试1】启发式模式（语义推断）");
    println!("{}", "-".repeat(50));

    let config = GraphBuildConfig::default();
    let builder = GraphBuilder::new(config);

    let text = "房子里有桌子";
    println!("输入: {}", text);

    match builder.build(text) {
        Ok(graph) => {
            println!("✅ 构建成功!");
            println!("   实体数: {}", graph.entity_count());
            println!("   关系数: {}", graph.relation_count());

            // 显示实体
            for (id, entity) in &graph.entities {
                println!("   实体: {} (类型: {:?})", entity.name, entity.entity_type);
            }

            // 显示关系
            for rel in &graph.relations {
                if let (Some(from), Some(to)) = (graph.entities.get(&rel.from), graph.entities.get(&rel.to)) {
                    println!("   关系: {} --{:?}--> {}", from.name, rel.relation_type, to.name);
                }
            }
        }
        Err(e) => {
            println!("❌ 构建失败: {}", e);
        }
    }

    println!();
}

fn test_full_pipeline() {
    println!("【测试2】完整管道（文本 → 图谱 → 脉络 → 3D）");
    println!("{}", "-".repeat(50));

    let pipeline = Pipeline::new();
    let text = "房子里有红色的桌子和蓝色的椅子";

    println!("输入: {}", text);

    // Step 1: 文本 → 图谱
    match pipeline.text_to_graph(text) {
        Ok(graph) => {
            println!("\nStep 1: 图谱构建");
            println!("   实体: {} 个", graph.entity_count());
            for (_, e) in &graph.entities {
                println!("   - {} ({:?})", e.name, e.entity_type);
            }

            // Step 2: 图谱 → 脉络
            match pipeline.graph_to_meridian(&graph) {
                Ok(meridian) => {
                    println!("\nStep 2: 脉络生成");
                    println!("   节点: {} 个", meridian.node_count());

                    // Step 3: 脉络 → 3D
                    match pipeline.meridian_to_3d(&meridian) {
                        Ok(world) => {
                            println!("\nStep 3: 3D展开");
                            println!("   世界节点: {} 个", world.node_count());
                            println!("   边界框: {:?}", world.bounding_box.size());

                            // 显示3D节点
                            for (_, node) in &world.nodes {
                                println!("   - 位置: {:?}, 几何: {:?}", node.position, node.geometry);
                            }
                        }
                        Err(e) => {
                            println!("   ❌ 3D展开失败: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("   ❌ 脉络生成失败: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ 图谱构建失败: {}", e);
        }
    }

    println!();
}

fn test_various_inputs() {
    println!("【测试3】多种输入对比");
    println!("{}", "-".repeat(50));

    let test_cases = vec![
        "房子里有桌子",
        "红色的房子有蓝色的屋顶",
        "村庄里有房子和树",
        "人坐在椅子上",
        "森林里有树和花",
    ];

    let pipeline = Pipeline::new();

    for text in test_cases {
        print!("输入: {:30} → ", text);

        match pipeline.text_to_3d(text) {
            Ok(world) => {
                println!("✅ 成功 ({} 个节点)", world.node_count());
            }
            Err(e) => {
                println!("⚠️ 部分成功: {}", e);
            }
        }
    }

    println!();
}
