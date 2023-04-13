use crate::common::res::Res;
use axum::{http::StatusCode, response::IntoResponse, BoxError};

/// 请求超时
pub async fn handle_timeout_error(err: BoxError) -> impl IntoResponse {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            Res::<()>::from_fail(StatusCode::REQUEST_TIMEOUT, "请求超时"),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Res::<()>::from_fail(StatusCode::INTERNAL_SERVER_ERROR, err.to_string().as_str()),
        )
    }
}

/// 404
pub async fn fallback() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Res::<()>::from_fail(StatusCode::NOT_FOUND, "404"),
    )
}
