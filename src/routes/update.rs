use axum::{
    routing::get,
    Router,
};
use crate::controllers::update_controller::*;
use crate::database::connection::DbPool;

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route(
            "/api/v1/updates/{id}",
            get(get_update)
                .put(update_update)
        )
        .with_state(pool)
}