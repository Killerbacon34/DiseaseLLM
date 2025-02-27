use chrono::Utc;
use actix_web::error::ErrorInternalServerError;
use actix_web::{HttpResponse, Responder, post, web};
use base64::{self, Engine as _};
use serde::{Serialize, Deserialize};
use rand::RngCore;
use actix_web::web::Data;
use actix_web::web::Json;
use sqlx::PgPool;
use sqlx::Error;

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    username: String,
    pass: String,
    devid: String,
}

#[post("/api/login")]
pub async fn login(pool: web::Data<PgPool>, data: web::Json<LoginData>) -> impl Responder {
    // Query the database to check if the user exists
    let user_result = sqlx::query(
        "SELECT * FROM users WHERE username = $1 AND password = $2",
       
    )
    .bind(&data.username)
    .bind(&data.pass)
    .fetch_one(pool.get_ref())
    .await;

    match user_result {
        Ok(user) => {
            let token = gentoken().await;
            let time_created = Utc::now();
            println!("Token: {:?}", token);
            _ = sqlx::query(
                "INSERT INTO tokens (username, token, timecreated) VALUES ($1, $2, $3)",
            )
            .bind(&data.username)
            .bind(&token)
            .bind(time_created.to_rfc3339())
            .execute(pool.get_ref())
            .await
            .map_err(|e| ErrorInternalServerError(e));

            HttpResponse::Ok().json(token)
        }
        Err(Error::RowNotFound) => {
            HttpResponse::Unauthorized().body("Invalid username or password")
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {}", e))
        }
    }
}

async fn gentoken() -> String {
// TODO: REMEMBER TO ADD DEVID TO THE TOKEN
    let mut rando = [0u8; 32];
    rand::rng().fill_bytes(&mut rando);
    let token = base64::engine::general_purpose::URL_SAFE.encode(&rando);
    return token;
}