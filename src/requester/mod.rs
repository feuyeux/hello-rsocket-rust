use futures::prelude::*;
use crate::common::*;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;
use rsocket_rust::prelude::*;
use rsocket_rust_transport_tcp::TcpClientTransport;
use rsocket_rust::Client;

pub struct RequestCoon {
    pub client: Client
}

impl RequestCoon {
    pub async fn new() -> RequestCoon {
        RequestCoon {
            client: RSocketFactory::connect()
                .transport(TcpClientTransport::from("127.0.0.1:7878"))
                .start()
                .await
                .unwrap()
        }
    }

    pub async fn destroy(self) {
       self.client.wait_for_close().await;
    }

    pub async fn meta_push(&self){
        println!();
        println!("====ExecMetaPush====");
        let meta = Payload::builder().set_metadata_utf8("RUST").build();
        let result = self.client.metadata_push(meta).await;
        result.unwrap();
    }

    pub async fn fnf(&self) {
        println!();
        println!("====ExecFireAndForget====");
        let request = HelloRequest { id: "1".to_owned() };
        let json_data = request_to_json(&request);
        //let bytes = Bytes::from(json_data);
        let fnf = Payload::builder().set_data(json_data).build();
        let result = self.client.fire_and_forget(fnf).await;
        result.unwrap();
    }

    pub async fn request_response(&self) {
        println!();
        println!("====ExecRequestResponse====");
        let request = HelloRequest { id: "1".to_owned() };
        let json_data = request_to_json(&request);
        //let bytes = Bytes::from(json_data);
        let p = Payload::builder()
            .set_data(json_data)
            .set_metadata_utf8("RUST")
            .build();

        let resp = self.client.request_response(p).await.unwrap();
        let hello_response = data_to_response(resp.unwrap().data());
        println!("<< [request_response] response id:{},value:{}", hello_response.id, hello_response.value);
    }

    pub async fn request_stream(&self) {
        println!();
        println!("====ExecRequestStream====");
        let request = HelloRequests { ids: RequestCoon::random_ids(5) };
        let json_data = requests_to_json(&request);
        //let bytes = Bytes::from(json_data);
        let sending = Payload::builder()
            .set_data(json_data)
            .set_metadata_utf8("RUST")
            .build();
        let mut results = self.client.request_stream(sending);
        loop {
            match results.next().await {
                Some(Ok(v)) => {
                    let data = v.data();
                    let hello_response = data_to_response(data);
                    println!("<< [request_stream] response:{:?}", hello_response)
                }
                Some(Err(e)) => {
                    println!("CHANNEL_RESPONSE FAILED: {:?}", e);
                    break;
                }
                None => break,
            }
        }
    }

    pub async fn request_channel(&self) {
        println!();
        println!("====ExecRequestChannel====");

        let mut sends = vec![];
        for _ in 0..3 {
            let request = HelloRequests { ids: RequestCoon::random_ids(3) };
            let json_data = requests_to_json(&request);
            //let bytes = Bytes::from(json_data);
            let sending = Payload::builder()
                .set_data(json_data)
                .set_metadata_utf8("RUST")
                .build();
            sends.push(Ok(sending));
        }
        let iter = stream::iter(sends);
        let pin = Box::pin(iter);
        let mut responses = self.client.request_channel(pin);
        sleep(Duration::from_millis(1000));
        loop {
            match responses.next().await {
                Some(Ok(v)) => {
                    let data = v.data();
                    let hello_response = data_to_response(data);
                    println!("<< [request_channel] response:{:?}", hello_response);
                }
                Some(Err(e)) => {
                    println!("CHANNEL_RESPONSE FAILED: {:?}", e);
                    break;
                }
                None => break,
            }
        }
    }

    pub fn random_ids(max: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..max {
            let rnd_id = rng.gen_range(0..5);
            result.push(rnd_id.to_string());
        }
        result
    }
}