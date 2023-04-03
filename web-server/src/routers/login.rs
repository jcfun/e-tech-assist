use crate::{
    common::state::AppState,
    handlers::login::{captcha, login, register},
};
use axum::{
    routing::{get, post},
    Router,
};
pub fn login_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(login))
        .route("/register", post(register))
        .route("/captcha", get(captcha))
}
