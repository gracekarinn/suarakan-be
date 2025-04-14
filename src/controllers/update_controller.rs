// src/controllers/update_controller.rs
use axum::{
    extract::{Path, State},
    http::{Request, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    database::connection::DbPool,
    middleware::auth::extract_token_from_request,
    model::update::Update,
    services::update_service::UpdateService,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
    pub remarks: Option<String>,
    pub proof: Option<String>,
}

#[axum::debug_handler]
pub async fn update_status(
    State(pool): State<DbPool>,
    Path(update_id): Path<i32>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<UpdateStatusRequest>,
) -> impl IntoResponse {
    let mut builder = Request::builder();
    for (key, value) in headers.iter() {
        builder = builder.header(key, value);
    }

    match extract_token_from_request(&builder.body(()).unwrap()) {
        Ok(claims) => {
            if claims.user_type != "ADMIN" {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "Admin access required"})),
                ).into_response();
            }

            let admin_id = claims.user_id;

            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Database connection error"})),
                    ).into_response();
                }
            };

            match UpdateService::update_status(
                &mut conn,
                update_id,
                admin_id,
                payload.status,
                payload.remarks,
                payload.proof,
            ) {
                Ok(update) => {
                    (StatusCode::OK, Json(json!(update))).into_response()
                }
                Err(diesel::result::Error::NotFound) => (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Update not found"})),
                ).into_response(),
                Err(diesel::result::Error::RollbackTransaction) => (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Invalid status value. Must be one of: Received, Processing, Completed, Rejected"})),
                ).into_response(),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                ).into_response(),
            }
        }
        Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
    }
}

#[axum::debug_handler]
pub async fn get_update(
    State(pool): State<DbPool>,
    Path(update_id): Path<i32>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse {
    match extract_token_from_request(&req) {
        Ok(claims) => {
            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Database connection error"})),
                    ).into_response();
                }
            };

            match UpdateService::get_update_by_id(&mut conn, update_id) {
                Ok(update) => {
                    (StatusCode::OK, Json(json!(update))).into_response()
                }
                Err(diesel::result::Error::NotFound) => (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Update not found"})),
                ).into_response(),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                ).into_response(),
            }
        }
        Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
    }
}