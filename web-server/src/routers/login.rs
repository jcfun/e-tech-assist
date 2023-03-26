use crate::{common::state::AppState, handlers::login::login};
use axum::{
    routing::post,
    Router,
};
pub fn login_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
}