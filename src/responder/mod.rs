use tokio::sync::mpsc;
use futures::prelude::*;
use crate::common::{HelloResponse, response_to_json};
use crate::common::*;
use rsocket_rust::prelude::*;
use rsocket_rust::{Result, runtime};
use rsocket_rust_transport_tcp::TcpServerTransport;
use async_stream::stream;
use async_trait::async_trait;
use log::info;

static HELLO_LIST: [&str; 6] = ["Hello", "Bonjour", "Hola", "こんにちは", "Ciao", "안녕하세요"];

pub async fn start() -> Result<()> {
    RSocketFactory::receive()
        .transport(TcpServerTransport::from("127.0.0.1:7878"))
        .acceptor(Box::new(|_setup, _socket| {
            Ok(Box::new(ResponseCoon))
        }))
        .serve()
        .await
}


pub struct ResponseCoon;
#[async_trait]
impl RSocket for ResponseCoon {
    async fn metadata_push(&self, req: Payload) -> Result<()>{
        info!(">> [metadata_push]: {:?}", req);
        Ok(())
    }

    async fn fire_and_forget(&self, req: Payload) ->  Result<()> {
        let request = data_to_request(req.data());
        println!(">> [fire_and_forget] FNF:{}", request.id);
        Ok(())
    }

    async fn request_response(&self, req: Payload) -> Result<Option<Payload>>  {
        let request = data_to_request(req.data());
        println!(
            ">> [request_response] data:{:?}, meta={:?}", request, req.metadata()
        );
        let index = request.id.parse::<usize>().unwrap();
        let response = HelloResponse { id: request.id, value: HELLO_LIST[index].to_string() };
        let json_data = response_to_json(&response);
//        let bytes = Bytes::from(json_data);
        let p = Payload::builder()
            .set_data(json_data)
            .set_metadata_utf8("RUST")
            .build();
        Ok(Some((p)))
    }

    fn request_stream(&self, req: Payload) -> Flux<Result<Payload>> {
        let request = data_to_requests(req.data());
        println!(">> [request_stream] data:{:?}", request);
        let mut results = vec![];
        for _id in request.ids {
            let index = _id.parse::<usize>().unwrap();
            let response = HelloResponse { id: _id, value: HELLO_LIST[index].to_string() };
            let json_data = response_to_json(&response);
//            let bytes = Bytes::from(json_data);
            let p = Payload::builder()
                .set_data(json_data)
                .set_metadata_utf8("RUST")
                .build();
            results.push(Ok(p));
        }
        Box::pin(futures::stream::iter(results))
    }

    fn request_channel(&self, mut requests: Flux<Result<Payload>>) ->  Flux<Result<Payload>> {
        let (sender, mut receiver) = mpsc::unbounded_channel();
        runtime::spawn(async move {
            while let Some(Ok(p)) = requests.next().await {
                let request = data_to_requests(p.data());
                println!(">> [request_channel] data:{:?}", request);
                for _id in request.ids {
                    let index = _id.parse::<usize>().unwrap();
                    let response = HelloResponse { id: _id, value: HELLO_LIST[index].to_string() };
                    let json_data = response_to_json(&response);
                    //let bytes = Bytes::from(json_data);
                    let resp = Payload::builder()
                        .set_data(json_data)
                        .set_metadata_utf8("RUST")
                        .build();
                    sender.send(Ok(resp)).unwrap();
                }
            }
        });
        // Box::pin(receiver)
        Box::pin(stream! {
            while let Some(it) = receiver.recv().await {
                yield it;
            }
        })
    }
}
