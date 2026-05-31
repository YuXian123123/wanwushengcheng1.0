//! 学习器模块
//!
//! 负责递归读取目录文件，发送给世界模型

use std::path::Path;
use std::fs;
use uuid::Uuid;
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
        }
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
    pub fn scan_directory(
        &self,
        dir_path: &str,
        extensions: &[String],
    ) -> Result<Vec<std::path::PathBuf>, String> {
        let path = Path::new(dir_path);
        if !path.exists() {
            return Err(format!("目录不存在: {}", dir_path));
        }
        if !path.is_dir() {
            return Err(format!("不是目录: {}", dir_path));
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
    pub fn read_file(
        &mut self,
        file_path: &std::path::Path,
        root_dir: &str,
        total: usize,
    ) -> Result<KnowledgeFileEvent, String> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("读取文件失败: {}", e))?;

        let filename = file_path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        let extension = file_path.extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default();

        let relative_path = file_path.strip_prefix(root_dir)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| file_path.to_string_lossy().to_string());

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
