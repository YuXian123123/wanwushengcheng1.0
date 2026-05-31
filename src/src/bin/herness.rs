//! Herness Web Server
//!
//! 世界神经网络监控系统 Web 界面
//!
//! 启动方式: cargo run --bin herness
//! 访问地址: http://localhost:3000
//!
//! # 架构
//!
//! Herness 与 WorldMind 通过协议通信：
//! - WebSocket 客户端接收交易事件
//! - 交易事件由 WorldMind 中真实蛊虫的行为产生
//! - 通过 broadcast 通道广播给所有客户端

use std::net::SocketAddr;
use std::sync::Arc;

use lnn::herness_web::{create_router, HernessState};

#[tokio::main]
async fn main() {
    // 初始化日志
    env_logger::init();

    // 创建共享状态
    let state = Arc::new(HernessState::new());

    // 启动 WorldMind 运行时（产生真实的指标更新和交易事件）
    state.start_worldmind_runtime();

    // 创建路由
    let app = create_router(state);

    // 绑定地址 (后端 API 在 9000 端口)
    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    println!("🌐 Herness API Server running at http://localhost:9000");
    println!("📊 WebSocket endpoints:");
    println!("   - ws://localhost:9000/ws         (世界状态)");
    println!("   - ws://localhost:9000/ws/currency (货币流水 - 真实蛊虫交易)");

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
