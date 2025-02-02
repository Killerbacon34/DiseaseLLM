#[macro_use]
extern crate actix_web;
extern crate actix_jwt_auth_middleware;
use std::{env, io};
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_cors::Cors;
mod upload;
mod signup;
struct user {
    name: String,
    email: String,
    password: String,
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let KeyPair {
        pk: private_key,
        sk: public_key,
    } KeyPair::generate();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(Cors::default()
            .allow_any_origin() 
            .allow_any_method()
            .allow_any_header()
            .max_age(3600)
            .wrap(Logger::default())
            )
            // register HTTP requests handlers
            .service(upload::upload_file) // Register the upload route
            .use_jwt(auth, web::scope("").service(upload::upload_file))
            .service(signup::signup) // Register the signup route

    })
    .bind("127.0.0.1:5353")?
    .run()
    .await
}