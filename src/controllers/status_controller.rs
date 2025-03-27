use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::services::status_service::StatusService;
use crate::database::connection::DbPool;
use axum::http::Request;
use crate::middleware::auth::extract_token_from_request;

#[derive(Deserialize)]
pub struct CreateStatusRequest {
    pub data_id: i32,
    pub remarks: Option<String>,
    pub proof: Option<String>,
    pub status: String,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub update_id: i32,
    pub data_id: i32,
    pub status: Option<String>,
    pub remarks: Option<String>,
    pub proof: Option<String>,
}

pub async fn create_status(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateStatusRequest>,
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

            match StatusService::create_status(
                &mut conn,
                payload.data_id,
                payload.remarks,
                payload.proof,
                payload.status,
                claims.user_id, // Menggunakan user_id dari token admin
            ) {
                Ok(status) => (StatusCode::CREATED, Json(json!({
                    "update_id": status.updateid,
                    "data_id": status.dataid,
                    "status": status.status,
                    "remarks": status.remarks,
                    "proof": status.proof,
                }))).into_response(),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to create status"}))).into_response(),
            }
        },
        Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
    }
}

pub async fn get_status(
    State(pool): State<DbPool>,
    Path(update_id): Path<i32>,
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Database connection error"}))).into_response(),
    };

    match StatusService::get_status_by_id(&mut conn, update_id) {
        Ok(status) => (StatusCode::OK, Json(StatusResponse {
            update_id: status.updateid,
            data_id: status.dataid,
            status: status.status,
            remarks: status.remarks,
            proof: status.proof,
        })).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, Json(json!({"error": "Status not found"}))).into_response(),
    }
}