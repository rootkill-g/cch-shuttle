mod utils;

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use utils::compare::compare;

#[derive(Debug, Deserialize)]
struct Reindeer {
    // To mitigate the compiler warning about "name" not being used
    // we added "_" prefix to name but the
    // Data received will have variable "name" instead of "_name"
    // so to properly deserialize the data received we use serde rename
    #[serde(rename = "name")]
    _name: String,
    strength: i32,
}

#[derive(Debug, Deserialize)]
struct ContestingReindeer {
    name: String,
    strength: i32,
    speed: f64,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i32,
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn strength(Json(vec_reindeer): Json<Vec<Reindeer>>) -> impl IntoResponse {
    tracing::info!("🦌 DATA = {:?}", vec_reindeer);

    Json(vec_reindeer.iter().map(|r| r.strength).sum::<i32>())
}

async fn contest(Json(vec_contesting): Json<Vec<ContestingReindeer>>) -> impl IntoResponse {
    tracing::info!("CONTESTING 🦌 LIST = {:?}", vec_contesting);

    let result = compare(vec_contesting);

    Json(result)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/4/strength", post(strength))
        .route("/4/contest", post(contest));

    Ok(router.into())
}
