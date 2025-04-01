use std::collections::HashMap;
use actix_multipart::form::json::Json;
use actix_web::{error::ErrorInternalServerError, web, HttpResponse, Responder};
use r2d2_redis::redis::Commands;
use reqwest::Client;
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn queryDeepSeekR1(
    id: String,
    data: Value,
    db_pool: web::Data<PgPool>
) -> Result<(), actix_web::Error> {
    println!("QUERYING:::: {}", id);
    // Fetch user data from database
    // #[derive(sqlx::FromRow)]
    // struct UserInfo {
    //     height: Option<i32>,
    //     weight: Option<i32>,
    //     age: Option<i32>,
    //     gender: Option<String>,
    //     race: Option<String>,
    //     symptoms: Option<Vec<String>>,
    //     blood_pressure: Option<String>,
    //     heart_rate: Option<i32>,
    //     temperature: Option<f32>,
    //     medications: Option<Vec<String>>,
    //     allergies: Option<Vec<String>>,
    //     alcohol_use: Option<String>,
    //     smoking: Option<String>,
    //     drug_use: Option<String>,
    // }

    // let user_info: UserInfo = sqlx::query_as("
    //     SELECT Height, Weight, Age, Gender, Race, Symptoms, BloodPressure, 
    //            HeartRate, Temperature, Medications, Allergies, AlcoholUse, 
    //            Smoking, DrugUse
    //     FROM USERINFO WHERE id = $1")
    // .bind(&id)
    // .fetch_one(db_pool.get_ref())
    // .await
    //.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

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
        //TODO: FIND OUT WHAT THE INPUTS SHOULD BE
        "model": "deepseek/deepseek-r1-zero:free",
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
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let output= response.json::<Value>().await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    if let Some(content) = output
        .get("choices")
        .and_then(|choices| choices.get(0))
        .and_then(|choice| choice.get("message"))
        .and_then(|message| message.get("content"))
        .and_then(|content| content.as_str())
    {
        println!("Content: {}", content);
        // Add value to the database
        sqlx::query("UPDATE results SET Deepseek=$1 WHERE id = $2")
            .bind(content)
            .bind(&id)
            .execute(db_pool.get_ref())
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    } else {
        eprintln!("Error: Missing or invalid content in API response");
        return Err(actix_web::error::ErrorInternalServerError("Invalid API response"));
    }
    Ok(())
}

pub async fn queryGemini(
    id: String,
    data: Value,
    db_pool: web::Data<PgPool>
) -> Result<(), actix_web::Error> {
    println!("QUERYING:::: {}", id);
    // Fetch user data from database
    // #[derive(sqlx::FromRow)]
    // struct UserInfo {
    //     height: Option<i32>,
    //     weight: Option<i32>,
    //     age: Option<i32>,
    //     gender: Option<String>,
    //     race: Option<String>,
    //     symptoms: Option<Vec<String>>,
    //     blood_pressure: Option<String>,
    //     heart_rate: Option<i32>,
    //     temperature: Option<f32>,
    //     medications: Option<Vec<String>>,
    //     allergies: Option<Vec<String>>,
    //     alcohol_use: Option<String>,
    //     smoking: Option<String>,
    //     drug_use: Option<String>,
    // }

    // let user_info: UserInfo = sqlx::query_as("
    //     SELECT Height, Weight, Age, Gender, Race, Symptoms, BloodPressure, 
    //            HeartRate, Temperature, Medications, Allergies, AlcoholUse, 
    //            Smoking, DrugUse
    //     FROM USERINFO WHERE id = $1")
    // .bind(&id)
    // .fetch_one(db_pool.get_ref())
    // .await
    //.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

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
        //TODO: FIND OUT WHAT THE INPUTS SHOULD BE
        "model": "google/gemini-2.5-pro-exp-03-25:free",
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
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let output= response.json::<Value>().await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    println!("{:#?}", output["choices"][0]["message"]["content"].as_str().unwrap());
    // add value to the database
    sqlx::query("UPDATE results SET gemini=$1 WHERE id = $2")
        .bind(output["choices"][0]["message"]["content"].as_str().unwrap())
        .bind(&id)
        .execute(db_pool.get_ref())
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(())
}

pub async fn queryLlama(
    id: String,
    data: Value,
    db_pool: web::Data<PgPool>
) -> Result<(), actix_web::Error> {
    println!("QUERYING:::: {}", id);

    let mut prompt = String::new();
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
    } else {
        eprintln!("Error: Missing or invalid content in API response");
        return Err(actix_web::error::ErrorInternalServerError("Invalid API response"));
    }

    Ok(())
}