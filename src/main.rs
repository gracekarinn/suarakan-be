mod database;
mod auth;
mod model;
mod schema;

use auth::jwt::{JwtClaims, verify_token};
use axum::{
    routing::get,
    Router,
    http::{Request, StatusCode, header},
    response::IntoResponse,
    Json,
};
use dotenv::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use serde_json::json;

async fn public_route() -> &'static str {
    "This is a public route that anyone can access"
}

async fn protected_route(req: Request<axum::body::Body>) -> impl IntoResponse {
    match extract_token_from_request(&req) {
        Ok(claims) => {
            (StatusCode::OK, Json(json!({
                "message": format!("Hello, {}! Your role is: {}", claims.full_name, claims.user_type)
            })))
        },
        Err(status) => {
            (status, Json(json!({"error": "Authentication failed"})))
        }
    }
}

async fn admin_route(req: Request<axum::body::Body>) -> impl IntoResponse {
    match extract_token_from_request(&req) {
        Ok(claims) => {
            if claims.user_type != "ADMIN" {
                return (StatusCode::FORBIDDEN, Json(json!({"error": "Admin access required"})));
            }

            (StatusCode::OK, Json(json!({
                "message": format!("Hello, Admin {}! You have access to admin features.", claims.full_name)
            })))
        },
        Err(status) => {
            (status, Json(json!({"error": "Authentication failed"})))
        }
    }
}

fn extract_token_from_request(req: &Request<axum::body::Body>) -> Result<JwtClaims, StatusCode> {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let _client = database::connect_to_db().await?;
    println!("Connected to PostgreSQL");
    
    let app = Router::new()
        .route("/", get(public_route))
        
        .route("/api/v1", get(protected_route))
        
        .route("/admin", get(admin_route));
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);
    
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}