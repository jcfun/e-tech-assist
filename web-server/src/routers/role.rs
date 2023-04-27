use crate::handlers::role::*;
use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};
pub fn role_routes() -> Router {
    Router::new()
        .route("/", post(create_role))
        .route("/:id", delete(delete_role))
        .route("/", put(update_role))
        .route("/rp", put(update_role_perm))
        .route("/fq", post(query_roles_fq))
        .route("/:id/:disable_flag", patch(update_disable_flag))
        .route("/", get(query_roles))
}
