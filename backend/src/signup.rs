use actix_web::{HttpResponse, Responder, post, web};
use serde::{Serialize, Deserialize};
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

#[post("/api/signup")]
pub async fn signup(data: web::Json<SignupData>) -> impl Responder {
    let newuser = SignupData {
        username: data.username.clone(),
        role: data.role.clone(),
        pass: data.pass.clone(),
        origdevid: data.origdevid.clone(),
    };

    // Add the new user to the database (uncomment and implement this part)
    // let pool = web::Data::<Pool>::clone(&pool);
    // sqlx::query!("INSERT INTO users (id, role, pass, origdevid)
    //     VALUES ($1, $2, $3, $4)", data.id,
    //     data.role, data.pass, data.origdevid)
    //     .execute(&pool)
    //     .await
    //     .map_err(|e| ErrorInternalServerError(e))?;

    println!(
        "Hello user with id: {}, i see you are a {:?}!",
        newuser.username, newuser.role
    );

    HttpResponse::Ok().json(newuser)
}