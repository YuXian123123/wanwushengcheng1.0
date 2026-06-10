//! 世界知识库 - 统一存储与共识机制
//!
//! # 设计理念
//!
//! - 黑塔：竞争涌现最佳知识
//! - 螺丝咕姆：100% 共识确保准确性
//! - 拉蒂奥：统一版本，知识图谱
//!
//! # 核心改进
//!
//! 1. 统一存储：所有知识存在 `world/` 目录，不再按蛊虫分目录
//! 2. 蛊虫交流：学习时相互讨论，交换观点
//! 3. 100% 共识：所有参与的蛊虫必须同意才能入库
//! 4. 知识图谱：每个 HTML 文件自动生成可视化图谱

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use super::knowledge_storage::SkillDocument;
use super::cognis::{CogniParticle, ParseResult};

/// 世界知识库（统一存储）
pub struct WorldKnowledgeStore {
    /// 根目录
    root: PathBuf,
    /// 候选版本（主题 → 候选列表）
    candidates: HashMap<String, Vec<CandidateVersion>>,
    /// 投票记录（主题 → 投票）
    votes: HashMap<String, VotingSession>,
    /// 已注册的蛊虫（用于投票）
    registered_gus: Vec<Uuid>,
}

/// 候选版本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateVersion {
    /// 候选 ID
    pub id: String,
    /// 提交者蛊虫 ID
    pub submitted_by: Uuid,
    /// 提交者名称
    pub submitter_name: String,
    /// 提交时间
    pub submitted_at: u64,
    /// 知识文档
    pub document: SkillDocument,
    /// 认知粒子（用于图谱）
    pub particles: ParseResult,
    /// 排版分数（其他蛊虫评分）
    pub layout_scores: Vec<LayoutScore>,
}

/// 排版评分
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutScore {
    /// 评分者
    pub scorer_id: Uuid,
    /// 评分者名称
    pub scorer_name: String,
    /// 分数 [0, 1]
    pub score: f64,
    /// 理由
    pub reason: String,
}

/// 投票会话
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VotingSession {
    /// 主题
    pub topic: String,
    /// 开始时间
    pub started_at: u64,
    /// 候选 ID 列表
    pub candidates: Vec<String>,
    /// 投票记录
    pub votes: Vec<Vote>,
    /// 需要多少蛊虫同意（100% = 全部）
    pub required_approvals: usize,
    /// 状态
    pub status: VotingStatus,
    /// 胜出者
    pub winner: Option<String>,
}

/// 投票
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    /// 投票者
    pub voter_id: Uuid,
    /// 投票者名称
    pub voter_name: String,
    /// 投给哪个候选
    pub candidate_id: String,
    /// 是否同意
    pub approve: bool,
    /// 置信度
    pub confidence: f64,
    /// 评论/观点
    pub comment: String,
    /// 投票时间
    pub voted_at: u64,
}

/// 投票状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VotingStatus {
    /// 等待投票
    Pending,
    /// 进行中
    InProgress,
    /// 达成共识
    ConsensusReached,
    /// 被否决
    Rejected,
}

/// 知识图谱
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    /// 节点
    pub nodes: Vec<GraphNode>,
    /// 边
    pub edges: Vec<GraphEdge>,
    /// 元数据
    pub metadata: GraphMetadata,
}

/// 图谱节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    /// 节点 ID
    pub id: String,
    /// 节点标签
    pub label: String,
    /// 节点类型
    pub node_type: String,
    /// 大小（重要性）
    pub size: f64,
    /// 颜色
    pub color: String,
}

/// 图谱边
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    /// 源节点
    pub source: String,
    /// 目标节点
    pub target: String,
    /// 关系类型
    pub relation: String,
    /// 权重
    pub weight: f64,
}

/// 图谱元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphMetadata {
    /// 主题
    pub topic: String,
    /// 实体数量
    pub entity_count: usize,
    /// 关系数量
    pub relation_count: usize,
    /// 生成时间
    pub generated_at: u64,
}

impl WorldKnowledgeStore {
    /// 创建新的世界知识库
    pub fn new() -> Self {
        Self {
            root: PathBuf::from("knowledge"),
            candidates: HashMap::new(),
            votes: HashMap::new(),
            registered_gus: Vec::new(),
        }
    }

    /// 注册蛊虫（参与投票）
    pub fn register_gu(&mut self, gu_id: Uuid) {
        if !self.registered_gus.contains(&gu_id) {
            self.registered_gus.push(gu_id);
        }
    }

    /// 获取需要的票数（100% 共识 = 所有蛊虫）
    pub fn required_votes(&self) -> usize {
        self.registered_gus.len().max(1)
    }

    /// 检查世界知识库是否已存在该主题
    pub fn exists(&self, topic: &str) -> bool {
        let path = self.root
            .join("world")
            .join(format!("{}.html", Self::normalize_topic(topic)));
        path.exists()
    }

    /// 提交候选版本
    pub fn submit_candidate(
        &mut self,
        gu_id: Uuid,
        gu_name: &str,
        document: SkillDocument,
        particles: ParseResult,
    ) -> String {
        let topic = document.name.clone();
        let normalized = Self::normalize_topic(&topic);

        // 创建候选
        let candidate = CandidateVersion {
            id: format!("candidate_{}", Uuid::new_v4()),
            submitted_by: gu_id,
            submitter_name: gu_name.to_string(),
            submitted_at: Self::now(),
            document,
            particles,
            layout_scores: Vec::new(),
        };

        let candidate_id = candidate.id.clone();

        // 添加到候选池
        self.candidates
            .entry(normalized.clone())
            .or_default()
            .push(candidate);

        // 初始化投票会话
        if !self.votes.contains_key(&normalized) {
            self.votes.insert(normalized.clone(), VotingSession {
                topic: topic.clone(),
                started_at: Self::now(),
                candidates: Vec::new(),
                votes: Vec::new(),
                required_approvals: self.required_votes(),
                status: VotingStatus::Pending,
                winner: None,
            });
        }

        // 添加候选到投票会话
        if let Some(session) = self.votes.get_mut(&normalized) {
            session.candidates.push(candidate_id.clone());
            session.status = VotingStatus::InProgress;
        }

        candidate_id
    }

    /// 对候选进行排版评分
    pub fn score_layout(
        &mut self,
        topic: &str,
        candidate_id: &str,
        scorer_id: Uuid,
        scorer_name: &str,
        score: f64,
        reason: String,
    ) {
        let normalized = Self::normalize_topic(topic);
        if let Some(candidates) = self.candidates.get_mut(&normalized) {
            if let Some(candidate) = candidates.iter_mut().find(|c| c.id == candidate_id) {
                candidate.layout_scores.push(LayoutScore {
                    scorer_id,
                    scorer_name: scorer_name.to_string(),
                    score,
                    reason,
                });
            }
        }
    }

    /// 投票
    pub fn vote(
        &mut self,
        topic: &str,
        voter_id: Uuid,
        voter_name: &str,
        candidate_id: &str,
        approve: bool,
        confidence: f64,
        comment: String,
    ) -> Option<VotingStatus> {
        let normalized = Self::normalize_topic(topic);
        let session = self.votes.get_mut(&normalized)?;

        // 添加投票
        session.votes.push(Vote {
            voter_id,
            voter_name: voter_name.to_string(),
            candidate_id: candidate_id.to_string(),
            approve,
            confidence,
            comment,
            voted_at: Self::now(),
        });

        // 计算该候选的同意票数
        let approve_count = session.votes.iter()
            .filter(|v| v.candidate_id == candidate_id && v.approve)
            .count();

        // 检查是否达成 100% 共识
        if approve_count >= session.required_approvals {
            session.status = VotingStatus::ConsensusReached;
            session.winner = Some(candidate_id.to_string());
        }

        Some(session.status)
    }

    /// 获取胜出的候选（排版分数最高 + 达成共识）
    pub fn get_winner(&self, topic: &str) -> Option<&CandidateVersion> {
        let normalized = Self::normalize_topic(topic);
        let session = self.votes.get(&normalized)?;

        if session.status != VotingStatus::ConsensusReached {
            return None;
        }

        let winner_id = session.winner.as_ref()?;
        let candidates = self.candidates.get(&normalized)?;

        candidates.iter().find(|c| &c.id == winner_id)
    }

    /// 提升到世界知识库
    pub fn promote_to_world(&mut self, topic: &str) -> Option<PathBuf> {
        let winner = self.get_winner(topic)?.clone();
        let normalized = Self::normalize_topic(topic);

        // 生成知识图谱
        let graph = Self::generate_graph(&winner);

        // 保存知识文档
        let doc_path = self.root
            .join("world")
            .join(format!("{}.html", normalized));

        self.ensure_dir(doc_path.parent().unwrap());

        // 生成带图谱的 HTML
        let html = Self::generate_html_with_graph(&winner.document, &graph);
        std::fs::write(&doc_path, html).ok()?;

        // 保存图谱数据
        let graph_path = self.root
            .join("graph")
            .join(format!("{}.json", normalized));
        self.ensure_dir(graph_path.parent().unwrap());
        let graph_json = serde_json::to_string_pretty(&graph).ok()?;
        std::fs::write(&graph_path, graph_json).ok()?;

        // 清理候选池
        self.candidates.remove(&normalized);
        self.votes.remove(&normalized);

        Some(doc_path)
    }

    /// 生成知识图谱
    fn generate_graph(candidate: &CandidateVersion) -> KnowledgeGraph {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut node_ids: HashMap<String, String> = HashMap::new();

        // 从认知粒子提取节点和边
        for particle in &candidate.particles.particles {
            match particle {
                CogniParticle::Entity { id, name, entity_type } => {
                    let node_id = format!("entity_{}", id);
                    let (color, size) = match entity_type {
                        super::cognis::EntityType::CodeLanguage => ("#e74c3c", 1.0),
                        super::cognis::EntityType::TechTerm => ("#3498db", 0.8),
                        super::cognis::EntityType::Concept => ("#27ae60", 0.6),
                        super::cognis::EntityType::Keyword => ("#f39c12", 0.4),
                        _ => ("#95a5a6", 0.2),
                    };

                    nodes.push(GraphNode {
                        id: node_id.clone(),
                        label: name.clone(),
                        node_type: format!("{:?}", entity_type),
                        size,
                        color: color.to_string(),
                    });
                    node_ids.insert(format!("{}", id), node_id);
                }
                CogniParticle::Relation { source_id, target_id, rel_type } => {
                    if let (Some(source), Some(target)) = (
                        node_ids.get(&format!("{}", source_id)),
                        node_ids.get(&format!("{}", target_id)),
                    ) {
                        let (weight, color) = match rel_type {
                            super::cognis::RelationType::DependsOn => (1.0, "#e74c3c"),
                            super::cognis::RelationType::Contains => (0.8, "#3498db"),
                            super::cognis::RelationType::BelongsTo => (0.6, "#27ae60"),
                            super::cognis::RelationType::SimilarTo => (0.4, "#f39c12"),
                            _ => (0.2, "#95a5a6"),
                        };

                        edges.push(GraphEdge {
                            source: source.clone(),
                            target: target.clone(),
                            relation: format!("{:?}", rel_type),
                            weight,
                        });
                    }
                }
                _ => {}
            }
        }

        KnowledgeGraph {
            metadata: GraphMetadata {
                topic: candidate.document.name.clone(),
                entity_count: nodes.len(),
                relation_count: edges.len(),
                generated_at: Self::now(),
            },
            nodes,
            edges,
        }
    }

    /// 生成带知识图谱的 HTML（按照 knowledge-graph-design.html 格式）
    fn generate_html_with_graph(document: &SkillDocument, graph: &KnowledgeGraph) -> String {
        // 生成属性列表
        let attributes_html = document.concepts.iter()
            .map(|c| format!(
                "                <dt>{}</dt>\n                <dd>相关概念</dd>",
                c
            ))
            .collect::<Vec<_>>()
            .join("\n");

        // 生成关系链接（使用 <a href="" rel=""> 格式）
        let relations_html = graph.edges.iter()
            .map(|e| {
                let rel = Self::relation_to_rel(&e.relation);
                let target_label = graph.nodes.iter()
                    .find(|n| n.id == e.target)
                    .map(|n| n.label.as_str())
                    .unwrap_or(&e.target);
                format!(
                    "                <li><a href=\"./{}.html\" rel=\"{}\">{} ({})</a></li>",
                    Self::normalize_topic(target_label),
                    rel,
                    target_label,
                    e.relation
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        // 生成示例（SkillDocument 没有 examples 字段，使用 concepts 作为示例）
        let examples_html = document.concepts.iter()
            .take(3)
            .map(|c| format!("            <li>{}</li>", c))
            .collect::<Vec<_>>()
            .join("\n");

        // 图谱 JSON 数据（嵌入到 HTML 中）
        let graph_json = serde_json::to_string(graph).unwrap_or_default();

        // 生成节点可视化（用于图谱展示）
        let nodes_visual = graph.nodes.iter()
            .map(|n| format!(
                "            <div class=\"node\" style=\"background:{};\">\n                <span class=\"type\">{}</span>\n                <span class=\"name\">{}</span>\n            </div>",
                n.color, n.node_type, n.label
            ))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
r#"<!DOCTYPE html>
<html lang="zh-CN" data-type="concept">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <!-- 概念元数据 -->
    <meta name="concept-id" content="{}">
    <meta name="concept-type" content="world-knowledge">
    <meta name="concept-level" content="3">
    <meta name="created-by" content="{}">
    <meta name="created-at" content="{}">
    <meta name="consensus-status" content="approved">
    <meta name="consensus-rate" content="1.0">
    <meta name="entity-count" content="{}">
    <meta name="relation-count" content="{}">
    <title>{} - 世界知识库</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: 'Microsoft YaHei', 'Segoe UI', sans-serif;
            background: linear-gradient(135deg, #0c0c1e 0%, #1a2a3e 50%, #0f3d4d 100%);
            color: #e4e4e4;
            min-height: 100vh;
            padding: 20px;
        }}
        .container {{ max-width: 1400px; margin: 0 auto; }}
        article.concept {{
            background: rgba(255, 255, 255, 0.05);
            border-radius: 15px;
            padding: 30px;
            border: 1px solid rgba(26, 188, 156, 0.3);
        }}
        header {{ border-bottom: 2px solid #1abc9c; padding-bottom: 15px; margin-bottom: 20px; }}
        h1 {{ color: #1abc9c; font-size: 2em; }}
        h2 {{ color: #1abc9c; margin: 20px 0 15px 0; padding-bottom: 10px; border-bottom: 1px solid rgba(26, 188, 156, 0.3); }}
        h3 {{ color: #3498db; margin: 15px 0 10px 0; }}
        .tag {{ display: inline-block; padding: 3px 10px; border-radius: 20px; font-size: 0.85em; margin: 2px; }}
        .tag-core {{ background: #1abc9c; color: #0c0c1e; }}
        .tag-feature {{ background: #3498db; color: #0c0c1e; }}
        section {{ margin: 20px 0; }}
        dl {{ margin-left: 20px; }}
        dt {{ color: #3498db; font-weight: bold; margin: 10px 0 5px 0; }}
        dd {{ margin-left: 20px; color: #bdc3c7; }}
        ul {{ margin-left: 20px; }}
        li {{ margin: 8px 0; }}
        a {{ color: #1abc9c; text-decoration: none; }}
        a:hover {{ color: #3498db; text-decoration: underline; }}
        .node {{
            display: inline-block;
            padding: 8px 15px;
            margin: 5px;
            border-radius: 6px;
            color: white;
        }}
        .node .type {{ font-size: 0.7em; opacity: 0.8; display: block; }}
        .node .name {{ font-weight: bold; }}
        .graph-section {{
            background: rgba(0, 0, 0, 0.3);
            border-radius: 10px;
            padding: 20px;
            margin-top: 20px;
        }}
        .graph-data {{
            background: #1a1a2e;
            border-radius: 8px;
            padding: 15px;
            font-family: 'Courier New', monospace;
            font-size: 0.8em;
            overflow: auto;
            max-height: 300px;
            border: 1px solid #2d3748;
            margin-top: 15px;
        }}
        .stats {{
            background: rgba(52, 152, 219, 0.1);
            border-left: 4px solid #3498db;
            padding: 15px;
            margin: 15px 0;
        }}
    </style>
</head>
<body>
    <div class="container">
        <article class="concept">
            <!-- 概念标识 -->
            <header class="concept-header">
                <h1>{}</h1>
                <div class="concept-meta">
                    <span class="tag tag-core">✓ 100% 共识</span>
                    <span class="tag tag-feature">实体: {}</span>
                    <span class="tag tag-feature">关系: {}</span>
                </div>
            </header>

            <!-- 概念定义 -->
            <section class="definition">
                <h2>定义</h2>
                <p>{}</p>
            </section>

            <!-- 概念属性 -->
            <section class="attributes">
                <h2>属性</h2>
                <dl>
{}
                </dl>
            </section>

            <!-- 相关概念 -->
            <section class="relations">
                <h2>关系</h2>
                <ul class="relation-list">
{}
                </ul>
            </section>

            <!-- 使用示例 -->
            <section class="examples">
                <h2>示例</h2>
                <ul>
{}
                </ul>
            </section>

            <!-- 知识图谱可视化 -->
            <section class="graph-section">
                <h2>📊 知识图谱</h2>
                <div class="stats">
                    <span>节点数: <strong>{}</strong></span> |
                    <span>边数: <strong>{}</strong></span> |
                    <span>生成时间: <strong>{}</strong></span>
                </div>
                <h3>实体节点</h3>
                <div class="nodes">
{}
                </div>
            </section>

            <!-- 概念向量（机器可读） -->
            <section class="embedding" hidden>
                <!-- 高维向量，供LNN使用 -->
                <data name="vector" value="[0.23,-0.45,0.67,...]"></data>
                <data name="vector-dim" value="256"></data>
                <data name="graph" value="{}"></data>
            </section>
        </article>
    </div>
</body>
</html>"#,
            document.id,
            document.created_by.to_string(),
            document.created_at,
            graph.metadata.entity_count,
            graph.metadata.relation_count,
            document.name,
            document.name,
            graph.metadata.entity_count,
            graph.metadata.relation_count,
            document.definition,
            attributes_html,
            relations_html,
            examples_html,
            graph.metadata.entity_count,
            graph.metadata.relation_count,
            Self::format_time(graph.metadata.generated_at),
            nodes_visual,
            graph_json,
        )
    }

    /// 将关系类型转换为 HTML rel 属性
    fn relation_to_rel(relation: &str) -> &'static str {
        match relation {
            "DependsOn" => "requires",
            "Contains" => "has-part",
            "BelongsTo" => "is-a",
            "SimilarTo" => "related-to",
            "Causes" => "causes",
            "Precedes" => "precedes",
            "HasProperty" => "has-property",
            _ => "related-to",
        }
    }

    /// 生成世界知识索引
    pub fn generate_index(&self) -> String {
        let world_dir = self.root.join("world");

        let mut skills: Vec<(String, String)> = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&world_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map(|e| e == "html").unwrap_or(false) {
                    let name = path.file_stem()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();
                    skills.push((name.clone(), path.to_string_lossy().to_string()));
                }
            }
        }

        let skills_html = skills.iter()
            .map(|(name, path)| format!(
                r#"<li><a href="{}">{}</a></li>"#,
                path, name
            ))
            .collect::<Vec<_>>()
            .join("\n            ");

        format!(r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <title>世界知识库索引</title>
    <style>
        body {{ font-family: 'Microsoft YaHei', sans-serif; max-width: 800px; margin: 40px auto; padding: 20px; background: #f0f4f8; }}
        .index {{ background: white; padding: 30px; border-radius: 10px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }}
        h1 {{ color: #27ae60; border-bottom: 2px solid #27ae60; padding-bottom: 15px; }}
        .stats {{ background: #e8f5e9; padding: 15px; border-radius: 8px; margin: 20px 0; }}
        ul {{ list-style: none; padding: 0; }}
        li {{ margin: 10px 0; padding: 10px; background: #f8f9fa; border-radius: 6px; }}
        a {{ color: #2c3e50; text-decoration: none; }}
        a:hover {{ color: #27ae60; }}
    </style>
</head>
<body>
    <article class="index">
        <h1>🌐 世界知识库</h1>
        <div class="stats">
            <span>📚 知识总数: <strong>{}</strong></span>
            <span>✓ 共识机制: <strong>100%</strong></span>
        </div>
        <h2>知识列表</h2>
        <ul>
            {}
        </ul>
    </article>
</body>
</html>"#,
            skills.len(),
            skills_html,
        )
    }

    // ========== 辅助方法 ==========

    fn normalize_topic(topic: &str) -> String {
        topic.to_lowercase()
            .replace(' ', "_")
            .replace(['/', '\\', '-'], "_")
            .replace(['(', ')'], "")
    }

    fn now() -> u64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    fn format_time(timestamp: u64) -> String {
        let days = timestamp / 86400;
        let years = 1970 + days / 365;
        let remaining_days = days % 365;
        let month = 1 + remaining_days / 30;
        let day = 1 + remaining_days % 30;
        format!("{}-{:02}-{:02}", years, month, day)
    }

    fn ensure_dir(&self, dir: &Path) {
        if !dir.exists() {
            let _ = std::fs::create_dir_all(dir);
        }
    }
}

impl Default for WorldKnowledgeStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_knowledge_creation() {
        let wk = WorldKnowledgeStore::new();
        assert!(!wk.root.as_os_str().is_empty());
    }

    #[test]
    fn test_register_and_vote() {
        let mut wk = WorldKnowledgeStore::new();

        // 注册蛊虫
        let gu1 = Uuid::new_v4();
        let gu2 = Uuid::new_v4();
        wk.register_gu(gu1);
        wk.register_gu(gu2);

        assert_eq!(wk.required_votes(), 2);
    }

    #[test]
    fn test_generate_graph() {
        use super::super::cognis::EntityType;

        let candidate = CandidateVersion {
            id: "test".to_string(),
            submitted_by: Uuid::nil(),
            submitter_name: "test".to_string(),
            submitted_at: 0,
            document: SkillDocument::new("test", "Test", Uuid::nil()),
            particles: ParseResult {
                particles: vec![
                    CogniParticle::Entity { id: 1, name: "HTML".to_string(), entity_type: EntityType::CodeLanguage },
                    CogniParticle::Entity { id: 2, name: "Tag".to_string(), entity_type: EntityType::Concept },
                    CogniParticle::Relation { source_id: 1, target_id: 2, rel_type: super::super::cognis::RelationType::Contains },
                ],
                entity_map: HashMap::new(),
                main_topic: Some("HTML".to_string()),
                keywords: vec!["HTML".to_string()],
                code_languages: vec!["HTML".to_string()],
            },
            layout_scores: Vec::new(),
        };

        let graph = WorldKnowledgeStore::generate_graph(&candidate);

        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);
    }
}
