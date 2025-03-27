use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use crate::services::publication_service::PublicationService;
use crate::database::connection::DbPool;
use axum::http::Request;
use crate::middleware::auth::extract_token_from_request;

pub async fn delete_publication(
    State(pool): State<DbPool>,
    Path(publication_id): Path<i32>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse {
    match extract_token_from_request(&req) {
        Ok(claims) => {
            if claims.user_type != "ADMIN" {
                return (StatusCode::FORBIDDEN, Json(json!({"error": "Admin access required"}))).into_response();
            }
            
            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Database connection error"}))).into_response(),
            };

            match PublicationService::get_publication_by_id(&mut conn, publication_id) {
                Ok(_) => {
                    match PublicationService::delete_publication(&mut conn, publication_id) {
                        Ok(_) => (StatusCode::OK, Json(json!({"message": "Publication deleted successfully"}))).into_response(),
                        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to delete publication"}))).into_response(),
                    }
                },
                Err(_) => (StatusCode::NOT_FOUND, Json(json!({"error": "Publication not found"}))).into_response(),
            }
        },
        Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
    }
}