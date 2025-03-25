use actix_web::{HttpResponse, web, Responder, error::ErrorInternalServerError, post};
use actix_multipart::Multipart;
use sanitize_filename::sanitize;
use std::fs::File;
use std::fs;
use futures_util::stream::StreamExt;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct ManualData {
    physicalinfo: String,
    symptoms: String,
    biometricinfo: String,
    medicalhistory: String,
}


#[post("/api/manualupload")]
pub async fn manualupload(pool: web::Data<PgPool>, data: web::Json<ManualData>) -> impl Responder {
    // Insert the data into the database
    let result = sqlx::query(
        //TO-DO: CHANGE THIS TO YOUR TABLE NAME AND MAKE SQL SCHEMA FOR IT IN ACTUAL DB
        "INSERT INTO USERINFO (physicalinfo, symptoms, biometricinfo, medicalhistory) VALUES ($1, $2, $3, $4)"
    )
    .bind(&data.physicalinfo)
    .bind(&data.symptoms)
    .bind(&data.biometricinfo)
    .bind(&data.medicalhistory)
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
