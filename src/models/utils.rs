use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DiameterRange {
    pub estimated_diameter_min: f64,
    pub estimated_diameter_max: f64,
}

#[derive(Deserialize, Serialize)]
pub struct EstimatedDiameter {
    pub kilometers: DiameterRange,
    meters: DiameterRange,
    miles: DiameterRange,
    feet: DiameterRange,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RelativeVelocity {
    pub kilometers_per_second: String,
    kilometers_per_hour: String,
    miles_per_hour: String,
}
