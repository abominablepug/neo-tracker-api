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

async fn test() -> &'static str {
    "Hello, World!"
}

pub fn default_routes() -> Router<AppState> {
    Router::new().route("/", get(test))
}
