## hello-rsocket-rust

![](https://github.com/feuyeux/hello-rsocket/blob/master/doc/hello-rsocket.png)

```sh
cargo run --bin server

[2021-01-17T08:15:13.903Z INFO  server] +++++++ echo server started! +++++++
[2021-01-17T08:15:19.159Z INFO  server] accept setup: SetupPayload { m: None, d: None, keepalive: (20s, 90s), mime_m: Some(b"application/binary"), mime_d: Some(b"application/binary") }
[2021-01-17T08:15:19.159Z INFO  hello_rsocket::responder] >> [metadata_push]: Payload { m: Some(b"RUST"), d: None }
[2021-01-17T08:15:19.664Z INFO  hello_rsocket::responder] >> [fire_and_forget] FNF:1
[2021-01-17T08:15:20.167Z INFO  hello_rsocket::responder] >> [request_response] data:HelloRequest { id: "1" }, meta=Some(b"RUST")
[2021-01-17T08:15:20.671Z INFO  hello_rsocket::responder] >> [request_stream] data:HelloRequests { ids: ["1", "1", "0", "4", "1"] }
[2021-01-17T08:15:21.175Z INFO  hello_rsocket::responder] >> [request_channel] data:HelloRequests { ids: ["0", "4", "2"] }
[2021-01-17T08:15:21.175Z INFO  hello_rsocket::responder] >> [request_channel] data:HelloRequests { ids: ["0", "0", "1"] }
[2021-01-17T08:15:21.175Z INFO  hello_rsocket::responder] >> [request_channel] data:HelloRequests { ids: ["3", "4", "1"] }
```

```sh
cargo run --bin client

[2021-01-17T08:15:19.158Z INFO  hello_rsocket::requester] ====ExecMetaPush====
[2021-01-17T08:15:19.663Z INFO  hello_rsocket::requester] ====ExecFireAndForget====
[2021-01-17T08:15:20.166Z INFO  hello_rsocket::requester] ====ExecRequestResponse====
[2021-01-17T08:15:20.167Z INFO  hello_rsocket::requester] << [request_response] response id:1,value:Bonjour
[2021-01-17T08:15:20.670Z INFO  hello_rsocket::requester] ====ExecRequestStream====
[2021-01-17T08:15:20.672Z INFO  hello_rsocket::requester] << [request_stream] response:HelloResponse { id: "1", value: "Bonjour" }
[2021-01-17T08:15:20.672Z INFO  hello_rsocket::requester] << [request_stream] response:HelloResponse { id: "1", value: "Bonjour" }
[2021-01-17T08:15:20.672Z INFO  hello_rsocket::requester] << [request_stream] response:HelloResponse { id: "0", value: "Hello" }
[2021-01-17T08:15:20.672Z INFO  hello_rsocket::requester] << [request_stream] response:HelloResponse { id: "4", value: "Ciao" }
[2021-01-17T08:15:20.672Z INFO  hello_rsocket::requester] << [request_stream] response:HelloResponse { id: "1", value: "Bonjour" }
[2021-01-17T08:15:21.175Z INFO  hello_rsocket::requester] ====ExecRequestChannel====
[2021-01-17T08:15:22.175Z INFO  hello_rsocket::requester] << [request_channel] response:HelloResponse { id: "0", value: "Hello" }
[2021-01-17T08:15:22.175Z INFO  hello_rsocket::requester] << [request_channel] response:HelloResponse { id: "4", value: "Ciao" }
[2021-01-17T08:15:22.175Z INFO  hello_rsocket::requester] << [request_channel] response:HelloResponse { id: "2", value: "Hola" }
[2021-01-17T08:15:22.175Z INFO  hello_rsocket::requester] << [request_channel] response:HelloResponse { id: "0", value: "Hello" }
[2021-01-17T08:15:22.175Z INFO  hello_rsocket::requester] << [request_channel] response:HelloResponse { id: "0", value: "Hello" }
[2021-01-17T08:15:22.175Z INFO  hello_rsocket::requester] << [request_channel] response:HelloResponse { id: "1", value: "Bonjour" }
[2021-01-17T08:15:22.176Z INFO  hello_rsocket::requester] << [request_channel] response:HelloResponse { id: "3", value: "こんにちは" }
[2021-01-17T08:15:22.176Z INFO  hello_rsocket::requester] << [request_channel] response:HelloResponse { id: "4", value: "Ciao" }
[2021-01-17T08:15:22.176Z INFO  hello_rsocket::requester] << [request_channel] response:HelloResponse { id: "1", value: "Bonjour" }
```

- https://github.com/rsocket/rsocket-rust