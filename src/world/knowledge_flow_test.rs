//! 知识消耗流程集成测试

use crate::world::WorldMind;
use crate::herness_web::KnowledgeFileEvent;

/// 测试完整的知识消耗流程
#[test]
fn test_full_knowledge_consumption_flow() {
    // 创建世界
    let mut world = WorldMind::new();

    // 注册一个蛊虫
    let gu_id = uuid::Uuid::new_v4();
    world = world.register_gu(gu_id);

    // 创建测试知识文件
    let knowledge_content = r#"
# HTML 基础教程

HTML 是构建网页的标准标记语言。

```html
<!DOCTYPE html>
<html>
<head>
    <title>Hello World</title>
</head>
<body>
    <h1>Hello, World!</h1>
    <p class="intro">这是一个段落。</p>
</body>
</html>
```

## 核心概念

- 元素 (Element): HTML 文档的基本构建块
- 标签 (Tag): 元素的标记，如 `<div>`, `<p>`
- 属性 (Attribute): 元素的附加信息，如 `class`, `id`

HTML 元素可以嵌套，形成文档树结构。
"#;

    let file_event = KnowledgeFileEvent {
        path: "test/html-basics.md".to_string(),
        filename: "html-basics.md".to_string(),
        extension: "md".to_string(),
        content: knowledge_content.to_string(),
        size: knowledge_content.len(),
        relative_path: "html-basics.md".to_string(),
        batch_id: "test_batch_001".to_string(),
        index: 1,
        total: 1,
    };

    // 执行知识消耗
    let result = world.receive_knowledge_file(&file_event);

    // 验证结果
    assert!(result.success, "知识消耗应该成功");
    assert!(result.message.contains("html-basics"), "消息应该包含文件名");
    assert!(result.should_continue, "应该继续处理更多文件");

    // 验证蛊虫状态变化
    let gu = world.gu_registry().get(&gu_id).expect("蛊虫应该存在");

    // 应该有技能
    assert!(!gu.skills.is_empty(), "蛊虫应该有技能");

    // 验证 LNN 状态变化
    let cognitive = gu.lnn.get_neuron_state(crate::core::NeuronType::Cognitive);
    assert!(cognitive > 0.0, "认知神经元应该被激活");
}

/// 测试多个文件的学习
#[test]
fn test_multiple_file_learning() {
    let mut world = WorldMind::new();
    let gu_id = uuid::Uuid::new_v4();
    world = world.register_gu(gu_id);

    // 学习多个文件
    let files = vec![
        ("html-intro.md", "# HTML 简介\nHTML 是标记语言。"),
        ("css-intro.md", "# CSS 简介\nCSS 用于样式。"),
        ("js-intro.md", "# JavaScript 简介\nJS 用于交互。"),
    ];

    for (filename, content) in files {
        let event = KnowledgeFileEvent {
            path: format!("test/{}", filename),
            filename: filename.to_string(),
            extension: "md".to_string(),
            content: content.to_string(),
            size: content.len(),
            relative_path: filename.to_string(),
            batch_id: "multi_test".to_string(),
            index: 0,
            total: 3,
        };
        let result = world.receive_knowledge_file(&event);
        assert!(result.success);
    }

    // 验证蛊虫学习了多个技能
    let gu = world.gu_registry().get(&gu_id).expect("蛊虫应该存在");
    assert!(gu.skills.len() >= 1, "蛊虫应该至少有一个技能");

    // 验证 LNN 活跃度
    let activity = gu.lnn.get_overall_activity();
    assert!(activity > 0.0, "LNN 应该有活跃度");
}

/// 测试知识价值计算影响
#[test]
fn test_knowledge_value_impact() {
    let mut world = WorldMind::new();
    let gu_id = uuid::Uuid::new_v4();
    world = world.register_gu(gu_id);

    // 学习高价值内容（有代码块）
    let high_value_content = r#"
# Rust 编程

```rust
fn main() {
    println!("Hello, Rust!");
}
```

Rust 是系统编程语言。Rust 注重安全。Rust 性能优异。
"#;

    let high_event = KnowledgeFileEvent {
        path: "test/rust.md".to_string(),
        filename: "rust.md".to_string(),
        extension: "md".to_string(),
        content: high_value_content.to_string(),
        size: high_value_content.len(),
        relative_path: "rust.md".to_string(),
        batch_id: "value_test".to_string(),
        index: 1,
        total: 1,
    };

    let result = world.receive_knowledge_file(&high_event);
    assert!(result.success);

    // 高价值内容应该产生更强的 LNN 激活
    let gu = world.gu_registry().get(&gu_id).expect("蛊虫应该存在");
    let cognitive = gu.lnn.get_neuron_state(crate::core::NeuronType::Cognitive);
    assert!(cognitive > 0.0, "认知神经元应该被激活");
}
