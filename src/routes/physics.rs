use crate::AppState;
use axum::{Router, routing::get};

async fn test() -> &'static str {
    "Hello, Physics!"
}

pub fn default_routes() -> Router<AppState> {
    Router::new().route("/", get(test))
}
