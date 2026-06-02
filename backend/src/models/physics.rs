use super::utils::{EstimatedDiameter, RelativeVelocity};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CalculatedEnergy {
    pub joules: f64,
    pub kilotons_of_tnt: f64,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CalculatedMass {
    pub kilograms: f64,
    pub grams: f64,
    pub pounds: f64,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct KinematicData {
    pub estimated_diameter: EstimatedDiameter,
    pub relative_velocity: RelativeVelocity,
    pub calculated_mass: CalculatedMass,
    pub calculated_energy: CalculatedEnergy,
    pub orbiting_body: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct HohmannTransfer {
    pub estimated_diameter: EstimatedDiameter,
    pub delta_v: f64,
    pub transfer_time_days: f64,
}
