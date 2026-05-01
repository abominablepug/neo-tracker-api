mod db;
mod error;
mod middleware;
mod models;
mod routes;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use http::StatusCode;
use routes::{
    asteroids::asteroid_routes, auth::auth_routes, default::default_routes,
    missions::mission_routes, physics::physics_routes,
};
use std::time::Duration;
use tower::{ServiceBuilder, limit::RateLimitLayer};
use tower_http::error_handling::HandleErrorLayer;

#[derive(Clone)]
pub struct AppState {
    db: sqlx::PgPool,
    nasa_api_key: String,
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let nasa_api_key = std::env::var("NASA_API_KEY").expect("NASA_API_KEY must be set");
    let pool = db::init_pool(&database_url)
        .await
        .expect("Failed to initialize database pool");

    let state = AppState {
        db: pool,
        nasa_api_key,
    };

    let limit_layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|err| async move {
            (
                StatusCode::TOO_MANY_REQUESTS,
                format!("Rate limit exceeded or server busy: {}", err),
            )
        }))
        .buffer(1024)
        .rate_limit(5, Duration::from_secs(1));

    let app = Router::new()
        .route("/", get(hello_world))
        .nest("/status", default_routes())
        .nest("/asteroids", asteroid_routes())
        .nest("/physics", physics_routes())
        .nest("/auth", auth_routes())
        .nest("/missions", mission_routes())
        .layer(limit_layer)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to address");

    println!("Server running on http://localhost:8080");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
