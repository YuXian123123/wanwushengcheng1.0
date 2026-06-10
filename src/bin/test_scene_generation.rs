//! 完整场景生成测试

use lnn::world_gen::Pipeline;

fn main() {
    println!("=== 完整场景生成测试 ===\n");

    // 创建带预训练词向量的 Pipeline
    let pipeline = Pipeline::with_pretrained_vectors("data/word_vectors.json");

    // 测试场景
    let test_scenes = vec![
        "房子里有桌子",
        "村庄里有房子和树",
        "房子里有桌子和椅子",
    ];

    for text in test_scenes {
        println!("\n场景: {}", text);
        println!("{}", "=".repeat(50));

        match pipeline.text_to_3d(text) {
            Ok(world) => {
                println!("\n📊 生成结果:");
                println!("   节点数: {}", world.node_count());

                // 显示边界框
                let bb = &world.bounding_box;
                println!("   边界框: ({:.1}, {:.1}, {:.1}) - ({:.1}, {:.1}, {:.1})",
                    bb.min[0], bb.min[1], bb.min[2],
                    bb.max[0], bb.max[1], bb.max[2]);

                let size = bb.size();
                println!("   尺寸: {:.1} x {:.1} x {:.1}",
                    size[0], size[1], size[2]);

                // 显示节点详情
                println!("\n📦 节点列表:");
                for node in world.nodes.values() {
                    println!("   - ID: {}", node.id);
                    println!("     几何: {:?}", node.geometry);
                    println!("     位置: ({:.2}, {:.2}, {:.2})",
                        node.position[0], node.position[1], node.position[2]);
                    println!("     缩放: ({:.2}, {:.2}, {:.2})",
                        node.scale[0], node.scale[1], node.scale[2]);
                }

                println!("\n✅ 场景生成成功!");
            }
            Err(e) => {
                println!("\n❌ 场景生成失败: {}", e);
            }
        }
    }

    println!("\n=== 测试完成 ===");
}
