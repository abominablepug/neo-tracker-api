use crate::error::ApiError;
use crate::models::auth::Claims;
use axum::{extract::Request, http::header, middleware::Next, response::Response};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, ApiError> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(ApiError::InvalidInput(
            "Missing Authorization header".to_string(),
        ))?;
    let auth_str = auth_header
        .to_str()
        .map_err(|_| ApiError::InvalidInput("Invalid Authorization header".to_string()))?;

    if !auth_str.starts_with("Bearer ") {
        return Err(ApiError::InvalidInput(
            "Invalid Authorization header format".to_string(),
        ));
    }

    let token = &auth_str[7..];
    let secret = b"temporary_secret_key";

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| ApiError::InvalidInput("Invalid or expired token".to_string()))?;

    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}
