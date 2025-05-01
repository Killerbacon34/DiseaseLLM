use actix_identity::Identity;
use actix_web::{error::ErrorInternalServerError, get, post, rt::spawn, web, HttpResponse, Responder};
use actix_multipart::Multipart;
use r2d2_redis::redis::Commands;
use rand::{random, RngCore};
use sanitize_filename::sanitize;
use serde::{Serialize, Deserialize};
use sqlx::pool;
use std::{fs::{self, File}, string};
use futures_util::{stream::StreamExt, FutureExt, future, future::join_all};
use std::io::Write;
use regex::Regex;
use std::sync::{Arc, Mutex};
use tokio::{sync::futures, time::{timeout, Duration}};
use crate::queryLLM; 
use base64::{engine::general_purpose::URL_SAFE, Engine};


#[post("/uploadFile")]
pub async fn upload_file(mut payload: Multipart) -> impl Responder {
    let dir = "./uploads/".to_owned();
    fs::create_dir_all(&dir).unwrap();

    let mut extracted_data = ManualData {
        height: 0,
        weight: 0,
        age: 0,
        gender: "".to_string(),
        race: "".to_string(),
        symptoms: vec![],
        bloodpressure: 0, 
        heartrate: 0,
        temperature: 0.0,
        medications: vec![],
        allergies: vec![],
        alcohol: "".to_string(),
        smoking: "".to_string(),
        druguse: "".to_string(),
    };

    let number_regex = Regex::new(r"\d+").unwrap(); // Regex to match numbers

    while let Some(field) = payload.next().await {
        let mut field = field.map_err(|_| ErrorInternalServerError("Error reading field"))?;
        let filename = field
            .content_disposition()
            .and_then(|cd| cd.get_filename().map(|name| sanitize(name)))
            .unwrap_or_else(|| "default_filename".to_string());
        if !filename.ends_with(".pdf") {
            return Err(ErrorInternalServerError("File is not a PDF"));
        }
        let filepath = format!("{}/{}", dir, filename);
        let mut f = web::block({
            let filepath = filepath.clone();
            move || -> std::io::Result<File> { File::create(filepath) }
        })
        .await
        .map_err(|_| ErrorInternalServerError("Error creating file"))??;

        while let Some(chunk) = field.next().await {
            let chunk = chunk.map_err(|_| ErrorInternalServerError("Error reading chunk"))?;
            f = web::block(move || -> std::io::Result<File> { f.write_all(&chunk).map(|_| f) })
                .await
                .map_err(|_| ErrorInternalServerError("Error writing chunk"))??;
        }

        let bytes = std::fs::read(filepath.clone()).unwrap();
        let text = pdf_extract::extract_text_from_mem(&bytes).unwrap();
        let text = text.trim().to_string();

        for line in text.lines() {
            println!("Line: {}", line); // Debugging line to see the content of each line
            let line = line.trim(); // Ensure the line is trimmed before processing
            if line.starts_with("Height:") {
                if let Some(captures) = number_regex.captures(&line.replace("Height:", "").trim()) {
                    extracted_data.height = captures[0].parse().unwrap_or(0);
                }
            } else if line.starts_with("Weight:") {
                if let Some(captures) = number_regex.captures(&line.replace("Weight:", "").trim()) {
                    extracted_data.weight = captures[0].parse().unwrap_or(0);
                }
            } else if line.starts_with("Age:") {
                if let Some(captures) = number_regex.captures(&line.replace("Age:", "").trim()) {
                    extracted_data.age = captures[0].parse().unwrap_or(0);
                }
            } else if line.starts_with("Gender:") {
                extracted_data.gender = line.replace("Gender:", "").trim().to_string();
                println!("Gender: {}", extracted_data.gender);
            } else if line.starts_with("Race:") {
                extracted_data.race = line.replace("Race:", "").trim().to_string();
            } else if line.starts_with("Symptoms:") {
                extracted_data.symptoms = line
                    .replace("Symptoms:", "")
                    .trim()
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
            } else if line.starts_with("Blood Pressure:") {
                 if let Some(captures) = number_regex.captures(&line.replace("Blood Pressure:", "").trim()) {
                    extracted_data.bloodpressure = captures[0].parse().unwrap_or(0);
                }
            } else if line.starts_with("Heart Rate:") {
                if let Some(captures) = number_regex.captures(&line.replace("Heart Rate:", "").trim()) {
                    extracted_data.heartrate = captures[0].parse().unwrap_or(0);
                }
            } else if line.starts_with("Temperature:") {
                if let Some(captures) = number_regex.captures(&line.replace("Temperature:", "").trim()) {
                    extracted_data.temperature = captures[0].parse().unwrap_or(0.0);
                } 
            } else if line.starts_with("Medications:") {
                extracted_data.medications = line
                    .replace("Medications:", "")
                    .trim()
                    .split(',')
                    .map(|m| m.trim().to_string())
                    .collect();
            } else if line.starts_with("Allergies:") {
                extracted_data.allergies = line
                    .replace("Allergies:", "")
                    .trim()
                    .split(',')
                    .map(|a| a.trim().to_string())
                    .collect();
            } else if line.starts_with("Alcohol Use:") {
                extracted_data.alcohol = line.replace("Alcohol Use:", "").trim().to_string();
            } else if line.starts_with("Smoking:") {
                extracted_data.smoking = line.replace("Smoking:", "").trim().to_string();
            } else if line.starts_with("Drug Use:") {
                extracted_data.druguse = line.replace("Drug Use:", "").trim().to_string();
            }
        }
        fs::remove_file(filepath).unwrap_or_else(|_| {
            println!("Failed to delete file: {}", filename);
        });
    }

    Ok::<HttpResponse, actix_web::Error>(
        HttpResponse::Ok().json(extracted_data)
    )
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ManualData {
    height: i32, 
    weight: i32,
    age: i32,
    gender: String,
    race: String,
    symptoms: Vec<String>,  
    bloodpressure: i32, 
    heartrate: i32,
    temperature: f64,
    medications: Vec<String>,
    allergies: Vec<String>,
    alcohol: String,
    smoking: String,
    druguse: String,
}

#[post("/uploadForm")]
pub async fn upload_form(
    //pool: web::Data<PgPool>,
    data: web::Json<ManualData>,
    redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>,
    id: Option<Identity>,
) -> Result<HttpResponse, actix_web::Error> {

    if let Some(id) = id {
        let user_id = id.id().unwrap().to_string();
        let mut con = redis_pool.get().map_err(ErrorInternalServerError)?;

        // Initialize Redis key to track task progress
        con.set(format!("{}_ready", user_id), 0)
            .map_err(|_| ErrorInternalServerError("Failed to set Redis key"))?;

        let data_value = serde_json::to_value((*data).clone())
            .map_err(|_| ErrorInternalServerError("Failed to serialize data"))?;
 

        // Spawn background tasks
        let redis_pool_clone = redis_pool.clone();
        /*sqlx::query("INSERT INTO results (id) VALUES ($1)")
            .bind(&user_id)
            .execute(pool.get_ref())
            .await
            .map_err(|_| ErrorInternalServerError("Failed to insert data into database"))?;*/
        let arr = Arc::new(Mutex::new(vec!["".to_string(); 3]));
        let deepseekShare = Arc::clone(&arr);
        let geminiShare = Arc::clone(&arr);
        let llamaShare = Arc::clone(&arr);
        let consensusShare = Arc::clone(&arr);

        tokio::spawn(async move {
            let tasks = vec![
                spawn_task_with_timeout(
                    "DeepseekR1",
                    Duration::from_secs(120), // TODO: change to a more reasonable timeout 
                    queryLLM::queryDeepSeekR1(user_id.clone(), data_value.clone(), Arc::clone(&deepseekShare))
                        .map(|res| res.unwrap_or_else(|err| {
                            eprintln!("Error in DeepSeekR1: {:?}", err);
                        })),
                    redis_pool_clone.clone(),
                    user_id.clone(),
                ),spawn_task_with_timeout(
                    "Gemini",
                    Duration::from_secs(125), // TODO: change to a more reasonable timeout
                    queryLLM::queryGemini(user_id.clone(), data_value.clone(), Arc::clone(&geminiShare))
                        .map(|res| res.unwrap_or_else(|err| {
                            eprintln!("Error in Gemini: {:?}", err);
                        })),
                    redis_pool_clone.clone(),
                    user_id.clone(),
                ),spawn_task_with_timeout(
                    "Llama",
                    Duration::from_secs(130), // TODO: change to a more reasonable timeout 
                    queryLLM::queryLlama(user_id.clone(), data_value.clone(), Arc::clone(&llamaShare))
                        .map(|res| res.unwrap_or_else(|err| {
                            eprintln!("Error in Llama: {:?}", err);
                        })),
                    redis_pool_clone.clone(),
                    user_id.clone(),
                ), spawn_task_with_timeout(
                    "Consensus",
                    Duration::from_secs(240), // TODO: change to a more reasonable timeout 
                    queryLLM::queryConsensus(user_id.clone(), redis_pool_clone.clone(), Arc::clone(&consensusShare))
                        .map(|res| res.unwrap_or_else(|err| {
                            eprintln!("Error in Consensus: {:?}", err);
                        })),
                    redis_pool_clone.clone(),
                    user_id.clone(),
                ),
            ];
        });
        
        return Ok(HttpResponse::Ok().body("Tasks are running in the background"));
    } else {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized"));
    }
}

/// Helper function to spawn a task with a timeout

#[get("/status")]
pub async fn status(redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>, id: Option<Identity>) -> Result<HttpResponse, actix_web::Error> {
    if let Some(id) = id {
        let mut con = redis_pool.get().map_err(ErrorInternalServerError)?;
        let k : Option<i32> = con.get(format!("{}_ready", id.id().unwrap())).map_err(|_| ErrorInternalServerError("Failed to get Redis key"))?;
        println!("Key: {:?}", k); // Debugging line to see the value of k
        if let Some(k) = k {
            if k >= 4 {
                println!("Finished");
                con.del(format!("{}_ready", id.id().unwrap())).map_err(|_| ErrorInternalServerError("Failed to delete Redis key"))?;
                return Ok(HttpResponse::Ok().body("Finished"));
            } else {
                return Ok(HttpResponse::Accepted().body(format!("{}", k)));
            }
        } else {
            return Ok(HttpResponse::Accepted().body("Not finished"));
        }
    } else {
        Ok(HttpResponse::Unauthorized().body("Unauthorized"))
    }
}

fn spawn_task_with_timeout<F>(
    task_name: &'static str,
    duration: Duration,
    task: F,
    redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>,
    user_id: String,
) -> tokio::task::JoinHandle<()>
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    tokio::spawn(async move {
        match timeout(duration, task).await {
            Ok(_) => {
                // Task completed successfully
                let mut con = redis_pool.get().unwrap();
                let _: i32 = con.incr(format!("{}_ready", user_id), 1).unwrap();
                println!("Task '{}' completed successfully", task_name);
            }
            Err(_) => {
                // Task timed out
                println!("Task '{}' timed out", task_name);
            }
        }
    })
}

/*#[get("/diagnostics")]
pub async fn diag () -> impl Responder {
     
}*/
#[post("/results")]
pub async fn anon_all_output(
    data: web::Json<ManualData>,
    redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>,
) -> Result<HttpResponse, actix_web::Error> {
 let mut random_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut random_bytes);
    let user_id = URL_SAFE.encode(&random_bytes);
        let mut con = redis_pool.get().map_err(ErrorInternalServerError)?;

        // Initialize Redis key to track task progress
        con.set(format!("{}_ready", user_id), 0)
            .map_err(|_| ErrorInternalServerError("Failed to set Redis key"))?;
        let data_value = serde_json::to_value((*data).clone())
            .map_err(|_| ErrorInternalServerError("Failed to serialize data"))?;
 

        // Spawn background tasks
        let redis_pool_clone = redis_pool.clone();
        /*sqlx::query("INSERT INTO results (id) VALUES ($1)")
            .bind(&user_id)
            .execute(pool.get_ref())
            .await
            .map_err(|_| ErrorInternalServerError("Failed to insert data into database"))?;*/
let arr = Arc::new(Mutex::new(vec!["".to_string(); 3]));
        let deepseekShare = Arc::clone(&arr);
        let geminiShare = Arc::clone(&arr);
        let llamaShare = Arc::clone(&arr);
        let consensusShare = Arc::clone(&arr);
        let tasks = vec![
                spawn_task_with_timeout(
                    "DeepseekR1",
                    Duration::from_secs(120), // TODO: change to a more reasonable timeout 
                    queryLLM::queryDeepSeekR1(user_id.clone(), data_value.clone(), Arc::clone(&deepseekShare))
                        .map(|res| res.unwrap_or_else(|err| {
                            eprintln!("Error in DeepSeekR1: {:?}", err);
                        })),
                    redis_pool_clone.clone(),
                    user_id.clone(),
                ),spawn_task_with_timeout(
                    "Gemini",
                    Duration::from_secs(125), // TODO: change to a more reasonable timeout
                    queryLLM::queryGemini(user_id.clone(), data_value.clone(), Arc::clone(&geminiShare))
                        .map(|res| res.unwrap_or_else(|err| {
                            eprintln!("Error in Gemini: {:?}", err);
                        })),
                    redis_pool_clone.clone(),
                    user_id.clone(),
                ),spawn_task_with_timeout(
                    "Llama",
                    Duration::from_secs(130), // TODO: change to a more reasonable timeout 
                    queryLLM::queryLlama(user_id.clone(), data_value.clone(), Arc::clone(&llamaShare))
                        .map(|res| res.unwrap_or_else(|err| {
                            eprintln!("Error in Llama: {:?}", err);
                        })),
                    redis_pool_clone.clone(),
                    user_id.clone(),
                ), spawn_task_with_timeout(
                    "Consensus",
                    Duration::from_secs(240), // TODO: change to a more reasonable timeout 
                    queryLLM::queryConsensus(user_id.clone(), redis_pool_clone.clone(), Arc::clone(&consensusShare))
                        .map(|res| res.unwrap_or_else(|err| {
                            eprintln!("Error in Consensus: {:?}", err);
                        })),
                    redis_pool_clone.clone(),
                    user_id.clone(),
                ),
            ];
        join_all(tasks).await;
        let mut con = redis_pool.get().map_err(ErrorInternalServerError)?;
        let res: String = con.get(format!("consensus_{}", user_id)).map_err(|_| ErrorInternalServerError("Failed to get Redis key"))?;
        let parts: Vec<&str> = res.split("#").collect();
        println!("Parts: {:?}", parts);
        con.del(format!("consensus_{}", user_id)).map_err(|_| ErrorInternalServerError("Failed to delete Redis key"))?;
        return Ok(HttpResponse::Ok().json(serde_json::json!({
            "Diagnosis": parts[0],
            "Treatment Plan": parts[1],
            "Drug Usage Plan": parts[2],
        })));
}