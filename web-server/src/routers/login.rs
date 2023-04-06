use crate::handlers::login::{captcha, login, register, reset_pwd};
use axum::{
    routing::{get, patch, post},
    Router,
};
pub fn login_routes() -> Router {
    Router::new()
        .route("/", post(login))
        .route("/register", post(register))
        .route("/captcha", get(captcha))
        .route("/reset", patch(reset_pwd))
}
