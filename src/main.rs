use std::env;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello broke world!")
}

// wish there was a way to auto-register this stuff
#[get("/revision")]
async fn revision() -> impl Responder {
    HttpResponse::Ok().body(env::var("REVISION").unwrap_or("UNKNOWN".to_string()))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(revision)
            .service(echo)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
