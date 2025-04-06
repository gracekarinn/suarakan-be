use axum::{
    routing::{post, get},
    Router,
};
use crate::controllers::status_controller;
use crate::database::connection::DbPool;

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route(
            "/api/v1/status", 
            post(status_controller::create_status)
                .get(status_controller::get_status)
        )
        .route(
            "/api/v1/status/{update_id}", 
            get(status_controller::get_status)
                .put(status_controller::update_status)
        )
        .with_state(pool)
}