use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Створюємо маршрутизатор
    let app = Router::new()
        .route("/", get(handler));

    // Вказуємо адресу для прослуховування
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Сервер запущено на http://{}", addr);

    // Запускаємо сервер
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Обробник для головної сторінки
async fn handler() -> &'static str {
    "Привіт, світ!"
}