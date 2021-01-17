use rsocket_rust::Result;
use hello_rsocket::requester;
use tokio::time::{sleep, Duration};
use log::info;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).format_timestamp_millis().init();
    hello_rsocket().await;
    Ok(())
}

async fn hello_rsocket() {
    let sleep_millis = Duration::from_millis(500);
    let request_coon = requester::RequestCoon::new().await;

    request_coon.meta_push().await;
    sleep(sleep_millis).await;

    request_coon.fnf().await;
    sleep(sleep_millis).await;

    request_coon.request_response().await;
    sleep(sleep_millis).await;

    request_coon.request_stream().await;
    sleep(sleep_millis).await;

    request_coon.request_channel().await;

    request_coon.destroy().await;

    info!("done");
}