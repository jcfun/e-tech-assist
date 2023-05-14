use crate::handlers::article::*;
use axum::{
    routing::post,
    Router,
};
pub fn article_routes() -> Router {
    Router::new()
        .route("/", post(create_article))
}
