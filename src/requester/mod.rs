use futures::prelude::*;
use rsocket_rust::prelude::*;
use crate::responder::ResponseCoon;
use crate::common::*;
use bytes::Bytes;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

pub struct RequestCoon {
    pub client: Client
}

impl RequestCoon {
    pub async fn new() -> RequestCoon {
        RequestCoon {
            client: RSocketFactory::connect()
                //.acceptor(|| Box::new(responder::ResponseCoon))
                .acceptor(|| Box::new(ResponseCoon))
                .transport("tcp://127.0.0.1:7878")
                .start()
                .await
                .unwrap()
        }
    }

    pub fn destroy(self) {
        self.client.close();
    }

    pub async fn meta_push(&self) {
        println!();
        println!("====ExecMetaPush====");
        let meta = Payload::builder().set_metadata_utf8("RUST").build();
        self.client.metadata_push(meta).await;
    }
    pub async fn fnf(&self) {
        println!();
        println!("====ExecFireAndForget====");
        let request = HelloRequest { id: "1".to_owned() };
        let json_data = request_to_json(&request);
        let fnf = Payload::builder().set_data(Bytes::from(json_data)).build();
        self.client.fire_and_forget(fnf).await;
    }

    pub async fn request_response(&self) {
        println!();
        println!("====ExecRequestResponse====");
        let request = HelloRequest { id: "1".to_owned() };
        let json_data = request_to_json(&request);
        let p = Payload::builder()
            .set_data(Bytes::from(json_data))
            .set_metadata_utf8("RUST")
            .build();

        let resp: Payload = self.client.request_response(p).await.unwrap();
        let data = resp.data();

        let hello_response = data_to_response(data);
        println!("<< [request_response] response id:{},value:{}", hello_response.id, hello_response.value);
    }

    pub async fn request_stream(&self) {
        println!();
        println!("====ExecRequestStream====");
        let request = HelloRequests { ids: RequestCoon::random_ids(5) };
        let json_data = requests_to_json(&request);
        let sending = Payload::builder()
            .set_data(Bytes::from(json_data))
            .set_metadata_utf8("RUST")
            .build();
        let mut results = self.client.request_stream(sending);
        loop {
            match results.next().await {
                Some(v) => {
                    let data = v.data();
                    let hello_response = data_to_response(data);
                    println!("<< [request_stream] response:{:?}", hello_response)
                }
                None => break,
            }
        }
    }

    pub async fn request_channel(&self) {
        println!();
        println!("====ExecRequestChannel====");

        let mut sends = vec![];
        for _i in 0..3 {
            let request = HelloRequests { ids: RequestCoon::random_ids(3) };
            let json_data = requests_to_json(&request);
            let sending = Payload::builder()
                .set_data(Bytes::from(json_data))
                .set_metadata_utf8("RUST")
                .build();
            sends.push(sending);
            sleep( Duration::from_millis(100));
        }
        sleep( Duration::from_millis(1000));

        let iter = stream::iter(sends);
        let pin = Box::pin(iter);

        let mut resps = self.client.request_channel(pin);
        while let Some(v) = resps.next().await {
            let data = v.data();
            let hello_response = data_to_response(data);
            println!("<< [request_channel] response:{:?}", hello_response);
        }
    }

    pub fn random_ids(max: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for i in 0..max {
            result.push(i.to_string());
        }
        result
    }

    pub fn random_id(max: i32) -> String {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0, max);
        x.to_string()
    }
}