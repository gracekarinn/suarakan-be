pub mod public;
pub mod protected;
pub mod admin;

use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new()
        .merge(public::routes())
        .merge(protected::routes())
        .merge(admin::routes())
}