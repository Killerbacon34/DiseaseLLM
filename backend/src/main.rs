use actix_web::{App, HttpServer, middleware::Logger, web, cookie::{self, SameSite, Key}};
use actix_identity::{IdentityMiddleware}; 
use actix_session::{config::PersistentSession, storage::{CookieSessionStore, RedisSessionStore}, SessionMiddleware};
use actix_cors::Cors;
//use sqlx::{database, postgres:: { PgPool, PgPoolOptions }, Connection, PgConnection};
use r2d2_redis::RedisConnectionManager;
use std::{env, time::Duration};
use dotenv::dotenv;
mod upload;
mod signup;
mod login;
mod queryLLM;
mod anonymous;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file, but prioritize existing environment variables
    dotenv::dotenv().ok();

    // Example: Load Redis configuration
    let redis_host = std::env::var("REDIS_HOST").unwrap_or_else(|_| "localhost".to_string());
    let redis_port = std::env::var("REDIS_PORT").unwrap_or_else(|_| "6379".to_string());
    let redis_link = format!("redis://{}:{}", redis_host, redis_port);
    println!("Connecting to Redis at {}", redis_link);

    let manager = match RedisConnectionManager::new(redis_link.clone()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("❌ Failed to create Redis connection manager: {}", e);
            // Handle the error appropriately for your application
            // (e.g., return an error, use a default value, etc.)
            panic!("Cannot continue without Redis connection");
        }
    };

    // Build the pool with a timeout and error handling
    let redis_pool = match r2d2::Pool::builder()
        .connection_timeout(std::time::Duration::from_secs(5))
        .build(manager) {
        Ok(pool) => {
            println!("✅ Successfully created Redis connection pool!");
            pool
        },
        Err(e) => {
            eprintln!("❌ Failed to build Redis connection pool: {}", e);
            panic!("Cannot continue without Redis connection pool");
        }
    };


    match redis_pool.get() {
        Ok(_) => println!("✅ Successfully connected to the Redis server!"),
        Err(e) => eprintln!("⚠️ Warning: Could not get a connection from the pool: {}", e)
    };    
    let key = Key::generate();

    let redis_session = RedisSessionStore::new(redis_link.clone()).await.unwrap();
    println!("✅ Successfully connected to the Redis session store!");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(
                IdentityMiddleware::builder()
                    .visit_deadline(Some(Duration::from_secs(60 * 15))) // 15 min
                    .login_deadline(Some(Duration::from_secs(60 * 30))) // 30 min
                    .build(),
            )
            .wrap(
                SessionMiddleware::builder(redis_session.clone(), key.clone())
                    .cookie_secure(false) // Set to `true` only if using HTTPS
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Lax) // Use `Lax` for better compatibility
                    .cookie_name("session_token".to_string())
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(cookie::time::Duration::hours(2)),
                    )
                    .build(),
            )
            .app_data(web::Data::new(redis_pool.clone()))
            .service(web::scope("/auth")
                .service(login::login)
                .service(signup::release)
                .service(signup::signup)
            )
            .service(web::scope("/api")
                .service(upload::upload_file)
                .service(upload::upload_form)
                .service(upload::status)
            )
            .service(web::scope("/anon")
                .service(anonymous::anon_manual_upload)
                .service(anonymous::anon_check_results)
                .service(anonymous::anon_release)
                .service(anonymous::checkconn)
                .service(anonymous::check_session)
            )
            .configure(|cfg| {
                if std::env::var("ENABLE_INSECURE").unwrap_or_else(|_| "false".to_string()) == "true" {
                    cfg.service(web::scope("/insecure")
                        .service(upload::anon_all_output)
                    );
                }
            })
    })
    .bind(format!("0.0.0.0:{}", std::env::var("BACKEND_PORT").unwrap_or_else(|_| "4545".to_string())))?
    .run()
    .await?;

    Ok(())
}