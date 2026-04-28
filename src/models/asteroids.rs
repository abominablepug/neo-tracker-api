use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
struct DiameterRange {
    estimated_diameter_min: f64,
    estimated_diameter_max: f64,
}

#[derive(Deserialize, Serialize)]
struct EstimatedDiameter {
    kilometers: DiameterRange,
    meters: DiameterRange,
    miles: DiameterRange,
    feet: DiameterRange,
}

#[derive(Deserialize, Serialize)]
struct RelativeVelocity {
    kilometers_per_second: String,
    kilometers_per_hour: String,
    miles_per_hour: String,
}

#[derive(Deserialize, Serialize)]
struct MissDistance {
    astronomical: String,
    lunar: String,
    kilometers: String,
    miles: String,
}

#[derive(Deserialize, Serialize)]
struct CloseApproachData {
    close_approach_date: String,
    close_approach_date_full: String,
    epoch_date_close_approach: i64,
    relative_velocity: RelativeVelocity,
    miss_distance: MissDistance,
    orbiting_body: String,
}

#[derive(Deserialize, Serialize)]
struct NearEarthObjects {
    id: String,
    name: String,
    nasa_jpl_url: String,
    absolute_magnitude_h: f64,
    estimated_diameter: EstimatedDiameter,
    is_potentially_hazardous_asteroid: bool,
    close_approach_data: Vec<CloseApproachData>,
    is_sentry_object: bool,
}

#[derive(Deserialize, Serialize)]
struct NeoLinks {
    next: String,
    #[serde(rename = "previous")]
    prev: Option<String>,
    #[serde(rename = "self")]
    current: String,
}

#[derive(Deserialize, Serialize)]
pub struct NeoResponse {
    links: NeoLinks,
    element_count: u32,
    near_earth_objects: HashMap<String, Vec<NearEarthObjects>>,
}
