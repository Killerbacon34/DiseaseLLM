#[macro_use]
extern crate actix_web;

use std::{env, io};
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_cors::Cors;
mod upload;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(Logger::default())
            .wrap(Cors::default()
            .allow_any_origin() 
            .allow_any_method()
            .allow_any_header()
            .max_age(3600)
            )
            // register HTTP requests handlers
            .service(upload::upload_file) // Register the upload route
    })
    .bind("127.0.0.1:5353")?
    .run()
    .await
}