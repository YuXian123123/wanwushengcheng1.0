//! 主题提取方法测试
//!
//! 比较不同推理方法从学习内容中提取中心主题的效果

use std::collections::{HashMap, HashSet};

/// 测试样本
struct TestCase {
    /// 文件名
    filename: &'static str,
    /// 内容摘要
    content: &'static str,
    /// 期望的主题
    expected_topic: &'static str,
    /// 期望的子主题
    expected_sub_topic: Option<&'static str>,
}

// ============================================================================
// 方法1: 抽象阶梯 (Abstraction Ladder) - da_clock
// ============================================================================

/// 从 da_clock 的抽象阶梯策略改编
/// 核心思想：识别模式，提升抽象层次
mod abstraction_ladder {
    use std::collections::HashMap;

    /// 识别内容中的模式
    pub fn identify_patterns(instances: &[&str]) -> Vec<String> {
        let mut word_counts: HashMap<String, usize> = HashMap::new();

        // 分词并统计词频
        for instance in instances {
            for word in instance.split_whitespace() {
                let word_lower = word.to_lowercase();
                // 过滤停用词
                if !is_stop_word(&word_lower) {
                    *word_counts.entry(word_lower).or_insert(0) += 1;
                }
            }
        }

        // 出现超过一次的词视为模式
        let threshold = (instances.len() as f32 * 0.5).ceil() as usize;
        let mut patterns: Vec<String> = word_counts
            .into_iter()
            .filter(|(_, count)| *count >= threshold.max(2))
            .map(|(word, _)| word)
            .collect();

        patterns.sort();
        patterns
    }

    /// 提升抽象层次
    pub fn abstract_up(instances: &[&str], current_level: usize) -> AbstractionLevel {
        let patterns = identify_patterns(instances);

        let description = if patterns.is_empty() {
            format!("第{}层抽象：无明显模式", current_level + 1)
        } else {
            format!("第{}层抽象：{}", current_level + 1, patterns.join(", "))
        };

        // 计算覆盖率
        let coverage = if instances.is_empty() {
            0.0
        } else {
            let covered = instances
                .iter()
                .filter(|inst| {
                    patterns
                        .iter()
                        .any(|p| inst.to_lowercase().contains(p))
                })
                .count();
            covered as f32 / instances.len() as f32
        };

        AbstractionLevel {
            level: current_level + 1,
            description,
            patterns,
            coverage,
        }
    }

    /// 从内容中提取主题（多层抽象）
    pub fn extract_topic(content: &str) -> (String, Option<String>) {
        // 将内容分割成句子/段落作为实例
        let instances: Vec<&str> = content
            .split(&['.', '!', '?', '。', '！', '？', '\n'][..])
            .filter(|s| !s.trim().is_empty())
            .collect();

        if instances.is_empty() {
            return ("未知".to_string(), None);
        }

        // 第一层抽象：识别低级模式
        let level1 = abstract_up(&instances, 0);

        // 第二层抽象：从模式中提取更高层次概念
        let level2 = abstract_up(
            &level1.patterns.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            1,
        );

        // 选择最优抽象层（覆盖率最高的层次）
        let best_level = if level2.coverage > level1.coverage && !level2.patterns.is_empty() {
            &level2
        } else {
            &level1
        };

        // 从模式中选择主主题
        let main_topic = best_level
            .patterns
            .first()
            .cloned()
            .unwrap_or_else(|| "未知".to_string());

        // 子主题
        let sub_topic = best_level.patterns.get(1).cloned();

        (main_topic, sub_topic)
    }

    fn is_stop_word(word: &str) -> bool {
        let stop_words = [
            "the", "a", "an", "is", "are", "was", "were", "be", "been", "being",
            "have", "has", "had", "do", "does", "did", "will", "would", "could",
            "should", "may", "might", "must", "shall", "can", "need", "dare",
            "ought", "used", "to", "of", "in", "for", "on", "with", "at", "by",
            "from", "as", "into", "through", "during", "before", "after",
            "above", "below", "between", "under", "again", "further", "then",
            "once", "here", "there", "when", "where", "why", "how", "all", "each",
            "few", "more", "most", "other", "some", "such", "no", "nor", "not",
            "only", "own", "same", "so", "than", "too", "very", "just", "and",
            "but", "if", "or", "because", "until", "while", "this", "that",
            "these", "those", "it", "its", "they", "them", "their", "what",
            "which", "who", "whom", "this", "that", "these", "those",
            // 中文停用词
            "的", "是", "在", "了", "和", "与", "或", "有", "被", "把",
            "这", "那", "这个", "那个", "这些", "那些", "我们", "你们",
            "他们", "它们", "它", "他", "她", "我", "你", "自己",
        ];
        stop_words.contains(&word)
    }

    pub struct AbstractionLevel {
        pub level: usize,
        pub description: String,
        pub patterns: Vec<String>,
        pub coverage: f32,
    }
}

// ============================================================================
// 方法2: 跨域类比 (Cross-Domain Analogy) - da_clock
// ============================================================================

mod cross_domain_analogy {
    use std::collections::{HashMap, HashSet};

    /// 领域知识库（预定义的领域中心词）
    const DOMAIN_CENTERS: &[(&str, &[&str])] = &[
        ("html", &["tag", "element", "attribute", "document", "html", "body", "head", "div", "span"]),
        ("css", &["style", "selector", "property", "value", "flex", "grid", "margin", "padding"]),
        ("javascript", &["function", "variable", "const", "let", "var", "async", "promise", "callback"]),
        ("rust", &["fn", "let", "mut", "struct", "impl", "trait", "borrow", "ownership"]),
        ("python", &["def", "class", "import", "self", "lambda", "list", "dict"]),
    ];

    /// 计算内容与领域的相似度（改进版：检查领域名本身）
    pub fn calculate_similarity(content: &str, domain: &str, domain_keywords: &[&str]) -> f32 {
        let content_lower = content.to_lowercase();

        let mut matches = 0;

        // 检查领域名称本身是否出现
        if content_lower.contains(&domain.to_lowercase()) {
            matches += 3; // 领域名本身出现，权重加3
        }

        // 检查关键词
        for keyword in domain_keywords {
            if content_lower.contains(keyword) {
                matches += 1;
            }
        }

        matches as f32 / (domain_keywords.len() + 3) as f32
    }

    /// 通过类比找到最相似的领域
    pub fn find_best_domain(content: &str) -> (String, f32) {
        let mut best_domain = "未知".to_string();
        let mut best_score = 0.0;

        for (domain, keywords) in DOMAIN_CENTERS {
            let score = calculate_similarity(content, domain, keywords);
            if score > best_score && score >= 0.15 {
                best_score = score;
                best_domain = domain.to_string();
            }
        }

        (best_domain, best_score)
    }

    /// 提取主题
    pub fn extract_topic(content: &str) -> (String, Option<String>) {
        let (domain, score) = find_best_domain(content);

        if score < 0.1 {
            return ("未知".to_string(), None);
        }

        // 尝试找到子主题
        let sub_topic = find_sub_topic(content, &domain);

        (domain, sub_topic)
    }

    fn find_sub_topic(content: &str, domain: &str) -> Option<String> {
        // 基于领域查找特定子主题关键词
        let sub_topics: HashMap<&str, Vec<(&str, &str)>> = [
            ("html", vec![("form", "表单"), ("table", "表格"), ("semantic", "语义化")]),
            ("css", vec![("flex", "flexbox"), ("grid", "grid布局"), ("animation", "动画")]),
            ("javascript", vec![("async", "异步"), ("dom", "DOM操作"), ("event", "事件")]),
        ].into_iter().collect();

        if let Some(topics) = sub_topics.get(domain) {
            for (keyword, name) in topics {
                if content.to_lowercase().contains(keyword) {
                    return Some(name.to_string());
                }
            }
        }

        None
    }
}

// ============================================================================
// 方法3: 信息效率评分 (Elegance Scorer) - da_clock 拉蒂奥设计
// ============================================================================

mod elegance_scorer {
    use std::collections::HashMap;

    /// 候选主题
    struct TopicCandidate {
        word: String,
        frequency: usize,
        position_score: f32,
        context_score: f32,
    }

    /// 评估候选主题的信息效率
    pub fn score_topic(candidate: &TopicCandidate, total_words: usize) -> f32 {
        // 频率得分：适度频率最优（不是太高也不是太低）
        let freq_score = {
            let ratio = candidate.frequency as f32 / total_words as f32;
            // 钟形曲线，最优在 0.01-0.1 之间
            if ratio < 0.01 {
                ratio * 100.0 // 低频词得分低
            } else if ratio > 0.1 {
                1.0 / (ratio * 10.0) // 过于高频的词得分也低
            } else {
                1.0 // 最优区间
            }
        };

        // 位置得分：出现在标题或开头更重要
        let pos_score = candidate.position_score;

        // 上下文得分：周围有相关词汇更重要
        let ctx_score = candidate.context_score;

        // 加权综合
        let total = freq_score * 0.3 + pos_score * 0.4 + ctx_score * 0.3;

        total.clamp(0.0, 1.0)
    }

    /// 从内容中提取主题候选
    pub fn extract_candidates(content: &str) -> Vec<TopicCandidate> {
        let mut candidates: HashMap<String, TopicCandidate> = HashMap::new();
        let words: Vec<&str> = content.split_whitespace().collect();
        let total = words.len();

        for (i, word) in words.iter().enumerate() {
            let word_lower = word.to_lowercase();

            // 过滤停用词和短词
            if word_lower.len() < 3 || is_stop_word(&word_lower) {
                continue;
            }

            let entry = candidates.entry(word_lower.clone()).or_insert(TopicCandidate {
                word: word_lower,
                frequency: 0,
                position_score: 0.0,
                context_score: 0.0,
            });

            entry.frequency += 1;

            // 位置得分：前10%的位置得分更高
            let position_ratio = i as f32 / total as f32;
            if position_ratio < 0.1 {
                entry.position_score = (entry.position_score + 1.0) / 2.0;
            } else if position_ratio < 0.3 {
                entry.position_score = (entry.position_score + 0.5) / 2.0;
            }
        }

        // 计算上下文得分
        // TODO: 实现更复杂的上下文分析

        candidates.into_values().collect()
    }

    /// 提取主题
    pub fn extract_topic(content: &str) -> (String, Option<String>) {
        let candidates = extract_candidates(content);
        let total_words = content.split_whitespace().count();

        if candidates.is_empty() {
            return ("未知".to_string(), None);
        }

        // 评分并排序
        let mut scored: Vec<(String, f32)> = candidates
            .iter()
            .map(|c| (c.word.clone(), score_topic(c, total_words)))
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let main_topic = scored.first().map(|(w, _)| w.clone()).unwrap_or("未知".to_string());
        let sub_topic = scored.get(1).map(|(w, _)| w.clone());

        (main_topic, sub_topic)
    }

    fn is_stop_word(word: &str) -> bool {
        let stop_words = [
            "the", "a", "an", "is", "are", "was", "were", "be", "been", "being",
            "have", "has", "had", "do", "does", "did", "will", "would", "could",
            "should", "may", "might", "must", "shall", "can", "to", "of", "in",
            "for", "on", "with", "at", "by", "from", "as", "and", "but", "if",
            "or", "this", "that", "it", "its", "they", "them", "their", "what",
            "which", "who", "的", "是", "在", "了", "和", "与", "或", "有",
        ];
        stop_words.contains(&word)
    }
}

// ============================================================================
// 方法4: 三天才裁决 (Trinity Decision) - 综合评估
// ============================================================================

mod trinity_decision {
    use super::*;

    /// 三天才裁决（改进权重分配）
    ///
    /// 权重分配：
    /// - 跨域类比（螺丝咕姆）：权重 0.5（最高，因为基于领域知识库）
    /// - 抽象阶梯（黑塔）：权重 0.3
    /// - 优雅评分（拉蒂奥）：权重 0.2
    pub fn decide_topic(
        content: &str,
        _filename: &str,
    ) -> (String, Option<String>) {
        // 获取三种方法的结果
        let (abs_topic, abs_sub) = abstraction_ladder::extract_topic(content);
        let (ana_topic, ana_sub) = cross_domain_analogy::extract_topic(content);
        let (ele_topic, ele_sub) = elegance_scorer::extract_topic(content);

        // 黑塔评分：创新性（代码语言识别）
        let black_tower_score = |topic: &str| -> f32 {
            let code_languages = ["html", "css", "javascript", "rust", "python", "java", "go"];
            let topic_lower = topic.to_lowercase();

            let mut score = 0.5;
            if code_languages.contains(&topic_lower.as_str()) {
                score = 0.9; // 代码语言，高分
            } else if content.to_lowercase().matches(&topic_lower).count() > 5 {
                score = 0.7; // 出现多次，中等分
            }
            score
        };

        // 螺丝咕姆评分：可信度（基于证据强度）
        let screwllum_score = |topic: &str, content: &str| -> f32 {
            // 检查主题在内容中出现的次数
            let count = content.to_lowercase().matches(topic).count();
            if count > 10 {
                0.9
            } else if count > 5 {
                0.7
            } else if count > 0 {
                0.5
            } else {
                0.1
            }
        };

        // 拉蒂奥评分：优雅度（简洁性）
        let latio_score = |topic: &str| -> f32 {
            // 短而精的主题得分更高
            let len = topic.len();
            if len <= 5 {
                0.9
            } else if len <= 10 {
                0.7
            } else {
                0.5
            }
        };

        // 计算加权得分
        let mut candidates: Vec<(String, Option<String>, f32, &str)> = Vec::new();

        // 黑塔评分（权重 0.3）
        let bt = black_tower_score(&abs_topic);
        candidates.push((abs_topic.clone(), abs_sub.clone(), bt * 0.3, "抽象阶梯"));

        // 螺丝咕姆评分（权重 0.5）- 最高权重
        let sc = screwllum_score(&ana_topic, content);
        candidates.push((ana_topic.clone(), ana_sub.clone(), sc * 0.5, "跨域类比"));

        // 拉蒂奥评分（权重 0.2）
        let la = latio_score(&ele_topic);
        candidates.push((ele_topic.clone(), ele_sub.clone(), la * 0.2, "优雅评分"));

        // 输出各候选得分
        for (topic, sub, weighted, method) in &candidates {
            println!(
                "  {} -> {} ({:.2})",
                method, topic, weighted
            );
        }

        // 选择得分最高的
        candidates.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        let best = candidates.first();
        best.map(|(topic, sub, _, _)| (topic.clone(), sub.clone()))
            .unwrap_or(("未知".to_string(), None))
    }
}

// ============================================================================
// 测试用例
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                filename: "html-basics.html",
                content: r#"
                    HTML Basics Tutorial
                    HTML is the standard markup language for Web pages.
                    HTML elements are the building blocks of HTML pages.
                    HTML elements are represented by tags.
                    Common HTML tags include: <html>, <head>, <body>, <div>, <span>, <p>, <a>.
                    HTML attributes provide additional information about elements.
                    The <!DOCTYPE html> declaration defines the document type.
                "#,
                expected_topic: "html",
                expected_sub_topic: None,
            },
            TestCase {
                filename: "css-flexbox-guide.html",
                content: r#"
                    CSS Flexbox Layout Guide
                    The Flexbox Layout module makes it easier to design flexible responsive layouts.
                    Flexbox is a one-dimensional layout method.
                    The flex container properties: flex-direction, flex-wrap, flex-flow, justify-content.
                    The flex item properties: order, flex-grow, flex-shrink, flex-basis.
                    CSS flexbox is essential for modern web design.
                "#,
                expected_topic: "css",
                expected_sub_topic: Some("flexbox"),
            },
            TestCase {
                filename: "javascript-async.html",
                content: r#"
                    JavaScript Asynchronous Programming
                    Asynchronous programming is essential in JavaScript.
                    async/await syntax makes asynchronous code look synchronous.
                    Promise is an object representing eventual completion of an async operation.
                    Callbacks are the oldest way to handle async operations.
                    JavaScript event loop manages the execution of code.
                "#,
                expected_topic: "javascript",
                expected_sub_topic: Some("async"),
            },
        ]
    }

    #[test]
    fn test_abstraction_ladder_method() {
        println!("\n=== 方法1: 抽象阶梯 ===\n");

        for case in get_test_cases() {
            let (topic, sub) = abstraction_ladder::extract_topic(case.content);
            println!(
                "文件: {} -> 主题: {}, 子主题: {:?}",
                case.filename, topic, sub
            );
            println!("期望: {}, {:?}", case.expected_topic, case.expected_sub_topic);
            println!();
        }
    }

    #[test]
    fn test_cross_domain_analogy_method() {
        println!("\n=== 方法2: 跨域类比 ===\n");

        for case in get_test_cases() {
            let (topic, sub) = cross_domain_analogy::extract_topic(case.content);
            println!(
                "文件: {} -> 主题: {}, 子主题: {:?}",
                case.filename, topic, sub
            );
            println!("期望: {}, {:?}", case.expected_topic, case.expected_sub_topic);
            println!();
        }
    }

    #[test]
    fn test_elegance_scorer_method() {
        println!("\n=== 方法3: 优雅评分 ===\n");

        for case in get_test_cases() {
            let (topic, sub) = elegance_scorer::extract_topic(case.content);
            println!(
                "文件: {} -> 主题: {}, 子主题: {:?}",
                case.filename, topic, sub
            );
            println!("期望: {}, {:?}", case.expected_topic, case.expected_sub_topic);
            println!();
        }
    }

    #[test]
    fn test_trinity_decision_method() {
        println!("\n=== 方法4: 三天才裁决 ===\n");

        for case in get_test_cases() {
            println!("文件: {}", case.filename);
            let (topic, sub) = trinity_decision::decide_topic(case.content, case.filename);
            println!("最终结果 -> 主题: {}, 子主题: {:?}", topic, sub);
            println!("期望: {}, {:?}", case.expected_topic, case.expected_sub_topic);
            println!();
        }
    }

    #[test]
    fn test_all_methods_comparison() {
        println!("\n=== 所有方法对比 ===\n");

        for case in get_test_cases() {
            println!("━━━ {} ━━━", case.filename);
            println!("期望: {} ({:?})", case.expected_topic, case.expected_sub_topic);

            let (abs_topic, _) = abstraction_ladder::extract_topic(case.content);
            let (ana_topic, ana_sub) = cross_domain_analogy::extract_topic(case.content);
            let (ele_topic, _) = elegance_scorer::extract_topic(case.content);
            let (tri_topic, tri_sub) = trinity_decision::decide_topic(case.content, case.filename);

            println!("  抽象阶梯:   {}", abs_topic);
            println!("  跨域类比:   {} ({:?})", ana_topic, ana_sub);
            println!("  优雅评分:   {}", ele_topic);
            println!("  三天才裁决: {} ({:?})", tri_topic, tri_sub);
            println!();
        }
    }
}
