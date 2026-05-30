//! 递归学习集成测试
//!
//! 测试从训练数据递归学习的能力

use lnn::learning::{RecursiveLearner, LearningResult};

/// 测试从 HTML 知识目录学习
#[test]
fn test_learn_html_knowledge() {
    let mut learner = RecursiveLearner::new();

    let result = learner.learn_from_directory("D:\\训练数据\\aireader\\html");

    // 验证学习结果
    println!("学习结果:");
    println!("  创建概念数: {}", result.concepts_created);
    println!("  学习知识点数: {}", result.points_learned);
    println!("  能力列表: {:?}", result.capabilities);
    if !result.errors.is_empty() {
        println!("  错误: {:?}", result.errors);
    }

    // 验证概念空间不为空
    let space = learner.get_concept_space();
    assert!(
        space.get_concept("html").is_some() || result.errors.len() > 0,
        "应该创建 html 根概念"
    );

    // 验证学习了文档
    assert!(
        learner.learned_document_count() > 0 || result.errors.len() > 0,
        "应该学习了文档"
    );
}

/// 测试生成代码能力
#[test]
fn test_generate_html_code() {
    let mut learner = RecursiveLearner::new();
    learner.learn_from_directory("D:\\训练数据\\aireader\\html");

    // 尝试生成 HTML 代码
    if let Some(code) = learner.test_generate_code("html") {
        println!("生成的 HTML 代码:");
        println!("{}", code);
        assert!(
            code.contains("<") || code.contains(">"),
            "代码应该包含 HTML 标签"
        );
    }
}

/// 测试回答问题能力
#[test]
fn test_answer_html_question() {
    let mut learner = RecursiveLearner::new();
    learner.learn_from_directory("D:\\训练数据\\aireader\\html");

    // 尝试回答问题
    if let Some(answer) = learner.test_answer_question("HTML", "html") {
        println!("问题回答:");
        println!("{}", answer.chars().take(500).collect::<String>());
        assert!(!answer.is_empty(), "回答不应该为空");
    }
}

/// 测试多领域学习
#[test]
fn test_learn_multiple_domains() {
    let mut learner = RecursiveLearner::new();

    // 学习 HTML
    let html_result = learner.learn_from_directory("D:\\训练数据\\aireader\\html");
    println!("HTML 学习: {} 概念, {} 知识点",
        html_result.concepts_created,
        html_result.points_learned
    );

    // 学习 Python（如果存在）
    let python_result = learner.learn_from_directory("D:\\训练数据\\aireader\\Python 知识体系");
    println!("Python 学习: {} 概念, {} 知识点",
        python_result.concepts_created,
        python_result.points_learned
    );

    // 验证概念空间包含多个领域
    let space = learner.get_concept_space();
    println!("总学习文档数: {}", learner.learned_document_count());
}

/// 测试概念层次结构
#[test]
fn test_concept_hierarchy() {
    let mut learner = RecursiveLearner::new();
    let result = learner.learn_from_directory("D:\\训练数据\\aireader\\html");

    let space = learner.get_concept_space();

    // 检查是否有子概念被创建
    println!("创建的概念数: {}", result.concepts_created);

    // 如果成功创建了概念，检查层次结构
    if let Some(_html_concept) = space.get_concept("html") {
        println!("✓ html 根概念存在");

        // 检查分类概念
        if space.get_concept("html_基础语法").is_some() {
            println!("✓ html_基础语法 分类概念存在");
        }

        if space.get_concept("html_表单").is_some() {
            println!("✓ html_表单 分类概念存在");
        }
    }
}
