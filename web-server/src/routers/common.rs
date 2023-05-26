use crate::handlers::article::*;
use axum::{
    routing::{get, post, put},
    Router,
};
pub fn article_query_routes() -> Router {
    Router::new()
        .route("/count/:id", get(query_user_article_count))
        .route("/fq", post(query_articles_fq))
        .route("/top", get(query_top_articles))
        .route("/hot", get(query_hot_articles))
        .route("/view/:id", put(update_article_view_count))
}
