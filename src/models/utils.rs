use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DiameterRange {
    estimated_diameter_min: f64,
    estimated_diameter_max: f64,
}

#[derive(Deserialize, Serialize)]
pub struct EstimatedDiameter {
    kilometers: DiameterRange,
    meters: DiameterRange,
    miles: DiameterRange,
    feet: DiameterRange,
}

#[derive(Deserialize, Serialize)]
pub struct RelativeVelocity {
    kilometers_per_second: String,
    kilometers_per_hour: String,
    miles_per_hour: String,
}
