//! 学习器模块
//!
//! 负责递归读取目录文件，解析格式，发送给世界模型
//!
//! # 文件解析流程
//!
//! ```text
//! 文件检测 → 格式分类 → 解析器插件 → Markdown → 学习
//! ```
//!
//! # 支持的文件格式
//!
//! - **通用格式**: md, txt, html, js, css, java, py, rs, etc.
//! - **特殊格式**: parquet, pdf, docx (需要对应 feature 或插件)

use std::path::Path;
use std::fs;
use uuid::Uuid;
use super::parser::ParserRegistry;
use super::protocol::{KnowledgeFileEvent, HernessCommand, WorldMindResponse, WorldMindMessage};

/// 学习器
pub struct Learner {
    /// 当前批次 ID
    batch_id: String,
    /// 已处理的文件数
    files_processed: usize,
    /// 总文件数
    total_files: usize,
    /// 是否已熔断
    halted: bool,
    /// 熔断原因
    halt_reason: Option<String>,
    /// 解析器注册表
    parser_registry: ParserRegistry,
}

impl Learner {
    /// 创建新学习器
    pub fn new() -> Self {
        Self {
            batch_id: Uuid::new_v4().to_string(),
            files_processed: 0,
            total_files: 0,
            halted: false,
            halt_reason: None,
            parser_registry: ParserRegistry::new(),
        }
    }

    /// 获取支持的文件扩展名
    pub fn supported_extensions(&self) -> Vec<String> {
        self.parser_registry.supported_extensions()
    }

    /// 检查是否已熔断
    pub fn is_halted(&self) -> bool {
        self.halted
    }

    /// 熔断
    pub fn halt(&mut self, reason: String) {
        self.halted = true;
        self.halt_reason = Some(reason);
    }

    /// 获取使用说明书内容
    pub fn load_manual() -> Option<String> {
        // 尝试多个可能的位置
        let paths = [
            "herness-web/HERNESS_MANUAL.md",
            "src/herness-web/HERNESS_MANUAL.md",
            "../herness-web/HERNESS_MANUAL.md",
        ];

        for path in &paths {
            if let Ok(content) = fs::read_to_string(path) {
                return Some(content);
            }
        }

        // 如果都找不到，返回内嵌的说明书
        Some(Self::embedded_manual())
    }

    /// 内嵌的说明书（作为后备）
    fn embedded_manual() -> String {
        r#"# Herness 使用说明书

## 你是谁

你是 WorldMind，一个由蛊虫智能体构成的世界神经网络。

## 你能做什么

### 学习目录
```
LearnDirectory { path: "D:\\训练数据\\aireader\\html", extensions: [".md", ".txt"] }
```

### 学习单文件
```
LearnFile { path: "D:\\文档\\知识.md" }
```

### 发送消息
```
SendMessage { content: "消息内容" }
```

### 停止学习
```
Halt { reason: "原因" }
```

## 学习流程

1. 理解说明书后，自主决定学习什么
2. 调用 LearnDirectory 开始学习
3. 文件逐个发送给你，内部分配给蛊虫
4. 觉得足够时调用 Halt 停止

现在，你可以开始行动了。
"#.to_string()
    }

    /// 递归扫描目录，返回所有文件路径
    /// 支持单个文件或目录
    pub fn scan_directory(
        &self,
        dir_path: &str,
        extensions: &[String],
    ) -> Result<Vec<std::path::PathBuf>, String> {
        let path = Path::new(dir_path);
        if !path.exists() {
            return Err(format!("路径不存在: {}", dir_path));
        }

        // 如果是单个文件，直接返回
        if path.is_file() {
            // 检查扩展名
            let ext = path.extension()
                .map(|e| e.to_string_lossy().to_lowercase())
                .unwrap_or_default();

            // 如果没有指定扩展名过滤，或者扩展名匹配
            if extensions.is_empty() || extensions.iter().any(|e| e.to_lowercase() == ext) {
                // 检查是否支持该格式
                if self.parser_registry.find_parser(&ext).is_some() {
                    return Ok(vec![path.to_path_buf()]);
                } else {
                    return Err(format!("不支持的文件格式: {}", ext));
                }
            } else {
                return Ok(vec![]);
            }
        }

        if !path.is_dir() {
            return Err(format!("不是文件或目录: {}", dir_path));
        }

        let mut files = Vec::new();
        self.scan_directory_recursive(path, extensions, &mut files)?;
        Ok(files)
    }

    /// 递归扫描
    fn scan_directory_recursive(
        &self,
        dir: &Path,
        extensions: &[String],
        files: &mut Vec<std::path::PathBuf>,
    ) -> Result<(), String> {
        let entries = fs::read_dir(dir)
            .map_err(|e| format!("读取目录失败: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("读取条目失败: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                // 跳过隐藏目录和常见的忽略目录
                if let Some(name) = path.file_name() {
                    let name = name.to_string_lossy();
                    if name.starts_with('.') || name == "node_modules" || name == "target" {
                        continue;
                    }
                }
                self.scan_directory_recursive(&path, extensions, files)?;
            } else if path.is_file() {
                // 检查扩展名
                let ext = path.extension()
                    .map(|e| e.to_string_lossy().to_lowercase())
                    .unwrap_or_default();

                let ext_with_dot = format!(".{}", ext);

                if extensions.is_empty() || extensions.iter().any(|e| e.to_lowercase() == ext || e.to_lowercase() == ext_with_dot) {
                    files.push(path);
                }
            }
        }

        Ok(())
    }

    /// 读取文件并生成 KnowledgeFileEvent
    ///
    /// 使用解析器插件系统自动检测文件类型并转换为 Markdown
    pub fn read_file(
        &mut self,
        file_path: &std::path::Path,
        root_dir: &str,
        total: usize,
    ) -> Result<KnowledgeFileEvent, String> {
        let filename = file_path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        let extension = file_path.extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        let relative_path = file_path.strip_prefix(root_dir)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| file_path.to_string_lossy().to_string());

        // 使用解析器系统解析文件
        let parsed = self.parser_registry.parse_file(file_path)
            .map_err(|e| format!("解析文件失败: {}", e))?;

        let content = parsed.content;
        let size = content.len();

        self.files_processed += 1;

        Ok(KnowledgeFileEvent {
            path: file_path.to_string_lossy().to_string(),
            filename,
            extension,
            content,
            size,
            relative_path,
            batch_id: self.batch_id.clone(),
            index: self.files_processed,
            total,
        })
    }

    /// 处理世界模型的响应
    pub fn handle_response(&mut self, response: &WorldMindResponse) -> Option<HernessCommand> {
        match response {
            WorldMindResponse::Halt { reason, .. } => {
                self.halt(reason.clone());
                None
            }
            WorldMindResponse::ToolCall { command, .. } => {
                Some(command.clone())
            }
            _ => None,
        }
    }

    /// 获取统计信息
    pub fn stats(&self) -> (usize, usize) {
        (self.files_processed, self.total_files)
    }
}

impl Default for Learner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_directory() {
        let learner = Learner::new();
        let result = learner.scan_directory("src", &["rs".to_string()]);
        assert!(result.is_ok());
        let files = result.unwrap();
        assert!(!files.is_empty());
    }

    #[test]
    fn test_load_manual() {
        let manual = Learner::load_manual();
        assert!(manual.is_some());
        assert!(manual.unwrap().contains("Herness"));
    }
}
