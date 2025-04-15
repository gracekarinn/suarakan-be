use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::controllers::report_controller::*;
use crate::database::connection::DbPool;

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route(
            "/api/v1/reports",
            post(create_report)
                .get(get_reports)
        )
        .route(
            "/api/v1/reports/{id}",
            get(get_report)
                .put(update_report)
                .delete(delete_report)
        )
        .with_state(pool)
}