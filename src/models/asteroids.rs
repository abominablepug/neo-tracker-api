use serde::{Deserialize, Serialize};

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
struct OrbitClass {
    orbit_class_type: String,
    orbit_class_description: String,
    orbit_class_range: String,
}

#[derive(Deserialize, Serialize)]
struct OrbitalData {
    orbit_id: String,
    orbit_determination_date: String,
    first_observation_date: String,
    last_observation_date: String,
    data_arc_in_days: u32,
    observations_used: u32,
    orbit_uncertainty: String,
    minimum_orbit_intersection: String,
    jupiter_tisserand_invariant: String,
    epoch_osculation: String,
    eccentricity: String,
    semi_major_axis: String,
    inclination: String,
    ascending_node_longitude: String,
    orbital_period: String,
    perihelion_distance: String,
    perihelion_argument: String,
    aphelion_distance: String,
    perihelion_time: String,
    mean_anomaly: String,
    mean_motion: String,
    orbit_class: OrbitClass,
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
    orbital_data: OrbitalData,
    is_sentry_object: bool,
}

#[derive(Deserialize, Serialize)]
struct NeoLinks {
    next: Option<String>,
    prev: Option<String>,
    #[serde(rename = "self")]
    current: String,
}

#[derive(Deserialize, Serialize)]
struct NeoPage {
    size: u32,
    total_elements: u32,
    total_pages: u32,
    number: u32,
}

#[derive(Deserialize, Serialize)]
pub struct NeoResponse {
    links: NeoLinks,
    page: NeoPage,
    near_earth_objects: Vec<NearEarthObjects>,
}
