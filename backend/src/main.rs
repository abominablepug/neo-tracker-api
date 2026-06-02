mod db;
mod docs;
mod error;
mod middleware;
mod models;
mod routes;

use crate::docs::ApiDoc;
use axum::error_handling::HandleErrorLayer;
use axum::http::StatusCode;
use axum::{Router, routing::get};
use dotenvy::dotenv;
use http::{
    Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use routes::{
    asteroids::asteroid_routes, auth::auth_routes, default::default_routes,
    missions::mission_routes, physics::physics_routes,
};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {
    db: sqlx::PgPool,
    nasa_api_key: String,
}

async fn hello_world() -> &'static str {
    "Connected to Neo Api Tracker!"
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

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([ACCEPT, AUTHORIZATION, CONTENT_TYPE]);

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
        .merge(SwaggerUi::new("/docs").url("/api-docs/openai.json", ApiDoc::openapi()))
        .route("/", get(hello_world))
        .nest("/status", default_routes())
        .nest("/asteroids", asteroid_routes())
        .nest("/physics", physics_routes())
        .nest("/auth", auth_routes())
        .nest("/missions", mission_routes())
        .layer(limit_layer)
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to address");

    println!("Server running on http://localhost:8080");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
