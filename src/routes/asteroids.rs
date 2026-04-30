use crate::error::ApiError;
use crate::models::asteroids::NeoResponse;
use crate::{AppState, models::asteroids::NearEarthObjects};
use axum::{
    Router,
    extract::{Path, Query, State},
    response::Json,
    routing::get,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    page: Option<u32>,
    size: Option<u32>,
}

async fn get_asteroids(
    State(state): State<AppState>,
    Query(params): Query<Params>,
) -> Result<Json<NeoResponse>, ApiError> {
    let asteroids_data = reqwest::get(format!(
        "https://api.nasa.gov/neo/rest/v1/neo/browse?api_key={}&page={}&size={}",
        &state.nasa_api_key,
        params.page.unwrap_or(0),
        params.size.unwrap_or(20)
    ))
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to fetch data: {}", e)))?
    .json::<NeoResponse>()
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to parse data: {}", e)))?;

    Ok(Json(asteroids_data))
}

async fn get_asteroid_by_id(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<NearEarthObjects>, ApiError> {
    let asteroid_data = reqwest::get(format!(
        "https://api.nasa.gov/neo/rest/v1/neo/{}?api_key={}",
        id, &state.nasa_api_key
    ))
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to fetch data: {}", e)))?
    .json::<NearEarthObjects>()
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to parse data: {}", e)))?;

    Ok(Json(asteroid_data))
}

pub fn default_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_asteroids))
        .route("/{id}", get(get_asteroid_by_id))
}
