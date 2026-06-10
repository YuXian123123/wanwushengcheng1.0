//! 场景生成 WebSocket 处理器（集成到 Herness）
//!
//! 从 HernessState 中提取 TrainingManager 并处理生成请求

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    extract::State,
};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;

use super::{HernessState, TrainingManager, GenerateRequest, GenerateResponse};
use crate::economy::CoinChange;

/// 生成 WebSocket 端点（集成版）
pub async fn generate_ws_herness(
    ws: WebSocketUpgrade,
    State(state): State<Arc<HernessState>>,
) -> Response {
    // 直接使用 HernessState 中的 Arc<TrainingManager>
    let manager = state.training_manager.clone();
    ws.on_upgrade(|socket| handle_generate_socket(socket, manager))
}

/// 处理生成 WebSocket 连接
async fn handle_generate_socket(socket: WebSocket, manager: Arc<TrainingManager>) {
    let (mut sender, mut receiver) = socket.split();

    println!("[生成WS] 连接建立");

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                // 解析请求
                match serde_json::from_str::<GenerateRequest>(&text) {
                    Ok(request) => {
                        // 执行生成
                        let response = manager.generate(request).await;

                        // 发送响应
                        let json = serde_json::to_string(&response).unwrap_or_default();
                        if sender.send(Message::Text(json)).await.is_err() {
                            break;
                        }

                        // 记录金币变化
                        if response.coin_change.amount != 0.0 {
                            println!(
                                "[生成WS] 用户 {} 金币变化: {} (余额: {})",
                                response.coin_change.change_type,
                                response.coin_change.amount,
                                response.coin_change.new_balance
                            );
                        }
                    }
                    Err(e) => {
                        println!("[生成WS] 解析请求失败: {}", e);
                        let error_response = GenerateResponse {
                            request_id: "unknown".to_string(),
                            success: false,
                            scene: None,
                            error: Some(format!("请求格式错误: {}", e)),
                            coin_change: CoinChange {
                                amount: 0.0,
                                change_type: "none".to_string(),
                                new_balance: 0.0,
                                reason: "请求格式错误".to_string(),
                                entity_count: 0,
                                relation_count: 0,
                            },
                            timestamp: current_timestamp(),
                        };
                        let json = serde_json::to_string(&error_response).unwrap_or_default();
                        let _ = sender.send(Message::Text(json)).await;
                    }
                }
            }
            Ok(Message::Close(_)) => {
                println!("[生成WS] 连接关闭");
                break;
            }
            Err(e) => {
                println!("[生成WS] 错误: {}", e);
                break;
            }
            _ => {}
        }
    }
}

/// 获取当前时间戳（秒）
fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
