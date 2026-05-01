mod db;
mod error;
mod models;
mod routes;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use routes::{asteroids, default};

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

    let app = Router::new()
        .route("/", get(hello_world))
        .nest("/status", default::default_routes())
        .nest("/asteroids", asteroids::default_routes())
        .nest("/physics", routes::physics::default_routes())
        .nest("/auth", routes::auth::default_routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to address");

    println!("Server running on http://localhost:8080");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
