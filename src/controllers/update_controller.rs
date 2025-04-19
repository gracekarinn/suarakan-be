use axum::{
    extract::{Path, State},
    http::{Request, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::Local;
use serde_json::json;
use regex::Regex;

use crate::{
    database::connection::DbPool,
    middleware::auth::extract_token_from_request,
    model::update::Update,
    services::update_service::UpdateService,
};

fn sanitize_string(input: Option<String>) -> Option<String> {
    input.map(|s| html_escape::encode_text(&s).to_string())
}

fn is_valid_url(url: &str) -> bool {
    let url_regex = Regex::new(r"^(https?://)?([a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}(/[^\s]*)?$").unwrap();
    url_regex.is_match(url)
}

fn is_valid_status(status: &str) -> bool {
    matches!(status, "Received" | "Processing" | "Completed" | "Rejected")
}

fn validate_update_request(update: &UpdateRequest) -> Result<(), (StatusCode, &'static str)> {
    // Validate proof URL if provided
    if let Some(proof) = &update.proof {
        if !proof.is_empty() && !is_valid_url(proof) {
            return Err((StatusCode::BAD_REQUEST, "Haru berupa link URL yang valid!"));
        }
    }
    
    if let Some(status) = &update.status {
        if !is_valid_status(status) {
            return Err((StatusCode::BAD_REQUEST, "Status harus berupa salah satu dari 'Received', 'Processing', 'Completed', dan 'Rejected'"));
        }
    }
    
    Ok(())
}

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

            match validate_update_request(&payload) {
                Ok(_) => {},
                Err((status, message)) => {
                    return (
                        status,
                        Json(json!({"error": message})),
                    ).into_response();
                }
            }

            let sanitized_remarks = sanitize_string(payload.remarks);
            let sanitized_proof = sanitize_string(payload.proof);
            let sanitized_status = sanitize_string(payload.status);

            let updated_update = Update {
                updateid: existing.updateid,
                createdat: existing.createdat,
                updatedat: Some(Local::now().naive_local()),
                remarks: sanitized_remarks.or(existing.remarks),
                proof: sanitized_proof.or(existing.proof),
                status: sanitized_status.or(existing.status),
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