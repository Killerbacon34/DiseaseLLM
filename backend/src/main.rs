use axum::{
    extract::Multipart,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::{net::SocketAddr, path::Path};
use tokio::fs;

async fn upload_file(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().map(String::from).unwrap_or_else(|| "file".to_string());
        let content = field.bytes().await.unwrap();

        // Save the file locally
        let file_path = format!("./uploads/{}", file_name);
        if let Err(e) = fs::write(&file_path, &content).await {
            return (500, format!("Failed to save file: {}", e));
        }

        println!("Uploaded file saved to: {}", file_path);
    }

    (200, "File uploaded successfully".to_string())
}

async fn index() -> &'static str {
    "Welcome to the Rust file upload API!"
}

#[tokio::main]
async fn main() {
    // Create an `uploads` directory if it doesn't exist
    if !Path::new("./uploads").exists() {
        fs::create_dir("./uploads").await.unwrap();
    }

    let app = Router::new()
        .route("/", get(index))
        .route("/upload", post(upload_file));

    let addr = SocketAddr::from(([127, 0, 0, 1], 5353));
    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
