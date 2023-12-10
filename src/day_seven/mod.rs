mod routes;

use axum::{routing::get, Router};
use routes::{bake, decode};

pub fn routes() -> Router {
    Router::new()
        .route("/7/decode", get(decode))
        .route("/7/bake", get(bake))
}
