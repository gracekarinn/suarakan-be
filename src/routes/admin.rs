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

pub fn routes() -> Router {
    Router::new()
        .route("/admin", get(admin_route))
}