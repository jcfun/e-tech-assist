use crate::handlers::article::*;
use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};
pub fn article_routes() -> Router {
    Router::new()
        .route("/", post(create_article))
        .route("/", put(update_article))
        .route("/:id", delete(delete_article))
        .route("/info", get(query_article_info_by_user_id))
        .route("/top/:id/:top_flag", patch(update_article_top_flag))
        .route("/status/:id/:status", patch(update_article_status))
}
