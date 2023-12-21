mod day_1;
mod day_4;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    day_4::run().await
}
