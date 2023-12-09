use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Qelf {
    elf: usize,
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn elfs(query: String) -> impl IntoResponse {
    let res = Qelf {
        elf: query.matches("elf").count(),
    };

    Json(res)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/6", post(elfs));

    Ok(router.into())
}
