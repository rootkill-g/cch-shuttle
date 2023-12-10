mod routes;
mod utils;

use axum::{routing::post, Router};
use routes::{contest, strength};

pub fn routes() -> Router {
    Router::new()
        .route("/4/strength", post(strength))
        .route("/4/contest", post(contest))
}
