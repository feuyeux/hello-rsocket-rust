use tokio::sync::mpsc;
use std::error::Error;
use futures::prelude::*;
use rsocket_rust::prelude::*;
use crate::common::{HelloResponse, response_to_json};
use bytes::Bytes;
use crate::common::*;

static HELLO_LIST: [&str; 6] = ["Hello", "Bonjour", "Hola", "こんにちは", "Ciao", "안녕하세요"];

pub async fn start() -> Result<(), Box<dyn Error + Send + Sync>> {
    RSocketFactory::receive()
        .transport("tcp://127.0.0.1:7878")
        .acceptor(|_setup, _socket| {
            Ok(Box::new(ResponseCoon))
        })
        .serve()
        .await
}

pub struct ResponseCoon;

impl RSocket for ResponseCoon {
    fn metadata_push(&self, req: Payload) -> Mono<()> {
        println!(">> [metadata_push]: {:?}", req);
        Box::pin(async {})
    }

    fn fire_and_forget(&self, req: Payload) -> Mono<()> {
        let request = data_to_request(req.data());
        println!(">> [fire_and_forget] FNF:{}", request.id);
        Box::pin(async {})
    }

    fn request_response(&self, req: Payload) -> Mono<Result<Payload, RSocketError>> {
        let request = data_to_request(req.data());
        println!(
            ">> [request_response] data:{:?}, meta={:?}", request, req.metadata()
        );
        let index = request.id.parse::<usize>().unwrap();
        let response = HelloResponse { id: request.id, value: HELLO_LIST[index].to_string() };
        let json_data = response_to_json(&response);
        let p = Payload::builder()
            .set_data(Bytes::from(json_data))
            .set_metadata_utf8("RUST")
            .build();
        Box::pin(future::ok::<Payload, RSocketError>(p))
    }

    fn request_stream(&self, req: Payload) -> Flux<Payload> {
        let request = data_to_requests(req.data());
        println!(">> [request_stream] data:{:?}", request);
        let mut results = vec![];
        for _id in request.ids {
            let index = _id.parse::<usize>().unwrap();
            let response = HelloResponse { id: _id, value: HELLO_LIST[index].to_string() };
            let json_data = response_to_json(&response);
            let p = Payload::builder()
                .set_data(Bytes::from(json_data))
                .set_metadata_utf8("RUST")
                .build();
            results.push(p);
        }
        Box::pin(futures::stream::iter(results))
    }

    fn request_channel(&self, mut reqs: Flux<Payload>) -> Flux<Payload> {
        let (sender, receiver) = mpsc::unbounded_channel::<Payload>();
        tokio::spawn(async move {
            while let Some(p) = reqs.next().await {
                let request = data_to_requests(p.data());
                println!(">> [request_channel] data:{:?}", request);
                for _id in request.ids {
                    let index = _id.parse::<usize>().unwrap();
                    let response = HelloResponse { id: _id, value: HELLO_LIST[index].to_string() };
                    let json_data = response_to_json(&response);
                    let resp = Payload::builder()
                        .set_data(Bytes::from(json_data))
                        .set_metadata_utf8("RUST")
                        .build();
                    sender.send(resp).unwrap();
                }
            }
        });
        Box::pin(receiver)
    }
}
