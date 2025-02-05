extern crate actix_jwt_auth_middleware;
//use std::{env, io};
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use actix_jwt_auth_middleware::{TokenSigner, Authority, use_jwt::UseJWTOnApp};
use serde::{Serialize, Deserialize};
use ed25519_compact::KeyPair ;
use jwt_compact::alg::{Ed25519, VerifyingKey, SigningKey};  // Use JWT's Ed25519 implementation
use actix_jwt_auth_middleware::FromRequest;
mod upload;
mod signup;

#[derive(Serialize, Deserialize, Clone, FromRequest)]
struct User {
    id: u32,
    role: Role,
    pass: String,
    origdevid: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
enum Role {
    Admin,
    RegularUser,
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key_pair = KeyPair::generate();
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init(); 
    HttpServer::new(move || {
        let auth = Authority::<User, Ed25519, _, _>::new()
            .refresh_authorizer(|| async move { Ok(()) })
            .token_signer(Some(
                TokenSigner::new()
                    .signing_key(SigningKey::from_slice(key_pair.sk.as_ref()).expect("Failed to create signing key"))
                    .algorithm(Ed25519)
                    .build()
                    .expect("Failed to build token signer"),
            ))
            .verifying_key(VerifyingKey::from_slice(&key_pair.pk.as_ref()).expect("Failed to create verifying key"))
            .build()
            .expect("Failed to build authority");
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::default()
                .allow_any_origin() 
                .allow_any_method()
                .allow_any_header()
                .max_age(3600)
            )
            //.service(upload::upload)
            .service(
              signup::signup
            )
            .use_jwt(auth.clone(), web::scope("").service(upload::upload))
            //.use_jwt(auth, web::scope("").service(signup::signup))
    })
    .bind("127.0.0.1:5353")?
    .run()
    .await?;
    Ok(())
}