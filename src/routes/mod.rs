pub mod public;
pub mod protected;
pub mod admin;
pub mod publication;
pub mod status;

use axum::Router;
use crate::database::DbPool;
use tower_http::cors::CorsLayer;
use http::{Method, header::{ORIGIN, AUTHORIZATION, CONTENT_TYPE}};

pub fn create_routes(pool: DbPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:4000".parse::<http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([ORIGIN, AUTHORIZATION, CONTENT_TYPE])
        .allow_credentials(true);

    Router::new()
        .merge(public::routes())
        .merge(protected::routes())
        .merge(admin::routes())
        .merge(publication::routes(pool.clone()))
        .merge(status::routes(pool))
        .layer(cors)
}