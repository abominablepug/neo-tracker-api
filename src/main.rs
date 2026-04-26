mod routes;

use axum::Router;
use dotenvy::dotenv;
use routes::default::default_routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let nasa_api_key = std::env::var("NASA_API_KEY").expect("NASA_API_KEY must be set");

    let app = Router::new().nest("/status", default_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to address");

    println!("Server running on http://localhost:8080");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
