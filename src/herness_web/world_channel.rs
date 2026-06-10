//! 世界模型通信通道
//!
//! 处理 Herness 与 WorldMind 之间的双向通信：
//! 1. 连接时发送使用说明书
//! 2. 接收世界模型的指令
//! 3. 执行指令（学习目录、熔断等）
//! 4. 返回执行结果

use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade, Message}, State},
    response::Response,
};
use std::sync::Arc;
use futures::{SinkExt, StreamExt};
use serde::{Serialize, Deserialize};
use super::HernessState;
use super::protocol::{HernessCommand, WorldMindResponse, KnowledgeFileEvent, WorldMindMessage};
use super::learner::Learner;

/// 世界通道消息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WorldChannelMessage {
    /// 使用说明书
    Manual { content: String },
    /// 世界模型的响应
    Response { data: WorldMindResponse },
    /// 知识文件
    KnowledgeFile { data: KnowledgeFileEvent },
    /// 工具调用结果
    ToolResult { call_id: String, success: bool, message: String },
    /// 错误
    Error { message: String },
}

/// 世界模型 WebSocket 处理器
pub async fn world_channel_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<HernessState>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_world_socket(socket, state))
}

/// 处理世界模型 WebSocket 连接
async fn handle_world_socket(socket: WebSocket, state: Arc<HernessState>) {
    let (sender, mut receiver) = socket.split();
    let mut learner = Learner::new();

    // 1. 加载使用说明书
    let manual = Learner::load_manual().unwrap_or_else(|| "说明书加载失败".to_string());

    // 2. 发送说明书到世界模型内部
    {
        let mut world = state.world.write().await;
        world.receive_manual(&manual);
    }

    // 3. 使用 Arc 和 Mutex 共享 sender
    let sender = Arc::new(tokio::sync::Mutex::new(sender));
    let sender_for_halt = sender.clone();
    let sender_for_recv = sender.clone();
    let state_for_halt = state.clone();

    // 4. 发送说明书到客户端
    {
        let manual_msg = WorldChannelMessage::Manual { content: manual };
        if let Ok(json) = serde_json::to_string(&manual_msg) {
            let mut s = sender.lock().await;
            if s.send(Message::Text(json)).await.is_err() {
                return;
            }
        }
    }

    // 5. 启动熔断检测任务
    let halt_task = tokio::spawn(async move {
        loop {
            // 检查是否应该熔断
            let halt_reason = {
                let world = state_for_halt.world.read().await;
                world.should_halt()
            };

            if let Some(reason) = halt_reason {
                let halt_msg = WorldChannelMessage::Response {
                    data: WorldMindResponse::Halt {
                        reason,
                        files_learned: 0,
                        concepts_learned: 0,
                    },
                };
                if let Ok(json) = serde_json::to_string(&halt_msg) {
                    let mut s = sender_for_halt.lock().await;
                    let _ = s.send(Message::Text(json)).await;
                }
                break;
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    // 6. 接收世界模型的指令
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                println!("[WorldChannel] 收到消息: {}", text.chars().take(200).collect::<String>());

                // 解析指令
                match serde_json::from_str::<HernessCommand>(&text) {
                    Ok(command) => {
                        println!("[WorldChannel] 解析命令成功: {:?}", command);
                        let result = execute_command(command, &mut learner, &state).await;

                        // 发送结果
                        if let Some(response) = result {
                            let msg = WorldChannelMessage::Response { data: response };
                            if let Ok(json) = serde_json::to_string(&msg) {
                                let mut s = sender_for_recv.lock().await;
                                if s.send(Message::Text(json)).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("[WorldChannel] 解析命令失败: {}", e);
                        // 尝试解析为其他消息类型
                        if let Ok(world_msg) = serde_json::from_str::<WorldMindMessage>(&text) {
                            // 处理世界模型发来的消息
                            if let WorldMindMessage::KnowledgeFile(event) = world_msg {
                                // 处理知识文件
                                let mut world = state.world.write().await;
                                let result = world.receive_knowledge_file(&event);

                                // 发送处理结果
                                let msg = WorldChannelMessage::ToolResult {
                                    call_id: event.batch_id.clone(),
                                    success: result.success,
                                    message: result.message,
                                };
                                if let Ok(json) = serde_json::to_string(&msg) {
                                    let mut s = sender_for_recv.lock().await;
                                    let _ = s.send(Message::Text(json)).await;
                                }
                            }
                        }
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            Err(e) => {
                println!("[WorldChannel] WebSocket 错误: {:?}", e);
                break;
            }
            _ => {}
        }
    }

    // 清理
    halt_task.abort();
}

/// 执行世界模型的指令
async fn execute_command(
    command: HernessCommand,
    learner: &mut Learner,
    state: &Arc<HernessState>,
) -> Option<WorldMindResponse> {
    match command {
        HernessCommand::LearnDirectory { path, extensions } => {
            println!("[WorldChannel] 执行学习目录: {} (扩展名: {:?})", path, extensions);

            // 扫描目录
            let files = match learner.scan_directory(&path, &extensions) {
                Ok(f) => f,
                Err(e) => {
                    println!("[WorldChannel] 扫描目录失败: {}", e);
                    return Some(WorldMindResponse::Error(e));
                }
            };

            println!("[WorldChannel] 找到 {} 个文件", files.len());

            if files.is_empty() {
                return Some(WorldMindResponse::Error("目录为空或不存在".to_string()));
            }

            let total = files.len();

            // 逐文件发送给世界模型
            for file_path in files {
                // 检查熔断
                if learner.is_halted() {
                    break;
                }

                // 读取文件
                let event = match learner.read_file(&file_path, &path, total) {
                    Ok(e) => e,
                    Err(e) => {
                        println!("[WorldChannel] 读取文件失败: {}", e);
                        continue;
                    }
                };

                println!("[WorldChannel] 处理文件: {}", event.filename);

                // 发送给世界模型
                {
                    let mut world = state.world.write().await;
                    world.receive_knowledge_file(&event);
                }
            }

            let (processed, _) = learner.stats();
            println!("[WorldChannel] 学习完成，处理了 {} 个文件", processed);
            Some(WorldMindResponse::ActionResult {
                success: true,
                message: format!("学习完成，处理了 {} 个文件", processed),
            })
        }

        HernessCommand::LearnFile { path } => {
            let event = match learner.read_file(
                std::path::Path::new(&path),
                &path,
                1
            ) {
                Ok(e) => e,
                Err(e) => {
                    return Some(WorldMindResponse::Error(e));
                }
            };

            // 发送给世界模型
            {
                let mut world = state.world.write().await;
                world.receive_knowledge_file(&event);
            }

            Some(WorldMindResponse::ActionResult {
                success: true,
                message: format!("文件 {} 已学习", path),
            })
        }

        HernessCommand::Halt { reason } => {
            learner.halt(reason.clone());
            Some(WorldMindResponse::Halt {
                reason,
                files_learned: learner.stats().0,
                concepts_learned: 0,
            })
        }

        HernessCommand::SendMessage { content } => {
            // 记录日志
            println!("[WorldMind] {}", content);
            Some(WorldMindResponse::ActionResult {
                success: true,
                message: "消息已记录".to_string(),
            })
        }
    }
}
