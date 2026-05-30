//! 递归学习模块
//!
//! 从知识文档递归学习，构建概念层次结构
//!
//! # 设计理念
//!
//! 根据"万物生成器"设计：
//! - 能力自生长（非程序写死）
//! - 数据驱动，蛊虫通过学习数据获得能力
//! - 知识闭环：理解→交流→结构化→回流

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::config::GlobalConfig;
use crate::language::concept::{ConceptSpace, ConceptLevel, ConceptVector};
use crate::language::encoder::Encoder;
use crate::language::decoder::Decoder;

/// 知识元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeMetadata {
    /// 总知识点数
    pub total_points: usize,
    /// 抓取时间
    #[serde(default)]
    pub crawl_time: String,
    /// 来源
    #[serde(default)]
    pub source: String,
    /// 分类及知识点数量
    pub categories: HashMap<String, usize>,
}

/// 知识文档
#[derive(Debug, Clone)]
pub struct KnowledgeDocument {
    /// 文件路径
    pub path: PathBuf,
    /// 分类
    pub category: String,
    /// 标题
    pub title: String,
    /// 内容
    pub content: String,
    /// 代码块
    pub code_blocks: Vec<String>,
    /// 关键概念
    pub key_concepts: Vec<String>,
}

/// 学习结果
#[derive(Debug, Clone)]
pub struct LearningResult {
    /// 创建的概念数量
    pub concepts_created: usize,
    /// 创建的关系数量
    pub relations_created: usize,
    /// 学习的知识点数
    pub points_learned: usize,
    /// 能力列表
    pub capabilities: Vec<String>,
    /// 错误列表
    pub errors: Vec<String>,
}

/// 递归学习器
pub struct RecursiveLearner {
    /// 概念空间
    pub concept_space: ConceptSpace,
    /// 编码器
    encoder: Encoder,
    /// 解码器
    decoder: Decoder,
    /// 配置
    config: GlobalConfig,
    /// 已学习的文档
    learned_docs: HashMap<String, KnowledgeDocument>,
}

impl RecursiveLearner {
    /// 创建新的递归学习器
    pub fn new() -> Self {
        Self {
            concept_space: ConceptSpace::new(),
            encoder: Encoder::new(),
            decoder: Decoder::new(),
            config: GlobalConfig::new(),
            learned_docs: HashMap::new(),
        }
    }

    /// 从目录学习知识
    ///
    /// # 参数
    /// - `dir_path`: 知识目录路径
    ///
    /// # 返回
    /// 学习结果
    pub fn learn_from_directory(&mut self, dir_path: &str) -> LearningResult {
        let mut result = LearningResult {
            concepts_created: 0,
            relations_created: 0,
            points_learned: 0,
            capabilities: Vec::new(),
            errors: Vec::new(),
        };

        let path = Path::new(dir_path);
        if !path.exists() {
            result.errors.push(format!("目录不存在: {}", dir_path));
            return result;
        }

        // 1. 读取元数据
        let metadata_path = path.join("metadata.json");
        let metadata = if metadata_path.exists() {
            match fs::read_to_string(&metadata_path) {
                Ok(content) => match serde_json::from_str::<KnowledgeMetadata>(&content) {
                    Ok(m) => Some(m),
                    Err(e) => {
                        result.errors.push(format!("解析元数据失败: {}", e));
                        None
                    }
                },
                Err(e) => {
                    result.errors.push(format!("读取元数据失败: {}", e));
                    None
                }
            }
        } else {
            None
        };

        // 2. 获取领域名称（目录名）
        let domain_name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // 3. 创建领域根概念
        match self.concept_space.create_concept(
            domain_name.clone(),
            domain_name.clone(),
            ConceptLevel::Basic,
        ) {
            Ok(_) => result.concepts_created += 1,
            Err(e) => result.errors.push(format!("创建领域概念失败: {}", e)),
        }

        // 4. 遍历分类目录
        if let Some(ref meta) = metadata {
            for (category, _point_count) in &meta.categories {
                let category_path = path.join(category);
                if category_path.exists() {
                    let category_result = self.learn_category(
                        &category_path,
                        &domain_name,
                        category,
                    );
                    result.concepts_created += category_result.concepts_created;
                    result.relations_created += category_result.relations_created;
                    result.points_learned += category_result.points_learned;
                    result.errors.extend(category_result.errors);
                }
            }
        } else {
            // 没有元数据，直接遍历所有文件
            self.learn_all_files(path, &domain_name, &mut result);
        }

        // 5. 生成能力描述
        result.capabilities = self.generate_capabilities(&domain_name);

        result
    }

    /// 学习一个分类
    fn learn_category(
        &mut self,
        category_path: &Path,
        domain: &str,
        category: &str,
    ) -> LearningResult {
        let mut result = LearningResult {
            concepts_created: 0,
            relations_created: 0,
            points_learned: 0,
            capabilities: Vec::new(),
            errors: Vec::new(),
        };

        // 创建分类概念
        let category_concept_id = format!("{}_{}", domain, category);
        match self.concept_space.create_child_concept(
            domain,
            category_concept_id.clone(),
            category.to_string(),
        ) {
            Ok(_) => result.concepts_created += 1,
            Err(_) => {}, // 可能已存在
        }

        // 遍历该分类下的所有文档
        if let Ok(entries) = fs::read_dir(category_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "md").unwrap_or(false) {
                    match self.learn_document(&path, &category_concept_id) {
                        Ok(doc_result) => {
                            result.concepts_created += doc_result.0;
                            result.points_learned += doc_result.1;
                        }
                        Err(e) => result.errors.push(e),
                    }
                }
            }
        }

        result
    }

    /// 学习所有文件（无元数据情况）
    fn learn_all_files(
        &mut self,
        path: &Path,
        domain: &str,
        result: &mut LearningResult,
    ) {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    // 递归处理子目录
                    let sub_category = entry_path.file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();
                    let sub_result = self.learn_category(&entry_path, domain, &sub_category);
                    result.concepts_created += sub_result.concepts_created;
                    result.relations_created += sub_result.relations_created;
                    result.points_learned += sub_result.points_learned;
                    result.errors.extend(sub_result.errors);
                } else if entry_path.extension().map(|e| e == "md").unwrap_or(false) {
                    match self.learn_document(&entry_path, domain) {
                        Ok(doc_result) => {
                            result.concepts_created += doc_result.0;
                            result.points_learned += doc_result.1;
                        }
                        Err(e) => result.errors.push(e),
                    }
                }
            }
        }
    }

    /// 学习单个文档
    fn learn_document(
        &mut self,
        path: &Path,
        parent_concept: &str,
    ) -> Result<(usize, usize), String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("读取文档失败: {}", e))?;

        let doc = self.parse_document(path, &content);

        // 创建文档概念
        let doc_concept_id = format!("{}_{}", parent_concept, doc.title.replace(' ', "_"));

        let created = if self.concept_space.get_concept(&doc_concept_id).is_none() {
            match self.concept_space.create_child_concept(
                parent_concept,
                doc_concept_id.clone(),
                doc.title.clone(),
            ) {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            false
        };

        // 学习文档中的关键概念
        let mut concepts_count = if created { 1 } else { 0 };

        for concept in &doc.key_concepts {
            let concept_id = format!("{}_{}", doc_concept_id, concept.replace(' ', "_"));
            if self.concept_space.get_concept(&concept_id).is_none() {
                if self.concept_space.create_child_concept(
                    &doc_concept_id,
                    concept_id,
                    concept.clone(),
                ).is_ok() {
                    concepts_count += 1;
                }
            }
        }

        // 关联代码块
        for (i, code) in doc.code_blocks.iter().enumerate() {
            let code_id = format!("{}_code_{}", doc_concept_id, i);
            if self.concept_space.get_concept(&code_id).is_none() {
                if self.concept_space.create_child_concept(
                    &doc_concept_id,
                    code_id,
                    format!("代码示例_{}", i),
                ).is_ok() {
                    concepts_count += 1;
                }
            }
        }

        // 存储文档
        let doc_id = path.to_string_lossy().to_string();
        self.learned_docs.insert(doc_id, doc);

        Ok((concepts_count, 1))
    }

    /// 解析文档
    fn parse_document(&self, path: &Path, content: &str) -> KnowledgeDocument {
        let title = path.file_stem()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "未知文档".to_string());

        // 提取分类（从路径）
        let category = path.parent()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        // 提取代码块
        let code_blocks = self.extract_code_blocks(content);

        // 提取关键概念（简化：提取标题和列表项）
        let key_concepts = self.extract_key_concepts(content);

        KnowledgeDocument {
            path: path.to_path_buf(),
            category,
            title,
            content: content.to_string(),
            code_blocks,
            key_concepts,
        }
    }

    /// 提取代码块
    fn extract_code_blocks(&self, content: &str) -> Vec<String> {
        let mut blocks = Vec::new();
        let mut in_code = false;
        let mut current_block = String::new();

        for line in content.lines() {
            if line.starts_with("```") {
                if in_code {
                    if !current_block.is_empty() {
                        blocks.push(current_block.clone());
                        current_block.clear();
                    }
                    in_code = false;
                } else {
                    in_code = true;
                }
            } else if in_code {
                current_block.push_str(line);
                current_block.push('\n');
            }
        }

        blocks
    }

    /// 提取关键概念
    fn extract_key_concepts(&self, content: &str) -> Vec<String> {
        let mut concepts = Vec::new();

        for line in content.lines() {
            // 提取标题
            if line.starts_with("## ") {
                concepts.push(line[3..].trim().to_string());
            }
            // 提取列表项
            else if line.starts_with("- `") || line.starts_with("- **") {
                let text = line[2..].trim();
                // 提取反引号或粗体中的内容
                if text.starts_with('`') {
                    if let Some(end) = text[1..].find('`') {
                        concepts.push(text[1..=end].to_string());
                    }
                } else if text.starts_with("**") {
                    if let Some(end) = text[2..].find("**") {
                        concepts.push(text[2..end+2].to_string());
                    }
                }
            }
        }

        concepts
    }

    /// 生成能力描述
    fn generate_capabilities(&self, domain: &str) -> Vec<String> {
        let mut capabilities = Vec::new();

        // 检查概念空间中的概念
        if self.concept_space.get_concept(domain).is_some() {
            capabilities.push(format!("理解{}的基本概念", domain));
        }

        // 检查是否有代码示例
        let code_count = self.learned_docs.values()
            .map(|d| d.code_blocks.len())
            .sum::<usize>();
        if code_count > 0 {
            capabilities.push(format!("能够生成{}代码示例（{}个已学习）", domain, code_count));
        }

        // 检查关键概念
        let concept_count = self.learned_docs.values()
            .map(|d| d.key_concepts.len())
            .sum::<usize>();
        if concept_count > 0 {
            capabilities.push(format!("掌握{}个{}相关知识概念", concept_count, domain));
        }

        capabilities
    }

    /// 测试能力：生成代码
    pub fn test_generate_code(&self, domain: &str) -> Option<String> {
        // 查找该领域下的代码示例
        for doc in self.learned_docs.values() {
            if doc.path.to_string_lossy().contains(domain) && !doc.code_blocks.is_empty() {
                return Some(doc.code_blocks[0].clone());
            }
        }
        None
    }

    /// 测试能力：回答问题
    pub fn test_answer_question(&self, question: &str, domain: &str) -> Option<String> {
        // 简化实现：在已学习文档中搜索相关内容
        for doc in self.learned_docs.values() {
            if doc.path.to_string_lossy().contains(domain) {
                // 检查问题是否与文档标题或概念相关
                for concept in &doc.key_concepts {
                    if question.contains(concept) || concept.contains(question) {
                        return Some(doc.content.clone());
                    }
                }
            }
        }
        None
    }

    /// 获取概念空间（用于测试）
    pub fn get_concept_space(&self) -> &ConceptSpace {
        &self.concept_space
    }

    /// 获取已学习文档数量
    pub fn learned_document_count(&self) -> usize {
        self.learned_docs.len()
    }
}

impl Default for RecursiveLearner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_learner() {
        let learner = RecursiveLearner::new();
        assert_eq!(learner.learned_document_count(), 0);
    }

    #[test]
    fn test_extract_code_blocks() {
        let learner = RecursiveLearner::new();
        let content = r#"
Some text
```html
<div>Hello</div>
```
More text
"#;
        let blocks = learner.extract_code_blocks(content);
        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].contains("div"));
    }

    #[test]
    fn test_extract_key_concepts() {
        let learner = RecursiveLearner::new();
        let content = r#"
## 基本结构
- `html` 根元素
- **body** 主体
"#;
        let concepts = learner.extract_key_concepts(content);
        assert!(concepts.contains(&"基本结构".to_string()));
    }
}
