use crate::AppState;
use axum::response::Json;
use axum::{Router, routing::get};
// no local utoipa derives needed in this file

#[utoipa::path(
    get,
    path = "/",
    tag = "Default",
    description = "Check the status of the Neo Tracker API.",
    responses(
        (status = 200, description = "API is running and healthy", body = String)
    )
)]
async fn check_status() -> Json<String> {
    Json("Everything looks good!".to_string())
}

pub fn default_routes() -> Router<AppState> {
    Router::new().route("/", get(check_status))
}
