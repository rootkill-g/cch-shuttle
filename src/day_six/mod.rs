mod routes;

use axum::{routing::post, Router};
use routes::elfs;

pub fn routes() -> Router {
    Router::new().route("/6", post(elfs))
}
