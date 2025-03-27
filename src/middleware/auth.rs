use axum::{
    http::{Request, StatusCode, header},
};
use crate::auth::jwt::{JwtClaims, verify_token};

pub fn extract_token_from_request<B>(req: &Request<B>) -> Result<JwtClaims, StatusCode> {
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .ok_or_else(|| {
            println!("No Authorization header found");
            StatusCode::UNAUTHORIZED
        })?;

    let auth_str = auth_header.to_str().map_err(|e| {
        println!("Invalid Authorization header: {}", e);
        StatusCode::UNAUTHORIZED
    })?;

    if !auth_str.starts_with("Bearer ") {
        println!("Authorization header doesn't start with 'Bearer '");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_str[7..]; // Remove "Bearer " prefix
    println!("Attempting to verify token");

    let claims = verify_token(token).map_err(|e| {
        println!("Token verification failed: {}", e);
        StatusCode::UNAUTHORIZED
    })?;

    println!("Token verified, checking token type");
    if claims.token_type != "access" {
        println!("Wrong token type: {}", claims.token_type);
        return Err(StatusCode::UNAUTHORIZED);
    }

    println!("Authentication successful for user: {}", claims.full_name);
    Ok(claims)
}