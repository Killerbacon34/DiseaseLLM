use std::collections::HashMap;

use actix_web::{HttpResponse, Responder, post, web, get};
use reqwest::Client;
use serde_json::{json, Value};
use dotenv::dotenv;
use sqlx::{pool, PgPool};
pub async fn queryDeepSeekR1(id: String, data: HashMap<String, String>, redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>) -> Result<HttpResponse, actix_web::Error> {
    let payload = json!({
        //TODO: FIND OUT WHAT THE INPUTS SHOULD BE
        "mode": "deepseek/deepseek-r1-zero:free",
        "messages": [
            {
                "role": "user",
                "content": "I have a headache, what are the options?"
            }
        ]
    });
    dotenv::dotenv().ok();

    let api_url = "https://openrouter.ai/api/v1/chat/completions";
    let client = Client::new();
    println!("API KEY: {}", dotenv::var("LLM_KEY").unwrap());
    let response = client.post(api_url)
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .header("Authorization", format!("Bearer {}", dotenv::var("LLM_KEY").unwrap()))
    .json(&payload)
    .send()
    .await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let output= response.json::<Value>().await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    println!("{:#?}", output);
    Ok(HttpResponse::Ok().json(output))
    
}

pub async fn queryGemini(id: String, data: HashMap<String, String>, redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>) -> Result<HttpResponse, actix_web::Error> {
    let payload = json!({
        //TODO: FIND OUT WHAT THE INPUTS SHOULD BE
        "mode": "google/gemini-2.5-pro-exp-03-25:free",
        "messages": [
            {
                "role": "user",
                "content": "I have a headache, what are the options?"
            }
        ]
    });
    dotenv::dotenv().ok();

    let api_url = "https://openrouter.ai/api/v1/chat/completions";
    let client = Client::new();
    println!("API KEY: {}", dotenv::var("LLM_KEY").unwrap());
    let response = client.post(api_url)
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .header("Authorization", format!("Bearer {}", dotenv::var("LLM_KEY").unwrap()))
    .json(&payload)
    .send()
    .await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let output= response.json::<Value>().await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    println!("{:#?}", output);
    Ok(HttpResponse::Ok().json(output))
    
}

pub async fn queryLlama(id: String, data: HashMap<String, String>, redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>) -> Result<HttpResponse, actix_web::Error> {
    let payload = json!({
        //TODO: FIND OUT WHAT THE INPUTS SHOULD BE
        "mode": "nvidia/llama-3.1-nemotron-70b-instruct:free",
        "messages": [
            {
                "role": "user",
                "content": "I have a headache, what are the options?"
            }
        ]
    });
    dotenv::dotenv().ok();

    let api_url = "https://openrouter.ai/api/v1/chat/completions";
    let client = Client::new();
    println!("API KEY: {}", dotenv::var("LLM_KEY").unwrap());
    let response = client.post(api_url)
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .header("Authorization", format!("Bearer {}", dotenv::var("LLM_KEY").unwrap()))
    .json(&payload)
    .send()
    .await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let output= response.json::<Value>().await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    println!("{:#?}", output);
    Ok(HttpResponse::Ok().json(output))
    
}