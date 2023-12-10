mod routes;

use axum::{routing::get, Router};

use routes::{fake_error, hello_world};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
}
