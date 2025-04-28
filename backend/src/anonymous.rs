use std::collections::HashMap;

use actix_web::{error::ErrorInternalServerError, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, cookie::Key};
use actix_identity::Identity;
use actix_session::Session;
use r2d2_redis::redis::Commands;
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};
use crate::queryLLM; // Import the queryLLM module
use base64::{engine::general_purpose::URL_SAFE, Engine};
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
    HttpResponse::Ok().body(format!("{}", session_token))
}

#[post("/manualupload")]
pub async fn anon_manual_upload(redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>, id: Option<Identity>, data: web::Json<HashMap<String, String>>) -> Result<HttpResponse, actix_web::Error> {
    if let Some(id) = id {
        let mut con = redis_pool.get().map_err(ErrorInternalServerError)?;
        con.set(format!("anonLlama_{}_data", id.id().unwrap()), "").map_err(ErrorInternalServerError)?;
        con.set(format!("anonDeepSeek_{}_data", id.id().unwrap()), "").map_err(ErrorInternalServerError)?;
        con.set(format!("anonGemini_{}_data", id.id().unwrap()), "").map_err(ErrorInternalServerError)?;
        con.set(format!("anonConsensus_{}_data", id.id().unwrap()), "").map_err(ErrorInternalServerError)?;
        // let _ = queryLLM::queryDeepSeekR1( id.id().unwrap(), data.clone(), redis_pool.clone(), db_pool.clone());
        // let _ = queryLLM::queryGemini( id.id().unwrap(), data.clone(), redis_pool.clone());
        // let _ = queryLLM::queryLlama( id.id().unwrap(), data.clone(), redis_pool.clone());
       Ok(HttpResponse::Ok().body("Data uploaded successfully"))
    } else {
       Ok(HttpResponse::Unauthorized().body("Unauthorized"))
    }
}

/*#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct ResultData {
    deepseek: String,
    gemini: String,
    llama: String,
}*/

#[get("/results")]
pub async fn anon_check_results(
    redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>,
    id: Option<Identity>,
) -> impl Responder {
    if let Some(id) = id {
        let mut con = redis_pool
            .get()
            .map_err(ErrorInternalServerError)
            .expect("Failed to get redis connection");
        if con.exists(format!("consensus_{}", id.id().unwrap())).unwrap_or(false) {
            let res: Result<String, r2d2_redis::redis::RedisError> = con.get(format!("consensus_{}", id.id().unwrap()));
            if res.is_ok() {
                let res_value = res.unwrap();
                let parts: Vec<&str> = res_value.split("#").collect();
                println!("Parts: {:?}", parts);
                return HttpResponse::Ok().json(serde_json::json!({
                    "Diagnosis": parts[0],
                    "Treatment Plan": parts[1],
                    "Drug Usage Plan": parts[2],
                }));
            }
            else {
                return HttpResponse::InternalServerError().body("No consensus found");
            }
        } else {
                return HttpResponse::InternalServerError().body("No consensus found");
        }
        
                /*} else {
                    return HttpResponse::Ok().body("No sandwiched content found");
                }*/
                    }
    else {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }
}

#[get("/check-session")]
pub async fn check_session(id: Option<Identity>) -> impl Responder {
    if let Some(id) = id {
        println!("Session ID: {}", id.id().unwrap_or("unknown".to_string()));
    } else {
        println!("No session found");
    }
    HttpResponse::Ok().body("Session check complete")
}

#[get("/checkconn")]
pub async fn checkconn() -> impl Responder {
    HttpResponse::Ok().body("Connection is active")
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct ResultData {
    deepseek: String,
    gemini: String,
    llama: String,
}


