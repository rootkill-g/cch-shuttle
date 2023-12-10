use axum::{
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Qelf {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn elfs(query: String) -> impl IntoResponse {
    let elf = query.matches("elf").count();
    let elf_on_a_shelf = query.matches("elf on a shelf").count();
    let shelf_with_no_elf_on_it = query.matches("shelf").count() - elf_on_a_shelf;

    let res = Qelf {
        elf,
        elf_on_a_shelf,
        shelf_with_no_elf_on_it,
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
