mod config;
mod db;
mod dto;
mod routes;
mod models;
mod handlers;

use std::{env, net::SocketAddr};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Logs
    tracing_subscriber::fmt::init();

    // Carga .env (en prod/K8s no falla)
    dotenv::dotenv().ok();

    // DB
    let pool = db::init().await.expect("Failed to connect to DB");

    // Router
    let app = routes::create_routes(pool);

    // Host y port desde env (con defaults)
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Invalid HOST or PORT");

    tracing::info!("🚀 Server started at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    tracing::info!("💤 Shutting down gracefully...");
}