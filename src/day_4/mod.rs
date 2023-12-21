use std::cmp::Ordering;

use axum::{extract::Json, http::StatusCode, routing, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Reindeer {
    name: String,
    strength: u32,
}

async fn parse_reindeer(Json(payload): Json<serde_json::Value>) -> Result<String, StatusCode> {
    let reindeers: Vec<Reindeer> =
        serde_json::from_value(payload).map_err(|_| StatusCode::BAD_REQUEST)?;
    let sum: u32 = reindeers.iter().map(|r| r.strength).sum();
    Ok(sum.to_string())
}

#[derive(Serialize, Deserialize, Debug)]
struct ReindeerBonus {
    name: String,
    strength: u32,
    speed: f32,
    height: u32,
    antler_width: u32,
    snow_magic_power: u64,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u32,
}

impl ReindeerBonus {
    fn parse_fastest(&self) -> String {
        format!(
            "Speeding past the finish line with a strength of {} is {}",
            self.strength, self.name
        )
    }

    fn parse_tallest(&self) -> String {
        format!(
            "{} is standing tall with his {} cm wide antlers",
            self.name, self.antler_width
        )
    }

    fn parse_magician(&self) -> String {
        format!(
            "{} could blast you away with a snow magic power of {}",
            self.name, self.snow_magic_power
        )
    }

    fn parse_consumer(&self) -> String {
        format!(
            "{} ate lots of candies, but also some {}",
            self.name, self.favorite_food
        )
    }

    fn strength(&self) -> u32 {
        self.strength
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn magic(&self) -> u64 {
        self.snow_magic_power
    }

    fn candies(&self) -> u32 {
        self.candies_eaten_yesterday
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct BonusReply {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

fn cmp<T: PartialOrd>(x: T, y: T) -> Ordering {
    if x < y {
        Ordering::Less
    } else if x == y {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}

fn get_max<U, C, V>(v: &[U], getter: C) -> Result<&U, StatusCode>
where
    C: Fn(&U) -> V,
    V: PartialOrd,
{
    v.iter()
        .max_by(|x, y| cmp(getter(x), getter(y)))
        .ok_or(StatusCode::BAD_REQUEST)
}

async fn parse_reindeer_bonus(
    Json(payload): Json<serde_json::Value>,
) -> Result<String, StatusCode> {
    let reindeers: Vec<ReindeerBonus> =
        serde_json::from_value(payload).map_err(|_| StatusCode::BAD_REQUEST)?;
    let fastest = get_max(&reindeers, ReindeerBonus::strength)?;
    let tallest = get_max(&reindeers, ReindeerBonus::height)?;
    let magician = get_max(&reindeers, ReindeerBonus::magic)?;
    let consumer = get_max(&reindeers, ReindeerBonus::candies)?;
    serde_json::to_string_pretty(&BonusReply {
        fastest: fastest.parse_fastest(),
        tallest: tallest.parse_tallest(),
        magician: magician.parse_magician(),
        consumer: consumer.parse_consumer(),
    })
    .map_err(|_| StatusCode::BAD_REQUEST)
}

pub fn router() -> Router {
    Router::new()
        .route("/4/strength", routing::post(parse_reindeer))
        .route("/4/contest", routing::post(parse_reindeer_bonus))
}
