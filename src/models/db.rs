use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct CachedNeo {
    pub nasa_id: String,
    pub name: String,
    pub estimated_diameter_km: f32,
    pub is_potentially_hazardous: bool,
    pub relative_velocity_km_s: f32,
    pub last_updated: NaiveDateTime,
}
