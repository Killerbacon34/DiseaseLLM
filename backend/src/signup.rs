use actix_web::{HttpResponse, Responder, post, web, error::ErrorInternalServerError};
use crypto::common::typenum::Integer;
use serde::{Serialize, Deserialize};
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
pub struct User {
    username: String,
    password: String,
    devid: Vec<String>,
    role: i32,
}
#[derive(Serialize, Deserialize)]
pub struct SignupData {
    username: String,
    role: i32,
    pass: String,
    origdevid: String,
}

#[post("/api/signup")]
pub async fn signup(pool: web::Data<PgPool>, data: web::Json<SignupData>) -> impl Responder {
    let mut device_ids = Vec::new(); 
    device_ids.push(data.origdevid.clone());
    let newuser =  User {
        username: data.username.clone(),
        role: data.role,
        password: data.pass.clone(),
        // append the new user's device id to the list of device ids, the input is a string to be converted to a vector
        devid: device_ids,
    };

    _ = sqlx::query(
    "INSERT INTO users (username, password, devid, role) VALUES ($1, $2, $3, $4)"
)
.bind(&newuser.username)  // No need for `.clone()`
.bind(&newuser.password)
.bind(&newuser.devid)
.bind(&newuser.role)
.execute(pool.get_ref())
.await
.map_err(|e| ErrorInternalServerError(e)); 
    println!(
        "Hello user with id: {}, i see you are a {:?}!",
        newuser.username, newuser.role
    );

    HttpResponse::Ok()
}
