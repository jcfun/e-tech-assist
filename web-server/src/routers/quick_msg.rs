use crate::handlers::quick_msg::*;
use axum::{
    routing::{get, post, put},
    Router,
};

pub fn quick_msg_routes() -> Router {
    Router::new()
        .route("/", post(send_quick_msg))
        .route("/:a/:b", get(query_quick_msg_log))
        .route("/", put(update_read_flag))
        .route("/:a", get(query_by_reply_id))
}
