use rsocket_rust::Result;
use hello_rsocket::requester;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
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

    // If you want to block until socket disconnected.
    request_coon.destroy().await;

    Ok(())
}