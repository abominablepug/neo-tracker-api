mod routes;

use axum::Router;
use routes::default::default_routes;

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/status", default_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to address");

    println!("Server running on http://localhost:8080");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
