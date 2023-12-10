use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn cube(Path(vec_path): Path<String>) -> impl IntoResponse {
    println!("🧊 #> {:<12} - cube - {vec_path:?}", "HANDLER");

    for c in vec_path.chars() {
        if !c.is_ascii_digit() && c != '/' {
            return (
                StatusCode::BAD_REQUEST,
                String::from("Invalid packets, only integers are allowed!"),
            );
        }
    }

    let separated: Vec<i32> = vec_path
        .split('/')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    println!("Separated = {:?}", separated);

    if separated.len() < 1 || separated.len() > 20 {
        return (
            StatusCode::BAD_REQUEST,
            String::from("Invalid number of packets!"),
        );
    }

    let mut xor_res = 0;

    for pk in separated {
        xor_res ^= pk;
    }

    let res = xor_res.pow(3);

    (StatusCode::OK, format!("{}", res))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/1/*key", get(cube));

    Ok(router.into())
}
