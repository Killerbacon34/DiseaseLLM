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

#[post("/api/login")]
pub async fn login(data: json) -> impl Responder {
    

    // Add the new user to the database (uncomment and implement this part)
    // let pool = web::Data::<Pool>::clone(&pool);
    // sqlx::query!("INSERT INTO users (id, role, pass, origdevid)
    //     VALUES ($1, $2, $3, $4)", data.id,
    //     data.role, data.pass, data.origdevid)
    //     .execute(&pool)
    //     .await
    //     .map_err(|e| ErrorInternalServerError(e))?;
    if data.username == "admin" && data.pass == "admin" {
        let token = gentoken();
        println!("Token: {}", token);
        return HttpResponse::Ok().json(token);
    } else {
        return HttpResponse::Unauthorized().json("Invalid username or password");
    }



    HttpResponse::Ok()
}

fn gentoken() -> String {
    let mut rng = rand::thread_rng();
    let rando = Os.Rng.gen::<[u8; 32]>().to_vec();
    let token = base64::encode(rando);
    println!("Token: {}", token);
    return token;
}