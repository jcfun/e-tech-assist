use crate::handlers::quick_msg::send_quick_msg;
use axum::{routing::get, Router};
pub fn test_routes() -> Router {
    Router::new().route("/test", get(send_quick_msg))
}
