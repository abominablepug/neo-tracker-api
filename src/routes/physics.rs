use crate::AppState;
use crate::error::ApiError;
use crate::models::asteroids::{NearEarthObjects, OrbitalData};
use crate::models::physics::{CalculatedEnergy, CalculatedMass, HohmannTransfer, KinematicData};
use crate::models::utils::{EstimatedDiameter, RelativeVelocity};
use axum::{
    Router,
    extract::{Path, State},
    response::Json,
    routing::get,
};
use std::f64::consts::PI;

const SECOND_IN_DAY: f64 = 86400.0;
const DENSITY_KG_PER_KM3: f64 = 2.6e12;
const SUN_GRAVITATIONAL_PARAMETER_KM3_S2: f64 = 1.32712440018e11;
const EARTH_GRAVITATIONAL_PARAMETER_KM3_S2: f64 = 3.986004418e5;
const EARTH_ORBITAL_RADIUS_KM: f64 = 149.6e6;
const EARTH_ORBITAL_VELOCITY_KM_PER_S: f64 = 29.78;
const KM_PER_AU: f64 = 149597870.7;

fn calculate_radius(estimated_diameter: &EstimatedDiameter) -> f64 {
    let diameter_km = (estimated_diameter.kilometers.estimated_diameter_min
        + estimated_diameter.kilometers.estimated_diameter_max)
        / 2.0;
    diameter_km / 2.0
}

fn calculate_mass(estimated_diameter: &EstimatedDiameter) -> CalculatedMass {
    let radius_km = calculate_radius(estimated_diameter);
    let volume_km3 = (4.0 / 3.0) * PI * radius_km.powi(3);
    let density_kg_per_km3 = DENSITY_KG_PER_KM3;

    let mass_kg = volume_km3 * density_kg_per_km3;
    CalculatedMass {
        kilograms: mass_kg,
        grams: mass_kg * 1000.0,
        pounds: mass_kg * 2.20462,
    }
}

fn calculate_energy(
    calculated_mass: &CalculatedMass,
    relative_velocity: &RelativeVelocity,
) -> CalculatedEnergy {
    let velocity_km_per_s: f64 = relative_velocity
        .kilometers_per_second
        .parse()
        .unwrap_or(0.0)
        * 1000.0; // Convert km/s to m/s
    let kinetic_energy_joules = 0.5 * calculated_mass.kilograms * velocity_km_per_s.powi(2);
    CalculatedEnergy {
        joules: kinetic_energy_joules,
        kilotons_of_tnt: kinetic_energy_joules / 4.184e12,
    }
}

fn calculate_vis_viva(semi_major_axis: f64, transfer_path_axis: f64) -> f64 {
    (SUN_GRAVITATIONAL_PARAMETER_KM3_S2 * (2.0 / semi_major_axis - 1.0 / transfer_path_axis)).sqrt()
}

fn calculate_hohmann_transfer(
    estimated_diameter: EstimatedDiameter,
    orbital_data: &OrbitalData,
) -> HohmannTransfer {
    let neo_semi_major_axis_km: f64 =
        orbital_data.semi_major_axis.parse().unwrap_or(0.0) * KM_PER_AU;
    let neo_inclination_deg: f64 = orbital_data.inclination.parse().unwrap_or(0.0);
    let transfer_path_axis = (EARTH_ORBITAL_RADIUS_KM + neo_semi_major_axis_km) / 2.0;

    let neo_orbital_speed_km_per_s =
        (SUN_GRAVITATIONAL_PARAMETER_KM3_S2 / neo_semi_major_axis_km).sqrt();

    let vper = calculate_vis_viva(EARTH_ORBITAL_RADIUS_KM, transfer_path_axis);
    let vap = calculate_vis_viva(neo_semi_major_axis_km, transfer_path_axis);

    let delta_v1 = (vper - EARTH_ORBITAL_VELOCITY_KM_PER_S).abs();
    let delta_v2 = (vap.powi(2) + neo_orbital_speed_km_per_s.powi(2)
        - 2.0 * vap * neo_orbital_speed_km_per_s * (neo_inclination_deg.to_radians().cos()))
    .sqrt();

    let total_delta_v = delta_v1 + delta_v2;
    let transfer_time_days = (PI
        * (transfer_path_axis.powi(3) / SUN_GRAVITATIONAL_PARAMETER_KM3_S2).sqrt())
        / SECOND_IN_DAY;

    HohmannTransfer {
        estimated_diameter,
        delta_v: total_delta_v,
        transfer_time_days,
    }
}

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

    let calculated_mass = calculate_mass(&estimated_diameter);

    let calculated_energy = calculate_energy(&calculated_mass, &relative_velocity);

    let orbiting_body = asteroid_data
        .close_approach_data
        .get(0)
        .ok_or_else(|| ApiError::Internal("No close approach data available".to_string()))?
        .orbiting_body
        .clone();

    let kinetic_data = KinematicData {
        estimated_diameter,
        relative_velocity,
        calculated_mass,
        calculated_energy,
        orbiting_body,
    };

    Ok(Json(kinetic_data))
}

async fn get_hohmann_transfer(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<HohmannTransfer>, ApiError> {
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
    let orbital_data: OrbitalData = asteroid_data.orbital_data;

    let hohmann_transfer = calculate_hohmann_transfer(estimated_diameter, &orbital_data);

    Ok(Json(hohmann_transfer))
}

pub fn physics_routes() -> Router<AppState> {
    Router::new()
        .route("/kinetics/{id}", get(get_kinetics))
        .route("/transfer/{id}", get(get_hohmann_transfer))
}
