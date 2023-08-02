use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

/// Handle the index request
async fn ping(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder
        .set_private_key_file("./muma-api/key.pem", SslFiletype::PEM)
        .unwrap();

    builder
        .set_certificate_chain_file("./muma-api/cert.pem")
        .unwrap();

    println!("Server started at https://localhost:3001");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new().wrap(cors).route("/ping", web::get().to(ping))
    })
    .bind_openssl("localhost:3001", builder)?
    .run()
    .await
}
