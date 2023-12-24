mod routes;

use axum::{routing::get, Router};
use routes::poke_weight;

pub fn routes() -> Router {
    Router::new().route("/8/weight/:pid", get(poke_weight))
}
