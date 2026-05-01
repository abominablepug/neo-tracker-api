use crate::AppState;
use crate::error::ApiError;
use crate::models::auth::Claims;
use axum::{
    Router,
    extract::State,
    http::{HeaderMap, StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Json},
    routing::{get, post},
};
use bcrypt::{DEFAULT_COST, hash};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Deserialize;
use sqlx::query;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
struct UserParams {
    username: String,
    password: String,
}

fn create_jwt(user_id: Uuid) -> Result<String, ApiError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
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

#[utoipa::path(
    post,
    path = "/register",
    tag = "Authentication",
    request_body = UserParams,
    responses(
        (status = 200, description = "User registered successfully", body = String),
        (status = 400, description = "Invalid input", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
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

#[utoipa::path(
    post,
    path = "/login",
    tag = "Authentication",
    request_body = UserParams,
    responses(
        (status = 200, description = "Login successful, returns JWT token", body = String),
        (status = 400, description = "Invalid input", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
async fn login(
    State(state): State<AppState>,
    Json(params): Json<UserParams>,
) -> Result<impl IntoResponse, ApiError> {
    let user = query!(
        "SELECT id, username, password_hash FROM users WHERE username = $1",
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
        let token = create_jwt(user.id)?;

        let mut headers = HeaderMap::new();
        let cookie_value = format!(
            "token={}; HttpOnly; Path=/; Max-Age=86400; SameSite=Strict",
            token
        );

        headers.insert(
            SET_COOKIE,
            cookie_value
                .parse()
                .map_err(|e| ApiError::Internal(format!("Failed to set cookie header: {}", e)))?,
        );

        Ok((StatusCode::OK, headers, Json(token)))
    } else {
        Err(ApiError::InvalidInput(
            "Invalid username or password".to_string(),
        ))
    }
}

#[utoipa::path(
    get,
    path = "/logout",
    tag = "Authentication",
    responses(
        (status = 200, description = "Logged out successfully", body = String)
    )
)]
async fn logout() -> impl IntoResponse {
    let mut headers = HeaderMap::new();

    headers.insert(
        SET_COOKIE,
        "token=; HttpOnly; Path=/; Max-Age=0".parse().unwrap(),
    );
    (
        StatusCode::OK,
        headers,
        Json("Logged out successfully".to_string()),
    )
}

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", get(logout))
}
