use std::time::Duration;

use self::login::login_routes;
use self::test::test_routes;
use self::user::user_routes;
use crate::common::state::{get_state, AppState};
use crate::middleware::errors::error_handle;
use crate::utils::jwt::Claims;
use axum::error_handling::HandleErrorLayer;
use axum::{middleware, Router};
use tower::ServiceBuilder;

pub mod login;
pub mod test;
pub mod user;

pub fn get_routers() -> Router {
    let state = get_state();
    let a = ServiceBuilder::new().layer(HandleErrorLayer::new(error_handle)).timeout(Duration::from_secs(20));
    let b = middleware::from_extractor::<Claims>();
    Router::<AppState>::new()
        .nest("/user", user_routes())
        .layer(middleware::from_extractor::<Claims>())
        .merge(login_routes())
        .merge(test_routes())
        .layer(ServiceBuilder::new().layer(a)
        .with_state(state)
}
