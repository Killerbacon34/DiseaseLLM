use actix_web::{HttpResponse, web, Responder, error::ErrorInternalServerError, post};
use actix_jwt_auth_middleware as jwt;
use sanitize_filename::sanitize;
//use sqlx::Pool;
#[derive(Serialize, Deserialize, Clone, FromRequest)]
struct UserClaims {
    id: u32,
    role: Role,
    pass: String,
    origdevid: vec<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
enum Role {
    Admin,
    RegularUser,
}

#[post("/api/signup")]
async fn signup(claims: UserClaims) -> impl Responder {
    let newuser = UserClaims {
        id: newuser.id,
        role: newuser.role,
        pass: newuser.pass,
        origdevid: newuser.origdevid,
    };
    //add the new user to the database
    //let pool = web::Data::<Pool>::clone(&pool);
    //sqlx::query!("INSERT INTO users (id, role, pass, origdevid
    //    VALUES ($1, $2, $3, $4)", newuser.id,
    //    newuser.role, newuser.pass, newuser.origdevid)
    //    .execute(&pool)
    //    .await
    //    .map_err(|e| ErrorInternalServerError(e))?;
    format!(
        "Hello user with id: {}, i see you are a {:?}!",
            user_claims.id, user_claims.role
    ) //testing the format with the jwt unpack 
    Ok::<HttpResponse, actix_web::Error>(HttpResponse::Ok().into())
}