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

mod login;
mod perm;
mod role;
mod test;
mod user;
mod quick_msg;

pub fn get_sys_routers() -> Router {
    Router::new()
        // 快捷消息
        .nest("/quickMsg", quick_msg::quick_msg_routes())
        // 权限管理
        .nest("/perm", perm::perm_routes())
        // 角色管理
        .nest("/role", role::role_routes())
        // 用户管理
        .nest("/user", user::user_routes())
        // token校验(上面都是需要校验的路由)
        .layer(middleware::from_extractor::<Claims>())
        // 登录注册
        .nest("/login", login::login_routes())
        // test
        .merge(test::test_routes())
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
