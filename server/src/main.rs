extern crate actix_web;
extern crate protobuf;
extern crate protos;

use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::middleware::cors::Cors;
use actix_web::{server, App, HttpRequest, HttpResponse};

use protobuf::Message;

use protos::timestamp::TimestampResponse;

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
        App::new().configure(|app| {
            Cors::for_app(app)
                .allowed_origin("http://127.0.0.1:8000")
                .resource("/api/timestamp", |r| r.f(fetch_timestamp))
                .register()
        })
    }).bind("127.0.0.1:8001")
        .expect("Can not bind to port 8001")
        .run();
}
