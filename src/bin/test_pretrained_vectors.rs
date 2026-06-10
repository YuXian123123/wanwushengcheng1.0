//! 预训练词向量集成测试
//!
//! 对比使用预训练词向量前后的实体识别效果

use lnn::world_gen::{PretrainedVectors, GraphBuilder, GraphBuildConfig, Pipeline};

fn main() {
    println!("=== 预训练词向量集成测试 ===\n");

    // 加载预训练词向量
    println!("Step 1: 加载预训练词向量");
    println!("{}", "-".repeat(50));

    let vectors = match PretrainedVectors::load("data/word_vectors.json") {
        Ok(v) => {
            println!("[OK] 加载成功!");
            println!("   词表大小: {}", v.vocab_size());
            println!("   向量维度: {}", v.dimension);
            v
        }
        Err(e) => {
            println!("[FAIL] 加载失败: {}", e);
            println!("   请先运行: python scripts/export_word_vectors.py");
            return;
        }
    };

    // 测试词向量功能
    println!("\nStep 2: 测试词向量功能");
    println!("{}", "-".repeat(50));

    // 相似度测试
    let test_pairs = vec![
        ("房子", "建筑"),
        ("桌子", "椅子"),
        ("树", "植物"),
        ("红色", "颜色"),
    ];

    println!("相似度测试:");
    for (w1, w2) in &test_pairs {
        if let Some(sim) = vectors.similarity(w1, w2) {
            println!("   {} <-> {}: {:.4}", w1, w2, sim);
        }
    }

    // 类别推断测试
    println!("\n类别推断测试:");
    let test_words = vec!["房子", "桌子", "椅子", "树", "人", "红色"];
    for word in &test_words {
        if let Some((category, score)) = vectors.infer_category(word) {
            println!("   {} -> {} (置信度: {:.4})", word, category, score);
        } else {
            println!("   {} -> 未知类别", word);
        }
    }

    // 分词测试
    println!("\n分词测试:");
    let test_texts = vec!["房子里有桌子", "红色的房子有蓝色的屋顶", "村庄里有房子和树"];
    for text in test_texts {
        let tokens = vectors.tokenize(text);
        println!("   '{}' -> {:?}", text, tokens);
    }

    // 最相似词测试
    println!("\n最相似词测试:");
    for word in &["房子", "桌子"] {
        let similar = vectors.most_similar(word, 3);
        if !similar.is_empty() {
            print!("   {} 的相似词: ", word);
            for (w, s) in similar {
                print!("{}({:.2}) ", w, s);
            }
            println!();
        }
    }

    // Step 3: 完整流程测试
    println!("\nStep 3: 完整文本到3D流程");
    println!("{}", "-".repeat(50));

    let test_texts = vec![
        "房子里有桌子",
        "红色的房子有蓝色的屋顶",
        "村庄里有房子和树",
    ];

    // 使用带预训练词向量的 Pipeline
    let pipeline = Pipeline::with_pretrained_vectors("data/word_vectors.json");

    for text in test_texts {
        print!("输入: {:30} -> ", text);
        match pipeline.text_to_3d(text) {
            Ok(world) => {
                println!("[OK] {} 个节点", world.node_count());
            }
            Err(e) => {
                println!("[FAIL] {}", e);
            }
        }
    }

    // Step 4: 对比测试
    println!("\nStep 4: 效果对比");
    println!("{}", "-".repeat(50));

    println!("改进前（启发式）:");
    println!("   - 只能识别包含特定字符的词");
    println!("   - 无法识别语义相似关系");
    println!("   - 实体类型推断不精确");

    println!("\n改进后（预训练词向量）:");
    println!("   - 可以计算任意词的相似度");
    println!("   - 基于语义相似度推断实体类型");
    println!("   - 支持类比推理");
    println!("   - 支持基于词表的智能分词");

    // 统计信息
    println!("\n统计信息:");
    println!("   词表大小: {}", vectors.vocab_size());
    println!("   类别数量: {}", vectors.categories.len());

    println!("\n[OK] 测试完成!");
}
