use axum::{
    routing::get,
    Router,
};

async fn public_route() -> &'static str {
    "This is a public route that anyone can access"
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(public_route))
}