use actix_web::error::ErrorInternalServerError;
use actix_web::{HttpResponse, Responder, post, web};
use base64::{self, Engine as _};
use serde::{Serialize, Deserialize};
use rand::RngCore;
use actix_web::web::Data;
use actix_web::web::Json;
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct  approvalData {
    username: String,
    approval: bool 
}
#[post("/api/verify")]
pub async fn verify(pool: web::Data<PgPool>, data: web::Json<approvalData>) -> Result<HttpResponse, actix_web::Error> {
    _ = sqlx::query(
    "SELECT * FROM users WHERE username = $1 AND approval = $2" 
    )
    .bind(&data.username)  // No need for `.clone()`
    .bind(&data.approval)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| ErrorInternalServerError(e));
}

#[post("/api/approval")]
pub async fn signoff(pool: web::Data<PgPool>, data: web::Json<>) -> Result<HttpResponse, actix_web::Error> {
    _ = sqlx::query(
    "SELECT * FROM users WHERE username = $1 AND password = $2"
    )
    .bind(&data.username)  // No need for `.clone()`
    .bind(&data.pass)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| ErrorInternalServerError(e)); 
    let token = gentoken(pool, &data.username).await;
    Ok(HttpResponse::Ok().json(token))
}

async fn gentoken(pool: web::Data<PgPool>, data: &String) -> String {
// Generate a random 32-byte token
    let mut rando = [0u8; 32];
    rand::rng().fill_bytes(&mut rando);
    let token = base64::engine::general_purpose::URL_SAFE.encode(&rando);
    println!("Token: {}", token);
    _ = sqlx::query("INSERT INTO tokens (username, token) VALUES ($1, $2)")
    .bind(&data)
    .bind(&token)
    .execute(pool.get_ref())
    .await
    .map_err(|e| ErrorInternalServerError(e));
    return token;
}