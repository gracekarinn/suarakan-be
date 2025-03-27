use axum::{
    routing::get,
    Router,
    http::Request,
    response::IntoResponse,
    Json,
    http::StatusCode,
};
use serde_json::json;
use crate::middleware::auth::extract_token_from_request;

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

pub fn routes() -> Router {
    Router::new()
        .route("/api/v1", get(protected_route))
}