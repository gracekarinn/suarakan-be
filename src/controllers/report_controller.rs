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
    model::report::{NewReport, Report},
    model::update::NewUpdate,
    services::report_service::ReportService,
    services::update_service::UpdateService,
};

pub async fn create_report(
    State(pool): State<DbPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<NewReport>,
) -> impl IntoResponse {
    let mut builder = Request::builder();
    for (key, value) in headers.iter() {
        builder = builder.header(key, value);
    }

    match extract_token_from_request(&builder.body(()).unwrap()) {
        Ok(claims) => {
            if claims.user_type != "PELAPOR" {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "Only PELAPOR can create reports"})),
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

            let mut new_report = payload;
            new_report.createdat = Some(Local::now().naive_local());
            new_report.reporterid = Some(claims.user_id);

            match ReportService::create_report(&mut conn, new_report) {
                Ok(report) => {
                    let new_update = NewUpdate {
                        createdat: Local::now().naive_local(),
                        updatedat: None,
                        remarks: Some(String::from("")),
                        proof: Some(String::from("")),
                        status: Some(String::from("Received")),
                        reportid: report.reportid,
                    };

                    match UpdateService::create_update(&mut conn, new_update) {
                        Ok(_) => {
                            (StatusCode::CREATED, Json(json!(report))).into_response()
                        }
                        Err(e) => {
                            let _ = ReportService::delete_report(&mut conn, report.reportid);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({"error": format!("Failed to create update: {}", e.to_string())})),
                            ).into_response()
                        }
                    }
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

pub async fn get_reports(
    State(pool): State<DbPool>,
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
                    ).into_response()
                }
            };

            let reports = match claims.user_type.as_str() {
                "ADMIN" => {
                    match ReportService::get_all_reports(&mut conn) {
                        Ok(reports) => reports,
                        Err(e) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({"error": e.to_string()})),
                            ).into_response()
                        }
                    }
                }
                "PELAPOR" => {
                    match ReportService::get_reports_by_reporter(&mut conn, claims.user_id) {
                        Ok(reports) => reports,
                        Err(e) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({"error": e.to_string()})),
                            ).into_response()
                        }
                    }
                }
                _ => {
                    return (
                        StatusCode::FORBIDDEN,
                        Json(json!({"error": "Unauthorized access"})),
                    ).into_response()
                }
            };

            let mut reports_with_updates = Vec::new();
            for report in reports {
                match UpdateService::get_update_by_report_id(&mut conn, report.reportid) {
                    Ok(update) => {
                        reports_with_updates.push(json!({
                            "report": report,
                            "update": update
                        }));
                    }
                    Err(e) => {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({"error": format!("Failed to get update: {}", e.to_string())})),
                        ).into_response()
                    }
                }
            }

            (StatusCode::OK, Json(json!(reports_with_updates))).into_response()
        }
        Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
    }
}

pub async fn get_report(
    State(pool): State<DbPool>,
    Path(report_id): Path<i32>,
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
                    ).into_response()
                }
            };

            match ReportService::get_report_by_id(&mut conn, report_id) {
                Ok(report) => {
                    if claims.user_type == "PELAPOR" && report.reporterid != Some(claims.user_id) {
                        return (
                            StatusCode::FORBIDDEN,
                            Json(json!({"error": "You are not authorized to view this report"})),
                        ).into_response();
                    }

                    match UpdateService::get_update_by_report_id(&mut conn, report_id) {
                        Ok(update) => {
                            (StatusCode::OK, Json(json!({
                                "report": report,
                                "update": update
                            }))).into_response()
                        }
                        Err(e) => {
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(json!({"error": format!("Failed to get update: {}", e.to_string())})),
                            ).into_response()
                        }
                    }
                }
                Err(diesel::result::Error::NotFound) => (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Report not found"})),
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

pub async fn update_report(
    State(pool): State<DbPool>,
    Path(report_id): Path<i32>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<Report>,
) -> impl IntoResponse {
    let mut builder = Request::builder();
    for (key, value) in headers.iter() {
        builder = builder.header(key, value);
    }

    match extract_token_from_request(&builder.body(()).unwrap()) {
        Ok(claims) => {
            if claims.user_type != "PELAPOR" {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "Only PELAPOR can update reports"})),
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

            let existing = match ReportService::get_report_by_id(&mut conn, report_id) {
                Ok(report) => report,
                Err(diesel::result::Error::NotFound) => {
                    return (
                        StatusCode::NOT_FOUND,
                        Json(json!({"error": "Report not found"})),
                    ).into_response();
                }
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": e.to_string()})),
                    ).into_response();
                }
            };

            if existing.reporterid != Some(claims.user_id) {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "You are not authorized to update this report"})),
                ).into_response();
            }

            match UpdateService::get_update_by_report_id(&mut conn, report_id) {
                Ok(update) => {
                    if update.status != Some(String::from("Received")) {
                        return (
                            StatusCode::FORBIDDEN,
                            Json(json!({"error": "Report can only be updated when status is 'Received'"})),
                        ).into_response();
                    }
                }
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": format!("Failed to get update: {}", e.to_string())})),
                    ).into_response();
                }
            }

            let mut updated_report = payload;
            updated_report.reportid = existing.reportid;
            updated_report.createdat = existing.createdat;
            updated_report.updatedat = Some(Local::now().naive_local());
            updated_report.reporterid = Some(claims.user_id);

            match ReportService::update_report(&mut conn, report_id, updated_report) {
                Ok(report) => {
                    (StatusCode::OK, Json(json!(report))).into_response()
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

pub async fn delete_report(
    State(pool): State<DbPool>,
    Path(report_id): Path<i32>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse {
    match extract_token_from_request(&req) {
        Ok(claims) => {
            if claims.user_type != "PELAPOR" {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "Only PELAPOR can delete reports"})),
                ).into_response();
            }

            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Database connection error"})),
                    ).into_response()
                }
            };

            let existing = match ReportService::get_report_by_id(&mut conn, report_id) {
                Ok(report) => report,
                Err(diesel::result::Error::NotFound) => {
                    return (
                        StatusCode::NOT_FOUND,
                        Json(json!({"error": "Report not found"})),
                    ).into_response();
                }
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": e.to_string()})),
                    ).into_response();
                }
            };

            if existing.reporterid != Some(claims.user_id) {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "You are not authorized to delete this report"})),
                ).into_response();
            }

            match UpdateService::get_update_by_report_id(&mut conn, report_id) {
                Ok(update) => {
                    if update.status != Some(String::from("Received")) && update.status != Some(String::from("Rejected")) {
                        return (
                            StatusCode::FORBIDDEN,
                            Json(json!({"error": "Report can only be deleted when status is 'Received' or 'Rejected'"})),
                        ).into_response();
                    }
                }
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": format!("Failed to get update: {}", e.to_string())})),
                    ).into_response();
                }
            }

            match UpdateService::delete_update_by_report_id(&mut conn, report_id) {
                Ok(_) => {
                    match ReportService::delete_report(&mut conn, report_id) {
                        Ok(_) => (
                            StatusCode::OK,
                            Json(json!({"message": "Report and associated update deleted successfully"})),
                        ).into_response(),
                        Err(e) => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({"error": format!("Failed to delete report: {}", e.to_string())})),
                        ).into_response(),
                    }
                }
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("Failed to delete update: {}", e.to_string())})),
                ).into_response(),
            }
        }
        Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
    }
}