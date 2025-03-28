use std::collections::HashMap;
use redis;

use actix_web::{HttpResponse, web, Responder, error::ErrorInternalServerError, post} ;
use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize)]
pub struct ManualData {
    height: i32, 
    weight: i32,
    age: i32,
    gender: String,
    race: String,
    //Double check if this is correct
    symptoms: Vec<String>,
    bloodpressure: String,
    heartrate: i32,
    temperature: f32,
    medications: Vec<String>, 
    allergies: Vec<String>,
    alcohol: String,
    smoking: String,
    druguse: String,
}


#[post("/api/manualupload")]
pub async fn manualupload(pool: web::Data<PgPool>, data: web::Json<ManualData>) -> impl Responder {
    // Insert the data into the database
    let result = sqlx::query(
        //TO-DO: CHANGE THIS TO YOUR TABLE NAME AND MAKE SQL SCHEMA FOR IT IN ACTUAL DB
        "INSERT INTO USERINFO (height, weight, age, gender, race, symptoms, bloodpressure, heartrate, temperature, medications, allergies, alcohol, smoking, druguse) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"
    )
    .bind(&data.height)
    .bind(&data.weight)
    .bind(&data.age)
    .bind(&data.gender)
    .bind(&data.race)
    .bind(&data.symptoms.join(","))
    .bind(&data.bloodpressure)
    .bind(&data.heartrate)
    .bind(&data.temperature)
    .bind(&data.medications.join(","))
    .bind(&data.allergies.join(","))
    .bind(&data.alcohol)
    .bind(&data.smoking)
    .bind(&data.druguse)
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Data uploaded successfully"),
        Err(e) => {
            println!("Error inserting data: {}", e);
            HttpResponse::InternalServerError().body("Failed to upload data")
        }
    }
}

