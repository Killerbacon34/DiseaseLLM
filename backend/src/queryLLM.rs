use std::collections::HashMap;
use actix_multipart::form::json::Json;
use actix_web::{error::ErrorInternalServerError, web, HttpResponse, Responder};
use chrono::Duration;
use r2d2_redis::redis::Commands;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use tokio::time::sleep;
use sqlx::{pool, PgPool};

pub async fn queryDeepSeekR1(
    id: String,
    data: Value,
    db_pool: web::Data<PgPool>
) -> Result<(), actix_web::Error> {
    println!("QUERYING:::: {}", id);

    let mut prompt = String::new();

    prompt.push_str(&format!("I am a {} year old {} {}. ", 
        data.get("age")
            .and_then(|age| age.as_i64())
            .unwrap_or(0),
        data.get("gender")
            .and_then(|gender| gender.as_str())
            .unwrap_or("unknown"),
        data.get("race")
            .and_then(|race| race.as_str())
            .unwrap_or("unknown")
    ));
    
    prompt.push_str(&format!("My height is {} cm and weight is {} kg. ", 
        data.get("height")
            .and_then(|height| height.as_i64())
            .unwrap_or(0),
        data.get("weight")
            .and_then(|weight| weight.as_i64())
            .unwrap_or(0)
    ));
     

    let bp = data.get("blood_pressure")
        .and_then(|bp| bp.as_str())
        .unwrap_or("unknown");
    prompt.push_str(&format!("My blood pressure is {}. ", bp));
    
    if let Some(hr) = data.get("heart_rate").and_then(|hr| hr.as_i64()) {
        prompt.push_str(&format!("My heart rate is {} bpm. ", hr));
        prompt.push_str(&format!("My heart rate is {} bpm. ", hr));
    }

    if let Some(temp) = data.get("temperature").and_then(|temp| temp.as_f64()) {
        prompt.push_str(&format!("My temperature is {:.1}°C. ", temp));
    }
    if let Some(symptoms) = data.get("symptoms").and_then(|symptoms| symptoms.as_array()) {
        if !symptoms.is_empty() {
            let symptom_list: Vec<String> = symptoms
                .iter()
                .filter_map(|symptom| symptom.as_str().map(|s| s.to_string()))
                .collect();
            prompt.push_str(&format!("I'm experiencing these symptoms: {}. ", 
                symptom_list.join(", ")
            ));
        }
    }
    if let Some(medications) = data.get("medications").and_then(|medications| medications.as_array()) {
        if !medications.is_empty() {
            let medication_list: Vec<String> = medications
                .iter()
                .filter_map(|medication| medication.as_str().map(|m| m.to_string()))
                .collect();
            prompt.push_str(&format!("I'm currently taking these medications: {}. ", 
                medication_list.join(", ")
            ));
        }
    }
    if let Some(allergies) = data.get("allergies").and_then(|allergies| allergies.as_array()) {
        if !allergies.is_empty() {
            let allergy_list: Vec<String> = allergies
                .iter()
                .filter_map(|allergy| allergy.as_str().map(|a| a.to_string()))
                .collect();
            prompt.push_str(&format!("I have these allergies: {}. ", 
                allergy_list.join(", ")
            ));
        }
    }
    if let Some(alcohol) = data.get("alcohol_use").and_then(|alcohol| alcohol.as_str()) {
        prompt.push_str(&format!("Alcohol use: {}. ", alcohol));
    }
    if let Some(smoking) = data.get("smoking").and_then(|smoking| smoking.as_str()) {
        prompt.push_str(&format!("Smoking status: {}. ", smoking));
    }
    if let Some(drugs) = data.get("drug_use").and_then(|drugs| drugs.as_str()) {
        prompt.push_str(&format!("Drug use: {}. ", drugs));
    }
    prompt.push_str("What advice can you give me about my health?");

    let payload = json!({
        "model": "deepseek/deepseek-r1:free",
        "messages": [
            {
                "role": "user",
                "content": "You are a knowledgeable medical assistant. Provide helpful, evidence-based advice while reminding users to consult with their doctor for professional medical advice."
            },
            {
                "role": "user",
                "content": prompt
            }
        ]
    });

    dotenv::dotenv().ok();

    let mut flag = true;
    while flag {
        let api_url = "https://openrouter.ai/api/v1/chat/completions";
        let client = Client::new();
        println!("API KEY: {}", dotenv::var("LLM_KEY").unwrap());

        let response = client
            .post(api_url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", dotenv::var("LLM_KEY").unwrap()))
            .json(&payload)
            .send()
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            eprintln!("API call failed: {} - {}", status, error_message);
            return Err(actix_web::error::ErrorInternalServerError("API call failed"));
        }

        let output = response
            .json::<Value>()
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        println!("API Response: {:#?}", output);

        if let Some(content) = output
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
        {
            println!("Content: {}", content);
            sqlx::query("UPDATE results SET deepseek = $1 WHERE id = $2")
                .bind(content)
                .bind(&id)
                .execute(db_pool.get_ref())
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            flag = false;
        } else {
            eprintln!("Error: Missing or invalid content in API response");
            sleep(std::time::Duration::from_secs(15)).await;
        }
    }
    Ok(())
}

pub async fn queryGemini(
    id: String,
    data: Value,
    db_pool: web::Data<PgPool>
) -> Result<(), actix_web::Error> {
    println!("QUERYING:::: {}", id);

    let mut prompt = String::new();

    prompt.push_str(&format!("I am a {} year old {} {}. ", 
        data.get("age")
            .and_then(|age| age.as_i64())
            .unwrap_or(0),
        data.get("gender")
            .and_then(|gender| gender.as_str())
            .unwrap_or("unknown"),
        data.get("race")
            .and_then(|race| race.as_str())
            .unwrap_or("unknown")
    ));
    
    prompt.push_str(&format!("My height is {} cm and weight is {} kg. ", 
        data.get("height")
            .and_then(|height| height.as_i64())
            .unwrap_or(0),
        data.get("weight")
            .and_then(|weight| weight.as_i64())
            .unwrap_or(0)
    ));
     

    let bp = data.get("blood_pressure")
        .and_then(|bp| bp.as_str())
        .unwrap_or("unknown");
    prompt.push_str(&format!("My blood pressure is {}. ", bp));
    
    if let Some(hr) = data.get("heart_rate").and_then(|hr| hr.as_i64()) {
        prompt.push_str(&format!("My heart rate is {} bpm. ", hr));
        prompt.push_str(&format!("My heart rate is {} bpm. ", hr));
    }

    if let Some(temp) = data.get("temperature").and_then(|temp| temp.as_f64()) {
        prompt.push_str(&format!("My temperature is {:.1}°C. ", temp));
    }
    if let Some(symptoms) = data.get("symptoms").and_then(|symptoms| symptoms.as_array()) {
        if !symptoms.is_empty() {
            let symptom_list: Vec<String> = symptoms
                .iter()
                .filter_map(|symptom| symptom.as_str().map(|s| s.to_string()))
                .collect();
            prompt.push_str(&format!("I'm experiencing these symptoms: {}. ", 
                symptom_list.join(", ")
            ));
        }
    }
    if let Some(medications) = data.get("medications").and_then(|medications| medications.as_array()) {
        if !medications.is_empty() {
            let medication_list: Vec<String> = medications
                .iter()
                .filter_map(|medication| medication.as_str().map(|m| m.to_string()))
                .collect();
            prompt.push_str(&format!("I'm currently taking these medications: {}. ", 
                medication_list.join(", ")
            ));
        }
    }
    if let Some(allergies) = data.get("allergies").and_then(|allergies| allergies.as_array()) {
        if !allergies.is_empty() {
            let allergy_list: Vec<String> = allergies
                .iter()
                .filter_map(|allergy| allergy.as_str().map(|a| a.to_string()))
                .collect();
            prompt.push_str(&format!("I have these allergies: {}. ", 
                allergy_list.join(", ")
            ));
        }
    }
    if let Some(alcohol) = data.get("alcohol_use").and_then(|alcohol| alcohol.as_str()) {
        prompt.push_str(&format!("Alcohol use: {}. ", alcohol));
    }
    if let Some(smoking) = data.get("smoking").and_then(|smoking| smoking.as_str()) {
        prompt.push_str(&format!("Smoking status: {}. ", smoking));
    }
    if let Some(drugs) = data.get("drug_use").and_then(|drugs| drugs.as_str()) {
        prompt.push_str(&format!("Drug use: {}. ", drugs));
    }
    prompt.push_str("What advice can you give me about my health?");

    let payload = json!({
        "model": "google/gemini-2.5-pro-exp-03-25:free",
        "messages": [
            {
                "role": "user",
                "content": "You are a knowledgeable medical assistant. Provide helpful, evidence-based advice while reminding users to consult with their doctor for professional medical advice."
            },
            {
                "role": "user",
                "content": prompt
            }
        ]
    });

    dotenv::dotenv().ok();

    let mut flag = true;
    while flag {
        let api_url = "https://openrouter.ai/api/v1/chat/completions";
        let client = Client::new();
        println!("API KEY: {}", dotenv::var("LLM_KEY").unwrap());

        let response = client
            .post(api_url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", dotenv::var("LLM_KEY").unwrap()))
            .json(&payload)
            .send()
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            eprintln!("API call failed: {} - {}", status, error_message);
            return Err(actix_web::error::ErrorInternalServerError("API call failed"));
        }

        let output = response
            .json::<Value>()
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        println!("API Response: {:#?}", output);

        if let Some(content) = output
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
        {
            println!("Content: {}", content);
            sqlx::query("UPDATE results SET gemini = $1 WHERE id = $2")
                .bind(content)
                .bind(&id)
                .execute(db_pool.get_ref())
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            flag = false;
        } else {
            eprintln!("Error: Missing or invalid content in API response");
            sleep(std::time::Duration::from_secs(15)).await;
        }
    }
    Ok(())
}

pub async fn queryLlama(
    id: String,
    data: Value,
    db_pool: web::Data<PgPool>
) -> Result<(), actix_web::Error> {
    println!("QUERYING:::: {}", id);

    let mut prompt = String::new();

    prompt.push_str(&format!("I am a {} year old {} {}. ", 
        data.get("age")
            .and_then(|age| age.as_i64())
            .unwrap_or(0),
        data.get("gender")
            .and_then(|gender| gender.as_str())
            .unwrap_or("unknown"),
        data.get("race")
            .and_then(|race| race.as_str())
            .unwrap_or("unknown")
    ));
    
    prompt.push_str(&format!("My height is {} cm and weight is {} kg. ", 
        data.get("height")
            .and_then(|height| height.as_i64())
            .unwrap_or(0),
        data.get("weight")
            .and_then(|weight| weight.as_i64())
            .unwrap_or(0)
    ));
     

    let bp = data.get("blood_pressure")
        .and_then(|bp| bp.as_str())
        .unwrap_or("unknown");
    prompt.push_str(&format!("My blood pressure is {}. ", bp));
    
    if let Some(hr) = data.get("heart_rate").and_then(|hr| hr.as_i64()) {
        prompt.push_str(&format!("My heart rate is {} bpm. ", hr));
        prompt.push_str(&format!("My heart rate is {} bpm. ", hr));
    }

    if let Some(temp) = data.get("temperature").and_then(|temp| temp.as_f64()) {
        prompt.push_str(&format!("My temperature is {:.1}°C. ", temp));
    }
    if let Some(symptoms) = data.get("symptoms").and_then(|symptoms| symptoms.as_array()) {
        if !symptoms.is_empty() {
            let symptom_list: Vec<String> = symptoms
                .iter()
                .filter_map(|symptom| symptom.as_str().map(|s| s.to_string()))
                .collect();
            prompt.push_str(&format!("I'm experiencing these symptoms: {}. ", 
                symptom_list.join(", ")
            ));
        }
    }
    if let Some(medications) = data.get("medications").and_then(|medications| medications.as_array()) {
        if !medications.is_empty() {
            let medication_list: Vec<String> = medications
                .iter()
                .filter_map(|medication| medication.as_str().map(|m| m.to_string()))
                .collect();
            prompt.push_str(&format!("I'm currently taking these medications: {}. ", 
                medication_list.join(", ")
            ));
        }
    }
    if let Some(allergies) = data.get("allergies").and_then(|allergies| allergies.as_array()) {
        if !allergies.is_empty() {
            let allergy_list: Vec<String> = allergies
                .iter()
                .filter_map(|allergy| allergy.as_str().map(|a| a.to_string()))
                .collect();
            prompt.push_str(&format!("I have these allergies: {}. ", 
                allergy_list.join(", ")
            ));
        }
    }
    if let Some(alcohol) = data.get("alcohol_use").and_then(|alcohol| alcohol.as_str()) {
        prompt.push_str(&format!("Alcohol use: {}. ", alcohol));
    }
    if let Some(smoking) = data.get("smoking").and_then(|smoking| smoking.as_str()) {
        prompt.push_str(&format!("Smoking status: {}. ", smoking));
    }
    if let Some(drugs) = data.get("drug_use").and_then(|drugs| drugs.as_str()) {
        prompt.push_str(&format!("Drug use: {}. ", drugs));
    }
    prompt.push_str("What advice can you give me about my health?");
    prompt.push_str(&format!(
        "I am a {} year old {} {}. ",
        data.get("age").and_then(|age| age.as_i64()).unwrap_or(0),
        data.get("gender").and_then(|gender| gender.as_str()).unwrap_or("unknown"),
        data.get("race").and_then(|race| race.as_str()).unwrap_or("unknown")
    ));
    prompt.push_str("What advice can you give me about my health?");

    let payload = json!({
        "model": "meta-llama/llama-3.3-70b-instruct:free",
        "messages": [
            {
                "role": "user",
                "content": "You are a knowledgeable medical assistant. Provide helpful, evidence-based advice while reminding users to consult with their doctor for professional medical advice."
            },
            {
                "role": "user",
                "content": prompt
            }
        ]
    });

    dotenv::dotenv().ok();
    let mut flag = true;
    while (flag) {
        let api_url = "https://openrouter.ai/api/v1/chat/completions";
        let client = Client::new();
        println!("API KEY: {}", dotenv::var("LLM_KEY").unwrap());

        let response = client
            .post(api_url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", dotenv::var("LLM_KEY").unwrap()))
            .json(&payload)
            .send()
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            eprintln!("API call failed: {} - {}", status, error_message);
            return Err(actix_web::error::ErrorInternalServerError("API call failed"));
        }

        let output = response
            .json::<Value>()
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        println!("API Response: {:#?}", output);

        if let Some(content) = output
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
        {
            println!("Content: {}", content);
            sqlx::query("UPDATE results SET llama = $1 WHERE id = $2")
                .bind(content)
                .bind(&id)
                .execute(db_pool.get_ref())
                .await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            flag = false;
        } else {
            eprintln!("Error: Missing or invalid content in API response");
            sleep(std::time::Duration::from_secs(15)).await;
        }
    }
    Ok(())
}
#[derive(Serialize, Deserialize, sqlx::FromRow)]
struct ResultData {
    deepseek: String,
    gemini: String,
    llama: String,
}
pub async fn queryConsensus(
    id: String,
    db_pool: web::Data<PgPool>,
    pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>,
) -> Result<(), actix_web::Error> {
    let mut con = pool.get().map_err(|e| {
        eprintln!("Error getting Redis connection: {}", e);
        actix_web::error::ErrorInternalServerError("Internal server error")
    })?;
    println!("QUERYING:::: {}", id);
    let mut flag = true;
    while (flag) {
        sleep(std::time::Duration::from_secs(5)).await;
        let k : Option<i32> = con.get(format!("{}_ready", id.clone())).map_err(|_| ErrorInternalServerError("Failed to get Redis key"))?;
        if k == Some(3) {
            println!("All models are ready for ID: {}", id);
            flag = false;
        } else {
            println!("Waiting for models to be ready for ID: {}", id);
        }
    }
    let res = sqlx::query_as::<_, ResultData>("SELECT deepseek, gemini, llama FROM results WHERE id = $1")
                .bind(id.clone())
                .fetch_one(db_pool.get_ref())
                .await;
            match res {
                Err(e) => {
                    eprintln!("Error fetching results: {}", e);
                    return Err(actix_web::error::ErrorInternalServerError("Internal server error"))
                }
                Ok(res) => {
                    let data = format!(
                        "DeepSeek: {}, Gemini: {}, Llama: {}",
                        res.deepseek, res.gemini, res.llama
                    );
                let payload = json!({
                    "model": "openrouter/quasar-alpha",
                    "messages": [
                        {
                            "role": "user",
                            "content": "Based on the patient's medical information, 
                            there are three large language modes that each predicted 
                            a diagnosis for the patient. This query was used to get
                            a diagnosis from the models: Based on the following patient
                            information, provide: 1. A preliminary diagnosis or diagnoses 
                            and your confidence level based on the provided information 
                            2. A treatment plan 3. A drug usage plan (including dosage,
                            frequency, and duration), A diagnosis with its corresponding
                            brief treatment plan and drug usage plan where each is split by a \'/\'. 
                            (e.g., \"Influenza (80 percent confidence)/ Rest and hydration/ Oseltamivir 75 mg twice daily for 5 days\").
                            Return your results as a single line for each possible diagnosis in this format:
                            Diagnosed Disease Name (X percent confidence)/ Treatment Plan / Drug Usage Plan.
                            Do not repeat symptoms as a diagnosis. Use established disease names 
                            (e.g., \"Influenza\", \"Acute Bronchitis\", \"COVID-19\", etc.). DO NOT RESTATE EACH LLM's OUTPUT. Just summarize them together and add a confidence score for all of them. 
                            Determine a diagnosis concensus from the three different diagnoses and explain your reasoning in a separate section, sandwiched within the delimiters xxx"
                        },
                        {
                            "role": "user",
                            "content": data 
                        }
                    ]
                });
//hi
                dotenv::dotenv().ok();

                let api_url = "https://openrouter.ai/api/v1/chat/completions";
                let client = Client::new();
                println!("API KEY: {}", dotenv::var("LLM_KEY").unwrap());

                let response = client
                    .post(api_url)
                    .header("Accept", "application/json")
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", dotenv::var("LLM_KEY").unwrap()))
                    .json(&payload)
                    .send()
                    .await
                    .map_err(|e: reqwest::Error| actix_web::error::ErrorInternalServerError(e))?;

                if !response.status().is_success() {
                    let status = response.status();
                    let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                    eprintln!("API call failed: {} - {}", status, error_message);
                    return Err(actix_web::error::ErrorInternalServerError("API call failed"));
                }

                let output = response
                    .json::<Value>()
                    .await
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

                println!("API Response: {:#?}", output);

                if let Some(content) = output
                    .get("choices")
                    .and_then(|choices| choices.get(0))
                    .and_then(|choice| choice.get("message"))
                    .and_then(|message| message.get("content"))
                    .and_then(|content| content.as_str())
                {
                    println!("Content: {}", content);

                    sqlx::query("UPDATE results SET consensus = $1 WHERE id = $2")
                        .bind(content)
                        .bind(&id)
                        .execute(db_pool.get_ref())
                        .await
                        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
                } else {
                    eprintln!("Error: Missing or invalid content in API response");
                    return Err(actix_web::error::ErrorInternalServerError("Invalid API response"));
                }
            }
        }
    Ok(())
}