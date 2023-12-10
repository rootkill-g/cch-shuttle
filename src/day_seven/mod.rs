mod routes;

use axum::{routing::get, Router};
use routes::decode;

use self::routes::bake;

pub fn routes() -> Router {
    Router::new()
        .route("/7/decode", get(decode))
        .route("/6/bake", get(bake))
}
