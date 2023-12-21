mod day_1;
mod day_4;
mod day_5;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(day_5::router().into())
}
