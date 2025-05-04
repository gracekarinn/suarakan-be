use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub token_type: String,
    pub exp: usize,
    pub iat: usize,
    pub jti: String,
    pub user_id: i64,
    pub email: String,
    pub full_name: String,
    pub user_type: String,
    pub is_email_verified: bool,
}

pub fn verify_token(token: &str) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
    let jwt_secret = match env::var("PODS_JWT_SECRET") {
        Ok(secret) => {
            println!("Using JWT secret: {}", secret);
            secret
        },
        Err(e) => {
            println!("Error reading JWT_SECRET: {}", e);
            panic!("JWT_SECRET must be set in .env file");
        }
    };
    
    println!("Token to verify: {}", token);

    let token_data = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    
    println!("Token decoded successfully");
    Ok(token_data.claims)
}