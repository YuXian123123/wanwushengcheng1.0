//! WebSocket 实时推送
//!
//! # 架构
//!
//! Herness 与 WorldMind 通过协议通信：
//! - 订阅 MetricsUpdate 广播通道
//! - WorldMind 推送指标更新到通道
//! - WebSocket 客户端接收真实监控数据

use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade, Message}, State},
    response::Response,
};
use std::sync::Arc;
use futures::{SinkExt, StreamExt};
use tokio::time::{interval, Duration};
use serde::Serialize;
use super::HernessState;
use super::protocol::MetricsUpdate;

/// 实时数据推送消息
#[derive(Debug, Serialize)]
pub struct RealtimeMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub timestamp: u64,
    pub data: RealtimeData,
}

/// 实时数据
#[derive(Debug, Serialize)]
pub struct RealtimeData {
    // 核心指标
    pub health: f64,
    pub sync_rate: f64,
    pub safety_score: f64,
    pub consciousness_emerged: bool,
    pub emergence_factor: f64,

    // 接入点活动
    pub activity: f64,
    pub perception: f64,
    pub cognition: f64,
    pub action: f64,
    pub communication: f64,
    pub survival: f64,
}

/// WebSocket 处理器
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<HernessState>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// 处理 WebSocket 连接
async fn handle_socket(socket: WebSocket, state: Arc<HernessState>) {
    // 增加客户端计数
    {
        let mut count = state.connected_clients.write().await;
        *count += 1;
    }

    let (mut sender, mut receiver) = socket.split();

    // 订阅指标更新通道
    let mut metrics_rx = state.metrics_tx.subscribe();

    // 发送任务
    let send_task = tokio::spawn(async move {
        loop {
            // 从协议通道接收指标更新
            match metrics_rx.recv().await {
                Ok(metrics) => {
                    // 构建实时指标消息
                    let msg = RealtimeMessage {
                        msg_type: "metrics".to_string(),
                        timestamp: metrics.timestamp,
                        data: RealtimeData {
                            health: metrics.health,
                            sync_rate: metrics.sync_rate,
                            safety_score: metrics.safety_score,
                            consciousness_emerged: metrics.consciousness_emerged,
                            emergence_factor: metrics.emergence_factor,
                            activity: metrics.activity,
                            perception: metrics.perception,
                            cognition: metrics.cognition,
                            action: metrics.action,
                            communication: metrics.communication,
                            survival: metrics.survival,
                        },
                    };

                    let json = match serde_json::to_string(&msg) {
                        Ok(j) => j,
                        Err(_) => continue,
                    };

                    if sender.send(Message::Text(json)).await.is_err() {
                        break;
                    }
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
            }
        }
    });

    // 接收客户端消息
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(_text)) => {
                // 可以处理客户端发来的订阅请求
            }
            Ok(Message::Close(_)) => break,
            Err(_) => break,
            _ => {}
        }
    }

    // 清理
    send_task.abort();
    {
        let mut count = state.connected_clients.write().await;
        *count = count.saturating_sub(1);
    }
}
