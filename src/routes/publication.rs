use axum::{
    routing::delete,
    Router,
};
use crate::controllers::publication_controller;
use crate::database::connection::DbPool;

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route(
            "/api/v1/publications/{id}",
            delete(publication_controller::delete_publication)
        )
        .with_state(pool)
}