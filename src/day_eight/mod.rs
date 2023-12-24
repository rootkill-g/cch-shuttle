mod routes;

use axum::{routing::get, Router};
use routes::poke_weight;

use self::routes::poke_momentum;

pub fn routes() -> Router {
    Router::new()
        .route("/8/weight/:pid", get(poke_weight))
        .route("/8/drop/:pid", get(poke_momentum))
}
