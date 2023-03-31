use crate::{
    common::state::AppState,
    handlers::login::{login, register},
};
use axum::{
    routing::post,
    Router,
};
pub fn login_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}
