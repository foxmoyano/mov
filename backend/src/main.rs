mod config;
mod db;
mod routes;
mod models;
mod handlers;
use axum::Server;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let pool = db::init().await.expect("Failed to connect to DB");
    let app = routes::create_routes(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Listening on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
