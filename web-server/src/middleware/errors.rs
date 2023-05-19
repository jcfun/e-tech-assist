use crate::common::res::Res;
use axum::{
    body,
    http::StatusCode,
    response::{IntoResponse, Response},
    BoxError,
};
use http_body::Full;
use hyper::header;
use std::any::Any;

/// 请求超时
pub async fn handle_timeout_error(err: BoxError) -> impl IntoResponse {
    if err.is::<tower::timeout::error::Elapsed>() {
        (StatusCode::REQUEST_TIMEOUT, "请求超时".to_string())
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
}

/// 404
pub async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "请求路径不存在")
}

// panic
pub fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response {
    let details = if let Some(s) = err.downcast_ref::<&str>() {
        s
    } else if let Some(s) = err.downcast_ref::<String>() {
        s
    } else {
        "A fatal error has occurred"
    };

    let res = Res::<()>::from_panic_msg(details);

    tracing::error!("响应体 ===========> {:?}", res);

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(body::boxed(Full::from(res.to_string())))
        .unwrap()
}
