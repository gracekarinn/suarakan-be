use axum::{
    routing::{post, get},
    Router,
};
use crate::controllers::report_controller;
use crate::database::connection::DbPool;

pub fn routes(pool: DbPool) -> Router {
    Router::new()
        .route(
            "/api/v1/reports", 
            post(report_controller::create_report)
            .get(report_controller::validate_report_form)
        )
        .with_state(pool)
}