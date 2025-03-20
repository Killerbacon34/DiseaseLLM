//use std::{env, io};
use actix_web::{App, HttpServer, middleware::Logger, web};
use actix_cors::Cors;
use sqlx::{database, postgres:: { PgPool, PgPoolOptions }};
use std::env; //FOR KEY STORAGE
use dotenv::dotenv;
mod upload;
mod signup;
mod login;
//mod queryLLM;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = "postgres:///?0.0.0.0&port=5432&dbname=diseasellm&user=user&password=cybears";
    println!("Connecting to {}", &database_url);
    let pool = PgPoolOptions::new().max_connections(10).connect(&database_url).await
    .expect("Failed to create pool");
    println!("✅ Successfully connected to the database!");
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
            .app_data(web::Data::new(pool.clone()))
            .service(upload::upload)
            .service(
              signup::signup
            )
            .service(
                upload::upload
            )
            .service(   
                login::login
            )
            /* .service(
                queryLLM::query_clinical_bert
            )*/
    })
    .bind("0.0.0.0:5432")?
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