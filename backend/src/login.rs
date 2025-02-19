use actix_web::error::ErrorInternalServerError;
use actix_web::{HttpResponse, Responder, post, web};
use base64::{self, Engine as _};
use serde::{Serialize, Deserialize};
use rand::RngCore;
use actix_web::web::Data;
use actix_web::web::Json;
use sqlx::PgPool;
//use jsonwebtoken::{encode, Header, EncodingKey};
//use std::time::{SystemTime, UNIX_EPOCH};
//use std::env;
/*fn create_jwt(user: &UserClaims) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set"); // SECRET KEY SET USING ENVIRONMENT VARIABLE, CHANGE THIS on CLOUD DEPLOYMENT    
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize + 3600; // Token valid for 1 hour

    let claims = UserClaims {
        exp: expiration,
        ..user.clone()
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}*/

#[derive(Serialize, Deserialize, Clone, Debug)]
enum Role {
    Admin,
    RegularUser,
}

#[derive(Serialize, Deserialize)]
pub struct SignupData {
    username: String,
    role: Role,
    pass: String,
    origdevid: Vec<String>,
}

#[post("/api/login")]
pub async fn login(_pool: web::Data<PgPool>, data: web::Json<SignupData>) -> impl Responder {
    

    // Add the new user to the database (uncomment and implement this part)
    if data.username == "admin" && data.pass == "admin" {
        let token = gentoken();
        println!("Token: {}", token);
        return HttpResponse::Ok().json(token);
    } else {
        return HttpResponse::Unauthorized().json("Invalid username or password");
    }
}

fn gentoken() -> String {
// Generate a random 32-byte token
    let mut rando = [0u8; 32];
    rand::rng().fill_bytes(&mut rando);
    let token = base64::engine::general_purpose::URL_SAFE.encode(&rando);
    println!("Token: {}", token);
    return token;
}