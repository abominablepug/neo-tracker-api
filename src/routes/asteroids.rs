use crate::AppState;
use crate::error::ApiError;
use crate::models::asteroids::NeoResponse;
use axum::{
    Router,
    extract::{Query, State},
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

pub fn default_routes() -> Router<AppState> {
    Router::new().route("/", get(get_asteroids))
}
