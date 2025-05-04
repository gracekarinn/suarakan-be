pub mod public;
pub mod protected;
pub mod admin;
pub mod publication;
pub mod report;
pub mod update;

use axum::Router;
use crate::database::DbPool;
use tower_http::cors::CorsLayer;
use http::{Method, header::{ORIGIN, AUTHORIZATION, CONTENT_TYPE}};
use std::env;

pub fn create_routes(pool: DbPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(http::HeaderValue::from_static("https://kelompok-3-suarakan-fe.pkpl.cs.ui.ac.id"))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([ORIGIN, AUTHORIZATION, CONTENT_TYPE])
        .allow_credentials(true);

    Router::new()
        .merge(public::routes())
        .merge(protected::routes())
        .merge(admin::routes())
        .merge(publication::routes(pool.clone()))
        .merge(report::routes(pool.clone()))
        .merge(update::routes(pool.clone()))
        .layer(cors)
}