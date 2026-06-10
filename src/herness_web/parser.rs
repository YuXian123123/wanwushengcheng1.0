//! 文件解析器插件系统
//!
//! # 架构设计
//!
//! ```text
//! 文件检测 → 格式分类 → 解析器插件 → 转换器 → Markdown → 学习
//! ```
//!
//! # 解析器类型
//!
//! - **通用解析器**: 源码文件 (md, txt, html, js, css, java, py, rs, etc.)
//! - **特殊解析器**: 二进制格式 (parquet, pdf, docx, xlsx, etc.)
//!
//! # 扩展性
//!
//! 新增格式只需实现 `FileParser` trait 并注册到 `ParserRegistry`。

use std::path::Path;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// 解析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedContent {
    /// 文件标题（用于知识主题）
    pub title: String,
    /// Markdown 格式的内容
    pub content: String,
    /// 提取的关键词
    pub keywords: Vec<String>,
    /// 内容类型
    pub content_type: ContentType,
    /// 元数据
    pub metadata: HashMap<String, String>,
}

/// 内容类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    /// 文档
    Document,
    /// 源代码
    SourceCode,
    /// 对话数据
    Conversation,
    /// 数据表格
    Spreadsheet,
    /// 结构化数据
    StructuredData,
    /// 多媒体
    Media,
}

/// 文件解析器 trait
pub trait FileParser: Send + Sync {
    /// 解析器名称
    fn name(&self) -> &str;

    /// 支持的文件扩展名
    fn extensions(&self) -> &[&str];

    /// 是否支持该文件
    fn supports(&self, extension: &str) -> bool {
        self.extensions().iter().any(|&ext| ext.eq_ignore_ascii_case(extension))
    }

    /// 解析文件内容
    fn parse(&self, file_path: &Path, raw_content: &[u8]) -> Result<ParsedContent, ParseError>;
}

/// 解析错误
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub parser_name: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.parser_name, self.message)
    }
}

impl std::error::Error for ParseError {}

// ============================================================================
// 通用解析器
// ============================================================================

/// 通用文本解析器
///
/// 处理纯文本和源码文件
pub struct TextParser;

impl FileParser for TextParser {
    fn name(&self) -> &str {
        "text"
    }

    fn extensions(&self) -> &[&str] {
        &[
            // 文档
            "md", "txt", "rst", "adoc",
            // 标记语言
            "html", "htm", "xml", "xhtml",
            "css", "scss", "sass", "less",
            "json", "yaml", "yml", "toml",
            // 脚本语言
            "js", "jsx", "ts", "tsx",
            "py", "pyw", "rb", "php",
            "sh", "bash", "zsh", "ps1", "bat",
            // 编程语言
            "java", "kt", "scala", "groovy",
            "c", "cpp", "cc", "cxx", "h", "hpp",
            "rs", "go", "swift", "m", "mm",
            "cs", "fs", "vb",
            // 函数式语言
            "hs", "lhs", "ml", "mli", "clj", "cljs",
            // 其他
            "sql", "lua", "r", "dart", "ex", "exs",
            "vue", "svelte", "astro",
        ]
    }

    fn parse(&self, file_path: &Path, raw_content: &[u8]) -> Result<ParsedContent, ParseError> {
        let content = String::from_utf8_lossy(raw_content);
        let extension = file_path.extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        let filename = file_path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        // 判断内容类型
        let content_type = Self::detect_content_type(&extension);

        // 构建标题
        let title = match content_type {
            ContentType::SourceCode => format!("{} 源码: {}", extension.to_uppercase(), filename),
            ContentType::Document => filename.clone(),
            _ => filename.clone(),
        };

        // 提取关键词
        let keywords = Self::extract_keywords(&content, &extension);

        // 转换为 Markdown 格式
        let markdown = match content_type {
            ContentType::SourceCode => {
                let lang = Self::get_code_language(&extension);
                format!(
                    "# {}\n\n```{}\n{}\n```\n",
                    title, lang, content
                )
            }
            ContentType::Document => {
                // 已经是文档，直接使用
                content.to_string()
            }
            _ => {
                format!("# {}\n\n```\n{}\n```\n", title, content)
            }
        };

        Ok(ParsedContent {
            title,
            content: markdown,
            keywords,
            content_type,
            metadata: {
                let mut m = HashMap::new();
                m.insert("extension".to_string(), extension);
                m.insert("filename".to_string(), filename);
                m
            },
        })
    }
}

impl TextParser {
    /// 检测内容类型
    fn detect_content_type(extension: &str) -> ContentType {
        match extension {
            "md" | "txt" | "rst" | "adoc" => ContentType::Document,
            "html" | "htm" | "xml" | "xhtml" => ContentType::Document,
            "json" | "yaml" | "yml" | "toml" => ContentType::StructuredData,
            _ => ContentType::SourceCode,
        }
    }

    /// 获取代码语言标识
    fn get_code_language(extension: &str) -> &str {
        match extension {
            "js" | "jsx" => "javascript",
            "ts" | "tsx" => "typescript",
            "py" | "pyw" => "python",
            "rb" => "ruby",
            "java" => "java",
            "kt" | "kts" => "kotlin",
            "scala" => "scala",
            "c" => "c",
            "cpp" | "cc" | "cxx" => "cpp",
            "h" => "c",
            "hpp" => "cpp",
            "rs" => "rust",
            "go" => "go",
            "swift" => "swift",
            "m" => "objective-c",
            "mm" => "objective-cpp",
            "cs" => "csharp",
            "fs" => "fsharp",
            "sh" | "bash" | "zsh" => "bash",
            "ps1" => "powershell",
            "sql" => "sql",
            "html" | "htm" => "html",
            "css" | "scss" | "sass" | "less" => "css",
            "json" => "json",
            "yaml" | "yml" => "yaml",
            "toml" => "toml",
            "xml" => "xml",
            "vue" => "vue",
            "svelte" => "svelte",
            _ => extension,
        }
    }

    /// 提取关键词
    fn extract_keywords(content: &str, extension: &str) -> Vec<String> {
        let mut keywords = Vec::new();

        // 添加文件类型作为关键词
        keywords.push(extension.to_uppercase());

        // 提取代码中的定义（函数、类等）
        if let ContentType::SourceCode = Self::detect_content_type(extension) {
            // 简单的关键词提取
            for line in content.lines().take(50) {
                let line = line.trim();

                // 函数定义
                if line.starts_with("fn ") || line.starts_with("def ") || line.starts_with("function ") {
                    if let Some(name) = line.split_whitespace().nth(1) {
                        keywords.push(name.split('(').next().unwrap_or(name).to_string());
                    }
                }

                // 类定义
                if line.starts_with("class ") || line.starts_with("struct ") || line.starts_with("interface ") {
                    if let Some(name) = line.split_whitespace().nth(1) {
                        keywords.push(name.split('{').next().unwrap_or(name).to_string());
                    }
                }
            }
        }

        keywords.truncate(10);
        keywords
    }
}

// ============================================================================
// 特殊解析器
// ============================================================================

/// Parquet 解析器
///
/// 解析 Apache Parquet 格式（如 HuggingFace 数据集）
pub struct ParquetParser;

impl FileParser for ParquetParser {
    fn name(&self) -> &str {
        "parquet"
    }

    fn extensions(&self) -> &[&str] {
        &["parquet", "par"]
    }

    fn parse(&self, file_path: &Path, _raw_content: &[u8]) -> Result<ParsedContent, ParseError> {
        // Parquet 是二进制格式，需要特殊处理
        // 这里使用简化的方式，实际解析需要 parquet 库
        #[cfg(feature = "parquet")]
        {
            self.parse_parquet(file_path)
        }

        #[cfg(not(feature = "parquet"))]
        {
            Err(ParseError {
                message: "parquet 格式需要启用 'parquet' feature，或使用 Python 脚本转换".to_string(),
                parser_name: self.name().to_string(),
            })
        }
    }
}

#[cfg(feature = "parquet")]
impl ParquetParser {
    fn parse_parquet(&self, file_path: &Path) -> Result<ParsedContent, ParseError> {
        use std::fs::File;
        use parquet::file::reader::{FileReader, SerializedFileReader};

        let file = File::open(file_path)
            .map_err(|e| ParseError {
                message: format!("打开文件失败: {}", e),
                parser_name: self.name().to_string(),
            })?;

        let reader = SerializedFileReader::new(file)
            .map_err(|e| ParseError {
                message: format!("读取 parquet 失败: {}", e),
                parser_name: self.name().to_string(),
            })?;

        let filename = file_path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let mut content = format!("# Parquet 数据集: {}\n\n", filename);

        // 获取 schema 信息
        let metadata = reader.metadata();
        let schema = metadata.file_metadata().schema();
        content.push_str(&format!("## Schema\n```\n{:?}\n```\n\n", schema));

        // 读取前几行作为示例
        content.push_str("## 数据示例\n\n");

        let iter = reader.get_row_iter(None)
            .map_err(|e| ParseError {
                message: format!("迭代行失败: {}", e),
                parser_name: self.name().to_string(),
            })?;

        for (idx, row_result) in iter.enumerate() {
            if idx >= 10 {
                break;
            }

            if let Ok(row) = row_result {
                content.push_str(&format!("### 行 {}\n```\n{:?}\n```\n\n", idx + 1, row));
            }
        }

        Ok(ParsedContent {
            title: format!("Parquet 数据: {}", filename),
            content,
            keywords: vec!["parquet".to_string(), "dataset".to_string()],
            content_type: ContentType::StructuredData,
            metadata: {
                let mut m = HashMap::new();
                m.insert("format".to_string(), "parquet".to_string());
                m
            },
        })
    }
}

/// PDF 解析器（占位）
pub struct PdfParser;

impl FileParser for PdfParser {
    fn name(&self) -> &str {
        "pdf"
    }

    fn extensions(&self) -> &[&str] {
        &["pdf"]
    }

    fn parse(&self, _file_path: &Path, _raw_content: &[u8]) -> Result<ParsedContent, ParseError> {
        Err(ParseError {
            message: "PDF 解析需要安装 pdf 解析插件或使用 Python 脚本".to_string(),
            parser_name: self.name().to_string(),
        })
    }
}

/// Word 文档解析器（占位）
pub struct DocxParser;

impl FileParser for DocxParser {
    fn name(&self) -> &str {
        "docx"
    }

    fn extensions(&self) -> &[&str] {
        &["docx", "doc"]
    }

    fn parse(&self, _file_path: &Path, _raw_content: &[u8]) -> Result<ParsedContent, ParseError> {
        Err(ParseError {
            message: "Word 文档解析需要安装 docx 解析插件或使用 Python 脚本".to_string(),
            parser_name: self.name().to_string(),
        })
    }
}

// ============================================================================
// 解析器注册表
// ============================================================================

/// 解析器注册表
pub struct ParserRegistry {
    parsers: Vec<Box<dyn FileParser>>,
}

impl ParserRegistry {
    /// 创建新的注册表
    pub fn new() -> Self {
        let mut registry = Self {
            parsers: Vec::new(),
        };

        // 注册默认解析器
        registry.register(Box::new(TextParser));
        registry.register(Box::new(ParquetParser));
        registry.register(Box::new(PdfParser));
        registry.register(Box::new(DocxParser));

        registry
    }

    /// 注册解析器
    pub fn register(&mut self, parser: Box<dyn FileParser>) {
        self.parsers.push(parser);
    }

    /// 根据扩展名查找解析器
    pub fn find_parser(&self, extension: &str) -> Option<&dyn FileParser> {
        let ext = extension.trim_start_matches('.').to_lowercase();
        self.parsers.iter()
            .find(|p| p.supports(&ext))
            .map(|p| p.as_ref())
    }

    /// 解析文件
    pub fn parse_file(&self, file_path: &Path) -> Result<ParsedContent, ParseError> {
        let extension = file_path.extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default();

        let parser = self.find_parser(&extension)
            .ok_or_else(|| ParseError {
                message: format!("不支持的文件格式: {}", extension),
                parser_name: "unknown".to_string(),
            })?;

        // 读取原始内容
        let raw_content = std::fs::read(file_path)
            .map_err(|e| ParseError {
                message: format!("读取文件失败: {}", e),
                parser_name: parser.name().to_string(),
            })?;

        parser.parse(file_path, &raw_content)
    }

    /// 列出所有支持的扩展名
    pub fn supported_extensions(&self) -> Vec<String> {
        let mut exts: Vec<String> = self.parsers.iter()
            .flat_map(|p| p.extensions().iter().map(|s| s.to_string()))
            .collect();
        exts.sort();
        exts.dedup();
        exts
    }
}

impl Default for ParserRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = ParserRegistry::new();
        assert!(!registry.parsers.is_empty());
    }

    #[test]
    fn test_find_parser() {
        let registry = ParserRegistry::new();

        // 测试通用格式
        assert!(registry.find_parser("md").is_some());
        assert!(registry.find_parser("txt").is_some());
        assert!(registry.find_parser("py").is_some());
        assert!(registry.find_parser("rs").is_some());

        // 测试特殊格式
        assert!(registry.find_parser("parquet").is_some());
        assert!(registry.find_parser("pdf").is_some());
    }

    #[test]
    fn test_text_parser() {
        let parser = TextParser;

        assert!(parser.supports("md"));
        assert!(parser.supports("py"));
        assert!(parser.supports("rs"));
        assert!(parser.supports("js"));

        let content = b"# Test\n\nHello world";
        let result = parser.parse(Path::new("test.md"), content);

        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert!(parsed.content.contains("Test"));
    }
}
