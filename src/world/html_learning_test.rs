//! HTML 知识学习测试
//!
//! 测试从 D:\训练数据\aireader\html 目录学习 HTML 知识
//!
//! # 新特性：知识共享与共识机制
//!
//! 所有蛊虫学习同一主题时，会：
//! 1. 先检查共享知识库是否存在
//! 2. 不存在则提交候选版本
//! 3. 通过投票验证后，统一存入共享知识库

use std::fs;
use std::path::Path;

#[test]
fn test_learn_html_training_data() {
    use crate::world::WorldMind;
    use crate::herness_web::KnowledgeFileEvent;

    // 创建世界
    let mut world = WorldMind::new();

    // 注册多个蛊虫用于竞争学习
    let gu_ids: Vec<_> = (0..3).map(|_| {
        let gu_id = uuid::Uuid::new_v4();
        world = world.register_gu(gu_id);
        gu_id
    }).collect();

    // 扫描 HTML 训练数据目录
    let html_dir = Path::new("D:/训练数据/aireader/html");
    if !html_dir.exists() {
        println!("跳过测试：HTML 训练数据目录不存在");
        return;
    }

    // 递归查找所有 .md 文件
    let mut files: Vec<_> = Vec::new();
    fn scan_dir(dir: &Path, files: &mut Vec<std::path::PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    scan_dir(&path, files);
                } else if path.extension().map(|e| e == "md").unwrap_or(false) {
                    files.push(path);
                }
            }
        }
    }
    scan_dir(html_dir, &mut files);

    println!("找到 {} 个 HTML 知识文件", files.len());

    // 学习每个文件（轮询分配给不同蛊虫）
    let mut success_count = 0;
    let total = files.len();

    for (index, file_path) in files.iter().enumerate() {
        // 读取文件内容
        let content = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => {
                println!("读取文件失败 {}: {}", file_path.display(), e);
                continue;
            }
        };

        // 轮询分配给不同蛊虫（模拟竞争学习）
        let gu_idx = index % gu_ids.len();

        // 创建知识文件事件
        let relative_path = file_path
            .strip_prefix(html_dir)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        let filename = file_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        let event = KnowledgeFileEvent {
            path: file_path.to_string_lossy().to_string(),
            filename: filename.clone(),
            extension: "md".to_string(),
            content,
            size: file_path.metadata().map(|m| m.len() as usize).unwrap_or(0),
            relative_path,
            batch_id: "html_training".to_string(),
            index: index + 1,
            total,
        };

        // 执行学习
        let result = world.receive_knowledge_file(&event);
        if result.success {
            success_count += 1;
            print!("[{}/{}] ✓", index + 1, total);
            if index % 5 == 4 {
                println!();
            }
        }
    }

    println!("\n========== 学习统计 ==========");
    println!("总文件数: {}", total);
    println!("成功学习: {}", success_count);
    println!("成功率: {:.1}%", success_count as f64 / total as f64 * 100.0);

    // 验证蛊虫状态
    println!("\n蛊虫学习情况:");
    for (i, gu_id) in gu_ids.iter().enumerate() {
        if let Some(gu) = world.gu_registry().get(gu_id) {
            println!("  蛊虫 {}: 技能数={}", i + 1, gu.skills.len());
        }
    }

    // 验证共享知识库
    let shared_skills = std::fs::read_dir("D:/ai_006/knowledge/skills/shared")
        .map(|d| d.count())
        .unwrap_or(0);
    println!("\n共享技能库文件数: {}", shared_skills);

    assert!(success_count > 0, "应该至少成功学习一些文件");
}

#[test]
fn test_learn_single_html_file() {
    use crate::world::WorldMind;
    use crate::herness_web::KnowledgeFileEvent;

    // 创建世界
    let mut world = WorldMind::new();
    let gu_id = uuid::Uuid::new_v4();
    world = world.register_gu(gu_id);

    // 读取单个文件
    let file_path = "D:/训练数据/aireader/html/基础语法/001_HTML 文档结构.md";
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => {
            println!("跳过测试：文件不存在");
            return;
        }
    };

    let event = KnowledgeFileEvent {
        path: file_path.to_string(),
        filename: "001_HTML 文档结构.md".to_string(),
        extension: "md".to_string(),
        content,
        size: 0,
        relative_path: "基础语法/001_HTML 文档结构.md".to_string(),
        batch_id: "single_test".to_string(),
        index: 1,
        total: 1,
    };

    let result = world.receive_knowledge_file(&event);
    assert!(result.success, "学习应该成功");

    let gu = world.gu_registry().get(&gu_id).unwrap();
    println!("技能数: {}", gu.skills.len());
    println!("技能列表: {:?}", gu.skills.iter().map(|s| &s.name).collect::<Vec<_>>());
}
