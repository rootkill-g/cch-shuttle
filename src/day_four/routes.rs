use super::utils::compare::compare;
use axum::{response::IntoResponse, Json};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Reindeer {
    pub name: String,
    pub strength: i32,
    #[serde(default)]
    pub speed: f64,
    #[serde(default)]
    pub height: i32,
    #[serde(default)]
    pub antler_width: i32,
    #[serde(default)]
    pub snow_magic_power: i32,
    #[serde(default)]
    pub favorite_food: String,
    #[serde(default, rename = "cAnD13s_3ATeN-yesT3rdAy")]
    pub candies_eaten_yesterday: i32,
}

pub async fn strength(Json(vec_reindeer): Json<Vec<Reindeer>>) -> impl IntoResponse {
    tracing::info!("🦌 DATA = {:?}", vec_reindeer);

    Json(vec_reindeer.iter().map(|r| r.strength).sum::<i32>())
}

pub async fn contest(Json(vec_contesting): Json<Vec<Reindeer>>) -> impl IntoResponse {
    tracing::info!("CONTESTING 🦌 LIST = {:?}", vec_contesting);

    let result = compare(vec_contesting);

    Json(result)
}
