// // src/controllers/report_controller.rs
// use axum::{
//     extract::{Path, State},
//     http::{Request, StatusCode},
//     response::IntoResponse,
//     Json,
// };
// use chrono::Local;
// use serde::{Deserialize, Serialize};
// use serde_json::json;

// use crate::{
//     database::connection::DbPool,
//     middleware::auth::extract_token_from_request,
//     model::report::{NewReport, Report},
//     services::report_service::ReportService,
// };

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ReportResponse {
//     pub report: Report,
//     pub status: String,
// }

// #[axum::debug_handler]
// pub async fn create_report(
//     State(pool): State<DbPool>,
//     headers: axum::http::HeaderMap,
//     Json(payload): Json<NewReport>,
// ) -> impl IntoResponse {
//     let mut builder = Request::builder();
//     for (key, value) in headers.iter() {
//         builder = builder.header(key, value);
//     }

//     match extract_token_from_request(&builder.body(()).unwrap()) {
//         Ok(claims) => {
//             if claims.user_type != "PELAPOR" {
//                 return (
//                     StatusCode::FORBIDDEN,
//                     Json(json!({"error": "Only reporters can create reports"})),
//                 ).into_response();
//             }

//             let reporter_id = claims.user_id as i32; // Convert i64 to i32

//             let mut conn = match pool.get() {
//                 Ok(conn) => conn,
//                 Err(_) => {
//                     return (
//                         StatusCode::INTERNAL_SERVER_ERROR,
//                         Json(json!({"error": "Database connection error"})),
//                     ).into_response();
//                 }
//             };

//             match ReportService::create_report(&mut conn, payload, reporter_id) {
//                 Ok(report) => {
//                     (StatusCode::CREATED, Json(json!(report))).into_response()
//                 }
//                 Err(e) => (
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     Json(json!({"error": e.to_string()})),
//                 ).into_response(),
//             }
//         }
//         Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
//     }
// }

// #[axum::debug_handler]
// pub async fn get_all_reports(
//     State(pool): State<DbPool>,
//     req: Request<axum::body::Body>,
// ) -> impl IntoResponse {
//     match extract_token_from_request(&req) {
//         Ok(claims) => {
//             if claims.user_type != "ADMIN" {
//                 return (
//                     StatusCode::FORBIDDEN,
//                     Json(json!({"error": "Admin access required"})),
//                 ).into_response();
//             }

//             let mut conn = match pool.get() {
//                 Ok(conn) => conn,
//                 Err(_) => {
//                     return (
//                         StatusCode::INTERNAL_SERVER_ERROR,
//                         Json(json!({"error": "Database connection error"})),
//                     ).into_response();
//                 }
//             };

//             match ReportService::get_reports_for_admin(&mut conn) {
//                 Ok(reports) => {
//                     (StatusCode::OK, Json(json!(reports))).into_response()
//                 }
//                 Err(e) => (
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     Json(json!({"error": e.to_string()})),
//                 ).into_response(),
//             }
//         }
//         Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
//     }
// }

// #[axum::debug_handler]
// pub async fn get_my_reports(
//     State(pool): State<DbPool>,
//     req: Request<axum::body::Body>,
// ) -> impl IntoResponse {
//     match extract_token_from_request(&req) {
//         Ok(claims) => {
//             if claims.user_type != "PELAPOR" {
//                 return (
//                     StatusCode::FORBIDDEN,
//                     Json(json!({"error": "Reporter access required"})),
//                 ).into_response();
//             }

//             let reporter_id = claims.user_id as i32;

//             let mut conn = match pool.get() {
//                 Ok(conn) => conn,
//                 Err(_) => {
//                     return (
//                         StatusCode::INTERNAL_SERVER_ERROR,
//                         Json(json!({"error": "Database connection error"})),
//                     ).into_response();
//                 }
//             };

//             match ReportService::get_reports_for_reporter(&mut conn, reporter_id) {
//                 Ok(reports) => {
//                     (StatusCode::OK, Json(json!(reports))).into_response()
//                 }
//                 Err(e) => (
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     Json(json!({"error": e.to_string()})),
//                 ).into_response(),
//             }
//         }
//         Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
//     }
// }

// #[axum::debug_handler]
// pub async fn get_report(
//     State(pool): State<DbPool>,
//     Path(report_id): Path<i32>,
//     req: Request<axum::body::Body>,
// ) -> impl IntoResponse {
//     match extract_token_from_request(&req) {
//         Ok(claims) => {
//             let mut conn = match pool.get() {
//                 Ok(conn) => conn,
//                 Err(_) => {
//                     return (
//                         StatusCode::INTERNAL_SERVER_ERROR,
//                         Json(json!({"error": "Database connection error"})),
//                     ).into_response();
//                 }
//             };

//             match ReportService::get_report_by_id(&mut conn, report_id) {
//                 Ok(report) => {
//                     // If user is a reporter, check if report belongs to them
//                     if claims.user_type == "PELAPOR" && report.reporterid != claims.user_id as i32 {
//                         return (
//                             StatusCode::FORBIDDEN,
//                             Json(json!({"error": "You can only view your own reports"})),
//                         ).into_response();
//                     }

//                     (StatusCode::OK, Json(json!(report))).into_response()
//                 }
//                 Err(diesel::result::Error::NotFound) => (
//                     StatusCode::NOT_FOUND,
//                     Json(json!({"error": "Report not found"})),
//                 ).into_response(),
//                 Err(e) => (
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     Json(json!({"error": e.to_string()})),
//                 ).into_response(),
//             }
//         }
//         Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
//     }
// }

// #[axum::debug_handler]
// pub async fn update_report(
//     State(pool): State<DbPool>,
//     Path(report_id): Path<i32>,
//     headers: axum::http::HeaderMap,
//     Json(payload): Json<Report>,
// ) -> impl IntoResponse {
//     let mut builder = Request::builder();
//     for (key, value) in headers.iter() {
//         builder = builder.header(key, value);
//     }

//     match extract_token_from_request(&builder.body(()).unwrap()) {
//         Ok(claims) => {
//             if claims.user_type != "PELAPOR" {
//                 return (
//                     StatusCode::FORBIDDEN,
//                     Json(json!({"error": "Only reporters can update reports"})),
//                 ).into_response();
//             }

//             let reporter_id = claims.user_id as i32;

//             let mut conn = match pool.get() {
//                 Ok(conn) => conn,
//                 Err(_) => {
//                     return (
//                         StatusCode::INTERNAL_SERVER_ERROR,
//                         Json(json!({"error": "Database connection error"})),
//                     ).into_response();
//                 }
//             };

//             // Set the updated timestamp
//             let mut updated_report = payload;
//             updated_report.updatedat = Some(Local::now().naive_local());

//             match ReportService::update_report(&mut conn, report_id, reporter_id, updated_report) {
//                 Ok(report) => {
//                     (StatusCode::OK, Json(json!(report))).into_response()
//                 }
//                 Err(diesel::result::Error::NotFound) => (
//                     StatusCode::NOT_FOUND,
//                     Json(json!({"error": "Report not found or not owned by you"})),
//                 ).into_response(),
//                 Err(diesel::result::Error::RollbackTransaction) => (
//                     StatusCode::BAD_REQUEST,
//                     Json(json!({"error": "Report can only be updated when in 'Received' status"})),
//                 ).into_response(),
//                 Err(e) => (
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     Json(json!({"error": e.to_string()})),
//                 ).into_response(),
//             }
//         }
//         Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
//     }
// }

// #[axum::debug_handler]
// pub async fn delete_report(
//     State(pool): State<DbPool>,
//     Path(report_id): Path<i32>,
//     req: Request<axum::body::Body>,
// ) -> impl IntoResponse {
//     match extract_token_from_request(&req) {
//         Ok(claims) => {
//             if claims.user_type != "PELAPOR" {
//                 return (
//                     StatusCode::FORBIDDEN,
//                     Json(json!({"error": "Only reporters can delete reports"})),
//                 ).into_response();
//             }

//             let reporter_id = claims.user_id as i32;

//             let mut conn = match pool.get() {
//                 Ok(conn) => conn,
//                 Err(_) => {
//                     return (
//                         StatusCode::INTERNAL_SERVER_ERROR,
//                         Json(json!({"error": "Database connection error"})),
//                     ).into_response();
//                 }
//             };

//             match ReportService::delete_report(&mut conn, report_id, reporter_id) {
//                 Ok(_) => {
//                     (
//                         StatusCode::OK,
//                         Json(json!({"message": "Report deleted successfully"})),
//                     ).into_response()
//                 }
//                 Err(diesel::result::Error::NotFound) => (
//                     StatusCode::NOT_FOUND,
//                     Json(json!({"error": "Report not found or not owned by you"})),
//                 ).into_response(),
//                 Err(diesel::result::Error::RollbackTransaction) => (
//                     StatusCode::BAD_REQUEST,
//                     Json(json!({"error": "Report can only be deleted when in 'Received' or 'Rejected' status"})),
//                 ).into_response(),
//                 Err(e) => (
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     Json(json!({"error": e.to_string()})),
//                 ).into_response(),
//             }
//         }
//         Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
//     }
// }