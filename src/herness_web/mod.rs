//! Herness Web API 服务器
//!
//! 提供世界神经网络的 Web 监控界面
//!
//! # 架构原则
//!
//! Herness 与 WorldMind 之间通过**协议通信**，不直接引用：
//! - Claude Code ↔ Claude API：HTTP/JSON 协议
//! - Herness ↔ WorldMind：消息协议（broadcast 通道）
//!
//! WorldMind 是独立运行的智能体，Herness 只是观察者/监控终端。
//!
//! # 数据流
//!
//! ```text
//! WorldMind 运行时
//!     │
//!     ├── MetricsUpdate ──→ metrics_tx ──→ WebSocket (/ws)
//!     │
//!     └── TransactionEvent ──→ transaction_tx ──→ WebSocket (/ws/currency)
//!
//! 用户输入流
//!     │
//!     ├── 用户输入 ──→ 世界模型理解 ──→ 返回指令 ──→ Herness 执行
//!     │                                      │
//!     │                                      └── 无法理解时退到规则匹配
//!     │
//!     └── 学习目录 ──→ 文件解析器 ──→ Markdown 转换 ──→ 世界学习
//! ```
//!
//! # 文件解析器插件系统
//!
//! ```text
//! 文件检测 → 格式分类 → 解析器插件 → 转换器 → Markdown → 学习
//! ```
//!
//! - **通用解析器**: 源码文件 (md, txt, html, js, css, java, py, rs, etc.)
//! - **特殊解析器**: 二进制格式 (parquet, pdf, docx, etc.)

pub mod parser;
pub mod http;
pub mod ws;
pub mod currency_ws;
pub mod handlers;
pub mod protocol;
pub mod learner;
pub mod world_channel;
pub mod training_manager;
pub mod training_ws;
pub mod generate_ws;
pub mod training_ws_herness;
pub mod generate_ws_herness;
pub mod training_router;

// 重导出解析器
pub use parser::{FileParser, ParserRegistry, ParsedContent, ContentType};
pub use training_manager::{TrainingManager, TrainingConfig, TrainingState, TrainingCommand, TrainingStatusMessage, GenerateRequest, GenerateResponse, UserFeedback};

use axum::Router;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{CorsLayer, Any};
use crate::world::WorldMind;
use crate::world::{GuWallet, GuBehavior, Task, Resource, Skill, Knowledge as GuKnowledge, ActionResult};
use crate::world::event::{TransactionData, TransactionKind as WorldTransactionKind};
use uuid::Uuid;

// 重导出协议类型，方便外部使用
pub use protocol::{TransactionEvent, MetricsUpdate, WorldSnapshot, GuSnapshot};
pub use protocol::{TransactionKind, HernessCommand, WorldMindResponse, HernessAction, KnowledgeFileEvent};

// === Herness 状态 ===

/// API 共享状态
pub struct HernessState {
    /// 世界状态（用于 HTTP API 查询，但不直接在 WebSocket 中访问）
    pub world: RwLock<WorldMind>,
    /// 连接的客户端数量
    pub connected_clients: RwLock<usize>,
    /// 日志缓冲
    pub log_buffer: RwLock<Vec<LogEntry>>,
    /// 交易事件广播通道（从 WorldMind 接收）
    pub transaction_tx: tokio::sync::broadcast::Sender<TransactionEvent>,
    /// 指标更新广播通道（从 WorldMind 接收）
    pub metrics_tx: tokio::sync::broadcast::Sender<MetricsUpdate>,
    /// 训练管理器（Arc 包装，支持共享）
    pub training_manager: Arc<TrainingManager>,
}

/// 日志条目
#[derive(Debug, Clone, serde::Serialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

impl HernessState {
    pub fn new() -> Self {
        let mut world = WorldMind::new();

        // 初始化一些模拟蛊虫，使数据更真实
        // 注册25个蛊虫
        for _ in 0..25 {
            let gu_id = Uuid::new_v4();
            world = world.register_gu(gu_id);
        }

        let (transaction_tx, _) = tokio::sync::broadcast::channel(1000);
        let (metrics_tx, _) = tokio::sync::broadcast::channel(1000);

        Self {
            world: RwLock::new(world),
            connected_clients: RwLock::new(0),
            log_buffer: RwLock::new(Vec::new()),
            transaction_tx,
            metrics_tx,
            training_manager: Arc::new(TrainingManager::new()),
        }
    }

    /// 接收交易事件（由 WorldMind 运行时调用）
    pub fn receive_transaction(&self, event: TransactionEvent) {
        // 广播给所有 WebSocket 客户端
        let _ = self.transaction_tx.send(event);
    }

    /// 接收指标更新（由 WorldMind 运行时调用）
    pub fn receive_metrics(&self, metrics: MetricsUpdate) {
        // 广播给所有 WebSocket 客户端
        let _ = self.metrics_tx.send(metrics);
    }

    /// 启动 WorldMind 运行时（产生真实的指标更新和交易事件）
    pub fn start_worldmind_runtime(self: &Arc<Self>) {
        let state = self.clone();

        tokio::spawn(async move {
            let mut tick = tokio::time::interval(std::time::Duration::from_millis(500)); // 2Hz
            let mut phase: f64 = 0.0;
            let mut learner = learner::Learner::new();

            loop {
                tick.tick().await;
                phase = (phase + 0.1) % (std::f64::consts::TAU);

                // 1. 生成指标更新
                {
                    let world = state.world.read().await;

                    let base_health = world.health();
                    let base_sync = world.consciousness_sync_rate();
                    let base_safety = world.safety_score();

                    // 添加自然的波动
                    let health_fluctuation = (phase.sin() * 0.05).abs();
                    let sync_fluctuation = ((phase * 1.3).sin() * 0.08).abs();

                    let health = (base_health + health_fluctuation).min(1.0);
                    let sync_rate = (base_sync + sync_fluctuation).min(1.0);
                    let safety_score = (base_safety + (phase.cos() * 0.03).abs()).min(1.0);

                    let emergence_factor = (sync_rate * health).sqrt();
                    let consciousness_emerged = sync_rate > 0.7 && emergence_factor > 0.5;

                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();

                    let metrics = MetricsUpdate {
                        timestamp,
                        health,
                        sync_rate,
                        activity: 0.4 + (phase.sin() * 0.3).abs(),
                        perception: 0.6 + ((phase * 1.1).sin() * 0.2).abs(),
                        cognition: 0.5 + ((phase * 0.9).sin() * 0.3).abs(),
                        action: 0.7 + ((phase * 1.5).sin() * 0.15).abs(),
                        communication: 0.4 + ((phase * 0.8).sin() * 0.25).abs(),
                        survival: 0.8 + ((phase * 1.2).sin() * 0.1).abs(),
                        safety_score,
                        consciousness_emerged,
                        emergence_factor,
                    };

                    state.receive_metrics(metrics);
                }

                // 2. 每秒执行一次蛊虫行为（产生交易）
                if (phase * 10.0) as u32 % 2 == 0 {
                    let transaction = {
                        let mut world = state.world.write().await;
                        world.random_action()
                    };

                    // 如果产生了交易事件，广播给客户端
                    if let Some(tx) = transaction {
                        // 转换 world::event::TransactionKind 到 protocol::TransactionKind
                        let kind = match tx.kind {
                            WorldTransactionKind::Deposit => TransactionKind::Deposit,
                            WorldTransactionKind::Withdraw => TransactionKind::Withdraw,
                            WorldTransactionKind::Transfer => TransactionKind::Transfer,
                        };

                        let event = TransactionEvent {
                            id: tx.id,
                            timestamp: tx.timestamp,
                            from_id: tx.from_id,
                            from_name: tx.from_name,
                            from_balance: tx.from_balance,
                            to_id: tx.to_id,
                            to_name: tx.to_name,
                            to_balance: tx.to_balance,
                            amount: tx.amount,
                            kind,
                            reason: tx.reason,
                            detail: tx.detail,
                        };
                        state.receive_transaction(event);
                    }
                }

                // 3. 定期执行世界更新（自动任务分配等）
                if (phase * 10.0) as u32 % 4 == 0 {
                    let mut world = state.world.write().await;
                    world.update();
                }

                // 4. 检测并执行学习任务
                if (phase * 10.0) as u32 % 4 == 0 {
                    Self::process_learning_tasks(&state, &mut learner).await;
                }
            }
        });
    }

    /// 处理学习任务
    ///
    /// 检查任务描述中是否包含学习路径，如果有则执行学习
    async fn process_learning_tasks(state: &Arc<Self>, learner: &mut learner::Learner) {
        // 查找需要进行学习的任务
        let learning_tasks: Vec<(Uuid, String)> = {
            let world = state.world.read().await;
            world.get_tasks().iter()
                .filter(|t| t.status == crate::world::behavior::TaskStatus::InProgress)
                .filter_map(|t| {
                    // 解析任务描述中的学习路径
                    Self::extract_learning_path(&t.description).map(|path| (t.id, path))
                })
                .collect()
        };

        for (task_id, path) in learning_tasks {
            println!("[Herness] 检测到学习任务，路径: {}", path);

            // 检查是否已熔断
            if learner.is_halted() {
                println!("[Herness] 学习已熔断，跳过");
                break;
            }

            // 扫描目录
            let files = match learner.scan_directory(&path, &[]) {
                Ok(f) => f,
                Err(e) => {
                    println!("[Herness] 扫描目录失败: {}", e);
                    continue;
                }
            };

            if files.is_empty() {
                println!("[Herness] 目录为空或不存在: {}", path);
                continue;
            }

            println!("[Herness] 找到 {} 个文件，开始学习", files.len());
            let total = files.len();

            // 逐文件发送给世界意识
            for file_path in files {
                if learner.is_halted() {
                    break;
                }

                // 读取并解析文件
                let event = match learner.read_file(&file_path, &path, total) {
                    Ok(e) => e,
                    Err(e) => {
                        println!("[Herness] 读取文件失败: {}", e);
                        continue;
                    }
                };

                println!("[Herness] 学习文件: {}", event.filename);

                // 发送给世界意识
                {
                    let mut world = state.world.write().await;
                    world.receive_knowledge_file(&event);
                }
            }

            let (processed, _) = learner.stats();
            println!("[Herness] 学习完成，处理了 {} 个文件", processed);

            // 标记任务完成
            {
                let mut world = state.world.write().await;
                let _ = world.complete_task(task_id);
            }
        }
    }

    /// 从任务描述中提取学习路径
    ///
    /// 支持格式：
    /// - "学习 D:/path/to/dir"
    /// - "Learn D:/path/to/dir"
    /// - "D:/path/to/dir" (直接是路径)
    fn extract_learning_path(description: &str) -> Option<String> {
        let desc = description.trim();

        // 尝试匹配 "学习 xxx" 或 "Learn xxx"
        if let Some(path) = desc.strip_prefix("学习 ") {
            return Some(path.trim().to_string());
        }
        if let Some(path) = desc.strip_prefix("Learn ") {
            return Some(path.trim().to_string());
        }
        if let Some(path) = desc.strip_prefix("learn ") {
            return Some(path.trim().to_string());
        }

        // 检查是否直接是路径（包含 / 或 \）
        if desc.contains('/') || desc.contains('\\') {
            // 简单验证：路径应该存在或看起来像路径
            if std::path::Path::new(desc).exists() {
                return Some(desc.to_string());
            }
        }

        None
    }
}

/// 创建 API 路由
pub fn create_router(state: Arc<HernessState>) -> Router {
    Router::new()
        // === HTTP API (低频数据) ===
        // 世界状态
        .route("/api/world", axum::routing::get(handlers::get_world_state))
        // 蛊虫列表
        .route("/api/gus", axum::routing::get(handlers::get_gu_list))
        // 蛊虫详情
        .route("/api/gu/:id", axum::routing::get(handlers::get_gu_detail))
        // 统计数据
        .route("/api/stats", axum::routing::get(handlers::get_stats))
        // 执行命令
        .route("/api/command", axum::routing::post(handlers::execute_command))
        // 获取配置
        .route("/api/config", axum::routing::get(handlers::get_config))

        // === 任务管理 API ===
        // 任务列表
        .route("/api/tasks", axum::routing::get(handlers::get_tasks))
        // 创建任务
        .route("/api/tasks", axum::routing::post(handlers::create_task))
        // 分配任务
        .route("/api/tasks/:id/assign", axum::routing::post(handlers::assign_task))
        // 完成任务
        .route("/api/tasks/:id/complete", axum::routing::post(handlers::complete_task))
        // 取消任务
        .route("/api/tasks/:id/cancel", axum::routing::post(handlers::cancel_task))

        // === 学习 API ===
        // 扫描目录（检测文件格式）
        .route("/api/learn/scan", axum::routing::post(handlers::scan_directory))
        // 学习目录
        .route("/api/learn/directory", axum::routing::post(handlers::learn_directory))
        // 学习单个文件
        .route("/api/learn/file", axum::routing::post(handlers::learn_file))

        // === 聊天频道 API ===
        // 获取频道列表
        .route("/api/chat/channels", axum::routing::get(handlers::get_chat_channels))
        // 获取频道消息
        .route("/api/chat/channels/:id/messages", axum::routing::get(handlers::get_chat_messages))
        // 发送消息
        .route("/api/chat/channels/:id/messages", axum::routing::post(handlers::send_chat_message))

        // === WebSocket (高频实时数据) ===
        // 世界状态 WebSocket
        .route("/ws", axum::routing::get(ws::websocket_handler))
        // 货币流水 WebSocket
        .route("/ws/currency", axum::routing::get(currency_ws::currency_websocket_handler))
        // 世界模型通信通道
        .route("/ws/world", axum::routing::get(world_channel::world_channel_handler))
        // 训练状态 WebSocket
        .route("/ws/training", axum::routing::get(training_ws_herness::training_ws_herness))
        // 场景生成 WebSocket
        .route("/ws/generate", axum::routing::get(generate_ws_herness::generate_ws_herness))

        // 静态文件
        .fallback(handlers::serve_static)

        // CORS 支持 - 允许所有来源和方法
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )
        .with_state(state)
}
