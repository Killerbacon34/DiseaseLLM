//use std::{env, io};
use actix_web::{App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use sqlx::postgres::PgPool;
mod upload;
mod signup;


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPool::connect("postgres://postgres:password@localhost:5432/postgres")
        .await?;
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init(); 
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::default()
                .allow_any_origin() 
                .allow_any_method()
                .allow_any_header()
                .max_age(3600)
            )
            //.service(upload::upload)
            .service(
              signup::signup
            )
    })
    .bind("127.0.0.1:5353")?
    .run()
    .await?;
    Ok(())
}

async fn validate_token(pool: &PgPool, token: &str) -> bool {
    sqlx::query("SELECT 1 FROM users WHERE token = $1")
        .bind(token)
        .fetch_one(pool)
        .await
        .is_ok()
}