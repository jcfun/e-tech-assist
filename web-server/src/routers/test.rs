use crate::handlers::test::test;
use axum::{routing::get, Router};
pub fn test_routes() -> Router {
    Router::new().route("/test", get(test))
}
