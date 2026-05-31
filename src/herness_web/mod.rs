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
//! ```
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
//!     └── 学习目录 ──→ 递归读文件 ──→ 逐文件发送给世界 ──→ 世界内部分配消化
//! ```

pub mod http;
pub mod ws;
pub mod currency_ws;
pub mod handlers;
pub mod protocol;
pub mod learner;
pub mod world_channel;

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
                    let (new_world, transaction) = {
                        let world = state.world.read().await;
                        world.random_action()
                    };

                    // 更新世界状态
                    {
                        let mut world = state.world.write().await;
                        *world = new_world;
                    }

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
                    let new_world = {
                        let world = state.world.read().await;
                        world.update()
                    };

                    // 更新世界状态
                    {
                        let mut w = state.world.write().await;
                        *w = new_world;
                    }
                }
            }
        });
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

        // === WebSocket (高频实时数据) ===
        // 世界状态 WebSocket
        .route("/ws", axum::routing::get(ws::websocket_handler))
        // 货币流水 WebSocket
        .route("/ws/currency", axum::routing::get(currency_ws::currency_websocket_handler))
        // 世界模型通信通道
        .route("/ws/world", axum::routing::get(world_channel::world_channel_handler))

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
