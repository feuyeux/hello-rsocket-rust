```rust
▶ cargo run

====ExecFireAndForget====

====ExecRequestResponse====
>>>>>>>> fire_and_forget: Payload { m: None, d: Some(b"Mock FNF") }
>>>>>>>> metadata_push: Payload { m: Some(b"metadata only!"), d: None }
>>>>>>>> request_response: data=Some(b"Hello World!"),meta=Some(b"Rust")
<<<<<<<< : Some(b"Hello World!")

====ExecRequestStream====
>>>>>>>> request_stream: Payload { m: Some(b"foobar"), d: Some(b"Hello Rust!") }
<<<<<<<< STREAM: Payload { m: Some(b"foobar"), d: Some(b"Hello Rust!") }
<<<<<<<< STREAM: Payload { m: Some(b"foobar"), d: Some(b"Hello Rust!") }
<<<<<<<< STREAM: Payload { m: Some(b"foobar"), d: Some(b"Hello Rust!") }

====ExecRequestChannel====
Payload { m: Some(b"Rust"), d: Some(b"Hello#0") }
Payload { m: Some(b"Rust"), d: Some(b"Hello#1") }
Payload { m: Some(b"Rust"), d: Some(b"Hello#2") }
<<<<<<<< CHANNEL: Payload { m: Some(b"Rust"), d: Some(b"Hello#0") }
<<<<<<<< CHANNEL: Payload { m: Some(b"Rust"), d: Some(b"Hello#1") }
<<<<<<<< CHANNEL: Payload { m: Some(b"Rust"), d: Some(b"Hello#2") }

```