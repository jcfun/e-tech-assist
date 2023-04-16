use crate::handlers::perm::*;
use axum::{
    routing::{delete, patch, post, put},
    Router,
};
pub fn perm_routes() -> Router {
    Router::new()
        .route("/", post(create_perm))
        .route("/:id", delete(delete_perm))
        .route("/", put(update_perm))
        .route("/fq", post(query_perms))
        .route("/:id/:disable_flag", patch(update_disable_flag))
}
