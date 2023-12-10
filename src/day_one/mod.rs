mod routes;

use axum::{routing::get, Router};
use routes::cube;

pub fn routes() -> Router {
    Router::new().route("/1/*num", get(cube))
}
