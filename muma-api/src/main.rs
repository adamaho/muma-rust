mod broadcaster;

use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use muma_config::Config;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};

use crate::broadcaster::{Broadcaster, MessageQuery};

#[derive(Serialize, Deserialize)]
struct Count {
    count: usize,
}

/// Handle the index request
async fn ping(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

/// Handle the update count
async fn publish(
    _req: HttpRequest,
    query: web::Query<MessageQuery>,
    broadcaster: web::Data<Broadcaster>,
) -> HttpResponse {
    broadcaster.publish(&query).await;
    HttpResponse::Ok().finish()
}

/// Handle the get count
async fn subscribe(_req: HttpRequest, broadcaster: web::Data<Broadcaster>) -> HttpResponse {
    let rx = broadcaster.new_client().await;
    HttpResponse::Ok().streaming(rx)
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

        App::new()
            .wrap(cors)
            .route("/ping", web::get().to(ping))
            .service(
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
