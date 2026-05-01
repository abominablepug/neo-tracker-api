use crate::AppState;
use crate::error::ApiError;
use axum::{
    Router,
    extract::State,
    response::{IntoResponse, Json},
    routing::post,
};
use bcrypt::{DEFAULT_COST, hash};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use sqlx::query;
use uuid::Uuid;

#[derive(Deserialize)]
struct UserParams {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt(username: &str) -> Result<String, ApiError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    let secret = b"temporary_secret_key";
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
    .map_err(|e| ApiError::Internal(format!("JWT encoding error: {}", e)))
}

async fn register(
    State(state): State<AppState>,
    Json(params): Json<UserParams>,
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

async fn login(
    State(state): State<AppState>,
    Json(params): Json<UserParams>,
) -> Result<impl IntoResponse, ApiError> {
    let user = query!(
        "SELECT username, password_hash FROM users WHERE username = $1",
        params.username
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| ApiError::Internal(format!("Database error: {}", e)))?;

    let user = user.ok_or(ApiError::InvalidInput(
        "Invalid username or password".to_string(),
    ))?;

    if bcrypt::verify(&params.password, &user.password_hash)
        .map_err(|e| ApiError::Internal(format!("Password verification error: {}", e)))?
    {
        let token = create_jwt(&user.username)?;
        Ok(Json(token))
    } else {
        Err(ApiError::InvalidInput(
            "Invalid username or password".to_string(),
        ))
    }
}

pub fn default_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}
