use std::collections::HashMap;

use actix_web::{error::ErrorInternalServerError, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, cookie::Key};
use actix_identity::Identity;
use actix_session::Session;
use r2d2_redis::redis::Commands;
use rand::RngCore;
use crate::queryLLM; // Import the queryLLM module
use base64::{engine::general_purpose::URL_SAFE, Engine};
use sqlx::PgPool;
/*#[get("/anonapi/release")]
pub async fn anon_release(request: HttpRequest, session: Session)-> impl Responder {
    let mut random_bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut random_bytes);
    let session_token = URL_SAFE.encode(&random_bytes);
    session.insert("session_token", &session_token).unwrap();
    Identity::login(&request.extensions(),session_token.clone()).unwrap();
    println!("Provisioned session token: {}", session_token);
    HttpResponse::Ok().body(format!("Session token provisioned: {}", session_token))
}*/

#[get("/release")]
pub async fn anon_release(request: HttpRequest,)-> impl Responder {
    let mut random_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut random_bytes);
    let session_token = URL_SAFE.encode(&random_bytes);
    Identity::login(&request.extensions(),session_token.clone()).unwrap();
    println!("Provisioned session token: {}", session_token);
    HttpResponse::Ok().body(format!("Session token provisioned: {}", session_token))
}

#[post("/manualupload")]
pub async fn anon_manual_upload(redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>, id: Option<Identity>, data: web::Json<HashMap<String, String>>, db_pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    if let Some(id) = id {
        let mut con = redis_pool.get().map_err(ErrorInternalServerError)?;
        con.set(format!("anonLlama_{}_data", id.id().unwrap()), "").map_err(ErrorInternalServerError)?;
        con.set(format!("anonDeepSeek_{}_data", id.id().unwrap()), "").map_err(ErrorInternalServerError)?;
        con.set(format!("anonGemini_{}_data", id.id().unwrap()), "").map_err(ErrorInternalServerError)?;
        con.set(format!("anonConsensus_{}_data", id.id().unwrap()), "").map_err(ErrorInternalServerError)?;
        let _ = queryLLM::queryDeepSeekR1( id.id().unwrap(), data.clone(), redis_pool.clone(), db_pool.clone());
        let _ = queryLLM::queryGemini( id.id().unwrap(), data.clone(), redis_pool.clone());
        let _ = queryLLM::queryLlama( id.id().unwrap(), data.clone(), redis_pool.clone());
       Ok(HttpResponse::Ok().body("Data uploaded successfully"))
    } else {
       Ok(HttpResponse::Unauthorized().body("Unauthorized"))
    }
}

#[get("/checkResults")]
pub async fn anon_check_results(redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>, id: Option<Identity>) -> impl Responder {
    if let Some(id) = id {
        let mut con = redis_pool.get().map_err(ErrorInternalServerError).expect("Failed to get redis connection");
        //let result_llama: Option<String> = con.get(format!("anonLlama_{}_data", session_id.clone())).map_err(ErrorInternalServerError)?;
        //let result_deepseek: Option<String> = con.get(format!("anonDeepSeek_{}_data", session_id.clone())).map_err(ErrorInternalServerError)?;
        //let result_gemini: Option<String> = con.get(format!("anonGemini_{}_data", session_id.clone())).map_err(ErrorInternalServerError)?;
        let result: Option<String> = con.get(format!("anonConsensus_{}_data", id.id().unwrap())).ok().clone();
        if result.is_some() {
            if let Err(e) = con.del::<_, ()>(format!("anonLlama_{}_data", id.id().unwrap())) {
                eprintln!("Failed to delete key: {}", e);
            }
            if let Err(e) = con.del::<_, ()>(format!("anonDeepSeek_{}_data", id.id().unwrap())) {
                eprintln!("Failed to delete key: {}", e);
            }
            if let Err(e) = con.del::<_, ()>(format!("anonGemini_{}_data", id.id().unwrap())) {
                eprintln!("Failed to delete key: {}", e);
            }
            if let Err(e) = con.del::<_, ()>(format!("anonConsensus_{}_data", id.id().unwrap())) {
                eprintln!("Failed to delete key: {}", e);
            }
            return HttpResponse::Ok().body(result.unwrap())
        } else {
            return HttpResponse::Accepted().body("false")
        }
           
    } else {
        return HttpResponse::Unauthorized().body("Unauthorized")
    }
}

#[get("/check-session")]
pub async fn check_session(id: Identity) -> impl Responder {
    HttpResponse::Ok().body(format!("Session is active for token: {}", id.id().unwrap_or("unknown".to_string())))
}




