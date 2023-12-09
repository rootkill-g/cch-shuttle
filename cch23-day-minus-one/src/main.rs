use axum::{
    http::{StatusCode, Uri},
    routing::get,
    Router,
};

async fn hello_world() -> StatusCode {
    println!("--> {:<12} - hello_world - ", "HANDLER");
    StatusCode::OK
}

async fn fake_error() -> StatusCode {
    println!("--> {:<12} - fake_error - ", "HANDLER");
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn fallback(uri: Uri) -> StatusCode {
    println!(
        "--> {:<12} - fallback - No URI Found for {uri:?}",
        "FALLBACK"
    );
    StatusCode::NOT_FOUND
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(fake_error))
        .fallback(fallback);

    Ok(router.into())
}
