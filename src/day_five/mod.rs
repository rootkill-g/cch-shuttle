mod routes;

use axum::{routing::post, Router};
use routes::msg_for_mr_grinch;

pub fn routes() -> Router {
    Router::new().route("/5", post(msg_for_mr_grinch))
}
