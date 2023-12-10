use std::collections::HashMap;

use axum::{
    http::{HeaderMap, Request, StatusCode},
    response::{ErrorResponse, Html, IntoResponse},
    routing::get,
    Json, Router,
};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct Recipe {
    flour: i32,
    sugar: i32,
    butter: i32,
    #[serde(rename = "baking powder")]
    baking_powder: i32,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Pantry {
    flour: i32,
    sugar: i32,
    butter: i32,
    #[serde(rename = "baking powder")]
    baking_powder: i32,
    #[serde(rename = "chocolate chips")]
    chocolate_chips: i32,
}

#[derive(Debug, Serialize)]
struct BakeResponse {
    cookie: i32,
    pantry: Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct AnyStruct {
    fields: HashMap<String, Value>,
}

fn parse_json_to_any_struct(json_string: &str) -> Result<AnyStruct, serde_json::Error> {
    let data: HashMap<String, Value> = serde_json::from_str(json_string)?;
    Ok(AnyStruct { fields: data })
}

fn parse(hmap: &HeaderMap) -> Result<String, StatusCode> {
    match hmap.get("Cookie") {
        Some(cookie) => {
            let cookie_str = cookie
                .to_str()
                .map_err(|_| StatusCode::UNSUPPORTED_MEDIA_TYPE)?;
            let trimmed_cookie = cookie_str.trim_start_matches("recipe=");

            let recipe_bytes = match general_purpose::STANDARD.decode(trimmed_cookie) {
                Ok(bytes) => bytes,
                Err(_) => return Err(StatusCode::BAD_REQUEST),
            };

            String::from_utf8(recipe_bytes).map_err(|_| StatusCode::UNSUPPORTED_MEDIA_TYPE)
        }
        None => Err(StatusCode::BAD_REQUEST),
    }
}

async fn decode<B>(req: Request<B>) -> Result<Json<String>, ErrorResponse> {
    tracing::info!("decode => RECEIVED: {:?}", req.headers());
    Ok(Json(parse(req.headers())?))
}

async fn bake<B>(req: Request<B>) -> impl IntoResponse {
    tracing::info!("bake => RECEIVED: {:?}", req.headers());

    let cookie = match parse(req.headers()) {
        Ok(cookie) => cookie,
        Err(err) => return Err(err),
    };

    tracing::info!("bake => COOKIE: {:?}", cookie);

    let recpan = parse_json_to_any_struct(&cookie).unwrap();

    tracing::info!("bake => RECPAN: {:?}", recpan.fields);

    let recipe: Recipe =
        match serde_json::from_value(recpan.fields.get("recipe").unwrap().to_owned()) {
            Ok(rec) => rec,
            Err(_) => {
                // This means the request was sent by an Elf
                // So we create a response specifically for Elves
                let pantry = recpan.fields.get("pantry").unwrap().to_owned();

                tracing::info!("bake => PANTRY: {:?}", pantry);

                return Ok(Json(BakeResponse { cookie: 0, pantry }));
            }
        };

    tracing::info!("bake => RECIPE: {:?}", recipe);

    let pantry: Pantry =
        match serde_json::from_value(recpan.fields.get("pantry").unwrap().to_owned()) {
            Ok(pan) => pan,
            Err(_) => {
                // This means the request was sent by an Elf
                // So we create a response specifically for Elves
                let pantry = recpan.fields.get("pantry").unwrap().to_owned();

                tracing::info!("bake => PANTRY: {:?}", pantry);

                return Ok(Json(BakeResponse { cookie: 0, pantry }));
            }
        };

    tracing::info!("bake => PANTRY: {:?}", pantry);

    let cooked_cookies = pantry.flour / recipe.flour;

    let left_pantry: Pantry = Pantry {
        flour: pantry.flour - (cooked_cookies * recipe.flour),
        sugar: pantry.sugar - (cooked_cookies * recipe.sugar),
        butter: pantry.butter - (cooked_cookies * recipe.butter),
        baking_powder: pantry.baking_powder - (cooked_cookies * recipe.baking_powder),
        chocolate_chips: pantry.chocolate_chips - (cooked_cookies * recipe.chocolate_chips),
    };

    Ok(Json(BakeResponse {
        cookie: cooked_cookies,
        pantry: match serde_json::to_value(left_pantry) {
            Ok(pantry) => pantry,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }))
}

async fn hello() -> impl IntoResponse {
    Html("<h1>Hello, World!</h1>")
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello))
        .route("/7/decode", get(decode))
        .route("/7/bake", get(bake));

    Ok(router.into())
}
