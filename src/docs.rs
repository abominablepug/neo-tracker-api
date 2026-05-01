use crate::models;
use crate::routes;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
paths(
    routes::default::check_status,
    routes::auth::register,
    routes::auth::login,
    routes::auth::logout,
    routes::asteroids::get_asteroids,
    routes::asteroids::get_asteroid_by_id,
    routes::asteroids::get_saved_asteroids,
    routes::asteroids::save_asteroid,
    routes::missions::create_mission,
    routes::missions::get_missions,
    routes::missions::delete_mission,
    routes::physics::get_kinetics,
    routes::physics::get_hohmann_transfer
),
components(
    schemas(
        models::asteroids::NeoResponse,
        models::asteroids::NearEarthObjects,
        models::asteroids::OrbitalData,
        models::asteroids::CloseApproachData,
        models::db::CachedNeo,
        models::db::Mission,
        models::utils::EstimatedDiameter,
        models::utils::RelativeVelocity,
        models::physics::KinematicData,
        models::physics::HohmannTransfer,
        models::physics::CalculatedMass,
        models::physics::CalculatedEnergy,
    ),
),
tags(
    (name = "Default", description = "Default routes for checking API status."),
    (name = "NEOs", description = "Routes for fetching Near Earth Object data from NASA's API."),
    (name = "Missions", description = "Routes for managing user missions to Near Earth Objects."),
    (name = "Physics", description = "Routes for calculating physics-related data for Near Earth Objects."),
    (name = "Authentication", description = "Routes for user registration, login, and logout.")
)
)]
pub struct ApiDoc;
