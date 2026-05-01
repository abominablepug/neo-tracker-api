use crate::AppState;
use axum::response::Json;
use axum::{Router, routing::get};

async fn test() -> Json<String> {
    Json("Connected new routes!".to_string())
}

pub fn mission_routes() -> Router<AppState> {
    Router::new().route("/", get(test))
}
