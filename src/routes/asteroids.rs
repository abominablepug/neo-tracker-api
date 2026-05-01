use crate::error::ApiError;
use crate::middleware::auth::auth_middleware;
use crate::{
    AppState,
    models::{
        asteroids::{NearEarthObjects, NeoResponse},
        db::CachedNeo,
    },
};
use axum::{
    Router,
    extract::{Path, Query, State},
    middleware::from_fn,
    response::Json,
    routing::get,
};
use serde::Deserialize;
use sqlx::{query, query_as};

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

async fn get_saved_asteroids(
    State(state): State<AppState>,
) -> Result<Json<Vec<CachedNeo>>, ApiError> {
    let saved_asteroids = query_as!(CachedNeo, "SELECT * FROM neos")
        .fetch_all(&state.db)
        .await
        .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    Ok(Json(saved_asteroids))
}

pub fn default_routes() -> Router<AppState> {
    Router::new()
        .route("/saved", get(get_saved_asteroids))
        .layer(from_fn(auth_middleware))
        .route("/", get(get_asteroids))
        .route("/{id}", get(get_asteroid_by_id))
}
