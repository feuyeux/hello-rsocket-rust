use tokio::runtime::Runtime;
use std::thread::sleep;
use std::time::Duration;
use hello_rsocket::{responder, requester};

fn main() {
    rsocket_demo();
}

fn rsocket_demo() {
    let is_run_server = true;
    let is_run_client = true;

    let server_runtime = Runtime::new().unwrap();
    let mut client_runtime = Runtime::new().unwrap();

    if is_run_server {
        if is_run_client {
            server_runtime.spawn(async move {
                responder::start().await
            });
            sleep(Duration::from_millis(500));
        } else {
            client_runtime.block_on(async {
                let _ = responder::start().await;
            });
        }
    }

    if is_run_client {
        client_runtime.block_on(run_request());
    }
}

async fn run_request() {
    let sleep_millis = Duration::from_millis(500);
    let request_coon = requester::RequestCoon::new().await;
    request_coon.meta_push().await;
    sleep(sleep_millis);
    request_coon.fnf().await;
    sleep(sleep_millis);
    request_coon.request_response().await;
    sleep(sleep_millis);
    request_coon.request_stream().await;
    sleep(sleep_millis);
    request_coon.request_channel().await;
}