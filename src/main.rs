use axum::{extract::Path, http::StatusCode, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn bits_cube(Path(s): Path<String>) -> Result<String, StatusCode> {
    let v: Vec<u32> = s
        .split("/")
        .into_iter()
        .map(|number| u32::from_str_radix(number, 10).map_err(|_| StatusCode::BAD_REQUEST))
        .collect::<Result<_, _>>()?;
    Ok(v.iter().fold(0, |acc, x| acc ^ x).pow(3).to_string())
}

async fn error_handler() -> Result<String, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/1/*v", get(bits_cube))
        .route("/-1/error", get(error_handler));

    Ok(router.into())
}
