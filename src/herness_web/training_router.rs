//! 训练页面路由器
//!
//! 创建包含训练管理器的路由

use axum::Router;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use tower_http::services::ServeDir;

use super::training_manager::TrainingManager;
use super::training_ws;
use super::generate_ws;

/// 创建训练页面路由器
pub fn create_training_router(training_manager: Arc<TrainingManager>) -> Router {
    Router::new()
        // WebSocket 端点
        .route("/ws/training", axum::routing::get(training_ws::training_ws))
        .route("/ws/generate", axum::routing::get(generate_ws::generate_ws))

        // 静态文件服务（训练页面）
        .nest_service("/training", ServeDir::new("herness-web"))

        // CORS 支持
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )

        // 添加训练管理器扩展
        .layer(axum::Extension(training_manager))
}

/// 创建独立的训练服务器
pub async fn run_training_server(port: u16) {
    use axum::Extension;
    use std::net::SocketAddr;

    let training_manager = Arc::new(TrainingManager::new());
    let app = Router::new()
        .route("/ws/training", axum::routing::get(training_ws::training_ws))
        .route("/ws/generate", axum::routing::get(generate_ws::generate_ws))
        .nest_service("/", ServeDir::new("herness-web"))
        .layer(Extension(training_manager))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 训练服务器启动于 http://{}", addr);
    println!("📊 训练页面: http://{}/training.html", addr);

    // 使用 tokio::net::TcpListener 和 axum::serve
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_training_router() {
        let manager = Arc::new(TrainingManager::new());
        let _router = create_training_router(manager);
    }
}
