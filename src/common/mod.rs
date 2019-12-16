use serde::{Deserialize, Serialize};
use bytes::{Bytes, Buf};
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct HelloRequest {
    pub id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HelloRequests {
    pub ids: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HelloResponse {
    pub  id: String,
    pub  value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Address {
    pub  street: String,
    pub city: String,
}

pub fn json_to_request(json_data: String) -> HelloRequest {
    let request: HelloRequest = serde_json::from_str(json_data.as_str()).unwrap();
    request
}

pub fn request_to_json(address: &HelloRequest) -> String {
    let json_request = serde_json::to_string(&address).unwrap();
    json_request
}

pub fn json_to_requests(json_data: String) -> HelloRequests {
    let request: HelloRequests = serde_json::from_str(json_data.as_str()).unwrap();
    request
}

pub fn requests_to_json(address: &HelloRequests) -> String {
    let json_request = serde_json::to_string(&address).unwrap();
    json_request
}

pub fn data_to_response(data: &Option<Bytes>) -> HelloResponse {
    let bytes_response = data.as_ref().unwrap();
    let s = match str::from_utf8(bytes_response.bytes()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let json_response = String::from(s);
    let hello_response = json_to_response(json_response);
    hello_response
}

pub fn json_to_response(json_data: String) -> HelloResponse {
    let response: HelloResponse = serde_json::from_str(json_data.as_str()).unwrap();
    response
}

pub fn response_to_json(address: &HelloResponse) -> String {
    let json_response = serde_json::to_string(&address).unwrap();
    json_response
}
