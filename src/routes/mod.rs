pub mod public;
pub mod protected;
pub mod admin;
pub mod publication;
pub mod status;

use axum::Router;
use crate::database::DbPool;

pub fn create_routes(pool: DbPool) -> Router {
    Router::new()
        .merge(public::routes())
        .merge(protected::routes())
        .merge(admin::routes())
        .merge(publication::routes(pool))
        .merge(status::routes(pool))
}