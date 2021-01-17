use rsocket_rust::prelude::*;
use rsocket_rust::Result;
use rsocket_rust_transport_tcp::TcpServerTransport;
use hello_rsocket::responder;

use log::info;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).format_timestamp_millis().init();
    RSocketFactory::receive()
        .transport(TcpServerTransport::from("127.0.0.1:7878"))
        .acceptor(Box::new(|setup, _socket| {
            info!("accept setup: {:?}", setup);
            Ok(Box::new(responder::ResponseCoon))
        }))
        .on_start(Box::new(|| info!("+++++++ echo server started! +++++++")))
        .serve()
        .await
}