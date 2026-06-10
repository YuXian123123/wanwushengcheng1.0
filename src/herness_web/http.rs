//! HTTP API 处理器
//!
//! 处理低频更新的数据请求

use axum::{
    extract::{Path, State},
    http::{StatusCode, Uri, header},
    response::{Response, IntoResponse, Html},
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use super::HernessState;

// ============================================================================
// 响应类型
// ============================================================================

/// 世界状态响应
#[derive(Debug, Serialize)]
pub struct WorldStateResponse {
    /// 健康度
    pub health: f64,
    /// 种群数量
    pub population: u64,
    /// 接入点数量
    pub access_point_count: usize,
    /// 健康状态
    pub health_status: String,
    /// 同步率 (黑塔设计)
    pub sync_rate: f64,
    /// 意识是否涌现
    pub consciousness_emerged: bool,
    /// 安全评分 (螺丝咕姆设计)
    pub safety_score: f64,
    /// 信任熵
    pub trust_entropy: f64,
    /// 降级阶段
    pub degradation_phase: String,
}

/// 蛊虫列表响应
#[derive(Debug, Serialize)]
pub struct GuListResponse {
    pub gus: Vec<GuSummary>,
    pub total: usize,
}

/// 蛊虫摘要
#[derive(Debug, Serialize)]
pub struct GuSummary {
    pub id: String,
    pub name: String,
    pub health: f64,
    pub trust_score: f64,
    pub active: bool,
    pub abilities: Vec<String>,
    pub resources: u32,
    pub access_points: AccessPointsStatus,
    /// 颜色（十六进制格式，如 "#FF0000"）
    pub color: String,
    /// 世代数
    pub generation: u32,
    /// 是否为原种
    pub is_primordial: bool,
}

/// 接入点状态（对象格式）
#[derive(Debug, Serialize)]
pub struct AccessPointsStatus {
    pub perception: f64,
    pub action: f64,
    pub communication: f64,
    pub memory: f64,
    pub reasoning: f64,
}

/// 蛊虫详情响应
#[derive(Debug, Serialize)]
pub struct GuDetailResponse {
    pub id: String,
    pub name: String,
    pub health: f64,
    pub trust_score: f64,
    pub expertise: std::collections::HashMap<String, f64>,
    pub access_points: Vec<AccessPointDetail>,
}

/// 接入点详情
#[derive(Debug, Serialize)]
pub struct AccessPointDetail {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub load: f64,
    pub capacity: f64,
    pub connections: usize,
}

/// 统计数据响应
#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub world: WorldStats,
    pub network: NetworkStats,
    pub safety: SafetyStats,
}

#[derive(Debug, Serialize)]
pub struct WorldStats {
    pub total_gus: usize,
    pub active_gus: usize,
    pub total_access_points: usize,
    pub avg_health: f64,
}

#[derive(Debug, Serialize)]
pub struct NetworkStats {
    pub sync_rate: f64,
    pub resonance_strength: f64,
    pub mean_frequency: f64,
}

#[derive(Debug, Serialize)]
pub struct SafetyStats {
    pub safety_score: f64,
    pub trust_entropy: f64,
    pub degradation_phase: String,
}

/// 命令请求
#[derive(Debug, Deserialize)]
pub struct CommandRequest {
    pub command: String,
    pub args: Option<Vec<String>>,
}

/// 命令响应
#[derive(Debug, Serialize)]
pub struct CommandResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

// ============================================================================
// 任务相关请求/响应
// ============================================================================

/// 创建任务请求
#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub description: String,
    pub reward: f64,
    #[serde(default)]
    pub skills: Vec<String>,
    #[serde(default = "default_difficulty")]
    pub difficulty: f64,
}

fn default_difficulty() -> f64 { 0.5 }

/// 任务响应
#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub difficulty: f64,
    pub reward: f64,
    pub required_skills: Vec<String>,
    pub status: String,
    pub assigned_to: Option<String>,
    pub assigned_to_name: Option<String>,
    pub created_at: u64,
    pub completed_at: Option<u64>,
}

/// 任务列表响应
#[derive(Debug, Serialize)]
pub struct TaskListResponse {
    pub tasks: Vec<TaskResponse>,
    pub total: usize,
}

/// 完成任务响应
#[derive(Debug, Serialize)]
pub struct CompleteTaskResponse {
    pub success: bool,
    pub task_id: String,
    pub reward: f64,
    pub gu_id: String,
    pub gu_name: String,
    pub message: String,
}

// ============================================================================
// 处理函数
// ============================================================================

/// 获取世界状态
pub async fn get_world_state(
    State(state): State<Arc<HernessState>>,
) -> Result<Json<WorldStateResponse>, StatusCode> {
    let world = state.world.read().await;

    Ok(Json(WorldStateResponse {
        health: world.health(),
        population: world.population(),
        access_point_count: world.access_point_count(),
        health_status: format!("{:?}", world.health_status()),
        sync_rate: world.consciousness_sync_rate(),
        consciousness_emerged: world.is_conscious(),
        safety_score: world.safety_score(),
        trust_entropy: world.trust_entropy(),
        degradation_phase: format!("{:?}", world.degradation_phase()),
    }))
}

/// 获取蛊虫列表
pub async fn get_gu_list(
    State(state): State<Arc<HernessState>>,
) -> Result<Json<GuListResponse>, StatusCode> {
    let world = state.world.read().await;

    let gus: Vec<GuSummary> = world.all_gus()
        .iter()
        .enumerate()
        .map(|(i, (id, info))| {
            // 从蛊虫神经网络获取真实的健康度
            let health = info.lnn.get_overall_activity();
            let active = health > 0.3;

            // 从蛊虫的技能列表获取真实能力（学习中形成的）
            // 如果没有技能，显示 "无"
            let abilities: Vec<String> = if info.skills.is_empty() {
                vec!["无".to_string()]
            } else {
                info.skills.iter()
                    .map(|skill| skill.name.clone())
                    .collect()
            };

            // 从蛊虫信息获取颜色
            let (color, generation, is_primordial) = {
                let gene = &info.color_gene;
                (gene.display_color().to_hex(), gene.generation, gene.primordial.is_some())
            };

            // 根据颜色确定名称
            let name = info.color_gene.color_name();

            // 从蛊虫神经网络获取真实的接入点状态
            let ap_status = info.lnn.get_access_point_status();

            // 从钱包获取真实资源
            let resources = info.wallet.balance as u32;

            GuSummary {
                id: id.to_string(),
                name,
                health,
                trust_score: info.trust_score,
                active,
                abilities,
                resources,
                access_points: AccessPointsStatus {
                    perception: ap_status.perception,
                    action: ap_status.action,
                    communication: ap_status.communication,
                    memory: ap_status.memory,
                    reasoning: ap_status.reasoning,
                },
                color,
                generation,
                is_primordial,
            }
        })
        .collect();

    Ok(Json(GuListResponse {
        total: gus.len(),
        gus,
    }))
}

/// 获取蛊虫详情
pub async fn get_gu_detail(
    State(state): State<Arc<HernessState>>,
    Path(id): Path<String>,
) -> Result<Json<GuDetailResponse>, StatusCode> {
    let world = state.world.read().await;

    // 解析 UUID
    let gu_id = match uuid::Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    // 查找蛊虫
    let info = match world.get_gu(&gu_id) {
        Some(info) => info,
        None => return Err(StatusCode::NOT_FOUND),
    };

    // 蛊虫名称列表
    let gu_names = [
        "火灵虫", "冰灵虫", "雷灵虫", "风灵虫", "土灵虫",
        "金灵虫", "木灵虫", "水灵虫", "光灵虫", "暗灵虫",
        "炎魔虫", "霜寒虫", "电光虫", "旋风虫", "岩石虫",
        "玄铁虫", "青木虫", "深海虫", "圣光虫", "幽影虫",
        "星辰虫", "月华虫", "日炎虫", "云雾虫", "山岳虫",
    ];

    // 查找蛊虫索引
    let idx = world.all_gus().keys().position(|k| k == &gu_id).unwrap_or(0);

    let health = 0.5 + (idx as f64 * 0.02).min(0.45);

    Ok(Json(GuDetailResponse {
        id: id.clone(),
        name: gu_names[idx % gu_names.len()].to_string(),
        health,
        trust_score: info.trust_score,
        expertise: info.expertise.clone(),
        access_points: info.access_points.iter().enumerate().map(|(i, ap_id)| {
            AccessPointDetail {
                id: ap_id.to_string(),
                name: match i {
                    0 => "Perceive",
                    1 => "Cognitive",
                    2 => "Behavior",
                    3 => "Comm",
                    4 => "Survival",
                    _ => "Unknown",
                }.to_string(),
                active: true,
                load: 0.2 + (i as f64 * 0.1),
                capacity: 1.0,
                connections: 3 + i,
            }
        }).collect(),
    }))
}

/// 获取统计数据
pub async fn get_stats(
    State(state): State<Arc<HernessState>>,
) -> Result<Json<StatsResponse>, StatusCode> {
    let world = state.world.read().await;

    Ok(Json(StatsResponse {
        world: WorldStats {
            total_gus: world.population() as usize,
            active_gus: world.population() as usize,
            total_access_points: world.access_point_count(),
            avg_health: world.health(),
        },
        network: NetworkStats {
            sync_rate: world.consciousness_sync_rate(),
            resonance_strength: world.resonance_field.resonance_strength,
            mean_frequency: world.resonance_field.mean_frequency(),
        },
        safety: SafetyStats {
            safety_score: world.safety_score(),
            trust_entropy: world.trust_entropy(),
            degradation_phase: format!("{:?}", world.degradation_phase()),
        },
    }))
}

/// 获取配置
pub async fn get_config(
    State(state): State<Arc<HernessState>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "emergence_threshold": 0.7,
        "base_frequency": 40.0,
        "survival_threshold": 0.5,
        "min_population": 10,
    })))
}

/// 执行命令
pub async fn execute_command(
    State(state): State<Arc<HernessState>>,
    Json(cmd): Json<CommandRequest>,
) -> Result<Json<CommandResponse>, StatusCode> {
    // 实现命令执行
    Ok(Json(CommandResponse {
        success: true,
        message: format!("命令 '{}' 已执行", cmd.command),
        data: None,
    }))
}

// ============================================================================
// 任务管理 API
// ============================================================================

/// 获取任务列表
pub async fn get_tasks(
    State(state): State<Arc<HernessState>>,
) -> Result<Json<TaskListResponse>, StatusCode> {
    let world = state.world.read().await;
    let tasks = world.get_tasks();

    let task_responses: Vec<TaskResponse> = tasks.iter().map(|t| {
        let (assigned_to, assigned_to_name) = match t.assigned_to {
            Some(gu_id) => {
                let name = world.get_gu(&gu_id)
                    .map(|gu| gu.name.clone())
                    .unwrap_or_else(|| "未知".to_string());
                (Some(gu_id.to_string()), Some(name))
            }
            None => (None, None),
        };

        TaskResponse {
            id: t.id.to_string(),
            name: t.name.clone(),
            description: t.description.clone(),
            difficulty: t.difficulty,
            reward: t.reward,
            required_skills: t.required_skills.clone(),
            status: format!("{:?}", t.status),
            assigned_to,
            assigned_to_name,
            created_at: t.created_at,
            completed_at: t.completed_at,
        }
    }).collect();

    Ok(Json(TaskListResponse {
        total: task_responses.len(),
        tasks: task_responses,
    }))
}

/// 创建任务
pub async fn create_task(
    State(state): State<Arc<HernessState>>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<Json<TaskResponse>, StatusCode> {
    let mut world = state.world.write().await;

    let task_id = if req.skills.is_empty() {
        world.create_task(req.name.clone(), req.description.clone(), req.reward)
    } else {
        world.create_task_with_skills(
            req.name.clone(),
            req.description.clone(),
            req.reward,
            req.skills.clone(),
            req.difficulty,
        )
    };

    // 返回创建的任务
    let task = world.get_tasks().iter()
        .find(|t| t.id == task_id)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TaskResponse {
        id: task.id.to_string(),
        name: task.name.clone(),
        description: task.description.clone(),
        difficulty: task.difficulty,
        reward: task.reward,
        required_skills: task.required_skills.clone(),
        status: format!("{:?}", task.status),
        assigned_to: None,
        assigned_to_name: None,
        created_at: task.created_at,
        completed_at: None,
    }))
}

/// 分配任务给蛊虫
pub async fn assign_task(
    State(state): State<Arc<HernessState>>,
    Path(task_id): Path<String>,
    Json(body): Json<AssignTaskRequest>,
) -> Result<Json<TaskResponse>, StatusCode> {
    let task_uuid = uuid::Uuid::parse_str(&task_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let gu_uuid = uuid::Uuid::parse_str(&body.gu_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut world = state.world.write().await;
    world.assign_task(task_uuid, gu_uuid)
        .map_err(|e| StatusCode::BAD_REQUEST)?;

    // 返回更新后的任务
    let task = world.get_tasks().iter()
        .find(|t| t.id == task_uuid)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let gu_name = world.get_gu(&gu_uuid)
        .map(|gu| gu.name.clone())
        .unwrap_or_else(|| "未知".to_string());

    Ok(Json(TaskResponse {
        id: task.id.to_string(),
        name: task.name.clone(),
        description: task.description.clone(),
        difficulty: task.difficulty,
        reward: task.reward,
        required_skills: task.required_skills.clone(),
        status: format!("{:?}", task.status),
        assigned_to: Some(gu_uuid.to_string()),
        assigned_to_name: Some(gu_name),
        created_at: task.created_at,
        completed_at: task.completed_at,
    }))
}

/// 分配任务请求
#[derive(Debug, Deserialize)]
pub struct AssignTaskRequest {
    pub gu_id: String,
}

/// 完成任务（用户确认）
pub async fn complete_task(
    State(state): State<Arc<HernessState>>,
    Path(task_id): Path<String>,
) -> Result<Json<CompleteTaskResponse>, StatusCode> {
    let task_uuid = uuid::Uuid::parse_str(&task_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut world = state.world.write().await;
    let transaction = world.complete_task(task_uuid)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // 广播交易事件
    let event = crate::herness_web::TransactionEvent {
        id: transaction.id.clone(),
        timestamp: transaction.timestamp,
        from_id: transaction.from_id.clone(),
        from_name: transaction.from_name.clone(),
        from_balance: transaction.from_balance,
        to_id: transaction.to_id.clone(),
        to_name: transaction.to_name.clone(),
        to_balance: transaction.to_balance,
        amount: transaction.amount,
        kind: crate::herness_web::TransactionKind::Deposit,
        reason: transaction.reason.clone(),
        detail: transaction.detail.clone(),
    };
    state.receive_transaction(event);

    Ok(Json(CompleteTaskResponse {
        success: true,
        task_id: task_id.clone(),
        reward: transaction.amount,
        gu_id: transaction.to_id.clone(),
        gu_name: transaction.to_name.clone(),
        message: format!("任务完成，奖励 {} 金币已发放", transaction.amount),
    }))
}

/// 取消任务
pub async fn cancel_task(
    State(state): State<Arc<HernessState>>,
    Path(task_id): Path<String>,
) -> Result<Json<CommandResponse>, StatusCode> {
    let task_uuid = uuid::Uuid::parse_str(&task_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut world = state.world.write().await;
    world.cancel_task(task_uuid)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json(CommandResponse {
        success: true,
        message: "任务已取消".to_string(),
        data: None,
    }))
}

/// 静态文件服务
///
/// 服务前端构建产物（从 herness-web/build 目录）
pub async fn serve_static(uri: Uri) -> impl IntoResponse {
    let path = uri.path();

    // 尝试多个可能的前端构建目录
    let build_dirs: &[&str] = &[
        "herness-web/build",      // 从 src 目录运行
        "../herness-web/build",   // 从其他目录运行
        "./build",                // 从 herness-web 目录运行
    ];

    // 查找存在的构建目录
    let build_dir = build_dirs.iter()
        .find(|dir| std::path::Path::new(*dir).join("index.html").exists())
        .map(|s| std::path::Path::new(s))
        .unwrap_or(std::path::Path::new("herness-web/build"));

    // 确定要服务的文件路径
    let file_path = if path == "/" || path == "" {
        build_dir.join("index.html")
    } else {
        // 移除开头的斜杠
        let relative_path = path.trim_start_matches('/');
        build_dir.join(relative_path)
    };

    // 尝试读取文件
    match std::fs::read(&file_path) {
        Ok(content) => {
            // 根据文件扩展名确定 MIME 类型
            let (mime_type, cache_control) = match file_path.extension().and_then(|e| e.to_str()) {
                Some("html") => ("text/html", "no-cache, no-store, must-revalidate"),
                Some("js") => ("application/javascript", "no-cache, no-store, must-revalidate"),
                Some("css") => ("text/css", "max-age=86400"),
                Some("json") => ("application/json", "no-cache"),
                Some("png") => ("image/png", "max-age=31536000"),
                Some("jpg") | Some("jpeg") => ("image/jpeg", "max-age=31536000"),
                Some("svg") => ("image/svg+xml", "max-age=31536000"),
                Some("ico") => ("image/x-icon", "max-age=31536000"),
                Some("woff") | Some("woff2") => ("font/woff2", "max-age=31536000"),
                _ => ("application/octet-stream", "no-cache"),
            };

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime_type)
                .header(header::CACHE_CONTROL, cache_control)
                .body(content.into())
                .unwrap()
        }
        Err(_) => {
            // 如果文件不存在，返回 index.html（支持 SPA 路由）
            let index_path = build_dir.join("index.html");
            match std::fs::read(index_path) {
                Ok(content) => {
                    Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, "text/html")
                        .body(content.into())
                        .unwrap()
                }
                Err(_) => {
                    // 前端未构建，返回提示信息
                    Html(format!(r#"<!DOCTYPE html>
<html>
<head><title>Herness - 前端未构建</title></head>
<body style="background:#0a0a0f;color:#f0f0f0;font-family:system-ui;padding:40px;">
<h1>🌐 Herness Web</h1>
<p>前端尚未构建，请运行以下命令：</p>
<pre style="background:#1a1a2e;padding:16px;border-radius:8px;overflow-x:auto;">
cd herness-web && npm install && npm run build
</pre>
<p style="margin-top:24px;color:#6b7280;">
后端 API 已就绪：<br>
• HTTP API: <code>http://localhost:9000/api/*</code><br>
• WebSocket: <code>ws://localhost:9000/ws</code><br>
• 货币流水: <code>ws://localhost:9000/ws/currency</code>
</p>
</body>
</html>"#)).into_response()
                }
            }
        }
    }
}

// ============================================================================
// 聊天频道 API
// ============================================================================

/// 聊天频道响应
#[derive(Debug, Serialize)]
pub struct ChatChannelResponse {
    pub id: String,
    pub name: String,
    pub channel_type: String,
    pub online_count: usize,
    pub message_count: usize,
}

/// 获取聊天频道列表
pub async fn get_chat_channels(
    State(state): State<Arc<HernessState>>,
) -> Json<Vec<ChatChannelResponse>> {
    let world = state.world.read().await;
    let chat_system = world.chat_system();

    chat_system.get_all_channels().iter()
        .map(|c| ChatChannelResponse {
            id: c.id.clone(),
            name: c.name.clone(),
            channel_type: format!("{:?}", c.channel_type),
            online_count: c.online_participants.len(),
            message_count: c.messages.len(),
        })
        .collect::<Vec<_>>()
        .into()
}

/// 聊天消息响应
#[derive(Debug, Serialize)]
pub struct ChatMessageResponse {
    pub id: String,
    pub sender_id: String,
    pub sender_name: String,
    pub sender_role: String,
    pub content: serde_json::Value,
    pub sent_at: u64,
}

/// 获取频道消息
pub async fn get_chat_messages(
    State(state): State<Arc<HernessState>>,
    Path(channel_id): Path<String>,
) -> Result<Json<Vec<ChatMessageResponse>>, StatusCode> {
    let world = state.world.read().await;
    let chat_system = world.chat_system();

    let messages = chat_system.get_messages(&channel_id, 100);

    let response: Vec<ChatMessageResponse> = messages.iter()
        .map(|m| ChatMessageResponse {
            id: m.id.clone(),
            sender_id: m.sender_id.to_string(),
            sender_name: m.sender_name.clone(),
            sender_role: format!("{:?}", m.sender_role),
            content: serde_json::to_value(&m.content).unwrap_or(serde_json::Value::Null),
            sent_at: m.sent_at,
        })
        .collect();

    Ok(Json(response))
}

/// 发送消息请求
#[derive(Debug, Deserialize)]
pub struct SendChatMessageRequest {
    pub sender_id: String,
    pub sender_name: String,
    pub content: String,
}

/// 发送消息响应
#[derive(Debug, Serialize)]
pub struct SendChatMessageResponse {
    pub success: bool,
    pub message_id: Option<String>,
    pub error: Option<String>,
}

/// 发送聊天消息
pub async fn send_chat_message(
    State(state): State<Arc<HernessState>>,
    Path(channel_id): Path<String>,
    Json(body): Json<SendChatMessageRequest>,
) -> Result<Json<SendChatMessageResponse>, StatusCode> {
    let sender_id = uuid::Uuid::parse_str(&body.sender_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut world = state.world.write().await;
    let chat_system = world.chat_system_mut();

    let message_id = chat_system.send_message(
        &channel_id,
        sender_id,
        &body.sender_name,
        crate::world::chat_channel::SenderRole::Gu,
        crate::world::chat_channel::MessageContent::Text(body.content),
    );

    Ok(Json(SendChatMessageResponse {
        success: message_id.is_some(),
        message_id,
        error: None,
    }))
}

// ============================================================================
// 学习 API
// ============================================================================

/// 学习目录请求
#[derive(Debug, Deserialize)]
pub struct LearnDirectoryRequest {
    /// 目录路径
    pub path: String,
    /// 文件扩展名（可选，默认 ["md", "txt"]）
    #[serde(default)]
    pub extensions: Vec<String>,
}

/// 学习文件请求
#[derive(Debug, Deserialize)]
pub struct LearnFileRequest {
    /// 文件路径
    pub path: String,
}

/// 学习响应
#[derive(Debug, Serialize)]
pub struct LearnResponse {
    pub success: bool,
    pub files_processed: usize,
    pub message: String,
    pub skills_created: Vec<String>,
}

/// 扫描结果
#[derive(Debug, Serialize)]
pub struct ScanResponse {
    /// 目录路径
    pub path: String,
    /// 文件列表
    pub files: Vec<FileInfo>,
    /// 按扩展名分组统计
    pub extension_stats: std::collections::HashMap<String, usize>,
    /// 总文件数
    pub total: usize,
    /// 是否有需要转换器的文件
    pub has_special_formats: bool,
}

/// 文件信息
#[derive(Debug, Serialize)]
pub struct FileInfo {
    pub path: String,
    pub extension: String,
    pub size: u64,
    /// 是否需要转换器
    pub needs_converter: bool,
    /// 解析器类型
    pub parser_type: String,
}

/// 扫描目录
///
/// 检测目录中的文件格式，返回需要使用的解析器信息
pub async fn scan_directory(
    Json(req): Json<LearnDirectoryRequest>,
) -> Result<Json<ScanResponse>, StatusCode> {
    use super::learner::Learner;
    use super::parser::ParserRegistry;

    let path_str = &req.path;
    let path = std::path::Path::new(path_str);
    if !path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let learner = Learner::new();
    let registry = ParserRegistry::new();

    // 获取支持的扩展名
    let supported = registry.supported_extensions();

    // 扫描所有文件
    let files = match learner.scan_directory(path_str, &[]) {
        Ok(f) => f,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let mut file_infos: Vec<FileInfo> = Vec::new();
    let mut extension_stats: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    let mut has_special_formats = false;

    for file_path in files {
        let ext = file_path.extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        let size = file_path.metadata().map(|m| m.len()).unwrap_or(0);

        // 判断是否需要转换器
        let needs_converter = !["md", "txt", "html", "css", "js", "ts", "json", "yaml", "xml"].contains(&ext.as_str());
        if needs_converter {
            has_special_formats = true;
        }

        // 判断解析器类型
        let parser_type = if supported.contains(&ext) {
            if needs_converter { "special" } else { "common" }.to_string()
        } else {
            "unknown".to_string()
        };

        // 统计扩展名
        *extension_stats.entry(ext.clone()).or_insert(0) += 1;

        file_infos.push(FileInfo {
            path: file_path.to_string_lossy().to_string(),
            extension: ext,
            size,
            needs_converter,
            parser_type,
        });
    }

    let total = file_infos.len();

    Ok(Json(ScanResponse {
        path: path_str.clone(),
        files: file_infos,
        extension_stats,
        total,
        has_special_formats,
    }))
}

/// 学习目录
///
/// 扫描目录中的所有文件，发送给世界模型学习
pub async fn learn_directory(
    State(state): State<Arc<HernessState>>,
    Json(req): Json<LearnDirectoryRequest>,
) -> Result<Json<LearnResponse>, StatusCode> {
    use super::learner::Learner;
    use super::protocol::KnowledgeFileEvent;

    let path_str = &req.path;
    let path = std::path::Path::new(path_str);
    if !path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let mut learner = Learner::new();

    // 如果没有指定扩展名，使用所有支持的格式
    let extensions = if req.extensions.is_empty() {
        learner.supported_extensions()
    } else {
        req.extensions
    };

    let files = match learner.scan_directory(path_str, &extensions) {
        Ok(f) => f,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    if files.is_empty() {
        return Ok(Json(LearnResponse {
            success: true,
            files_processed: 0,
            message: "目录中没有可学习的文件".to_string(),
            skills_created: vec![],
        }));
    }

    let total = files.len();
    let mut processed = 0;
    let mut skills = std::collections::HashSet::new();

    for file_path in files {
        let event = match learner.read_file(&file_path, path_str, total) {
            Ok(e) => e,
            Err(_) => continue,
        };

        let mut world = state.world.write().await;
        let result = world.receive_knowledge_file(&event);
        if result.success {
            processed += 1;
            if let Some(skill_name) = result.skill_name {
                skills.insert(skill_name);
            }
        }
    }

    Ok(Json(LearnResponse {
        success: processed > 0,
        files_processed: processed,
        message: format!("成功学习 {}/{} 个文件", processed, total),
        skills_created: skills.into_iter().collect(),
    }))
}

/// 学习单个文件
pub async fn learn_file(
    State(state): State<Arc<HernessState>>,
    Json(req): Json<LearnFileRequest>,
) -> Result<Json<LearnResponse>, StatusCode> {
    use super::learner::Learner;

    let path = std::path::Path::new(&req.path);
    if !path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let root_dir = path.parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| req.path.clone());

    let mut learner = Learner::new();
    let event = match learner.read_file(path, &root_dir, 1) {
        Ok(e) => e,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    let mut world = state.world.write().await;
    let result = world.receive_knowledge_file(&event);

    let skills = result.skill_name.map(|s| vec![s]).unwrap_or_default();

    Ok(Json(LearnResponse {
        success: result.success,
        files_processed: if result.success { 1 } else { 0 },
        message: result.message,
        skills_created: skills,
    }))
}
