use self::login::login_routes;
use self::test::test_routes;
use self::user::user_routes;
use crate::common::state::{get_state, AppState};
use crate::middleware::{
    errors::{fallback, handle_timeout_error},
    filter::mid_handler,
};
use crate::utils::jwt::Claims;
use axum::error_handling::HandleErrorLayer;
use axum::{middleware, Router};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub mod login;
pub mod test;
pub mod user;

pub fn get_routers() -> Router {
    // 获取共享状态
    let state = get_state();
    Router::<AppState>::new()
        // 用户管理
        .nest("/user", user_routes())
        // token校验(上面都是需要校验的路由)
        .layer(middleware::from_extractor::<Claims>())
        // 登录
        .merge(login_routes())
        // test
        .merge(test_routes())
        // 超时响应
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(30)),
        )
        // http info
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(mid_handler))
        // 404
        .fallback(fallback)
        // 注入全局共享状态
        .with_state(state)
}
