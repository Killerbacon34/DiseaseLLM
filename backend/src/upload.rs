use actix_web::{HttpResponse, web, Responder, error::ErrorInternalServerError};
use actix_multipart::Multipart;
use sanitize_filename::sanitize;
use std::fs::File;
use std::fs;
use futures_util::stream::StreamExt;
use std::io::Write;
#[post("/api/upload")]
async fn upload_file(mut payload: Multipart) -> impl Responder {
    let dir = "./uploads/".to_owned();
    fs::create_dir_all(&dir).unwrap();
    while let Some(field) = payload.next().await {
        let mut field = field.map_err(|_| ErrorInternalServerError("Error reading field"))?;
        let filename = field
            .content_disposition() 
            .get_filename()
            .map(|name| sanitize(name))
            .unwrap_or_else(|| "default_filename".to_string()); 
        let filepath = format!("{}/{}", dir, filename);
        let mut f = web::block(|| File::create(filepath))
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
    }

    Ok::<HttpResponse, actix_web::Error>(HttpResponse::Ok().into())
}