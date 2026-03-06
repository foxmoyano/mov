mod config;
mod db;
mod dto;
mod routes;
mod models;
mod handlers;
mod storage;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    init_tracing();

    info!("starting backend...");

    let settings = config::Settings::from_env();

    info!("initializing db pool...");
    let pool = db::init(&settings.database_url)
        .await
        .expect("Failed to connect to DB");
    info!("db ok");

    let s3 = storage::build_client();

    let state = routes::AppState {
        pool,
        settings: settings.clone(),
        s3,
    };

    let app = routes::create_routes(state);

    let addr: SocketAddr = format!("{}:{}", settings.host, settings.port)
        .parse()
        .expect("Invalid HOST or PORT");

    info!("binding {}", addr);
    let listener = TcpListener::bind(addr).await.expect("bind failed");

    info!("serving...");
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");

    warn!("server stopped");
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=info,sqlx=warn"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};

        let mut sigterm = signal(SignalKind::terminate()).expect("Failed to install SIGTERM handler");
        let mut sigint = signal(SignalKind::interrupt()).expect("Failed to install SIGINT handler");

        tokio::select! {
            _ = sigterm.recv() => tracing::info!("SIGTERM received, shutting down..."),
            _ = sigint.recv() => tracing::info!("SIGINT received, shutting down..."),
        }
    }

    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
        tracing::info!("Ctrl+C received, shutting down...");
    }
}