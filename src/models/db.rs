use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Serialize)]
pub struct CachedNeo {
    pub nasa_id: String,
    pub name: String,
    pub estimated_diameter_min_km: f64,
    pub estimated_diameter_max_km: f64,
    pub is_potentially_hazardous: bool,
    pub relative_velocity_km_s: f64,
    pub last_updated: NaiveDateTime,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Mission {
    pub id: Uuid,
    pub user_id: Uuid,
    pub neo_id: String,
    pub launch_date: NaiveDateTime,
    pub travel_time_days: f64,
    pub status: String,
}
