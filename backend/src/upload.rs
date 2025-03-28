use actix_web::{HttpResponse, web, Responder, error::ErrorInternalServerError, post};
use actix_multipart::Multipart;
use sanitize_filename::sanitize;
use std::fs::File;
use std::fs;
use futures_util::stream::StreamExt;
use std::io::Write;
use pdf_extract::*;
#[post("/api/upload")]
pub async fn upload (mut payload: Multipart) -> impl Responder {
    let dir = "./uploads/".to_owned();
    fs::create_dir_all(&dir).unwrap();
    let mut gender: String = "Not Found".to_string();
    let mut symptoms: String = "Not Found".to_string();
    let mut blood_type: String = "Not Found".to_string();
    let mut allergies: String = "Not Found".to_string();
    let mut medications: String = "Not Found".to_string();
    let mut medical_conditions: String = "Not Found".to_string();
    while let Some(field) = payload.next().await {
        let mut field = field.map_err(|_| ErrorInternalServerError("Error reading field"))?;
        let filename = field
            .content_disposition()
            .and_then(|cd| cd.get_filename().map(|name| sanitize(name)))
            .unwrap_or_else(|| "default_filename".to_string());
        if filename.ends_with(".pdf") == false {
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
            f = web::block(move || {
                f.write_all(&chunk).map(|_| f)
            })
            .await
            .map_err(|_| ErrorInternalServerError("Error writing chunk"))??;
        }
        let bytes = std::fs::read(filepath).unwrap();
        let text = pdf_extract::extract_text_from_mem(&bytes).unwrap();        
        let text = text.trim().to_string(); 
        
        for line in text.lines() {
            if line.starts_with("Gender:") {
                gender = line.replace("Gender:", "").trim().to_string();
            } else if line.starts_with("Symptoms:") {
                symptoms = line.replace("Symptoms:", "").trim().to_string();
            } else if line.starts_with("Blood Type:") {
                blood_type = line.replace("Blood Type:", "").trim().to_string();
            } else if line.starts_with("Allergies:") {
                allergies = line.replace("Allergies:", "").trim().to_string();
            } else if line.starts_with("Medications:") {
                medications = line.replace("Medications:", "").trim().to_string();
            } else if line.starts_with("Medical Conditions:") {
                medical_conditions = line.replace("Medical Conditions:", "").trim().to_string();
            }
        }
    }
    println!("File uploaded successfully");
    println!("Gender: {}\nSymptoms: {}\nBlood Type: {}\nAllergies: {}\nMedications: {}\nMedical Conditions: {}\n", 
    gender, symptoms, blood_type, allergies, medications, medical_conditions);
    Ok::<HttpResponse, actix_web::Error>(HttpResponse::Ok().into())
}

fn parsePDF(file: &str) -> String {
    // Parse the PDF file and return the text
    return "This is a placeholder".to_string();
}


#[post("/api/checkconn")]
pub async fn checkconn() -> impl Responder {
    println!("Connection check successful!");
    HttpResponse::Ok().body("Connection check successful!")
}