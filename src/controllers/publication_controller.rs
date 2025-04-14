use axum::{
    extract::{Path, State},
    http::{Request, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::Local;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use serde_json::json;
use regex::Regex;

use crate::{
    database::connection::DbPool,
    middleware::auth::extract_token_from_request,
    model::publication::{NewPublication, Publication},
    services::publication_service::PublicationService,
};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePublicationRequest {
    pub title: String,
    pub description: Option<String>,
    #[validate(url)]
    pub filelink: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdatePublicationRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    #[validate(url)]
    pub filelink: Option<String>,
}

fn sanitize_input(input: &str) -> String {
    let re = Regex::new(r"[<>\'%;()&]").unwrap();
    re.replace_all(input, "").to_string()
}

#[axum::debug_handler]
pub async fn create_publication(
    State(pool): State<DbPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<CreatePublicationRequest>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut builder = Request::builder();
    for (key, value) in headers.iter() {
        builder = builder.header(key, value);
    }

    if let Err(validation_errors) = payload.validate() {
        let error_response = json!({
            "error": "Validation failed",
            "details": validation_errors
        });
        return Ok::<_, (StatusCode, Json<serde_json::Value>)>((StatusCode::BAD_REQUEST, Json(error_response)).into_response());
    }

    let sanitized_title = sanitize_input(&payload.title);
    let sanitized_description = payload.description.as_ref().map(|desc| sanitize_input(desc));
    let sanitized_filelink = payload.filelink.as_ref().map(|link| sanitize_input(link));

    match extract_token_from_request(&builder.body(()).unwrap()) {
        Ok(claims) => {
            if claims.user_type != "ADMIN" {
                return Ok((
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "Admin access required"})),
                ).into_response());
            }

            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(_) => {
                    return Ok((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Database connection error"})),
                    ).into_response());
                }
            };

            let new_publication = NewPublication {
                title: sanitized_title,
                createdat: Local::now().naive_local(),
                updatedat: None,
                description: sanitized_description,
                filelink: sanitized_filelink,
                adminid: Some(claims.user_id),
            };

            match PublicationService::create_publication(&mut conn, new_publication) {
                Ok(publication) => {
                    Ok((StatusCode::CREATED, Json(json!(publication))).into_response())
                }
                Err(e) => Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
                    .into_response()),
            }
        }
        Err(status) => Ok((status, Json(json!({"error": "Authentication failed"}))).into_response()),
    }
}

pub async fn get_publications(
    State(pool): State<DbPool>,
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database connection error"})),
            )
                .into_response()
        }
    };

    match PublicationService::get_publications(&mut conn) {
        Ok(publications) => (StatusCode::OK, Json(json!(publications))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

pub async fn get_publication(
    State(pool): State<DbPool>,
    Path(publication_id): Path<i32>,
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database connection error"})),
            )
                .into_response()
        }
    };

    match PublicationService::get_publication_by_id(&mut conn, publication_id) {
        Ok(publication) => (StatusCode::OK, Json(json!(publication))).into_response(),
        Err(diesel::result::Error::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Publication not found"})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

#[axum::debug_handler]
pub async fn update_publication(
    State(pool): State<DbPool>,
    Path(publication_id): Path<i32>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<UpdatePublicationRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut builder = Request::builder();
    for (key, value) in headers.iter() {
        builder = builder.header(key, value);
    }
    
    if let Err(validation_errors) = payload.validate() {
        let error_response = json!({
            "error": "Validation failed",
            "details": validation_errors
        });
        return Ok((StatusCode::BAD_REQUEST, Json(error_response)).into_response());
    }

    let sanitized_title = payload.title.as_ref().map(|title| sanitize_input(title));
    let sanitized_description = payload.description.as_ref().map(|desc| sanitize_input(desc));
    let sanitized_filelink = payload.filelink.as_ref().map(|link| sanitize_input(link));

    match extract_token_from_request(&builder.body(()).unwrap()) {
        Ok(claims) => {
            if claims.user_type != "ADMIN" {
                return Ok((
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "Admin access required"})),
                ).into_response());
            }

            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(_) => {
                    return Ok((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Database connection error"})),
                    )
                        .into_response());
                }
            };

            let existing = match PublicationService::get_publication_by_id(&mut conn, publication_id) {
                Ok(publication) => publication,
                Err(diesel::result::Error::NotFound) => {
                    return Ok((
                        StatusCode::NOT_FOUND,
                        Json(json!({"error": "Publication not found"})),
                    )
                        .into_response());
                }
                Err(e) => {
                    return Ok((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": e.to_string()})),
                    )
                        .into_response());
                }
            };

            if existing.adminid != Some(claims.user_id) {
                return Ok((
                    StatusCode::FORBIDDEN,
                    Json(json!({"error": "You are not authorized to update this publication"})),
                )
                    .into_response());
            }

            let updated_publication = Publication {
                publicationid: existing.publicationid,
                title: sanitized_title.unwrap_or_else(|| existing.title),
                createdat: existing.createdat,
                updatedat: Some(Local::now().naive_local()),
                description: sanitized_description.or(existing.description),
                filelink: sanitized_filelink.or(existing.filelink),
                adminid: Some(claims.user_id),
            };

            match PublicationService::update_publication(&mut conn, publication_id, updated_publication) {
                Ok(publication) => {
                    Ok((StatusCode::OK, Json(json!(publication))).into_response())
                }
                Err(e) => Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
                    .into_response()),
            }
        }
        Err(status) => Ok((status, Json(json!({"error": "Authentication failed"}))).into_response()),
    }
}

pub async fn delete_publication(
    State(pool): State<DbPool>,
    Path(publication_id): Path<i32>,
    req: Request<axum::body::Body>,
) -> impl IntoResponse {
    match extract_token_from_request(&req) {
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
                    )
                        .into_response()
                }
            };

            match PublicationService::get_publication_by_id(&mut conn, publication_id) {
                Ok(_) => match PublicationService::delete_publication(&mut conn, publication_id) {
                    Ok(_) => (
                        StatusCode::OK,
                        Json(json!({"message": "Publication deleted successfully"})),
                    )
                        .into_response(),
                    Err(_) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Failed to delete publication"})),
                    )
                        .into_response(),
                },
                Err(_) => (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Publication not found"})),
                )
                    .into_response(),
            }
        }
        Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
    }
}