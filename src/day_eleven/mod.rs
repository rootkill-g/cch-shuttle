use axum::{
    extract::Multipart, http::StatusCode, response::IntoResponse, routing::post, Json, Router,
};
use tower_http::services::ServeDir;

pub fn routes() -> Router {
    Router::new()
        .route("/11/red_pixels", post(magical_red))
        .nest_service("/11/assets", ServeDir::new("assets"))
}

pub async fn magical_red(mut multipart: Multipart) -> impl IntoResponse {
    tracing::info!("magical_red: POST: RECEIVED: {:?}", multipart);

    let mut reds = 0;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        if name.clone() == "image" {
            let image = image::load_from_memory(&data).unwrap();

            reds = image
                .as_rgb8()
                .unwrap()
                .pixels()
                .map(|p| (p.0[0] as u16, p.0[1] as u16, p.0[2] as u16))
                .filter(|p| p.0 > p.1 + p.2)
                .count();
        } else {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    Ok(Json(reds))
}
