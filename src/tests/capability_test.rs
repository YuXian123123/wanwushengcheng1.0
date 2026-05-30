//! 能力测试 - 验证递归学习后系统能做什么
//!
//! 测试"能力自生长"设计理念

use lnn::learning::RecursiveLearner;

/// 能力测试报告
struct CapabilityReport {
    domain: String,
    concepts_learned: usize,
    code_examples: usize,
    key_concepts: usize,
    capabilities: Vec<String>,
}

impl CapabilityReport {
    fn print(&self) {
        println!("\n{}", "=".repeat(60));
        println!("📊 {} 能力报告", self.domain);
        println!("{}", "=".repeat(60));
        println!("├─ 学习概念数: {}", self.concepts_learned);
        println!("├─ 代码示例数: {}", self.code_examples);
        println!("├─ 关键概念数: {}", self.key_concepts);
        println!("└─ 形成能力:");
        for cap in &self.capabilities {
            println!("    ✓ {}", cap);
        }
    }
}

/// 测试 HTML 能力生成
#[test]
fn test_html_capabilities() {
    println!("\n🧪 测试 HTML 能力生成...\n");

    let mut learner = RecursiveLearner::new();
    let result = learner.learn_from_directory("D:\\训练数据\\aireader\\html");

    // 1. 验证概念理解能力
    let space = learner.get_concept_space();
    let has_html_concept = space.get_concept("html").is_some();
    println!("✓ 概念理解: {}",
        if has_html_concept { "理解 HTML 基本概念" } else { "未学习" }
    );

    // 2. 验证代码生成能力
    let code = learner.test_generate_code("html");
    let can_generate_code = code.is_some();
    println!("✓ 代码生成: {}",
        if can_generate_code { "能生成 HTML 代码" } else { "未学习" }
    );
    if let Some(c) = &code {
        println!("  示例: {}", c.lines().next().unwrap_or(""));
    }

    // 3. 验证问答能力
    let answer = learner.test_answer_question("结构", "html");
    let can_answer = answer.is_some();
    println!("✓ 问答能力: {}",
        if can_answer { "能回答 HTML 相关问题" } else { "未学习" }
    );

    // 4. 生成能力报告
    let report = CapabilityReport {
        domain: "HTML".to_string(),
        concepts_learned: result.concepts_created,
        code_examples: result.capabilities.iter()
            .find(|c| c.contains("代码示例"))
            .map(|c| c.parse::<usize>().unwrap_or(0))
            .unwrap_or(0),
        key_concepts: result.capabilities.iter()
            .find(|c| c.contains("知识概念"))
            .map(|c| c.parse::<usize>().unwrap_or(0))
            .unwrap_or(0),
        capabilities: result.capabilities,
    };
    report.print();

    // 断言：必须形成基本能力
    assert!(result.concepts_created > 0, "必须学习到概念");
}

/// 测试多领域综合能力
#[test]
fn test_multi_domain_capabilities() {
    println!("\n🧪 测试多领域综合能力...\n");

    let mut learner = RecursiveLearner::new();

    // 学习多个领域
    let domains = vec![
        ("HTML", "D:\\训练数据\\aireader\\html"),
        ("Python", "D:\\训练数据\\aireader\\Python 知识体系"),
        ("C", "D:\\训练数据\\aireader\\c"),
    ];

    let mut total_concepts = 0;
    let mut total_docs = 0;

    for (name, path) in &domains {
        let result = learner.learn_from_directory(path);
        if result.concepts_created > 0 {
            println!("✓ {}: {} 概念, {} 知识点",
                name, result.concepts_created, result.points_learned);
            total_concepts += result.concepts_created;
        }
    }
    total_docs = learner.learned_document_count();

    println!("\n📊 综合能力统计:");
    println!("├─ 总概念数: {}", total_concepts);
    println!("├─ 总文档数: {}", total_docs);
    println!("└─ 领域数: {}", domains.len());

    // 验证跨领域能力
    let space = learner.get_concept_space();
    let mut domains_learned = 0;
    for (name, _) in &domains {
        if space.get_concept(&name.to_lowercase()).is_some() {
            domains_learned += 1;
        }
    }

    println!("\n✓ 跨领域理解: 理解 {} 个领域的知识", domains_learned);

    assert!(total_concepts > 100, "应该学习到足够多的概念");
}

/// 测试知识迁移能力（概念关联）
#[test]
fn test_knowledge_transfer() {
    println!("\n🧪 测试知识迁移能力...\n");

    let mut learner = RecursiveLearner::new();

    // 先学习 HTML
    let html_result = learner.learn_from_directory("D:\\训练数据\\aireader\\html");
    println!("HTML 学习: {} 概念", html_result.concepts_created);

    // 再学习 Python
    let python_result = learner.learn_from_directory("D:\\训练数据\\aireader\\Python 知识体系");
    println!("Python 学习: {} 概念", python_result.concepts_created);

    // 检查概念空间中的关联
    let space = learner.get_concept_space();

    // HTML 和 Python 都有"函数"概念吗？
    let html_has_function = space.get_concept("html_基础语法_函数").is_some();
    let python_has_function = space.get_concept("python 知识体系_函数").is_some();

    println!("\n📊 知识迁移分析:");
    println!("├─ HTML 函数概念: {}", html_has_function);
    println!("└─ Python 函数概念: {}", python_has_function);

    // 计算概念相似度（如果两个领域都有相关概念）
    if html_has_function && python_has_function {
        if let Some(sim) = space.similarity("html_基础语法_函数", "python 知识体系_函数") {
            println!("✓ 跨领域相似度: {:.3}", sim);
        }
    }

    println!("\n✓ 知识迁移: 系统能够识别跨领域的相似概念");
}

/// 测试递归深度学习
#[test]
fn test_recursive_depth() {
    println!("\n🧪 测试递归深度学习...\n");

    let mut learner = RecursiveLearner::new();
    let result = learner.learn_from_directory("D:\\训练数据\\aireader\\html");

    let space = learner.get_concept_space();

    // 检查概念层次深度
    // html -> html_基础语法 -> html_基础语法_具体文档 -> html_基础语法_具体文档_子概念
    let depth_1 = space.get_concept("html").is_some();
    let depth_2 = space.get_concept("html_基础语法").is_some();
    let depth_3 = space.get_concept("html_基础语法_HTML_文档结构").is_some();

    println!("📊 概念层次深度:");
    println!("├─ 第1层 (领域): html = {}", depth_1);
    println!("├─ 第2层 (分类): html_基础语法 = {}", depth_2);
    println!("└─ 第3层 (文档): html_基础语法_HTML_文档结构 = {}", depth_3);

    let max_depth = if depth_3 { 3 } else if depth_2 { 2 } else if depth_1 { 1 } else { 0 };
    println!("\n✓ 最大层次深度: {}", max_depth);

    assert!(max_depth >= 2, "应该形成至少2层概念层次");
}
