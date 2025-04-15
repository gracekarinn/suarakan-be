use axum::{
    extract::{Path, State},
    http::{Request, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::Local;
use serde_json::json;

use crate::{
    database::connection::DbPool,
    middleware::auth::extract_token_from_request,
    model::update::Update,
    services::update_service::UpdateService,
};

pub async fn get_update(
    State(pool): State<DbPool>,
    Path(update_id): Path<i32>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse {
    match extract_token_from_request(&req) {
        Ok(_) => {
            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Database connection error"})),
                    ).into_response()
                }
            };

            match UpdateService::get_update_by_id(&mut conn, update_id) {
                Ok(update) => (StatusCode::OK, Json(json!(update))).into_response(),
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

#[derive(serde::Deserialize)]
pub struct UpdateRequest {
    pub remarks: Option<String>,
    pub proof: Option<String>,
    pub status: Option<String>,
}

pub async fn update_update(
    State(pool): State<DbPool>,
    Path(update_id): Path<i32>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<UpdateRequest>,
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

            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Database connection error"})),
                    ).into_response();
                }
            };

            let existing = match UpdateService::get_update_by_id(&mut conn, update_id) {
                Ok(update) => update,
                Err(diesel::result::Error::NotFound) => {
                    return (
                        StatusCode::NOT_FOUND,
                        Json(json!({"error": "Update not found"})),
                    ).into_response();
                }
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": e.to_string()})),
                    ).into_response();
                }
            };

            if let Some(status) = &payload.status {
                if !["Received", "Processing", "Completed", "Rejected"].contains(&status.as_str()) {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({"error": "Invalid status. Must be one of: Received, Processing, Completed, Rejected"})),
                    ).into_response();
                }
            }

            let updated_update = Update {
                updateid: existing.updateid,
                createdat: existing.createdat,
                updatedat: Some(Local::now().naive_local()),
                remarks: payload.remarks.or(existing.remarks),
                proof: payload.proof.or(existing.proof),
                status: payload.status.or(existing.status),
                reportid: existing.reportid,
            };

            match UpdateService::update_update(&mut conn, update_id, updated_update) {
                Ok(update) => {
                    (StatusCode::OK, Json(json!(update))).into_response()
                }
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                ).into_response(),
            }
        }
        Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
    }
}