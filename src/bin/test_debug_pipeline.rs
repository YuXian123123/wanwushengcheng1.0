//! 调试测试：检查每个阶段的数据

use lnn::world_gen::Pipeline;

fn main() {
    println!("=== 调试测试：检查每个阶段 ===\n");

    let pipeline = Pipeline::with_pretrained_vectors("data/word_vectors.json");

    let text = "房子里有桌子";
    println!("输入: {}\n", text);

    // Step 1: 图谱
    println!("Step 1: 图谱构建");
    println!("{}", "-".repeat(50));

    match pipeline.text_to_graph(text) {
        Ok(graph) => {
            println!("实体数量: {}", graph.entity_count());
            for (id, entity) in &graph.entities {
                println!("  - {} [{}] ID={}", entity.name, entity.entity_type.name(), id);
            }

            println!("\n关系数量: {}", graph.relations.len());
            for rel in &graph.relations {
                println!("  - {} --{:?}--> {}", rel.from, rel.relation_type, rel.to);
            }

            println!("\n层次结构:");
            println!("  根节点: {:?}", graph.hierarchy.roots);
            println!("  子节点映射: {:?}", graph.hierarchy.children_map);
            println!("  父节点映射: {:?}", graph.hierarchy.parent_map);

            // Step 2: 脉络
            println!("\nStep 2: 脉络生成");
            println!("{}", "-".repeat(50));

            match pipeline.graph_to_meridian(&graph) {
                Ok(meridian) => {
                    println!("脉络节点数: {}", meridian.node_count());
                    for (id, node) in &meridian.nodes {
                        println!("  - {} [{}] ID={}", node.name, node.node_type.name(), id);
                        println!("    实体ID: {:?}", node.entity_id);
                        println!("    父节点: {:?}", node.parent);
                        println!("    子节点: {:?}", node.children);
                    }

                    // Step 3: 3D世界
                    println!("\nStep 3: 3D展开");
                    println!("{}", "-".repeat(50));

                    match pipeline.meridian_to_3d(&meridian) {
                        Ok(world) => {
                            println!("世界节点数: {}", world.node_count());
                            for (id, node) in &world.nodes {
                                println!("  - ID={}", id);
                                println!("    几何: {:?}", node.geometry);
                                println!("    位置: {:?}", node.position);
                            }
                        }
                        Err(e) => println!("[FAIL] 3D展开失败: {}", e),
                    }
                }
                Err(e) => println!("[FAIL] 脉络生成失败: {}", e),
            }
        }
        Err(e) => println!("[FAIL] 图谱构建失败: {}", e),
    }
}
