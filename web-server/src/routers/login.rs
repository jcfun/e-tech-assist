use crate::handlers::login::{captcha, login, register, reset_pwd, login_wxapp, wxapp_register, query_login_log_fq};
use axum::{
    routing::{get, patch, post},
    Router,
};
pub fn login_routes() -> Router {
    Router::new()
        .route("/", post(login))
        .route("/register", post(register))
        .route("/captcha", get(captcha))
        .route("/reset-pwd", patch(reset_pwd))
        .route("/wxapp", post(login_wxapp))
        .route("/wr", post(wxapp_register))
        .route("/log/fq", post(query_login_log_fq))
}
