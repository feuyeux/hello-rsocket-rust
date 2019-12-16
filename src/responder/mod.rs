use tokio::sync::mpsc;
use std::error::Error;
use futures::prelude::*;
use rsocket_rust::prelude::*;
use crate::common::{HelloResponse, response_to_json};
use bytes::{Bytes};

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
        println!(">>>>>>>> metadata_push: {:?}", req);
        Box::pin(async {})
    }

    fn fire_and_forget(&self, req: Payload) -> Mono<()> {
        println!(">>>>>>>> fire_and_forget: {:?}", req);
        Box::pin(async {})
    }

    fn request_response(&self, req: Payload) -> Mono<Result<Payload, RSocketError>>  {
        println!(
            ">>>>>>>> request_response: data={:?},meta={:?}",
            req.data(),
            req.metadata()
        );
        let response = HelloResponse { id: "1".to_owned(),value:"hello".to_owned() };
        let json_data = response_to_json(&response);
        let p = Payload::builder()
            .set_data(Bytes::from(json_data))
            .set_metadata_utf8("RUST")
            .build();
        Box::pin(future::ok::<Payload, RSocketError>(p))
    }

    fn request_stream(&self, req: Payload) -> Flux<Payload> {
        println!(">>>>>>>> request_stream: {:?}", req);
        let mut results = vec![];
        for _n in 0..3 {
            let response = HelloResponse { id: "1".to_owned(),value:"hello".to_owned() };
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
                println!("{:?}", p);

                let response = HelloResponse { id: "1".to_owned(),value:"hello".to_owned() };
                let json_data = response_to_json(&response);
                let resp = Payload::builder()
                    .set_data(Bytes::from(json_data))
                    .set_metadata_utf8("RUST")
                    .build();
                sender.send(resp).unwrap();
            }
        });
        Box::pin(receiver)
    }
}
