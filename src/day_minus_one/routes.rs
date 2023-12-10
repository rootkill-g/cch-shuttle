use axum::http::StatusCode;

pub async fn hello_world() -> StatusCode {
    println!("--> {:<12} - hello_world - ", "HANDLER");
    StatusCode::OK
}

pub async fn fake_error() -> StatusCode {
    println!("--> {:<12} - fake_error - ", "HANDLER");
    StatusCode::INTERNAL_SERVER_ERROR
}
