use axum::{
    routing::{post, get},
    Router,
};
use crate::controllers::status_controller;
use crate::database::connection::DbPool;

pub fn status_routes(pool: DbPool) -> Router {
    Router::new()
        .route(
            "/api/v1/status",
            post(status_controller::create_status)
        )
        .route(
            "/api/v1/status/{id}",
            get(status_controller::get_status)
        )
        .with_state(pool)
}