use axum::{routing::get, Router, http::StatusCode, extract::Path};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn bits_cube(Path((x, y)): Path<(u32, u32)>) -> String {
    (x ^ y).pow(3).to_string()
}

async fn error_handler() -> Result<String, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
    .route("/", get(hello_world))
    .route("/1/:x/:y", get(bits_cube))
    .route("/-1/error", get(error_handler));


    Ok(router.into())
}
