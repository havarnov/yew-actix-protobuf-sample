extern crate actix_web;
extern crate protobuf;
extern crate protos;

use std::time::{SystemTime, UNIX_EPOCH};
use std::path::PathBuf;

use actix_web::{server, App, HttpRequest, HttpResponse, fs, http::Method};
use actix_web::Result as ActixResult;

use protobuf::Message;

use protos::timestamp::TimestampResponse;

fn index(_req: HttpRequest) -> ActixResult<fs::NamedFile> {
    let path: PathBuf = "./index.html".into();
    Ok(fs::NamedFile::open(path)?)
}

fn wasm(_req: HttpRequest) -> ActixResult<fs::NamedFile> {
    let path: PathBuf = "./static/client.wasm".into();
    Ok(fs::NamedFile::open(path)?)
}

fn fetch_timestamp(_req: HttpRequest) -> HttpResponse {
    let mut response = TimestampResponse::new();

    if let Ok(n) = SystemTime::now().duration_since(UNIX_EPOCH) {
        response.set_timestamp(n.as_secs());
    }

    match response.write_to_bytes() {
        Ok(bytes) => HttpResponse::Ok().body(bytes),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

fn main() {
    server::new(|| {
        App::new()
            .handler(
                "/static",
                fs::StaticFiles::new("./static"))
            .resource("/api/timestamp", |r| r.f(fetch_timestamp))
            .resource("/client.wasm", |r| r.f(wasm))
            .default_resource(|r| r.method(Method::GET).f(index))
    })
    .bind("127.0.0.1:8001")
    .expect("Can not bind to port 8001")
    .run();
}
