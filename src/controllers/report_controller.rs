use axum::{
    routing::{post, get},
    extract::State,
    Json,
    response::IntoResponse,
    http::{StatusCode, Request},
};
use serde_json::json;
use crate::services::report_service::{ReportService, ReportFormData};
use crate::database::connection::DbPool;
use crate::middleware::auth::extract_token_from_request;

pub async fn create_report(
    State(pool): State<DbPool>,
    req: Request<axum::body::Body>,
    Json(form_data): Json<ReportFormData>
) -> impl IntoResponse {
    match extract_token_from_request(&req) {
        Ok(claims) => {
            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Database connection error"}))).into_response(),
            };

            match ReportService::create_report(&mut conn, form_data) {
                Ok(report) => (
                    StatusCode::CREATED, 
                    Json(json!({
                        "status": "success",
                        "message": "Report created successfully",
                        "report_id": report.reportid
                    }))
                ).into_response(),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR, 
                    Json(json!({"error": format!("Failed to create report: {}", e)}))
                ).into_response()
            }
        },
        Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response()
    }
}

pub async fn validate_report_form(
    State(pool): State<DbPool>,
    req: Request<axum::body::Body>,
    Json(form_data): Json<ReportFormData>
) -> impl IntoResponse {
    match extract_token_from_request(&req) {
        Ok(_) => {
            let mut validation_errors = Vec::new();

            if form_data.reporter_phone.is_empty() {
                validation_errors.push("Reporter phone number is required");
            }
            
            if form_data.victim_full_name.is_empty() {
                validation_errors.push("Victim full name is required");
            }

            if validation_errors.is_empty() {
                (StatusCode::OK, Json(json!({"status": "valid"})))
            } else {
                (StatusCode::BAD_REQUEST, Json(json!({"errors": validation_errors})))
            }
        },
        Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response()
    }
}