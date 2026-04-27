mod db;
mod routes;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use routes::default::default_routes;

async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url)
        .await
        .expect("Failed to initialize database pool");

    let app = Router::<sqlx::PgPool>::new()
        .route("/", get(hello_world))
        .nest("/status", default_routes())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to address");

    println!("Server running on http://localhost:8080");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
