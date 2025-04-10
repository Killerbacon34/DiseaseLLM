use std::result;

use actix_web::{get, post, web, HttpMessage, HttpResponse, Responder};
use futures_util::future::UnwrapOrElse;
use serde::{Serialize, Deserialize};
use rand::RngCore;
use sqlx::PgPool;
use sqlx::Error;
use actix_web::HttpRequest;
use actix_identity::Identity;
use base64::{engine::general_purpose::URL_SAFE, Engine};



#[derive(Serialize, Deserialize)]
pub struct LoginData {
    username: String,
    pass: String,
    devid: String,
}

#[post("/login")]
pub async fn login(pool: web::Data<PgPool>, data: web::Json<LoginData>, request: HttpRequest,) -> impl Responder {
    // Query the database to check if the user exists
    let user_result = sqlx::query(
        "SELECT * FROM users WHERE username = $1 AND password = $2",
    )
    .bind(&data.username)
    .bind(&data.pass)
    .fetch_one(pool.get_ref())
    .await;

    match user_result {
        Ok(_user_result) => {
           let mut random_bytes = [0u8; 32];
            rand::rng().fill_bytes(&mut random_bytes);
            let session_token = URL_SAFE.encode(&random_bytes);
            Identity::login(&request.extensions(),session_token.clone()).unwrap();
            println!("Provisioned session token: {}", session_token);
            HttpResponse::Ok().body("Session token provisioned")
        }
        Err(Error::RowNotFound) => {
            println!("Invalid username or password");
            HttpResponse::Unauthorized().body("Invalid username or password")
        }
        Err(e) => {
            println!("Internal server error: {}", e);
            HttpResponse::InternalServerError().body(format!("Internal server error: {}", e))
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct TokenData {
    token: String,
}
/*
#[post("/api/checktoken")]
pub async fn checktoken(pool: web::Data<PgPool>, data: web::Json<TokenData>) -> impl Responder {
    let token = &data.token;
    let token_result = sqlx::query(
        "SELECT * FROM tokens WHERE token = $1",
    )
    .bind(token)
    .fetch_one(pool.get_ref())
    .await;

    match token_result {
        Ok(record) => {
            let time_str: String = match record.try_get("timecreated") {
                Ok(value) => value,
                Err(e) => {
                    println!("Error retrieving timecreated: {}", e);
                    return HttpResponse::InternalServerError().body("Error retrieving timecreated").into();
                }
            };
            let time_created = chrono::DateTime::parse_from_rfc3339(&time_str).unwrap();
            if time_created < Utc::now() - chrono::Duration::minutes(30) {
                revoketoken(pool, token).await;
                return HttpResponse::Unauthorized().body("Token expired");
            }
            HttpResponse::Ok().body("Token is valid")
        }
        Err(Error::RowNotFound) => {
            println!("Invalid token");
            HttpResponse::Unauthorized().body("Invalid token")
        }
        Err(e) => {
            println!("Internal server error: {}", e);
            HttpResponse::InternalServerError().body(format!("Internal server error: {}", e))
        }
    }
}*/

    /*sqlx::query("DELETE FROM tokens WHERE token = $1")
        .bind(token)
        .execute(pool.get_ref())
        .await
        .is_ok()*/