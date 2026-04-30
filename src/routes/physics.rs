use crate::AppState;
use crate::error::ApiError;
use crate::models::asteroids::NearEarthObjects;
use crate::models::physics::{CalculatedEnergy, CalculatedMass, KinematicData};
use crate::models::utils::{DiameterRange, EstimatedDiameter, RelativeVelocity};
use axum::{
    Router,
    extract::{Path, State},
    response::Json,
    routing::get,
};

async fn get_kinetics(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<KinematicData>, ApiError> {
    let asteroid_data = reqwest::get(format!(
        "https://api.nasa.gov/neo/rest/v1/neo/{}?api_key={}",
        id, &state.nasa_api_key
    ))
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to fetch data: {}", e)))?
    .json::<NearEarthObjects>()
    .await
    .map_err(|e| ApiError::Internal(format!("Failed to parse data: {}", e)))?;

    let estimated_diameter: EstimatedDiameter = asteroid_data.estimated_diameter;
    let relative_velocity: RelativeVelocity = asteroid_data
        .close_approach_data
        .get(0)
        .ok_or_else(|| ApiError::Internal("No close approach data available".to_string()))?
        .relative_velocity
        .clone();
    let diameter_km = (estimated_diameter.kilometers.estimated_diameter_min
        + estimated_diameter.kilometers.estimated_diameter_max)
        / 2.0;
    let radius_km = diameter_km / 2.0;
    let volume_km3 = (4.0 / 3.0) * std::f64::consts::PI * radius_km.powi(3);
    let density_kg_per_km3 = 2600.0; // Average density of rocky asteroids

    let mass_kg = volume_km3 * density_kg_per_km3;
    let calculated_mass = CalculatedMass {
        kilograms: mass_kg,
        grams: mass_kg * 1_000_000.0,
        pounds: mass_kg * 2.20462,
    };

    let velocity_km_per_s: f64 = relative_velocity
        .kilometers_per_second
        .parse()
        .map_err(|e| ApiError::Internal(format!("Failed to parse velocity: {}", e)))?;
    let kinetic_energy_joules = 0.5 * mass_kg * velocity_km_per_s.powi(2);
    let calculated_energy = CalculatedEnergy {
        joules: kinetic_energy_joules,
        kilotons_of_tnt: kinetic_energy_joules / 4.184e12,
    };

    let kinetic_data = KinematicData {
        estimated_diameter,
        relative_velocity,
        calculated_mass,
        calculated_energy,
    };

    Ok(Json(kinetic_data))
}

pub fn default_routes() -> Router<AppState> {
    Router::new().route("/kinetics/{id}", get(get_kinetics))
}
