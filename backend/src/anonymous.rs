use std::collections::HashMap;

use actix_web::{error::ErrorInternalServerError, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, cookie::Key};
use actix_identity::Identity;
use actix_session::Session;
use r2d2_redis::redis::Commands;
use rand::RngCore;
use serde::{Deserialize, Serialize};
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
    pool: web::Data<PgPool>,
    redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>,
    id: Option<Identity>,
) -> impl Responder {
    if let Some(id) = id {
        let con = redis_pool
            .get()
            .map_err(ErrorInternalServerError)
            .expect("Failed to get redis connection");
        let res = sqlx::query_as::<_, (String,)>("SELECT consensus FROM results WHERE id = $1")
            .bind(id.id().unwrap())
            .fetch_one(pool.get_ref())
            .await
            .map(|row| row.0);

        match res {
            Err(e) => {
                eprintln!("Error fetching results: {}", e);
                return HttpResponse::InternalServerError().body("Internal server error");
            }
            Ok(res) => {
                /*let start_delimiter = "xxx";
                let end_delimiter = "xxx";
                let input = res.clone();

                let (sandwiched, remaining) = if let Some(start_index) = input.find(start_delimiter) {
                    if let Some(end_index) =
                        input[start_index + start_delimiter.len()..].find(end_delimiter)
                    {
                        let sandwiched = input[start_index + start_delimiter.len()
                            ..start_index + start_delimiter.len() + end_index]
                            .trim()
                            .to_string();

                        let remaining = format!(
                            "{}{}",
                            &input[..start_index],
                            &input[start_index + start_delimiter.len() + end_index + end_delimiter.len()..]
                        )
                        .trim()
                        .to_string();

                        (Some(sandwiched), Some(remaining))
                    } else {
                        (None, Some(input))
                    }
                } else {
                    (None, Some(input))
                };*/
                //remaining = res;
                println!("remaining: {:?}", res);

                //if let Some(sandwiched) = sandwiched {
                    if !res.is_empty() {
                        // Split the remaining response by '/'
                        let parts: Vec<&str> = res.split('#').map(|s| s.trim()).collect();

                        // Extract the three parts, handling cases where there are fewer than three parts
                        let part1 = parts.get(0).unwrap_or(&"").to_string();
                        let part2 = parts.get(1).unwrap_or(&"").to_string();
                        let part3 = parts.get(2).unwrap_or(&"").to_string();

                        return HttpResponse::Ok().json(serde_json::json!({
                            "Diagnosis": part1,
                            "Treatment Plan": part2,
                            "Drug Usage Plan": part3
                        }));
                    } else {
                        return HttpResponse::Ok().body("No remaining content found");
                    }
                /*} else {
                    return HttpResponse::Ok().body("No sandwiched content found");
                }*/
            }
        }
    } else {
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

#[get("/alloutput")]
pub async fn anon_all_output(pool: web::Data<PgPool>, redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>, id: Option<Identity>) -> impl Responder {
    if let Some(id) = id {
        let mut con = redis_pool.get().map_err(ErrorInternalServerError).expect("Failed to get redis connection");
        let res = sqlx::query_as::<_, ResultData>("SELECT deepseek, gemini, llama FROM results WHERE id = $1")
            .bind(id.id().unwrap())
            .fetch_one(pool.get_ref())
            .await;
        match res {
            Err(e) => {
                eprintln!("Error fetching results: {}", e);
                return HttpResponse::InternalServerError().body("Internal server error")
            }
            Ok(results) => {
                return HttpResponse::Ok().json(results);
            }
        }
    }
    else {
        return HttpResponse::Unauthorized().body("Unauthorized")
    }
}

