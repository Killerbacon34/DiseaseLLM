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

    let manager = RedisConnectionManager::new(redis_link.clone()).unwrap();
    let redis_pool = r2d2::Pool::builder().build(manager).unwrap();
    println!("✅ Successfully connected to the Redis server!");

    let redis_session = RedisSessionStore::new(redis_link.clone()).await.unwrap();
    println!("✅ Successfully connected to the Redis session store!");

    // Generate a secure random key for session middleware
    let key = Key::generate();

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