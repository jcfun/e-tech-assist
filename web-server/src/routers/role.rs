use crate::handlers::role::*;
use axum::{
    routing::{delete, patch, post, put},
    Router,
};
pub fn role_routes() -> Router {
    Router::new()
        .route("/", post(create_role))
        .route("/:id", delete(delete_role))
        .route("/", put(update_role))
        .route("/fq", post(query_roles))
        .route("/:id/:disable_flag", patch(update_disable_flag))
}
