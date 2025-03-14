use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod handlers;
mod models;
mod services;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/crypto", get(handlers::get_crypto_data))
        .nest("/static", ServeDir::new("static"));  // Обслуговування статичних файлів

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Сайт запущено: http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}