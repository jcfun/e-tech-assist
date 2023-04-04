use crate::handlers::user::*;
use axum::{
    routing::{get, post},
    Router,
};
pub fn user_routes() -> Router{
    Router::new()
        .route(
            "/:id",
            get(get_user_by_id).delete(delete_user).put(update_user),
        )
        .route("/", post(create_user))
}
