use log::info;
use rsocket_rust::{Result};
use rsocket_rust::prelude::*;
use rsocket_rust_transport_tcp::TcpServerTransport;
use hello_rsocket::responder;

#[tokio::main]
async fn main() -> Result<()> {
    RSocketFactory::receive()
        .transport(TcpServerTransport::from("127.0.0.1:7878"))
        .acceptor(Box::new(|setup, _socket| {
            info!("accept setup: {:?}", setup);
            Ok(Box::new(responder::ResponseCoon))
        }))
        .serve()
        .await
}