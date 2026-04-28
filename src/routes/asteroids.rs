use crate::AppState;
use crate::error::ApiError;
use crate::models::asteroids::NeoResponse;
use axum::response::Json;
use axum::{Router, extract::State, routing::get};
use chrono::{Datelike, Local};

async fn get_asteroids(State(state): State<AppState>) -> Result<Json<NeoResponse>, ApiError> {
    let now = Local::now();
    let now_mod = now.with_month(now.month() + 1).unwrap();

    let asteroids_data = reqwest::get(format!(
        "https://api.nasa.gov/neo/rest/v1/feed?api_key={}&start_date={}&end_date={}",
        &state.nasa_api_key,
        now_mod.format("%Y-%m-%d"),
        now_mod.format("%Y-%m-%d")
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
