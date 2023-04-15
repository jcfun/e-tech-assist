use self::login::login_routes;
use self::test::test_routes;
use self::user::user_routes;
use crate::middleware::{
    cors,
    errors::{fallback, handle_panic, handle_timeout_error},
    filter::filter,
};
use crate::utils::jwt::Claims;
use axum::error_handling::HandleErrorLayer;
use axum::{middleware, Router};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{catch_panic::CatchPanicLayer, trace::TraceLayer};

pub mod login;
pub mod test;
pub mod user;

pub fn get_sys_routers() -> Router {
    Router::new()
        // 用户管理
        .nest("/user", user_routes())
        // token校验(上面都是需要校验的路由)
        .layer(middleware::from_extractor::<Claims>())
        // 登录注册
        .nest("/login", login_routes())
        // test
        .merge(test_routes())
        // 超时响应
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(600)),
        )
        // http info
        .layer(TraceLayer::new_for_http())
        // filter
        .layer(middleware::from_fn(filter))
        // panic捕获
        .layer(CatchPanicLayer::custom(handle_panic))
        // 跨域
        .layer(cors::cors())
        // 404
        .fallback(fallback)
}
