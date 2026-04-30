use super::utils::{EstimatedDiameter, RelativeVelocity};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CalculatedEnergy {
    pub joules: f64,
    pub kilotons_of_tnt: f64,
}

#[derive(Deserialize, Serialize)]
pub struct CalculatedMass {
    pub kilograms: f64,
    pub grams: f64,
    pub pounds: f64,
}

#[derive(Deserialize, Serialize)]
pub struct KinematicData {
    pub estimated_diameter: EstimatedDiameter,
    pub relative_velocity: RelativeVelocity,
    pub calculated_mass: CalculatedMass,
    pub calculated_energy: CalculatedEnergy,
}

#[derive(Deserialize, Serialize)]
pub struct HohmannTransfer {
    pub estimated_diameter: EstimatedDiameter,
    pub delta_v: f64,
    pub transfer_time_days: f64,
}
