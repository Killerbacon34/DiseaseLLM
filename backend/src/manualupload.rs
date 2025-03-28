use actix_web::{HttpResponse, web, Responder, post};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct ManualData {
    height: i32, 
    weight: i32,
    age: i32,
    gender: String,
    race: String,
    symptoms: Vec<String>,  
    bloodpressure: String,
    heartrate: i32,
    temperature: f64,
    medications: Vec<String>,
    allergies: Vec<String>,
    alcohol: String,
    smoking: String,
    druguse: String,
}

#[post("/manualupload")]
pub async fn manualupload(pool: web::Data<PgPool>, data: web::Json<ManualData>) -> impl Responder {
    // Insert the data into the database
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
        Ok(_) => HttpResponse::Ok().body("Data uploaded successfully"),
        Err(e) => {
            println!("Error inserting data: {}", e);
            HttpResponse::InternalServerError().body("Failed to upload data")
        }
    }
}