//! 认知素分解器 - 简化版（从大时钟移植）
//!
//! 将知识内容分解为认知粒子：
//! - Entity（实体）：概念、术语、关键词
//! - Attribute（属性）：特征、属性值
//! - Relation（关系）：实体之间的关联
//!
//! 这些认知粒子将转化为神经信号输入到 LNN

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 认知粒子类型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CogniParticle {
    /// 实体：具有唯一标识的概念
    Entity {
        id: u64,
        name: String,
        /// 实体类型（如：技术、概念、属性名）
        entity_type: EntityType,
    },
    /// 属性：实体的特征
    Attribute {
        target_id: u64,
        key: String,
        value: String,
    },
    /// 关系：实体之间的关联
    Relation {
        source_id: u64,
        target_id: u64,
        rel_type: RelationType,
    },
}

/// 实体类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    /// 技术术语（如 HTML, CSS, JavaScript）
    TechTerm,
    /// 概念（如 元素, 标签, 属性）
    Concept,
    /// 代码块语言
    CodeLanguage,
    /// 关键词
    Keyword,
    /// 其他
    Other,
}

/// 关系类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationType {
    /// 包含关系
    Contains,
    /// 属于关系
    BelongsTo,
    /// 依赖关系
    DependsOn,
    /// 相似关系
    SimilarTo,
    /// 关联关系
    RelatedTo,
}

/// 认知素解析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResult {
    /// 提取的认知粒子
    pub particles: Vec<CogniParticle>,
    /// 实体名称到 ID 的映射
    pub entity_map: HashMap<String, u64>,
    /// 主要主题（得分最高的实体）
    pub main_topic: Option<String>,
    /// 关键词列表
    pub keywords: Vec<String>,
    /// 代码语言列表
    pub code_languages: Vec<String>,
}

/// 认知素分解器
#[derive(Debug, Clone)]
pub struct CognisParser {
    /// 实体 ID 计数器
    next_id: u64,
    /// 已识别的实体名称到 ID 的映射
    entity_map: HashMap<String, u64>,
    /// 停用词列表
    stop_words: Vec<String>,
    /// 技术领域关键词
    tech_keywords: HashMap<String, EntityType>,
}

impl CognisParser {
    /// 创建新的解析器
    pub fn new() -> Self {
        Self {
            next_id: 1,
            entity_map: HashMap::new(),
            stop_words: Self::default_stop_words(),
            tech_keywords: Self::default_tech_keywords(),
        }
    }

    /// 默认停用词
    fn default_stop_words() -> Vec<String> {
        vec![
            // 英文
            "the", "a", "an", "is", "are", "was", "were", "be", "been", "being",
            "have", "has", "had", "do", "does", "did", "will", "would", "could",
            "should", "may", "might", "must", "shall", "can", "to", "of", "in",
            "for", "on", "with", "at", "by", "from", "as", "and", "but", "or",
            "not", "this", "that", "it", "its", "they", "them", "their", "what",
            "which", "who", "how", "when", "where", "why", "all", "each", "every",
            "both", "few", "more", "most", "other", "some", "such", "no", "nor",
            "only", "own", "same", "so", "than", "too", "very", "just", "also",
            // 中文
            "的", "是", "在", "了", "和", "与", "或", "有", "被", "把",
            "这", "那", "这个", "那个", "这些", "那些", "我们", "你们",
            "他们", "它们", "它", "他", "她", "我", "你", "自己", "什么",
            "怎么", "如何", "为什么", "因为", "所以", "但是", "然后", "如果",
        ].into_iter().map(|s| s.to_string()).collect()
    }

    /// 默认技术关键词
    fn default_tech_keywords() -> HashMap<String, EntityType> {
        let mut map = HashMap::new();

        // 编程语言
        for lang in &["html", "css", "javascript", "js", "typescript", "ts", "rust", "python", "java", "go", "cpp", "c"] {
            map.insert(lang.to_string(), EntityType::CodeLanguage);
        }

        // Web 技术
        for tech in &["dom", "api", "http", "json", "xml", "ajax", "fetch", "rest", "graphql"] {
            map.insert(tech.to_string(), EntityType::TechTerm);
        }

        // 通用概念
        for concept in &["element", "attribute", "tag", "property", "value", "function",
                         "variable", "class", "object", "array", "string", "number",
                         "method", "parameter", "argument", "return", "async", "await",
                         "promise", "callback", "event", "listener", "handler"] {
            map.insert(concept.to_string(), EntityType::Concept);
        }

        map
    }

    /// 解析内容，提取认知粒子
    pub fn parse(&mut self, content: &str) -> ParseResult {
        let mut particles: Vec<CogniParticle> = Vec::new();
        self.entity_map.clear();

        // 1. 提取代码语言
        let code_languages = self.extract_code_languages(content);
        for lang in &code_languages {
            let id = self.get_or_create_entity(lang, EntityType::CodeLanguage);
            particles.push(CogniParticle::Entity {
                id,
                name: lang.clone(),
                entity_type: EntityType::CodeLanguage,
            });
        }

        // 2. 提取标题关键词
        let title_words = self.extract_title_words(content);
        for word in &title_words {
            let entity_type = self.tech_keywords.get(&word.to_lowercase())
                .copied()
                .unwrap_or(EntityType::Keyword);
            let id = self.get_or_create_entity(word, entity_type);
            particles.push(CogniParticle::Entity {
                id,
                name: word.clone(),
                entity_type,
            });
        }

        // 3. 提取高频词
        let frequent_words = self.extract_frequent_words(content, 10);
        for (word, count) in &frequent_words {
            if *count >= 2 {
                let entity_type = self.tech_keywords.get(&word.to_lowercase())
                    .copied()
                    .unwrap_or(EntityType::Other);
                let id = self.get_or_create_entity(word, entity_type);
                particles.push(CogniParticle::Entity {
                    id,
                    name: word.clone(),
                    entity_type,
                });
            }
        }

        // 4. 建立实体间关系
        let entity_ids: Vec<u64> = particles.iter()
            .filter_map(|p| match p {
                CogniParticle::Entity { id, .. } => Some(*id),
                _ => None,
            })
            .collect();

        // 相邻实体建立关联
        for i in 0..entity_ids.len().saturating_sub(1) {
            particles.push(CogniParticle::Relation {
                source_id: entity_ids[i],
                target_id: entity_ids[i + 1],
                rel_type: RelationType::RelatedTo,
            });
        }

        // 5. 确定主要主题
        let main_topic = self.determine_main_topic(&particles, &frequent_words);

        // 6. 提取关键词列表
        let keywords = frequent_words.iter()
            .filter(|(_, count)| *count >= 2)
            .map(|(word, _)| word.clone())
            .take(5)
            .collect();

        ParseResult {
            particles,
            entity_map: self.entity_map.clone(),
            main_topic,
            keywords,
            code_languages,
        }
    }

    /// 从代码块中提取语言标识
    fn extract_code_languages(&self, content: &str) -> Vec<String> {
        let mut languages = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("```") {
                let lang = trimmed.trim_start_matches('`').trim();
                if !lang.is_empty() && lang.len() <= 20 && lang.chars().all(|c| c.is_alphanumeric()) {
                    let normalized = self.normalize_word(lang);
                    if !languages.contains(&normalized) {
                        languages.push(normalized);
                    }
                }
            }
            if languages.len() >= 3 {
                break;
            }
        }

        languages
    }

    /// 从 Markdown 标题中提取关键词
    fn extract_title_words(&self, content: &str) -> Vec<String> {
        let mut words = Vec::new();

        for line in content.lines().take(50) {
            let trimmed = line.trim();
            if trimmed.starts_with('#') {
                let title = trimmed.trim_start_matches('#').trim();
                for word in title.split_whitespace() {
                    let word = word.trim_matches(|c: char| !c.is_alphanumeric());
                    if word.len() >= 2 && word.len() <= 30 {
                        // 检查是否有大写字母（可能是专有名词）
                        let has_upper = word.chars().any(|c| c.is_uppercase());
                        let is_alnum = word.chars().all(|c| c.is_ascii_alphanumeric() || c == '-');
                        if (has_upper || word.len() <= 10) && is_alnum {
                            let normalized = self.normalize_word(word);
                            if !self.is_stop_word(&normalized) && !words.contains(&normalized) {
                                words.push(normalized);
                            }
                        }
                    }
                }
            }
        }

        words.truncate(5);
        words
    }

    /// 提取高频词
    fn extract_frequent_words(&self, content: &str, top_n: usize) -> Vec<(String, usize)> {
        let mut word_count: HashMap<String, usize> = HashMap::new();
        let content_preview: String = content.chars().take(3000).collect();

        for word in content_preview.split(|c: char| c.is_whitespace() || c.is_ascii_punctuation()) {
            let word = word.trim();
            if word.len() < 2 || word.len() > 30 {
                continue;
            }

            let word_lower = word.to_lowercase();
            if self.is_stop_word(&word_lower) {
                continue;
            }

            // 只保留有意义的词
            let has_alpha = word.chars().any(|c| c.is_alphabetic());
            if has_alpha {
                let normalized = self.normalize_word(word);
                *word_count.entry(normalized).or_insert(0) += 1;
            }
        }

        let mut sorted: Vec<(String, usize)> = word_count.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        sorted.truncate(top_n);
        sorted
    }

    /// 获取或创建实体 ID
    fn get_or_create_entity(&mut self, name: &str, entity_type: EntityType) -> u64 {
        let name_lower = name.to_lowercase();
        if let Some(&id) = self.entity_map.get(&name_lower) {
            return id;
        }

        let id = self.next_id;
        self.next_id += 1;
        self.entity_map.insert(name_lower, id);
        id
    }

    /// 判断是否为停用词
    fn is_stop_word(&self, word: &str) -> bool {
        let word_lower = word.to_lowercase();
        self.stop_words.contains(&word_lower) || word.len() < 2
    }

    /// 规范化单词（首字母大写）
    fn normalize_word(&self, word: &str) -> String {
        let mut chars = word.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => {
                let first_upper = first.to_uppercase().to_string();
                let rest: String = chars.map(|c| c.to_lowercase().next().unwrap_or(c)).collect();
                format!("{}{}", first_upper, rest)
            }
        }
    }

    /// 确定主要主题
    fn determine_main_topic(
        &self,
        particles: &[CogniParticle],
        frequent_words: &[(String, usize)],
    ) -> Option<String> {
        // 优先返回代码语言
        for particle in particles {
            if let CogniParticle::Entity { name, entity_type: EntityType::CodeLanguage, .. } = particle {
                return Some(name.clone());
            }
        }

        // 其次返回技术术语
        for particle in particles {
            if let CogniParticle::Entity { name, entity_type: EntityType::TechTerm, .. } = particle {
                return Some(name.clone());
            }
        }

        // 最后返回最高频词
        frequent_words.first().map(|(word, _)| word.clone())
    }
}

impl Default for CognisParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_html_content() {
        let content = r#"
# HTML Basics Tutorial

HTML is the standard markup language for Web pages.

```html
<div class="container">
  <p>Hello World</p>
</div>
```

HTML elements are the building blocks of HTML pages.
        "#;

        let mut parser = CognisParser::new();
        let result = parser.parse(content);

        // 应该识别出 HTML 语言
        assert!(result.code_languages.contains(&"Html".to_string()));

        // 应该有实体
        assert!(!result.particles.is_empty());

        // 应该有主要主题
        assert!(result.main_topic.is_some());
    }

    #[test]
    fn test_parse_code_languages() {
        let content = r#"
# JavaScript Guide

```javascript
const x = 1;
```

```css
.container { color: red; }
```
        "#;

        let mut parser = CognisParser::new();
        let result = parser.parse(content);

        assert!(result.code_languages.contains(&"Javascript".to_string()));
        assert!(result.code_languages.contains(&"Css".to_string()));
    }

    #[test]
    fn test_entity_extraction() {
        let content = "HTML elements have attributes. Elements contain tags.";
        let mut parser = CognisParser::new();
        let result = parser.parse(content);

        // 应该提取到实体
        let entities: Vec<_> = result.particles.iter()
            .filter(|p| matches!(p, CogniParticle::Entity { .. }))
            .collect();
        assert!(!entities.is_empty());
    }

    #[test]
    fn test_relation_extraction() {
        // 使用代码块来确保有多个实体
        let content = r#"
# HTML and CSS Guide

```html
<div>Hello</div>
```

```css
.container { color: red; }
```

HTML elements have attributes. HTML tags are building blocks.
CSS styles HTML elements.
        "#;
        let mut parser = CognisParser::new();
        let result = parser.parse(content);

        // 应该有实体（HTML 和 CSS 都应该被识别为代码语言）
        let entities: Vec<_> = result.particles.iter()
            .filter(|p| matches!(p, CogniParticle::Entity { .. }))
            .collect();
        assert!(!entities.is_empty(), "应该有实体");

        // 应该有关系（相邻实体建立关系）
        let relations: Vec<_> = result.particles.iter()
            .filter(|p| matches!(p, CogniParticle::Relation { .. }))
            .collect();
        assert!(!relations.is_empty(), "应该有关系");
    }
}
