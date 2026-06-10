//! 训练 WebSocket 处理器（集成到 Herness）
//!
//! 从 HernessState 中提取 TrainingManager 并处理 WebSocket 连接

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    extract::State,
};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;

use super::{HernessState, TrainingManager, TrainingCommand, TrainingStatusMessage, UserFeedback};

/// 训练 WebSocket 端点（集成版）
pub async fn training_ws_herness(
    ws: WebSocketUpgrade,
    State(state): State<Arc<HernessState>>,
) -> Response {
    // 直接使用 HernessState 中的 Arc<TrainingManager>
    let manager = state.training_manager.clone();
    ws.on_upgrade(|socket| handle_training_socket(socket, manager))
}

/// 处理训练 WebSocket 连接
async fn handle_training_socket(socket: WebSocket, manager: Arc<TrainingManager>) {
    let (mut sender, mut receiver) = socket.split();

    // 订阅训练状态
    let mut status_rx = manager.subscribe();

    println!("[训练WS] 连接建立");

    // 启动状态推送任务
    let send_task = tokio::spawn(async move {
        while let Ok(status) = status_rx.recv().await {
            let json = serde_json::to_string(&status).unwrap_or_default();
            if sender.send(Message::Text(json)).await.is_err() {
                break;
            }
        }
    });

    // 启动命令接收任务
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            if let Ok(Message::Text(text)) = msg {
                match handle_command(&text, &manager).await {
                    Ok(response) => {
                        println!("[训练WS] 命令成功: {}", response);
                    }
                    Err(e) => {
                        println!("[训练WS] 命令失败: {}", e);
                    }
                }
            }
        }
    });

    // 等待任一任务完成
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    println!("[训练WS] 连接关闭");
}

/// 处理训练命令
async fn handle_command(text: &str, manager: &TrainingManager) -> Result<String, String> {
    let command: TrainingCommand = serde_json::from_str(text)
        .map_err(|e| format!("解析命令失败: {}", e))?;

    match command {
        TrainingCommand::Start { config } => {
            manager.start_training(config).await?;
            Ok("训练已启动".to_string())
        }
        TrainingCommand::Stop => {
            manager.stop_training().await;
            Ok("训练已停止".to_string())
        }
        TrainingCommand::Pause => {
            manager.pause_training().await;
            Ok("训练已暂停".to_string())
        }
        TrainingCommand::Resume => {
            manager.resume_training().await;
            Ok("训练已恢复".to_string())
        }
        TrainingCommand::GetStatus => {
            let state = manager.get_state().await;
            Ok(format!("状态: {:?}", state))
        }
        TrainingCommand::Feedback { request_id, text, is_positive, correct_entities, correct_relations, timestamp } => {
            let feedback = UserFeedback {
                request_id,
                text,
                is_positive,
                correct_entities,
                correct_relations,
                timestamp,
            };
            manager.receive_feedback(feedback).await
        }
        TrainingCommand::SaveModel { path } => {
            manager.save_model(&path).await
        }
        TrainingCommand::LoadModel { path } => {
            manager.load_model(&path).await
        }
    }
}
