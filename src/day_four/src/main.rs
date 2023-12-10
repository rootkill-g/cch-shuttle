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
    name: String,
    strength: i32,
    #[serde(default)]
    speed: f64,
    #[serde(default)]
    height: i32,
    #[serde(default)]
    antler_width: i32,
    #[serde(default)]
    snow_magic_power: i32,
    #[serde(default)]
    favorite_food: String,
    #[serde(default, rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i32,
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn strength(Json(vec_reindeer): Json<Vec<Reindeer>>) -> impl IntoResponse {
    tracing::info!("🦌 DATA = {:?}", vec_reindeer);

    Json(vec_reindeer.iter().map(|r| r.strength).sum::<i32>())
}

async fn contest(Json(vec_contesting): Json<Vec<Reindeer>>) -> impl IntoResponse {
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
