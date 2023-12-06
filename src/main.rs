use axum::{routing::get, Router, http::StatusCode};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn error_handler() -> Result<String, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
    .route("/", get(hello_world))
    .route("/-1/error", get(error_handler));


    Ok(router.into())
}
