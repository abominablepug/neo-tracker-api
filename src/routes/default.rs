use axum::response::Json;
use axum::{Router, routing::get};

async fn check_status() -> Json<String> {
    Json("Everything looks good!".to_string())
}

pub fn default_routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route("/", get(check_status))
}
