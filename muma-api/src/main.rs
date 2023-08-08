mod broadcaster;
mod realtime;
mod stream;

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use muma_config::Config;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::broadcaster::Broadcaster;

/// Handle the update count
async fn publish(_req: HttpRequest, broadcaster: web::Data<Broadcaster>) -> HttpResponse {
    broadcaster.publish(Message::new("Hello World\n")).await;
    HttpResponse::Ok().finish()
}

/// Handle the get count
async fn subscribe(_req: HttpRequest, broadcaster: web::Data<Broadcaster>) -> impl Responder {
    let stream = broadcaster.new_client().await;
    HttpResponse::Ok().streaming(stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _config = Config::from_env().expect("(muma-api): missing environment variables");

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder
        .set_private_key_file("./muma-api/key.pem", SslFiletype::PEM)
        .unwrap();

    builder
        .set_certificate_chain_file("./muma-api/cert.pem")
        .unwrap();

    println!("Server started at https://localhost:3001");

    let broadcaster = Broadcaster::new();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new().wrap(cors).service(
            web::resource("/streaming")
                .app_data(web::Data::from(Arc::clone(&broadcaster)))
                .route(web::get().to(subscribe))
                .route(web::post().to(publish)),
        )
    })
    .bind_openssl("localhost:3001", builder)?
    .run()
    .await
}
