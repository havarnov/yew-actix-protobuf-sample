# sample web app and server in rust

This is a sample app of a web server written in actix-web and a web app written in yew. The communication between server and client is http requests with protobuf serialized messages.

## run server

```bash
cd server
cargo run
```

## run client

```bash
cargo +nightly install cargo-web
cd client
cargo +nightly web start --target=wasm32-unknown-unknown --release
```