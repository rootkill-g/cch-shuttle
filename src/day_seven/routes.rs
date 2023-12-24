#![allow(unused_assignments)]

use axum::{
    http::{HeaderMap, Request, StatusCode},
    response::{ErrorResponse, IntoResponse},
    Json,
};
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
struct BakeResponse {
    cookie: u64,
    pantry: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Cookie {
    fields: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Recipe {
    ingredients: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pantry {
    stock: HashMap<String, Value>,
}

fn parse_json_cookie(json_string: &str) -> Result<Cookie, serde_json::Error> {
    let data: HashMap<String, Value> = serde_json::from_str(json_string)?;
    Ok(Cookie { fields: data })
}

fn parse_value_to_recipe(val: Value) -> Result<Recipe, serde_json::Error> {
    let data: HashMap<String, Value> = serde_json::from_value(val)?;
    Ok(Recipe { ingredients: data })
}

fn parse_value_to_pantry(val: Value) -> Result<Pantry, serde_json::Error> {
    let data: HashMap<String, Value> = serde_json::from_value(val)?;
    Ok(Pantry { stock: data })
}

fn total_baked(recipe: &Recipe, pantry: &Pantry) -> u64 {
    let common_keys = pantry
        .stock
        .keys()
        .filter(|key| recipe.ingredients.contains_key(key.to_owned()))
        .cloned()
        .collect::<Vec<_>>();

    if common_keys.is_empty() {
        return 0;
    }

    let maximum_cookies = common_keys
        .iter()
        .map(|key| {
            pantry.stock.get(key).unwrap().as_u64().unwrap()
                / recipe.ingredients.get(key).unwrap().as_u64().unwrap()
        })
        .min()
        .unwrap();

    maximum_cookies
}

fn left_pantry(baked_cookies: u64, pantry: &mut Pantry, recipe: &Recipe) {
    pantry
        .stock
        .iter_mut()
        .filter_map(|(key, val_p)| {
            recipe.ingredients.get(key).map(|val_r| {
                (
                    key.to_owned(),
                    serde_json::to_value(
                        val_p.as_u64().unwrap() - (baked_cookies * val_r.as_u64().unwrap()),
                    )
                    .unwrap(),
                )
            })
        })
        .collect::<HashMap<String, Value>>()
        .into_iter()
        .for_each(|(key, value)| {
            pantry.stock.insert(key, value);
        });
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

pub async fn decode<B>(req: Request<B>) -> Result<Json<String>, ErrorResponse> {
    tracing::info!("decode => RECEIVED: {:?}", req.headers());
    Ok(Json(parse(req.headers())?))
}

pub async fn bake<B>(req: Request<B>) -> impl IntoResponse {
    tracing::info!("bake => RECEIVED: {:?}", req.headers());

    let cookie = match parse(req.headers()) {
        Ok(cookie) => cookie,
        Err(err) => return Err(err),
    };

    tracing::info!("bake => COOKIE: {:?}", cookie);

    let recpan = parse_json_cookie(&cookie).unwrap();

    tracing::info!("bake => RECPAN: {:?}", recpan.fields);

    let recipe = parse_value_to_recipe(recpan.fields.get("recipe").unwrap().to_owned()).unwrap();

    tracing::info!("bake => RECIPE: {:?}", recipe.ingredients);

    let mut pantry =
        parse_value_to_pantry(recpan.fields.get("pantry").unwrap().to_owned()).unwrap();

    tracing::info!("bake => PANTRY: {:?}", pantry.stock);

    let baked_cookies = total_baked(&recipe, &pantry);

    left_pantry(baked_cookies, &mut pantry, &recipe);

    Ok(Json(BakeResponse {
        cookie: baked_cookies,
        pantry: pantry.stock,
    }))
}
