mod day_1;
mod day_2;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    day_2::run().await
}