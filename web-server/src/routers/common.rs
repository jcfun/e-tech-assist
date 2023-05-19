use crate::handlers::article::*;
use axum::{
    routing::{get, post},
    Router,
};
pub fn article_query_routes() -> Router {
    Router::new()
        .route("/info/:id", get(query_user_article_count_and_avatar))
        .route("/fq", post(query_articles_fq))
        .route("/top", get(query_top_articles))
        .route("/hot", get(query_hot_articles))
}
