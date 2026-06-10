//! 货币流水 WebSocket 实时推送
//!
//! # 架构
//!
//! Herness 与 WorldMind 通过协议通信：
//! - 订阅 TransactionEvent 广播通道
//! - WorldMind 推送交易事件到通道
//! - WebSocket 客户端接收真实交易数据
//!
//! # 金币总供应量
//!
//! 总供应量 = 蛊虫数量 × 每只蛊虫出生携带金币(500)

use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade, Message}, State},
    response::Response,
};
use std::sync::Arc;
use futures::{SinkExt, StreamExt};
use tokio::time::{interval, Duration};
use serde::Serialize;
use super::HernessState;
use super::protocol::{TransactionEvent, CurrencyStats};

/// 蛊虫出生携带金币（与 CurrencyConfig.gu_birth_coins 一致）
const GU_BIRTH_COINS: f64 = 500.0;

/// 货币 WebSocket 消息
#[derive(Debug, Serialize)]
pub struct CurrencyMessage {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub data: serde_json::Value,
}

/// 货币 WebSocket 处理器
pub async fn currency_websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<HernessState>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_currency_socket(socket, state))
}

/// 处理货币 WebSocket 连接
async fn handle_currency_socket(socket: WebSocket, state: Arc<HernessState>) {
    let (mut sender, mut receiver) = socket.split();

    // 订阅交易事件通道
    let mut tx_rx = state.transaction_tx.subscribe();

    // 克隆 state 用于 send_task
    let state_for_stats = state.clone();

    // 发送任务
    let send_task = tokio::spawn(async move {
        // 定期发送统计信息
        let mut stats_tick = interval(Duration::from_secs(1));
        let mut total_transactions: u64 = 0;

        loop {
            tokio::select! {
                // 接收真实交易事件
                result = tx_rx.recv() => {
                    match result {
                        Ok(event) => {
                            total_transactions += 1;

                            // 发送交易事件
                            let tx_msg = CurrencyMessage {
                                msg_type: "transaction".to_string(),
                                data: serde_json::to_value(&event).unwrap_or(serde_json::json!(null)),
                            };

                            if let Ok(json) = serde_json::to_string(&tx_msg) {
                                if sender.send(Message::Text(json)).await.is_err() {
                                    break;
                                }
                            }
                        }
                        Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                        Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                    }
                }

                // 定期发送统计
                _ = stats_tick.tick() => {
                    // 从 WorldMind 获取蛊虫实际余额总和
                    let (gu_count, total_supply) = {
                        let world = state_for_stats.world.read().await;
                        let count = world.population() as usize;
                        // 计算所有蛊虫的实际余额总和
                        let supply: f64 = world.gu_registry()
                            .values()
                            .map(|info| info.wallet.balance)
                            .sum();
                        (count, supply)
                    };

                    let stats = CurrencyStats {
                        total_supply,
                        total_transactions,
                        velocity: if total_supply > 0.0 {
                            total_transactions as f64 / total_supply
                        } else {
                            0.0
                        },
                        inflation_rate: 0.02, // 简化计算
                    };

                    let stats_msg = CurrencyMessage {
                        msg_type: "stats".to_string(),
                        data: serde_json::to_value(&stats).unwrap_or(serde_json::json!(null)),
                    };

                    if let Ok(json) = serde_json::to_string(&stats_msg) {
                        if sender.send(Message::Text(json)).await.is_err() {
                            break;
                        }
                    }
                }
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

    send_task.abort();
}
