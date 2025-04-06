use axum::{
    extract::{Path, State},
    http::{Request, StatusCode, HeaderMap},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use crate::services::status_service::StatusService;
use crate::middleware::auth::extract_token_from_request;

#[derive(serde::Deserialize, Debug)]
pub struct CreateStatusRequest {
    pub data_id: i32,
    pub remarks: Option<String>,
    pub proof: Option<String>,
    pub status: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct UpdateStatusRequest {
    pub remarks: Option<String>,
    pub proof: Option<String>,
    pub status: Option<String>,
}

#[derive(serde::Serialize)]
pub struct StatusResponse {
    pub update_id: i32,
    pub data_id: i32,
    pub status: Option<String>,
    pub remarks: Option<String>,
    pub proof: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct ListStatusQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

fn validate_create_status_request(payload: &CreateStatusRequest) -> Result<(), String> {
    if payload.data_id <= 0 {
        return Err("Invalid data_id".to_string());
    }
    if payload.status.is_empty() {
        return Err("Status cannot be empty".to_string());
    }
    if let Some(remarks) = &payload.remarks {
        if remarks.len() > 1000 {
            return Err("Remarks too long".to_string());
        }
    }
    if let Some(proof) = &payload.proof {
        if proof.len() > 2000 {
            return Err("Proof description too long".to_string());
        }
    }
    Ok(())
}

fn validate_update_status_request(payload: &UpdateStatusRequest) -> Result<(), String> {
    if let Some(status) = &payload.status {
        if status.is_empty() {
            return Err("Status cannot be empty".to_string());
        }
    }
    if let Some(remarks) = &payload.remarks {
        if remarks.len() > 1000 {
            return Err("Remarks too long".to_string());
        }
    }
    if let Some(proof) = &payload.proof {
        if proof.len() > 2000 {
            return Err("Proof description too long".to_string());
        }
    }
    Ok(())
}

#[axum::debug_handler]
pub async fn create_status(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    headers: HeaderMap,
    Json(payload): Json<CreateStatusRequest>,
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
                    Json(json!({"error": "Admin access required"}))
                ).into_response();
            }

            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(e) => {
                    eprintln!("Database connection error: {:?}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Database connection error"}))
                    ).into_response();
                }
            };

            if let Err(validation_error) = validate_create_status_request(&payload) {
                eprintln!("Invalid create status request: {}", validation_error);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": validation_error}))
                ).into_response();
            }

            let admin_id = match claims.user_id.to_string().parse::<i32>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Invalid user_id format: {:?}", e);
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({"error": "Invalid user_id format"}))
                    ).into_response();
                }
            };

            match StatusService::create_status(
                &mut conn,
                payload.data_id,
                payload.remarks,
                payload.proof,
                payload.status,
                admin_id,
            ) {
                Ok(status) => {
                    println!("Status created for data_id: {} by admin: {}", status.dataid, admin_id);
                    (
                        StatusCode::CREATED,
                        Json(json!({
                            "update_id": status.updateid,
                            "data_id": status.dataid,
                            "status": status.status,
                            "remarks": status.remarks,
                            "proof": status.proof,
                        }))
                    ).into_response()
                },
                Err(e) => {
                    eprintln!("Failed to create status: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Failed to create status"}))
                    ).into_response()
                }
            }
        },
        Err(status) => {
            eprintln!("Authentication failed for status creation");
            (status, Json(json!({"error": "Authentication failed"}))).into_response()
        }
    }
}

pub async fn get_status(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Path(update_id): Path<i32>,
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Database connection error: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database connection error"}))
            ).into_response();
        }
    };

    match StatusService::get_status_by_id(&mut conn, update_id) {
        Ok(status) => (
            StatusCode::OK,
            Json(StatusResponse {
                update_id: status.updateid,
                data_id: status.dataid,
                status: status.status,
                remarks: status.remarks,
                proof: status.proof,
            })
        ).into_response(),
        Err(e) => {
            eprintln!("Status not found for update_id {}: {:?}", update_id, e);
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Status not found"}))
            ).into_response()
        }
    }
}

#[axum::debug_handler]
pub async fn update_status(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Path(update_id): Path<i32>,
    headers: HeaderMap,
    Json(payload): Json<UpdateStatusRequest>,
) -> impl IntoResponse {
    let mut builder = Request::builder();
    for (key, value) in headers.iter() {
        builder = builder.header(key, value);
    }
    if let Err(validation_error) = validate_update_status_request(&payload) {
        eprintln!("Invalid update status request: {}", validation_error);
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": validation_error}))
        ).into_response();
    }
    match extract_token_from_request(&builder.body(()).unwrap()) {
        Ok(claims) => {
            if claims.user_type != "ADMIN" {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "Admin access required"}))
                ).into_response();
            }
            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(e) => {
                    eprintln!("Database connection error: {:?}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Database connection error"}))
                    ).into_response();
                }
            };

            let admin_id = match claims.user_id.to_string().parse::<i32>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Invalid user_id format: {:?}", e);
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({"error": "Invalid user_id format"}))
                    ).into_response();
                }
            };

            match StatusService::update_status(
                &mut conn,
                update_id,
                payload.remarks,
                payload.proof,
                payload.status,
                admin_id,
            ) {
                Ok(status) => {
                    println!("Status updated for update_id: {} by admin: {}", update_id, admin_id);
                    (
                        StatusCode::OK,
                        Json(json!({
                            "update_id": status.updateid,
                            "data_id": status.dataid,
                            "status": status.status,
                            "remarks": status.remarks,
                            "proof": status.proof,
                        }))
                    ).into_response()
                },
                Err(e) => {
                    eprintln!("Failed to update status: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Failed to update status"}))
                    ).into_response()
                }
            }
        },
        Err(status) => {
            eprintln!("Authentication failed for status update");
            (status, Json(json!({"error": "Authentication failed"}))).into_response()
        }
    }
}

#[axum::debug_handler]
pub async fn delete_status(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    Path(update_id): Path<i32>,
    headers: HeaderMap,
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
                    Json(json!({"error": "Admin access required"}))
                ).into_response();
            }
            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(e) => {
                    eprintln!("Database connection error: {:?}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Database connection error"}))
                    ).into_response();
                }
            };

            let admin_id = match claims.user_id.to_string().parse::<i32>() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("Invalid user_id format: {:?}", e);
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(json!({"error": "Invalid user_id format"}))
                    ).into_response();
                }
            };

            match StatusService::soft_delete_status(&mut conn, update_id, admin_id) {
                Ok(_) => {
                    println!("Status deleted for update_id: {} by admin: {}", update_id, admin_id);
                    (
                        StatusCode::OK,
                        Json(json!({"message": "Status successfully deleted"}))
                    ).into_response()
                },
                Err(e) => {
                    eprintln!("Failed to delete status: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Failed to delete status"}))
                    ).into_response()
                }
            }
        },
        Err(status) => {
            eprintln!("Authentication failed for status deletion");
            (status, Json(json!({"error": "Authentication failed"}))).into_response()
        }
    }
}

#[axum::debug_handler]
pub async fn list_statuses(
    State(pool): State<Pool<ConnectionManager<PgConnection>>>,
    headers: HeaderMap,
    Json(query): Json<ListStatusQuery>,
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
                    Json(json!({"error": "Admin access required"}))
                ).into_response();
            }
            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(e) => {
                    eprintln!("Database connection error: {:?}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Database connection error"}))
                    ).into_response();
                }
            };

            match StatusService::list_statuses(
                &mut conn,
                query.page.unwrap_or(1),
                query.per_page.unwrap_or(10)
            ) {
                Ok(statuses) => {
                    let status_responses: Vec<StatusResponse> = statuses.into_iter().map(|status| StatusResponse {
                        update_id: status.updateid,
                        data_id: status.dataid,
                        status: status.status,
                        remarks: status.remarks,
                        proof: status.proof,
                    }).collect();

                    (
                        StatusCode::OK,
                        Json(json!({
                            "statuses": status_responses,
                            "page": query.page.unwrap_or(1),
                            "per_page": query.per_page.unwrap_or(10)
                        }))
                    ).into_response()
                },
                Err(e) => {
                    eprintln!("Failed to list statuses: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Failed to list statuses"}))
                    ).into_response()
                }
            }
        },
        Err(status) => {
            eprintln!("Authentication failed for status listing");
            (status, Json(json!({"error": "Authentication failed"}))).into_response()
        }
    }
}
