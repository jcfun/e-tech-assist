use crate::{
    common::state::AppState, handlers::test::test,
};
use axum::{
    routing::get,
    Router,
};
pub fn test_routes() -> Router<AppState> {
    Router::new()
        .route("/test", get(test))
}
