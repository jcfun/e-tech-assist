use crate::handlers::user::*;
use axum::{
    routing::{delete, patch, post, put},
    Router,
};
pub fn user_routes() -> Router {
    Router::new()
        .route("/wxapp", put(update_user_wx))
        .route("/", post(create_user))
        .route("/:id", delete(delete_user))
        .route("/", put(update_user))
        .route("/fq", post(query_users_fq))
        .route("/:id/:disable_flag", patch(update_disable_flag))
}
