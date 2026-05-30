//! 推理能力测试
//!
//! 验证蛊虫的真正推理能力

use lnn::{GuReasoningCore, InferenceType, ConceptSpace, ConceptLevel};

/// 测试推理核心创建
#[test]
fn test_reasoning_core_creation() {
    let core = GuReasoningCore::new();
    let state = core.network_state();

    println!("\n🧠 推理核心创建:");
    println!("├─ 神经元数: {}", state.neuron_count);
    println!("├─ 突触数: {}", state.synapse_count);
    println!("└─ 金币: {}", core.get_coins());

    assert_eq!(state.neuron_count, 0);
    assert_eq!(core.get_coins(), 0.0);
}

/// 测试从概念空间构建神经网络
#[test]
fn test_build_neural_network() {
    println!("\n🧠 测试神经网络构建...\n");

    // 创建概念空间
    let mut space = ConceptSpace::new();

    // 创建层次概念
    space.create_concept(
        "fruit".to_string(),
        "水果".to_string(),
        ConceptLevel::Basic,
    ).unwrap();

    space.create_child_concept("fruit", "apple".to_string(), "苹果".to_string()).unwrap();
    space.create_child_concept("fruit", "banana".to_string(), "香蕉".to_string()).unwrap();
    space.create_child_concept("fruit", "orange".to_string(), "橙子".to_string()).unwrap();

    // 从概念空间创建推理核心
    let core = GuReasoningCore::from_concept_space(space);
    let state = core.network_state();

    println!("📊 神经网络结构:");
    println!("├─ 概念神经元: {}", state.neuron_count);
    println!("└─ 概念连接(突触): {}", state.synapse_count);

    assert_eq!(state.neuron_count, 4); // fruit + apple + banana + orange
    assert_eq!(state.synapse_count, 3); // 3条父子连接
}

/// 测试演绎推理
#[test]
fn test_deductive_reasoning() {
    println!("\n🧠 测试演绎推理...\n");

    // 创建概念层次：动物 -> 哺乳动物 -> 狗
    let mut space = ConceptSpace::new();
    space.create_concept(
        "animal".to_string(),
        "动物".to_string(),
        ConceptLevel::Basic,
    ).unwrap();

    space.create_child_concept("animal", "mammal".to_string(), "哺乳动物".to_string()).unwrap();
    space.create_child_concept("mammal", "dog".to_string(), "狗".to_string()).unwrap();

    let mut core = GuReasoningCore::from_concept_space(space);

    // 演绎推理：从"动物"推导
    let result = core.reason(&["animal"], InferenceType::Deductive, 5);

    assert!(result.is_some());
    let r = result.unwrap();

    println!("📊 演绎推理结果:");
    println!("├─ 结论: {}", r.conclusion);
    println!("├─ 置信度: {:.2}", r.confidence);
    println!("├─ 推理路径:");
    for step in &r.reasoning_path {
        println!("│   └─ {}", step);
    }
    println!("└─ 激活概念: {:?}", r.activated_concepts);

    assert!(r.conclusion.contains("哺乳动物"));
    assert!(r.confidence > 0.5);
}

/// 测试类比推理
#[test]
fn test_analogical_reasoning() {
    println!("\n🧠 测试类比推理...\n");

    let mut space = ConceptSpace::new();

    // 创建相似概念
    space.create_concept(
        "car".to_string(),
        "汽车".to_string(),
        ConceptLevel::Common,
    ).unwrap();

    space.create_concept(
        "bicycle".to_string(),
        "自行车".to_string(),
        ConceptLevel::Common,
    ).unwrap();

    // 建立关联（它们都是交通工具）
    let _ = space.learn_association("car", "bicycle", 1.0);

    let mut core = GuReasoningCore::from_concept_space(space);

    // 类比推理：汽车类似什么？
    let result = core.reason(&["car"], InferenceType::Analogical, 5);

    assert!(result.is_some());
    let r = result.unwrap();

    println!("📊 类比推理结果:");
    println!("├─ 结论: {}", r.conclusion);
    println!("├─ 置信度: {:.2}", r.confidence);
    println!("└─ 激活概念: {:?}", r.activated_concepts);

    assert!(r.conclusion.contains("自行车") || r.conclusion.contains("相似"));
}

/// 测试归纳推理
#[test]
fn test_inductive_reasoning() {
    println!("\n🧠 测试归纳推理...\n");

    let mut space = ConceptSpace::new();

    // 创建多个相似概念
    space.create_concept("apple".to_string(), "苹果".to_string(), ConceptLevel::Common).unwrap();
    space.create_concept("banana".to_string(), "香蕉".to_string(), ConceptLevel::Common).unwrap();
    space.create_concept("orange".to_string(), "橙子".to_string(), ConceptLevel::Common).unwrap();

    // 建立关联
    let _ = space.learn_association("apple", "banana", 1.0);
    let _ = space.learn_association("banana", "orange", 1.0);

    let mut core = GuReasoningCore::from_concept_space(space);

    // 归纳推理：这些有什么共同点？
    let result = core.reason(&["apple", "banana", "orange"], InferenceType::Inductive, 5);

    assert!(result.is_some());
    let r = result.unwrap();

    println!("📊 归纳推理结果:");
    println!("├─ 结论: {}", r.conclusion);
    println!("├─ 置信度: {:.2}", r.confidence);
    println!("└─ 激活概念: {:?}", r.activated_concepts);

    assert!(r.conclusion.contains("共性") || r.conclusion.contains("类别"));
}

/// 测试经济系统训练推理
#[test]
fn test_economic_training() {
    println!("\n🧠 测试经济系统训练推理...\n");

    let mut core = GuReasoningCore::new();

    // 学习知识获得金币
    let reward1 = core.learn_knowledge("concept1", "这是关于HTML的知识").unwrap();
    let reward2 = core.learn_knowledge("concept2", "这是关于CSS的知识").unwrap();

    println!("📊 学习获得金币:");
    println!("├─ 知识1奖励: {:.2}", reward1);
    println!("├─ 知识2奖励: {:.2}", reward2);
    println!("└─ 总金币: {:.2}", core.get_coins());

    // 执行任务消耗推理能力
    let task_result = core.execute_task("测试任务", 5.0);

    println!("\n📊 执行任务:");
    if let Some(r) = task_result {
        println!("├─ 任务结果: {}", r.conclusion);
        println!("└─ 当前金币: {:.2}", core.get_coins());
    }

    assert!(core.get_coins() > 0.0);
}

/// 测试完整推理流程
#[test]
fn test_full_reasoning_pipeline() {
    println!("\n🧠 测试完整推理流程...\n");

    // 1. 创建概念空间
    let mut space = ConceptSpace::new();

    // 创建知识层次
    space.create_concept("programming".to_string(), "编程".to_string(), ConceptLevel::Basic).unwrap();
    space.create_child_concept("programming", "python".to_string(), "Python".to_string()).unwrap();
    space.create_child_concept("programming", "javascript".to_string(), "JavaScript".to_string()).unwrap();
    space.create_child_concept("python", "python_function".to_string(), "Python函数".to_string()).unwrap();
    space.create_child_concept("javascript", "js_function".to_string(), "JS函数".to_string()).unwrap();

    // 2. 构建推理核心
    let mut core = GuReasoningCore::from_concept_space(space);

    println!("📊 神经网络:");
    let state = core.network_state();
    println!("├─ 神经元: {}", state.neuron_count);
    println!("└─ 突触: {}", state.synapse_count);

    // 3. 执行多种推理
    println!("\n📊 推理测试:");

    // 演绎
    let ded = core.reason(&["programming"], InferenceType::Deductive, 5);
    if let Some(r) = ded {
        println!("├─ 演绎: {} (置信度: {:.2})", r.conclusion.chars().take(50).collect::<String>(), r.confidence);
    }

    // 类比
    let ana = core.reason(&["python"], InferenceType::Analogical, 5);
    if let Some(r) = ana {
        println!("├─ 类比: {} (置信度: {:.2})", r.conclusion.chars().take(50).collect::<String>(), r.confidence);
    }

    // 归纳
    let ind = core.reason(&["python", "javascript"], InferenceType::Inductive, 5);
    if let Some(r) = ind {
        println!("└─ 归纳: {} (置信度: {:.2})", r.conclusion.chars().take(50).collect::<String>(), r.confidence);
    }

    // 4. 查看推理历史
    println!("\n📊 推理历史: {} 次推理", core.reasoning_history().len());
}
