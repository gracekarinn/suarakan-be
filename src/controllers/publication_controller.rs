use axum::{
    extract::{Path, State},
    http::{Request, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    database::connection::DbPool,
    middleware::auth::extract_token_from_request,
    model::publication::{NewPublication, Publication},
    services::publication_service::PublicationService,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePublicationRequest {
    pub title: String,
    pub description: Option<String>,
    pub filelink: Option<String>,
    pub adminid: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePublicationRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub filelink: Option<String>,
    pub adminid: Option<i32>,
}

#[axum::debug_handler]
pub async fn create_publication(
    State(pool): State<DbPool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<CreatePublicationRequest>,
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
                    )
                        .into_response()
                }
            };

            let new_publication = NewPublication {
                title: payload.title,
                createdat: Local::now().naive_local(),
                updatedat: None,
                description: payload.description,
                filelink: payload.filelink,
                adminid: payload.adminid,
            };

            match PublicationService::create_publication(&mut conn, new_publication) {
                Ok(publication) => {
                    (StatusCode::CREATED, Json(json!(publication))).into_response()
                }
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
                    .into_response(),
            }
        }
        Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
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
                    )
                        .into_response()
                }
            };

            let existing = match PublicationService::get_publication_by_id(&mut conn, publication_id) {
                Ok(publication) => publication,
                Err(diesel::result::Error::NotFound) => {
                    return (
                        StatusCode::NOT_FOUND,
                        Json(json!({"error": "Publication not found"})),
                    )
                        .into_response()
                }
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": e.to_string()})),
                    )
                        .into_response()
                }
            };

            let updated_publication = Publication {
                publicationid: existing.publicationid,
                title: payload.title.unwrap_or(existing.title),
                createdat: existing.createdat,
                updatedat: Some(Local::now().naive_local()),
                description: payload.description.or(existing.description),
                filelink: payload.filelink.or(existing.filelink),
                adminid: payload.adminid.or(existing.adminid),
            };

            match PublicationService::update_publication(&mut conn, publication_id, updated_publication) {
                Ok(publication) => {
                    (StatusCode::OK, Json(json!(publication))).into_response()
                }
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
                    .into_response(),
            }
        }
        Err(status) => (status, Json(json!({"error": "Authentication failed"}))).into_response(),
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