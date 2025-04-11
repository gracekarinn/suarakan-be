use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::controllers::publication_controller::*;
use crate::database::connection::DbPool;


pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route(
            "/api/v1/publications",
            post(create_publication)
                .get(get_publications)
        )

        .route(
            "/api/v1/publications/:id",
            get(get_publication)
                .put(update_publication)
                .delete(delete_publication)
        )
        .with_state(pool)
}