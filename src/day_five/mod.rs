mod routes;

use axum::{routing::get, Router};
use routes::msg_for_mr_grinch;

pub fn routes() -> Router {
    Router::new().route("/day_5", get(msg_for_mr_grinch))
}
