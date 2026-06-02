use crate::error::ApiError;
use crate::models::auth::Claims;
use axum::{extract::Request, http::header, middleware::Next, response::Response};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, ApiError> {
    // Try Authorization header first, then fall back to `token` cookie.
    let token_opt: Option<String> = if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        let auth_str = auth_header
            .to_str()
            .map_err(|_| ApiError::InvalidInput("Invalid Authorization header".to_string()))?;

        if !auth_str.starts_with("Bearer ") {
            return Err(ApiError::InvalidInput(
                "Invalid Authorization header format".to_string(),
            ));
        }

        Some(auth_str[7..].to_string())
    } else if let Some(cookie_header) = req.headers().get(header::COOKIE) {
        let cookie_str = cookie_header
            .to_str()
            .map_err(|_| ApiError::InvalidInput("Invalid Cookie header".to_string()))?;

        // Parse cookies like "token=...; other=..." and find `token`
        cookie_str
            .split(';')
            .map(|s| s.trim())
            .find_map(|kv| {
                let mut parts = kv.splitn(2, '=');
                let name = parts.next()?;
                let val = parts.next()?;
                if name == "token" {
                    Some(val.to_string())
                } else {
                    None
                }
            })
    } else {
        None
    };

    let token = token_opt.ok_or(ApiError::InvalidInput(
        "Missing Authorization header or token cookie".to_string(),
    ))?;

    let secret = b"temporary_secret_key";

    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| ApiError::InvalidInput("Invalid or expired token".to_string()))?;

    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}
