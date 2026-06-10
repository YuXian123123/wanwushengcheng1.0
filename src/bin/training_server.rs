//! 训练生成服务器
//!
//! 提供边训练边生成的 Web 界面
//!
//! # 功能
//! - 实时训练状态推送
//! - 文本到 3D 场景生成
//! - 金币奖罚机制
//!
//! # 启动
//! ```bash
//! cargo run --bin training_server
//! ```
//!
//! # 访问
//! - 训练页面: http://localhost:3030/training.html

use lnn::herness_web::training_router::run_training_server;

#[tokio::main]
async fn main() {
    println!("╔════════════════════════════════════════════╗");
    println!("║     万物生成器 - 训练生成服务器            ║");
    println!("╚════════════════════════════════════════════╝");
    println!();

    // 加载训练数据
    println!("📦 加载训练数据...");
    println!("   数据路径: data/training/scenes_combined.json");
    println!();

    // 启动训练服务器
    let port = 3030;
    run_training_server(port).await;
}
