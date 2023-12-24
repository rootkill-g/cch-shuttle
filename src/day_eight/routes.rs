use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use rustemon::{client::RustemonClient, pokemon::pokemon};

pub async fn poke_weight(Path(pid): Path<i64>) -> impl IntoResponse {
    tracing::info!("poke_weight => GET: RECEIVED POKE_ID: {}", pid);

    let pokemon = match pokemon::get_by_id(pid, &RustemonClient::default()).await {
        Ok(poke) => poke,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(format!("{}", (pokemon.weight as f64) / 10.0))
}

pub async fn poke_momentum(Path(pid): Path<i64>) -> impl IntoResponse {
    tracing::info!("poke_momentum => GET: RECEIVED POKE_ID: {}", pid);

    let pokemon_weight = match pokemon::get_by_id(pid, &RustemonClient::default()).await {
        Ok(poke) => poke.weight as f64 / 10.0,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(format!(
        "{}",
        pokemon_weight * (f64::sqrt(2.0 * 9.825 * 10.0))
    ))
}
