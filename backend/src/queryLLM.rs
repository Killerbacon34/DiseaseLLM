/*use actix_web::{HttpResponse, Responder, post, web, get};
use reqwest::Client;
use serde_json::{json, Value};




#[get("/api/qClinicalBert")]
pub async fn query_clinical_bert() -> Result<HttpResponse, actix_web::Error> {
    let payload = json!({
        "inputs": "The answer to the universe is [MASK].",
        "parameters": {}
    });

    let api_url = "https://qhcnzi9owkihndum.us-east-1.aws.endpoints.huggingface.cloud/";
    let client = Client::new();
    let response = client.post(api_url)
    .header("Accept", "application/json")
    .header("Content-Type", "application/json")
    .json(&payload)
    .send()
    .await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    let output= response.json::<Value>().await.map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    println!("{:#?}", output);
    Ok(HttpResponse::Ok().json(output))
    
}

*/