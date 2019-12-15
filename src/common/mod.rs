use serde::{Deserialize, Serialize};
use serde_json::Result;
use rsocket_rust::frame::Payload;

#[derive(Serialize, Deserialize)]
pub struct HelloRequest {
    id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HelloRequests {
    ids: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HelloResponse {
    id: String,
    name: String,
}

/*
pub fn request_to_json() -> &'static str {
    let request = &HelloRequest { id: "1".to_owned() };
    let request_json = serde_json::to_string(request).unwrap();
    let data :&'static str = request_json.as_str();
    println!("data={:?}", data);
    data
}
*/