## hello-rsocket-rust

![](https://github.com/feuyeux/hello-rsocket/blob/master/doc/hello-rsocket.png)

```bash
▶ cargo run

====ExecMetaPush====
>> [metadata_push]: Payload { m: Some(b"RUST"), d: None }

====ExecFireAndForget====
>> [fire_and_forget] FNF:1

====ExecRequestResponse====
>> [request_response] data:HelloRequest { id: "1" }, meta=Some(b"RUST")
<< [request_response] response id:1,value:Bonjour

====ExecRequestStream====
>> [request_stream] data:HelloRequests { ids: ["0", "2", "2", "1", "3"] }
<< [request_stream] response:HelloResponse { id: "0", value: "Hello" }
<< [request_stream] response:HelloResponse { id: "2", value: "Hola" }
<< [request_stream] response:HelloResponse { id: "2", value: "Hola" }
<< [request_stream] response:HelloResponse { id: "1", value: "Bonjour" }
<< [request_stream] response:HelloResponse { id: "3", value: "こんにちは" }

====ExecRequestChannel====
>> [request_channel] data:HelloRequests { ids: ["1", "2", "1"] }
>> [request_channel] data:HelloRequests { ids: ["0", "2", "0"] }
>> [request_channel] data:HelloRequests { ids: ["3", "1", "4"] }
<< [request_channel] response:HelloResponse { id: "1", value: "Bonjour" }
<< [request_channel] response:HelloResponse { id: "2", value: "Hola" }
<< [request_channel] response:HelloResponse { id: "1", value: "Bonjour" }
<< [request_channel] response:HelloResponse { id: "0", value: "Hello" }
<< [request_channel] response:HelloResponse { id: "2", value: "Hola" }
<< [request_channel] response:HelloResponse { id: "0", value: "Hello" }
<< [request_channel] response:HelloResponse { id: "3", value: "こんにちは" }
<< [request_channel] response:HelloResponse { id: "1", value: "Bonjour" }
<< [request_channel] response:HelloResponse { id: "4", value: "Ciao" }
```