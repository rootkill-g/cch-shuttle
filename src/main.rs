mod day_eight;
mod day_eleven;
mod day_five;
mod day_four;
mod day_minus_one;
mod day_one;
mod day_seven;
mod day_six;

use axum::{
    http::{StatusCode, Uri},
    Router,
};

pub async fn fallback(uri: Uri) -> StatusCode {
    println!(
        "--> {:<12} - fallback - No URI Found for {uri:?}",
        "FALLBACK"
    );
    StatusCode::NOT_FOUND
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .merge(day_minus_one::routes())
        .merge(day_one::routes())
        .merge(day_four::routes())
        .merge(day_five::routes())
        .merge(day_six::routes())
        .merge(day_seven::routes())
        .merge(day_eight::routes())
        .merge(day_eleven::routes())
        .fallback(fallback);

    Ok(router.into())
}
