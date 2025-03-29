use actix_identity::Identity;
use actix_web::{HttpResponse, web, Responder, error::ErrorInternalServerError, post, get};
use actix_multipart::Multipart;
use r2d2_redis::redis::Commands;
use sanitize_filename::sanitize;
use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use futures_util::stream::StreamExt;
use std::io::Write;
use pdf_extract::*;
use sqlx::{query, PgPool};
use regex::Regex;

use crate::queryLLM; // Add this import for regex functionality

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
            move || File::create(filepath)
        })
        .await
        .map_err(|_| ErrorInternalServerError("Error creating file"))??;

        while let Some(chunk) = field.next().await {
            let chunk = chunk.map_err(|_| ErrorInternalServerError("Error reading chunk"))?;
            f = web::block(move || f.write_all(&chunk).map(|_| f))
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
pub async fn upload_form(pool: web::Data<PgPool>, data: web::Json<ManualData>, 
    redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>, id: Option<Identity>) -> Result<HttpResponse, actix_web::Error>{
    if let Some(id) = id {
        let mut con = redis_pool.get().map_err(ErrorInternalServerError)?;
        con.set(format!("{}_ready", id.id().unwrap()), 0).map_err(|_| ErrorInternalServerError("Failed to set Redis key"))?;
        let data_value = serde_json::to_value((*data).clone()).map_err(|_| ErrorInternalServerError("Failed to serialize data"))?;
        queryLLM::queryDeepSeekR1(id.id().unwrap(), data_value.clone(), redis_pool.clone(), pool.clone());
        //queryLLM::queryGemini(id.id().unwrap(), data_value.clone(), redis_pool.clone(), pool.clone());
        //queryLLM::queryLlama(id.id().unwrap(), data_value.clone(), redis_pool.clone(), pool.clone());
    } else {
        return Ok(HttpResponse::Unauthorized().body("Unauthorized"));
    } 


    // TODOL: ADD SESSION TOKEN TO CHECK IF THE USER IS AUTHORIZED TO UPLOAD DATA
    let result = sqlx::query(
        "
        INSERT INTO USERINFO (
            Height, Weight, Age, Gender, Race, 
            Symptoms, BloodPressure, HeartRate, Temperature, 
            Medications, Allergies, AlcoholUse, Smoking, DrugUse
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        "
    )
    .bind(data.height)
    .bind(data.weight)
    .bind(data.age)
    .bind(&data.gender)
    .bind(&data.race)
    .bind(&data.symptoms)
    .bind(&data.bloodpressure)
    .bind(data.heartrate)
    .bind(data.temperature)
    .bind(&data.medications)
    .bind(&data.allergies)
    .bind(&data.alcohol)
    .bind(&data.smoking)
    .bind(&data.druguse)
    .execute(pool.get_ref())
    .await;
    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("Data uploaded successfully")),
        Err(e) => {
            println!("Error inserting data: {}", e);
            Ok(HttpResponse::InternalServerError().body("Failed to upload data"))
        }
    }
}

#[get("/status")]
pub async fn status(redis_pool: web::Data<r2d2::Pool<r2d2_redis::RedisConnectionManager>>, id: Option<Identity>) -> Result<HttpResponse, actix_web::Error> {
    if let Some(id) = id {
        let mut con = redis_pool.get().map_err(ErrorInternalServerError)?;
        let k : Option<i32> = con.get(format!("{}_ready", id.id().unwrap())).map_err(|_| ErrorInternalServerError("Failed to get Redis key"))?;
        if let Some(k) = k {
            if k >= 3 {
                return Ok(HttpResponse::Ok().body("true"));
            } else {
                return Ok(HttpResponse::Accepted().body("false"));
            }
        } else {
            return Ok(HttpResponse::Accepted().body("false"));
        }
    } else {
        Ok(HttpResponse::Unauthorized().body("Unauthorized"))
    }
}