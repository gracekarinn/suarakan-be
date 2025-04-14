// src/routes/update.rs
use axum::{
    routing::{get, put},
    Router,
};
use crate::controllers::update_controller::*;
use crate::database::connection::DbPool;

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route(
            "/api/v1/updates/:id",
            get(get_update)
                .put(update_status)
        )
        .with_state(pool)
}