use self::login::login_routes;
use self::test::test_routes;
use self::user::user_routes;
use crate::common::state::{get_state, AppState};
use crate::middleware::{
    errors::{fallback, handle_timeout_error},
    filter::filter,
};
use crate::utils::jwt::Claims;
use axum::error_handling::HandleErrorLayer;
use axum::{middleware, Router};
use tower_http::trace::TraceLayer;
use std::time::Duration;
use tower::ServiceBuilder;

pub mod login;
pub mod test;
pub mod user;

pub fn get_sys_routers() -> Router {
    // 获取共享状态
    let state = get_state();
    Router::<AppState>::new()
        // 用户管理
        .nest("/user", user_routes())
        // token校验(上面都是需要校验的路由)
        .layer(middleware::from_extractor::<Claims>())
        // 登录注册
        .merge(login_routes())
        // test
        .merge(test_routes())
        // 超时响应
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(60)),
        )
        // .layer(TraceLayer::on_request())
        // http info
        .layer(
            TraceLayer::new_for_http()
        )
        // filter
        .layer(middleware::from_fn(filter))
        // 404
        .fallback(fallback)
        // 注入全局共享状态
        .with_state(state)
}
