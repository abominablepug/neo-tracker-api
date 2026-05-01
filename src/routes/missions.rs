use crate::AppState;
use crate::error::ApiError;
use crate::middleware::auth::auth_middleware;
use crate::models::{auth::Claims, db::CachedMission};
use axum::{
    Router,
    extract::{Extension, State},
    middleware::from_fn,
    response::Json,
    routing::get,
};
use sqlx::query_as;

async fn get_missions(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<String>>, ApiError> {
    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::Internal("Invalid User ID".to_string()))?;

    let missions_data = query_as!(
        CachedMission,
        "SELECT id, launch_date, travel_time_days, status FROM missions WHERE user_id = $1",
        user_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    Ok(Json(missions_data))
}

pub fn mission_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_missions))
        .layer(from_fn(auth_middleware))
}
