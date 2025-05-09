use std::sync::{Arc, Mutex}; 
use actix_web::{error::ErrorInternalServerError, web::{self, Data, Payload}, HttpResponse, Responder};
use r2d2::Pool;
use r2d2_redis::{redis::Commands, RedisConnectionManager};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use tokio::time::sleep;
use regex::Regex;

pub async fn queryDeepSeekR1(
    id: String,
    data: Value,
    arr: Arc<Mutex<Vec<String>>>,
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
    prompt.push_str("Provide a diagnosis and treatment plan for my health condition based on the provided health information.");


    let payload = json!({
        "model": "deepseek/deepseek-r1-zero:free",
        "messages": [
            {
                "role": "user",
                "content": "You are a knowledgeable doctor. Provide a helpful, 
                evidence-based diagnosis and treatment plan for my health condition based on the provided health information.
                If there are any values in the health information that are equal to zero, or don't make sense, DISREGARD them and do not use them in your response.
                Only use the information that makes sense. Do not tell me that you can't give a diagnosis based on insufficient information, give me your best guess and a confidence score. 
                Summarize your information in a few sentences. RETURN THE RESULTS IN ENGLISH AND ONLY ENGLISH."
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
        println!("QUERYING Deepseek_{}", id);

        let response = client
            .post(api_url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", std::env::var("LLM_KEY").unwrap()))
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
            if let Ok(mut arr_guard) = arr.lock() {
                arr_guard[0] = content.to_string();
                println!("{}\n--------------\n{}\n--------------\n{}\n--------------\n", arr_guard[0], arr_guard[1], arr_guard[2]);
            } else if let Err(poisoned) = arr.lock() {
                eprintln!("Mutex is poisoned. Recovering...");
                let mut arr_guard = poisoned.into_inner(); // Recover the data
                arr_guard[0] = content.to_string();
            }
            println!("{}", content); 
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
    arr: Arc<Mutex<Vec<String>>>,
) -> Result<(), actix_web::Error> {
    sleep(std::time::Duration::from_secs(5)).await;
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
    prompt.push_str("Provide a diagnosis and treatment plan for my health condition based on the provided health information.");


    /*let payload = json!({
        "model": "google/gemini-2.5-pro-exp-03-25:free",
        "messages": [
            {
                "role": "user",
                "content": "You are a knowledgeable doctor. Provide a helpful, 
                evidence-based diagnosis and treatment plan for my health condition based on the provided health information.
                Summarize your information in a few sentences."
            },
            { 
                "content": "You are a knowledgeable medical assistant. Provide helpful, evidence-based advice while reminding users to consult with their doctor for professional medical advice."
            },
            {
                "role": "user",
                "content": prompt
            }
        ]
    });*/

    let payload = json!({
        "contents": [
            {
                "parts": [
                    {
                        "text": "You are a knowledgeable doctor. Provide a helpful, 
                evidence-based diagnosis and treatment plan for my health condition based on the provided health information.
                If there are any values in the health information that are equal to zero, or don't make sense, DISREGARD them and do not use them in your response.
                Only use the information that makes sense. Do not tell me that you can't give a diagnosis based on insufficient information, give me your best guess and a confidence score. 
                Summarize your information in a few sentences. RETURN THE RESULTS IN ENGLISH AND ONLY ENGLISH."
                    },
                    {
                        "text": prompt
                    }
                ]
            }
        ]
    });

    dotenv::dotenv().ok();

    let mut flag = true;
    while flag {
        //let api_url = "https://openrouter.ai/api/v1/chat/completions";
        let client = Client::new();
        println!("TESTING GEMINI_{}", id);
        /* OPENROUTER QUERY FOR GEMINI 
        let response = client
            .post(api_url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", dotenv::var("LLM_KEY").unwrap()))
            .json(&payload)
            .send()
            .await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
        */
        let response = client
            .post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}", std::env::var("GEMINI_KEY").unwrap()))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            //.header("Authorization", format!("Bearer {}", dotenv::var("LLM_KEY").unwrap()))
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

        /*if let Some(content) = output
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(|content| content.as_str())
        {*/
        if let Some(content) = output
            .get("candidates")
            .and_then(|candidates| candidates.get(0))
            .and_then(|candidate| candidate.get("content"))
            .and_then(|content| content.get("parts"))
            .and_then(|parts| parts.get(0))
            .and_then(|part| part.get("text"))
            .and_then(|text| text.as_str())
        {
            if let Ok(mut arr_guard) = arr.lock() {
                arr_guard[1] = content.to_string();
                println!("{}\n--------------\n{}\n--------------\n{}\n--------------\n", arr_guard[0], arr_guard[1], arr_guard[2]);
            } else {
                eprintln!("Failed to lock the mutex for arr");
            }
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
    arr: Arc<Mutex<Vec<String>>>,
) -> Result<(), actix_web::Error> {
    sleep(std::time::Duration::from_secs(10)).await;
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
    prompt.push_str("Provide a diagnosis and treatment plan for my health condition based on the provided health information.");

    let payload = json!({
        "model": "meta-llama/llama-3.3-70b-instruct:free",
        "messages": [
            {
                "role": "user",
                "content": "You are a knowledgeable doctor. Provide a helpful, 
                evidence-based diagnosis and treatment plan for my health condition based on the provided health information.
                If there are any values in the health information that are equal to zero, or don't make sense, DISREGARD them and do not use them in your response.
                Only use the information that makes sense. Do not tell me that you can't give a diagnosis based on insufficient information, give me your best guess and a confidence score. 
                Summarize your information in a few sentences. RETURN THE RESULTS IN ENGLISH AND ONLY ENGLISH." 
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
        println!("RUNNING FOR LLAMA_{}", id);

        let response = client
            .post(api_url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", std::env::var("LLM_KEY").unwrap()))
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
            if let Ok(mut arr_guard) = arr.lock() {
                arr_guard[2] = content.to_string();
                println!("{}\n--------------\n{}\n--------------\n{}\n--------------\n", arr_guard[0], arr_guard[1], arr_guard[2]);
            } else {
                eprintln!("Failed to lock the mutex for arr");
                
            }
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
    redis_pool: Data<Pool<RedisConnectionManager>>, 
    arr: Arc<Mutex<Vec<String>>>
) -> Result<(), actix_web::Error> {
    let mut flag = true;
    while (flag) {
        sleep(std::time::Duration::from_secs(5)).await;
        if let Ok(arr_guard) = arr.lock() {
            if arr_guard.iter().all(|s| !s.is_empty()) {
                println!("All models are ready for ID: {}", id);
                
                flag = false;
        }  else {
          println!("Waiting for models to be ready for ID: {}", id);
        } 
    }
}
    flag = true;
    while flag {
        let mut data = String::new();
        if let Ok(arr_guard) = arr.lock() {
            data.push_str(&format!("{}#{}#{}", arr_guard[0], arr_guard[1], arr_guard[2]));
            println!("Data: {}", data);
        } else {
            eprintln!("Failed to lock the mutex for arr");
        }
        println!("{}", data);
        let payload = json!({
                    "model": "deepseek/deepseek-r1-zero:free",
                    "messages": [
                        {
                            "role": "user",
                            "content": "Based on the patient's medical information, 
                            there are three large language models that each predicted 
                            a diagnosis for the patient. This query was used to get
                            a diagnosis from the models: Based on the following patient
                            information, provide: 1. A preliminary diagnosis or diagnoses 
                            and your confidence level based on the provided information 
                            2. A treatment plan 3. A drug usage plan (including dosage,
                            frequency, and duration), A diagnosis with its corresponding
                            brief treatment plan and drug usage plan where each is split by a \'#\'. 
                            (e.g., \"Influenza # Rest and hydration # Oseltamivir 75 mg twice daily for 5 days\").
                            Return your results as a single line for the most likely possible diagnosis in this format:
                            Diagnosed Disease Name # Treatment Plan # Drug Usage Plan.
                            First, generate the response with Markdown formatting. Then, in the next step,
                             remove all Markdown characters and symbols. Deliver only the pure text 
                             with no formatting in the final output
                            Do not repeat symptoms as a diagnosis. Use established disease names
                            (e.g., \"Influenza\", \"Acute Bronchitis\", \"COVID-19\", etc.). DO NOT RESTATE EACH LLM's OUTPUT. 
                            Just summarize them together. 
                            RETURN THE RESULTS IN ENGLISH AND ONLY ENGLISH.
                            Determine a diagnosis concensus from the three different diagnoses."
                        },
                        {
                            "role": "user",
                            "content": data 
                        }
                    ]
                });
                dotenv::dotenv().ok();
//retest
                let api_url = "https://openrouter.ai/api/v1/chat/completions";
                let client = Client::new();
                println!("API KEY: {}", dotenv::var("LLM_KEY").unwrap());

                let response = client
                    .post(api_url)
                    .header("Accept", "application/json")
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", std::env::var("LLM_KEY").unwrap()))
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
                    let re = Regex::new(r"\\boxed\{(.*?)\}").unwrap();
                    let clean = re.replace_all(content, "$1").to_string();
                    println!("Content: {}", clean);
                    if !content.is_empty() {
                    let mut con = redis_pool
            .get()
            .map_err(ErrorInternalServerError)
            .expect("Failed to get redis connection");
            con.set(format!("consensus_{}", id), clean).map_err(ErrorInternalServerError)?;
            flag = false;
        }
        }
    }
    Ok(())
}