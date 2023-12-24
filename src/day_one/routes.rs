#![allow(clippy::manual_try_fold)]
use axum::{extract::Path, http::StatusCode, response::IntoResponse};

pub async fn cube(Path(vec_path): Path<String>) -> impl IntoResponse {
    println!("🧊 #> {:<12} - cube - {vec_path:?}", "HANDLER");

    vec_path
        .split('/')
        .map(|v| v.parse::<i64>().map_err(|_| StatusCode::BAD_REQUEST))
        .fold(Ok(0), |acc, e| match (acc, e) {
            (Ok(x), Ok(y)) => Ok(x ^ y),
            (Ok(_), Err(_)) | (Err(_), Ok(_)) | (Err(_), Err(_)) => Err(StatusCode::BAD_REQUEST),
        })
        .map(|acc| Ok(format!("{}", acc.pow(3))))
        .unwrap_or(Err(StatusCode::INTERNAL_SERVER_ERROR))
}
