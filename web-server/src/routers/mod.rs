use crate::middleware::{
    cors,
    errors::{fallback, handle_panic, handle_timeout_error},
    filter::filter,
};
use crate::utils::jwt::Claims;
use axum::error_handling::HandleErrorLayer;
use axum::{middleware, Router};
use axum_client_ip::SecureClientIpSource;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{catch_panic::CatchPanicLayer, trace::TraceLayer};

mod article;
mod assets;
mod common;
mod login;
mod perm;
mod quick_msg;
mod role;
mod test;
mod upload;
mod user;

pub fn get_sys_routers() -> Router {
    Router::new()
        // 文章管理
        .nest("/article", article::article_routes())
        // 文件上传
        .nest("/upload", upload::uploads_routes())
        // 快捷消息
        .nest("/quick-msg", quick_msg::quick_msg_routes())
        // 权限管理
        .nest("/perm", perm::perm_routes())
        // 角色管理
        .nest("/role", role::role_routes())
        // 用户管理
        .nest("/user", user::user_routes())
        // token校验(上面都是需要校验的路由)
        .layer(middleware::from_extractor::<Claims>())
        .nest("/articles", common::article_query_routes())
        // 静态资源服务
        .nest_service("/assets", assets::assets_routes())
        // 登录注册
        .nest("/login", login::login_routes())
        // test
        .merge(test::test_routes())
        // 超时响应
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(60)),
        )
        // ip
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        // panic捕获
        .layer(CatchPanicLayer::custom(handle_panic))
        // 404
        .fallback(fallback)
        // filter
        .layer(middleware::from_fn(filter))
        // http info
        .layer(TraceLayer::new_for_http())
        // 跨域
        .layer(cors::cors())
}
