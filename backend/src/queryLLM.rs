use std::collections::HashMap;
use actix_multipart::form::json::Json;
use actix_web::{HttpResponse, Responder, web};
use reqwest::Client;
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn queryDeepSeekR1(
    id: String,
    data: Value,
    redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>,
    db_pool: web::Data<PgPool>
){
    // Fetch user data from database
    #[derive(sqlx::FromRow)]
    struct UserInfo {
        height: Option<i32>,
        weight: Option<i32>,
        age: Option<i32>,
        gender: Option<String>,
        race: Option<String>,
        symptoms: Option<Vec<String>>,
        blood_pressure: Option<String>,
        heart_rate: Option<i32>,
        temperature: Option<f32>,
        medications: Option<Vec<String>>,
        allergies: Option<Vec<String>>,
        alcohol_use: Option<String>,
        smoking: Option<String>,
        drug_use: Option<String>,
    }

    let user_info: UserInfo = sqlx::query_as("
        SELECT Height, Weight, Age, Gender, Race, Symptoms, BloodPressure, 
               HeartRate, Temperature, Medications, Allergies, AlcoholUse, 
               Smoking, DrugUse
        FROM USERINFO WHERE id = $1")
    .bind(&id)
    .fetch_one(db_pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let mut prompt = String::new();

    prompt.push_str(&format!("I am a {} year old {} {}. ", 
        user_info.age.unwrap_or(0), 
        user_info.gender.as_deref().unwrap_or("unknown"), 
        user_info.race.as_deref().unwrap_or("")
    ));
    
    prompt.push_str(&format!("My height is {} cm and weight is {} kg. ", 
        user_info.height.unwrap_or(0), 
        user_info.weight.unwrap_or(0)
    ));
    
    if let Some(bp) = user_info.blood_pressure {
        prompt.push_str(&format!("My blood pressure is {}. ", bp));
    }
    if let Some(hr) = user_info.heart_rate {
        prompt.push_str(&format!("My heart rate is {} bpm. ", hr));
    }
    if let Some(temp) = user_info.temperature {
        prompt.push_str(&format!("My temperature is {}°C. ", temp));
    }
    
    if let Some(symptoms) = user_info.symptoms {
        if !symptoms.is_empty() {
            prompt.push_str(&format!("I'm experiencing these symptoms: {}. ", 
                symptoms.join(", ")
            ));
        }
    }
    
    if let Some(meds) = user_info.medications {
        if !meds.is_empty() {
            prompt.push_str(&format!("I'm currently taking these medications: {}. ", 
                meds.join(", ")
            ));
        }
    }
    
    if let Some(allergies) = user_info.allergies {
        if !allergies.is_empty() {
            prompt.push_str(&format!("I have these allergies: {}. ", 
                allergies.join(", ")
            ));
        }
    }
    
    if let Some(alcohol) = user_info.alcohol_use {
        prompt.push_str(&format!("Alcohol use: {}. ", alcohol));
    }
    if let Some(smoking) = user_info.smoking {
        prompt.push_str(&format!("Smoking status: {}. ", smoking));
    }
    if let Some(drugs) = user_info.drug_use {
        prompt.push_str(&format!("Drug use: {}. ", drugs));
    }
    
    if let Some(query) = data.get("query") {
        prompt.push_str(&format!("My question is: {}", query));
    } else {
        prompt.push_str("What advice can you give me about my health?");
    }

    let payload = json!({
        //TODO: FIND OUT WHAT THE INPUTS SHOULD BE
        "mode": "deepseek/deepseek-r1-zero:free",
        "messages": [
            {
                "role": "user",
                "content": "You are a knowledgeable medical assistant. Provide helpful, 
                evidence-based advice while reminding users to consult with their doctor for professional medical advice."
            }, 
            {
                "role": "user",
                "content": prompt
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
}

pub async fn queryGemini(id: String, data: Value, redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>, db_pool: web::Data<PgPool>) {
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
    
}

pub async fn queryLlama(id: String, data: HashMap<String, String>, redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>, db_pool: web::Data<PgPool>) {
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
}