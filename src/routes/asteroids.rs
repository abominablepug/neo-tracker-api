use crate::AppState;
use crate::error::ApiError;
use crate::models::asteroids::NeoResponse;
use axum::response::Json;
use axum::{Router, extract::State, routing::get};

async fn get_asteroids(State(state): State<AppState>) -> Result<Json<NeoResponse>, ApiError> {
    let asteroids_data = reqwest::get(format!(
        "https://api.nasa.gov/neo/rest/v1/neo/browse?api_key={}",
        &state.nasa_api_key,
    ))
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to fetch data: {}", e)))?
    .json::<NeoResponse>()
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to parse data: {}", e)))?;

    Ok(Json(asteroids_data))
}

pub fn default_routes() -> Router<AppState> {
    Router::new().route("/", get(get_asteroids))
}
