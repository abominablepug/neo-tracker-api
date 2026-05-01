use crate::AppState;
use crate::error::ApiError;
use axum::{Router, extract::State, response::Json, routing::post};
use bcrypt::{DEFAULT_COST, hash};
use serde::Deserialize;
use sqlx::{query, query_as};

#[derive(Deserialize)]
struct RegisterParams {
    username: String,
    password: String,
}

async fn register(
    State(state): State<AppState>,
    Json(params): Json<RegisterParams>,
) -> Result<Json<String>, ApiError> {
    let existing_user = query!(
        "SELECT username FROM users WHERE username = $1",
        params.username
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    if existing_user.is_some() {
        return Err(ApiError::InvalidInput(
            "Username already exists".to_string(),
        ));
    }

    let password_hash = hash(&params.password, DEFAULT_COST)
        .map_err(|e| ApiError::Internal(format!("Password hashing error: {}", e)))?;

    query!(
        "INSERT INTO users (username, password_hash) VALUES ($1, $2)",
        params.username,
        password_hash
    )
    .execute(&state.db)
    .await
    .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    Ok(Json("User registered successfully".to_string()))
}

pub fn default_routes() -> Router<AppState> {
    Router::new().route("/register", post(register))
}
