//! 知识存储模块 - HTML 文件持久化
//!
//! # 设计理念
//!
//! - 黑塔：知识以 HTML 超链接形式组织，形成知识图谱
//! - 螺丝咕姆：共享/私有分离，信任度决定可见性
//! - 拉蒂奥：统一格式，优雅的引用关系
//!
//! # 目录结构
//!
//! ```text
//! knowledge/
//! ├── skills/
//! │   ├── shared/       # 共享技能（高信任度）
//! │   └── private/      # 私有技能（隔离）
//! │       └── gu_xxx/   # 每个蛊虫的私有空间
//! ├── gus/              # 蛊虫索引
//! └── concepts/         # 概念库（已存在）
//! ```

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use uuid::Uuid;

/// 知识存储根目录
const KNOWLEDGE_ROOT: &str = "knowledge";

/// 技能存储
pub struct SkillStorage {
    /// 根目录路径
    root: PathBuf,
}

impl SkillStorage {
    /// 创建新的技能存储
    pub fn new() -> Self {
        Self {
            root: PathBuf::from(KNOWLEDGE_ROOT),
        }
    }

    /// 使用自定义根目录
    pub fn with_root(root: PathBuf) -> Self {
        Self { root }
    }

    /// 保存共享技能
    pub fn save_shared_skill(&self, skill: &SkillDocument) -> Result<PathBuf, String> {
        let path = self.root
            .join("skills")
            .join("shared")
            .join(format!("{}.html", skill.id));

        self.ensure_dir(path.parent().unwrap())?;
        let content = skill.to_html();
        fs::write(&path, content)
            .map_err(|e| format!("写入技能文件失败: {}", e))?;

        Ok(path)
    }

    /// 保存私有技能
    pub fn save_private_skill(&self, gu_id: &Uuid, skill: &SkillDocument) -> Result<PathBuf, String> {
        let path = self.root
            .join("skills")
            .join("private")
            .join(gu_id.to_string())
            .join(format!("{}.html", skill.id));

        self.ensure_dir(path.parent().unwrap())?;
        let content = skill.to_html();
        fs::write(&path, content)
            .map_err(|e| format!("写入私有技能文件失败: {}", e))?;

        Ok(path)
    }

    /// 读取技能
    pub fn read_skill(&self, skill_id: &str) -> Result<SkillDocument, String> {
        // 先尝试共享目录
        let shared_path = self.root
            .join("skills")
            .join("shared")
            .join(format!("{}.html", skill_id));

        if shared_path.exists() {
            return self.read_skill_from_file(&shared_path);
        }

        // 再尝试私有目录（需要遍历）
        let private_dir = self.root.join("skills").join("private");
        if private_dir.exists() {
            for entry in fs::read_dir(&private_dir)
                .map_err(|e| format!("读取私有目录失败: {}", e))?
            {
                let gu_dir = entry.map_err(|e| format!("读取目录项失败: {}", e))?.path();
                let skill_path = gu_dir.join(format!("{}.html", skill_id));
                if skill_path.exists() {
                    return self.read_skill_from_file(&skill_path);
                }
            }
        }

        Err(format!("技能 {} 不存在", skill_id))
    }

    /// 从文件读取技能
    fn read_skill_from_file(&self, path: &Path) -> Result<SkillDocument, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("读取技能文件失败: {}", e))?;

        SkillDocument::from_html(&content)
    }

    /// 保存蛊虫索引
    pub fn save_gu_index(&self, gu_id: &Uuid, index: &GuIndexDocument) -> Result<PathBuf, String> {
        let path = self.root
            .join("gus")
            .join(format!("{}.html", gu_id));

        self.ensure_dir(path.parent().unwrap())?;
        let content = index.to_html();
        fs::write(&path, content)
            .map_err(|e| format!("写入蛊虫索引失败: {}", e))?;

        Ok(path)
    }

    /// 读取蛊虫索引
    pub fn read_gu_index(&self, gu_id: &Uuid) -> Result<GuIndexDocument, String> {
        let path = self.root.join("gus").join(format!("{}.html", gu_id));
        let content = fs::read_to_string(&path)
            .map_err(|e| format!("读取蛊虫索引失败: {}", e))?;

        GuIndexDocument::from_html(&content)
    }

    /// 更新技能引用计数
    pub fn increment_ref_count(&self, skill_id: &str) -> Result<u32, String> {
        let mut skill = self.read_skill(skill_id)?;
        skill.ref_count += 1;
        self.save_shared_skill(&skill)?;
        Ok(skill.ref_count)
    }

    /// 确保目录存在
    fn ensure_dir(&self, dir: &Path) -> Result<(), String> {
        if !dir.exists() {
            fs::create_dir_all(dir)
                .map_err(|e| format!("创建目录失败: {}", e))?;
        }
        Ok(())
    }
}

impl Default for SkillStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// 技能文档
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDocument {
    /// 技能 ID
    pub id: String,
    /// 技能名称
    pub name: String,
    /// 技能类型（shared/private）
    pub skill_type: String,
    /// 创建者蛊虫 ID
    pub created_by: Uuid,
    /// 创建时间
    pub created_at: String,
    /// 共识状态
    pub consensus_status: String,
    /// 引用计数
    pub ref_count: u32,
    /// 定义
    pub definition: String,
    /// 核心概念
    pub concepts: Vec<String>,
    /// 知识粒子（JSON）
    pub particles: Option<String>,
    /// 相关技能
    pub relations: Vec<SkillRelation>,
    /// 学习者
    pub learners: Vec<LearnerInfo>,
}

/// 技能关系
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRelation {
    pub target_skill: String,
    pub relation_type: String, // prerequisite, next-step, related
    pub label: String,
}

/// 学习者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnerInfo {
    pub gu_id: Uuid,
    pub gu_name: String,
    pub mastery: f64,
}

impl SkillDocument {
    /// 创建新技能
    pub fn new(id: &str, name: &str, created_by: Uuid) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            skill_type: "shared".to_string(),
            created_by,
            created_at: chrono_like_now(),
            consensus_status: "pending".to_string(),
            ref_count: 0,
            definition: String::new(),
            concepts: Vec::new(),
            particles: None,
            relations: Vec::new(),
            learners: Vec::new(),
        }
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let type_color = if self.skill_type == "shared" { "#27ae60" } else { "#e67e22" };

        let concepts_html = self.concepts.iter()
            .map(|c| format!("<li><a href=\"../../concepts/{}.html\" rel=\"concept\">{}</a></li>", c, c))
            .collect::<Vec<_>>()
            .join("\n                ");

        let relations_html = self.relations.iter()
            .map(|r| {
                let rel = &r.relation_type;
                let label = match rel.as_str() {
                    "prerequisite" => "前置",
                    "next-step" => "进阶",
                    _ => "相关",
                };
                format!("<li>▸ <a href=\"./{}.html\" rel=\"{}\">{}: {}</a></li>",
                    r.target_skill, rel, label, r.label)
            })
            .collect::<Vec<_>>()
            .join("\n                ");

        let learners_html = self.learners.iter()
            .map(|l| format!("<li><a href=\"../../gus/{}.html\">{}</a> - 掌握度 {:.0}%</li>",
                l.gu_id, l.gu_name, l.mastery * 100.0))
            .collect::<Vec<_>>()
            .join("\n                ");

        let particles_html = if let Some(p) = &self.particles {
            format!("<data name=\"particles\" value='{}'>", p)
        } else {
            String::new()
        };

        format!(r#"<!DOCTYPE html>
<html lang="zh-CN" data-type="skill">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="skill-id" content="{}">
    <meta name="skill-type" content="{}">
    <meta name="created-by" content="{}">
    <meta name="created-at" content="{}">
    <meta name="consensus-status" content="{}">
    <meta name="ref-count" content="{}">
    <title>{} - 技能</title>
    <style>
        body {{ font-family: 'Microsoft YaHei', sans-serif; max-width: 800px; margin: 20px auto; padding: 20px; background: #f0f4f8; }}
        .skill {{ background: white; padding: 30px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }}
        .skill-header {{ border-bottom: 2px solid #9b59b6; padding-bottom: 15px; margin-bottom: 20px; }}
        .skill-meta {{ font-size: 0.85em; color: #666; }}
        .skill-type {{ background: {}; color: white; padding: 2px 8px; border-radius: 4px; }}
        section {{ margin: 20px 0; }}
        h2 {{ color: #9b59b6; font-size: 1.2em; border-left: 4px solid #9b59b6; padding-left: 10px; }}
        .concept-list, .relation-list, .learner-list {{ list-style: none; padding: 0; }}
        .concept-list li, .relation-list li, .learner-list li {{ margin: 8px 0; padding: 8px; background: #f8f9fa; border-radius: 6px; }}
        a {{ color: #3498db; text-decoration: none; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <article class="skill">
        <header class="skill-header">
            <h1>🌐 {}</h1>
            <div class="skill-meta">
                <span class="skill-type">{}</span>
                <span class="ref-count">被引用: {}次</span>
            </div>
        </header>

        <section class="definition">
            <h2>定义</h2>
            <p>{}</p>
        </section>

        <section class="concepts">
            <h2>核心概念</h2>
            <ul class="concept-list">
                {}
            </ul>
        </section>

        <section class="particles" hidden>
            {}
        </section>

        <section class="relations">
            <h2>相关技能</h2>
            <ul class="relation-list">
                {}
            </ul>
        </section>

        <section class="learners">
            <h2>学习者</h2>
            <ul class="learner-list">
                {}
            </ul>
        </section>

        <footer style="margin-top: 30px; padding-top: 20px; border-top: 1px solid #e9ecef; color: #666; font-size: 0.9em;">
            <p>技能知识 v1.0 | 由万物生成器维护</p>
        </footer>
    </article>
</body>
</html>"#,
            self.id,
            self.skill_type,
            self.created_by,
            self.created_at,
            self.consensus_status,
            self.ref_count,
            self.name,
            type_color,
            self.name,
            self.skill_type,
            self.ref_count,
            self.definition,
            concepts_html,
            particles_html,
            relations_html,
            learners_html,
        )
    }

    /// 从 HTML 解析（简化版，只提取元数据）
    pub fn from_html(html: &str) -> Result<Self, String> {
        // 提取 meta 标签
        let extract_meta = |name: &str| -> Result<String, String> {
            let pattern = format!("<meta name=\"{}\" content=\"", name);
            let start = html.find(&pattern)
                .ok_or_else(|| format!("找不到 meta {}", name))?;
            let content_start = start + pattern.len();
            let end = html[content_start..].find('"')
                .ok_or_else(|| format!("meta {} 格式错误", name))?;
            Ok(html[content_start..content_start + end].to_string())
        };

        let id = extract_meta("skill-id")?;
        let skill_type = extract_meta("skill-type")?;
        let created_by_str = extract_meta("created-by")?;
        let created_by = Uuid::parse_str(&created_by_str)
            .map_err(|e| format!("解析 UUID 失败: {}", e))?;
        let created_at = extract_meta("created-at")?;
        let consensus_status = extract_meta("consensus-status")?;
        let ref_count: u32 = extract_meta("ref-count")?.parse()
            .map_err(|e| format!("解析 ref-count 失败: {}", e))?;

        // 提取标题
        let title_start = html.find("<h1>").ok_or("找不到标题")? + 4;
        let title_end = html[title_start..].find("</h1>").ok_or("标题格式错误")?;
        let name = html[title_start..title_start + title_end]
            .replace("🌐 ", "")
            .replace("🔮 ", "")
            .replace("🔴 ", "");

        Ok(Self {
            id,
            name,
            skill_type,
            created_by,
            created_at,
            consensus_status,
            ref_count,
            definition: String::new(),
            concepts: Vec::new(),
            particles: None,
            relations: Vec::new(),
            learners: Vec::new(),
        })
    }
}

/// 蛊虫索引文档
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuIndexDocument {
    /// 蛊虫 ID
    pub gu_id: Uuid,
    /// 蛊虫名称
    pub gu_name: String,
    /// 创建时间
    pub created_at: String,
    /// 信任分数
    pub trust_score: f64,
    /// 技能列表
    pub skills: Vec<SkillRef>,
    /// 知识连接
    pub knowledge_links: Vec<String>,
}

/// 技能引用
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRef {
    pub skill_id: String,
    pub skill_name: String,
    pub is_private: bool,
    pub mastery: f64,
}

impl GuIndexDocument {
    /// 创建新索引
    pub fn new(gu_id: Uuid, gu_name: &str) -> Self {
        Self {
            gu_id,
            gu_name: gu_name.to_string(),
            created_at: chrono_like_now(),
            trust_score: 0.5,
            skills: Vec::new(),
            knowledge_links: Vec::new(),
        }
    }

    /// 添加技能
    pub fn add_skill(&mut self, skill_id: &str, skill_name: &str, is_private: bool, mastery: f64) {
        self.skills.push(SkillRef {
            skill_id: skill_id.to_string(),
            skill_name: skill_name.to_string(),
            is_private,
            mastery,
        });
    }

    /// 转换为 HTML
    pub fn to_html(&self) -> String {
        let skills_html = self.skills.iter()
            .map(|s| {
                let private_class = if s.is_private { "private" } else { "" };
                format!(
                    "<li><a href=\"../skills/{}/{}.html\" rel=\"skill\" data-mastery=\"{}\">{}<span class=\"mastery {}\">掌握度: {:.0}%</span></a></li>",
                    if s.is_private { "private" } else { "shared" },
                    s.skill_id,
                    s.mastery,
                    s.skill_name,
                    private_class,
                    s.mastery * 100.0
                )
            })
            .collect::<Vec<_>>()
            .join("\n                ");

        let links_html = self.knowledge_links.iter()
            .map(|l| format!("<li>▸ 学习了 <a href=\"../concepts/{}.html\">{}</a></li>", l, l))
            .collect::<Vec<_>>()
            .join("\n                ");

        format!(r#"<!DOCTYPE html>
<html lang="zh-CN" data-type="gu-index">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="gu-id" content="{}">
    <meta name="gu-name" content="{}">
    <meta name="created-at" content="{}">
    <meta name="trust-score" content="{}">
    <meta name="skill-count" content="{}">
    <title>{} - 蛊虫技能索引</title>
    <style>
        body {{ font-family: 'Microsoft YaHei', sans-serif; max-width: 800px; margin: 20px auto; padding: 20px; background: #f0f4f8; }}
        .gu-profile {{ background: white; padding: 30px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }}
        .gu-header {{ border-bottom: 2px solid #9b59b6; padding-bottom: 15px; margin-bottom: 20px; }}
        .gu-meta {{ font-size: 0.85em; color: #666; }}
        .gu-id {{ background: #9b59b6; color: white; padding: 2px 8px; border-radius: 4px; }}
        section {{ margin: 20px 0; }}
        h2 {{ color: #9b59b6; font-size: 1.2em; border-left: 4px solid #9b59b6; padding-left: 10px; }}
        .skill-list {{ list-style: none; padding: 0; }}
        .skill-list li {{ margin: 10px 0; padding: 10px; background: #f8f9fa; border-radius: 6px; }}
        .skill-list a {{ color: #2c3e50; text-decoration: none; display: block; }}
        .mastery {{ font-size: 0.8em; color: #666; background: #e9ecef; padding: 2px 6px; border-radius: 3px; margin-left: 8px; }}
        .mastery.private {{ background: #ffeaa7; }}
        a {{ color: #3498db; }}
    </style>
</head>
<body>
    <article class="gu-profile">
        <header class="gu-header">
            <h1>🔴 {}</h1>
            <div class="gu-meta">
                <span class="gu-id">{}</span>
                <span class="trust">信任度: {:.0}%</span>
            </div>
        </header>

        <section class="skills">
            <h2>已掌握技能</h2>
            <ul class="skill-list">
                {}
            </ul>
        </section>

        <section class="knowledge-graph">
            <h2>知识连接</h2>
            <ul class="relation-list">
                {}
            </ul>
        </section>

        <footer style="margin-top: 30px; padding-top: 20px; border-top: 1px solid #e9ecef; color: #666; font-size: 0.9em;">
            <p>蛊虫技能索引 v1.0 | 由万物生成器维护</p>
        </footer>
    </article>
</body>
</html>"#,
            self.gu_id,
            self.gu_name,
            self.created_at,
            self.trust_score,
            self.skills.len(),
            self.gu_name,
            self.gu_name,
            self.gu_id,
            self.trust_score * 100.0,
            skills_html,
            links_html,
        )
    }

    /// 从 HTML 解析
    pub fn from_html(html: &str) -> Result<Self, String> {
        let extract_meta = |name: &str| -> Result<String, String> {
            let pattern = format!("<meta name=\"{}\" content=\"", name);
            let start = html.find(&pattern)
                .ok_or_else(|| format!("找不到 meta {}", name))?;
            let content_start = start + pattern.len();
            let end = html[content_start..].find('"')
                .ok_or_else(|| format!("meta {} 格式错误", name))?;
            Ok(html[content_start..content_start + end].to_string())
        };

        let gu_id = Uuid::parse_str(&extract_meta("gu-id")?)
            .map_err(|e| format!("解析 UUID 失败: {}", e))?;
        let gu_name = extract_meta("gu-name")?;
        let created_at = extract_meta("created-at")?;
        let trust_score: f64 = extract_meta("trust-score")?.parse()
            .map_err(|e| format!("解析 trust-score 失败: {}", e))?;

        Ok(Self {
            gu_id,
            gu_name,
            created_at,
            trust_score,
            skills: Vec::new(),
            knowledge_links: Vec::new(),
        })
    }
}

/// 简单的时间戳函数（避免依赖 chrono）
fn chrono_like_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let days = secs / 86400;
    let years = 1970 + days / 365;
    let remaining_days = days % 365;
    let month = 1 + remaining_days / 30;
    let day = 1 + remaining_days % 30;
    let hour = (secs % 86400) / 3600;
    let minute = (secs % 3600) / 60;
    let second = secs % 60;
    format!("{}-{:02}-{:02}T{:02}:{:02}:{:02}Z", years, month, day, hour, minute, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_document_html_roundtrip() {
        let gu_id = Uuid::new_v4();
        let skill = SkillDocument::new("test_skill", "测试技能", gu_id);

        let html = skill.to_html();
        let parsed = SkillDocument::from_html(&html).unwrap();

        assert_eq!(skill.id, parsed.id);
        assert_eq!(skill.name, parsed.name);
        assert_eq!(skill.skill_type, parsed.skill_type);
    }

    #[test]
    fn test_gu_index_html_roundtrip() {
        let gu_id = Uuid::new_v4();
        let mut index = GuIndexDocument::new(gu_id, "测试蛊虫");
        index.add_skill("html-basics", "HTML 基础", false, 0.8);

        let html = index.to_html();
        let parsed = GuIndexDocument::from_html(&html).unwrap();

        assert_eq!(index.gu_id, parsed.gu_id);
        assert_eq!(index.gu_name, parsed.gu_name);
    }
}
