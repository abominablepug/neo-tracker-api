use axum::routing::get;
use axum::{Json, Router};

enum error {}

async fn check_status() -> Json<String> {
    Json("Everything looks good!".to_string())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/status", get(check_status));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to address");

    println!("Server running on http://localhost:8080");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
