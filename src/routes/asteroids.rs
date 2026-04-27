use crate::AppState;
use crate::error::ApiError;
use axum::response::Json;
use axum::{Router, extract::State, routing::get};
use dotenvy::dotenv;
use serde_json::Value;
use std::env;

async fn get_asteroids(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let asteroids_data = reqwest::get(format!(
        "https://api.nasa.gov/neo/rest/v1/feed?api_key={}",
        &state.nasa_api_key
    ))
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to fetch data: {}", e)))?
    .json::<Value>()
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to parse data: {}", e)))?;

    Ok(Json(asteroids_data))
}

pub fn default_routes() -> Router<AppState> {
    dotenv().ok();
    let nasa_api_key = env::var("NASA_API_KEY").expect("NASA_API_KEY must be set");

    Router::new().route("/", get(get_asteroids))
}
