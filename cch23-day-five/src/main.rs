use axum::{routing::get, Router};

async fn msg_for_mr_grinch() -> &'static str {
    "We Want Day 5 Back MR. GRINCH!!!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(msg_for_mr_grinch));

    Ok(router.into())
}
