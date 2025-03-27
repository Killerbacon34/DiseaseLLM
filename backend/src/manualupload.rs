use actix_web::{HttpResponse, web, Responder, error::ErrorInternalServerError, post};
use actix_multipart::Multipart;
use sanitize_filename::sanitize;
use std::fs::File;
use std::fs;
use futures_util::stream::StreamExt;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct ManualData {
    height: int, 
    weight: int,
    age: int,
    gender: String,
    race: String,
    //Double check if this is correct
    symptoms: String[],
    bloodpressure: String,
    heartrate: int,
    temperature: float,
    medications: String[],
    allergies: String[],
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
