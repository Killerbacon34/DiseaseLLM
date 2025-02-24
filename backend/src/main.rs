//use std::{env, io};
use actix_web::{App, HttpServer, middleware::Logger, web};
use actix_cors::Cors;
use sqlx::postgres:: { PgPoolOptions,  PgPool };
use std::env; //FOR KEY STORAGE
use dotenv::dotenv;
mod upload;
mod signup;
mod login;


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let protocol = env::var("DB_PROTOCOL").unwrap_or_else(|_| "postgres".to_string());
    let host = env::var("DB_HOST").expect("DB_HOST must be set");
    let port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
    let username = env::var("DB_USER").expect("DB_USER must be set");
    let password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| format!("{}://{}:{}@{}:{}/{}", protocol, username, password, host, port, db_name));
    println!("Connecting to {}", database_url);
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
            //.service(upload::upload)
            .service(
              signup::signup
            )
            .service(
                upload::upload
            )
            .service(   
                login::login
            )
    })
    .bind("0.0.0.0:4545")?
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