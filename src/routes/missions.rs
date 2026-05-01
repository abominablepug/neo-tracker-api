use crate::AppState;
use crate::error::ApiError;
use crate::middleware::auth::auth_middleware;
use crate::models::{
    asteroids::{NearEarthObjects, OrbitalData},
    auth::Claims,
    db::Mission,
    utils::EstimatedDiameter,
};
use crate::routes::physics::calculate_hohmann_transfer;
use axum::{
    Router,
    extract::{Extension, State},
    middleware::from_fn,
    response::Json,
    routing::{get, post},
};
use serde::Deserialize;
use sqlx::{query, query_as};

#[derive(Deserialize)]
struct MissionParams {
    neo_id: String,
}

async fn get_missions(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Mission>>, ApiError> {
    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::Internal("Invalid User ID".to_string()))?;

    let missions_data = query_as!(
        Mission,
        "SELECT id, user_id, neo_id, launch_date, travel_time_days, status FROM missions WHERE user_id = $1",
        user_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    Ok(Json(missions_data))
}

async fn create_mission(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(params): Json<MissionParams>,
) -> Result<Json<String>, ApiError> {
    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| ApiError::Internal("Invalid User ID".to_string()))?;

    let neo = reqwest::get(format!(
        "https://api.nasa.gov/neo/rest/v1/neo/{}?api_key={}",
        params.neo_id, &state.nasa_api_key
    ))
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to fetch data: {}", e)))?
    .json::<NearEarthObjects>()
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to parse data: {}", e)))?;

    let estimated_diameter: EstimatedDiameter = neo.estimated_diameter;
    let orbital_data: OrbitalData = neo.orbital_data;
    let travel_time =
        calculate_hohmann_transfer(estimated_diameter, &orbital_data).transfer_time_days;

    let new_mission_id = uuid::Uuid::new_v4();
    query!(
        "INSERT INTO missions (id, user_id, neo_id, travel_time_days) VALUES ($1, $2, $3, $4) RETURNING id",
        new_mission_id,
        user_id,
        params.neo_id,
        travel_time
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    Ok(Json(new_mission_id.to_string()))
}

pub fn mission_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_missions))
        .route("/", post(create_mission))
        .layer(from_fn(auth_middleware))
}
