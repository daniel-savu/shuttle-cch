use axum::{extract::Query, Json, http::StatusCode, Router, routing};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    offset: usize,
    limit: usize,
}

async fn subset(
    pagination: Option<Query<Pagination>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<String, StatusCode> {
    let Some(pagination) = pagination
    else {
        return Ok("".to_string())
    };
    let pagination = pagination.0;
    let names: Vec<String> = serde_json::from_value(payload).map_err(|_| StatusCode::BAD_REQUEST)?;
    let start = pagination.offset;
    let end = start + pagination.limit;
    let res = names[start..end].to_vec();
    Ok(format!("{:?}", res))
}

pub fn router() -> Router {
    Router::new()
        .route("/5", routing::post(subset))
}
