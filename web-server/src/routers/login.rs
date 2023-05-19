use crate::handlers::login::{
    captcha, login, login_wxapp, query_login_log_fq, register, reset_pwd, wxapp_register,
};
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
