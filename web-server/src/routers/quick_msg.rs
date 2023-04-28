use crate::handlers::quick_msg::*;
use axum::{routing::post, Router};

pub fn quick_msg_routes() -> Router {
    Router::new().route("/", post(send_quick_msg))
}
