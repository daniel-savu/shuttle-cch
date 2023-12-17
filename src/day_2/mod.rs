use serde::{Deserialize, Serialize};
use axum::{extract::Json, http::StatusCode, routing, Router};

#[derive(Serialize, Deserialize)]
struct Reindeer {
    name: String,
    strength: u32,
}

async fn parse_reindeer(Json(payload): Json<serde_json::Value>) -> Result<String, StatusCode> {
    let reindeers: Vec<Reindeer> = serde_json::from_value(payload).map_err(|_| StatusCode::BAD_REQUEST)?;
    let sum: u32 = reindeers.iter().map(|r| r.strength).sum();
    Ok(sum.to_string())
}

pub async fn run() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/4/strength", routing::post(parse_reindeer));

    Ok(router.into())
}
