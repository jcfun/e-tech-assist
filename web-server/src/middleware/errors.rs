use crate::common::res::Res;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    BoxError,
};

pub async fn handle_timeout_error(err: BoxError) -> impl IntoResponse {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            Res::<()>::from_msg(StatusCode::REQUEST_TIMEOUT, "请求超时"),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Res::<()>::from_msg(StatusCode::INTERNAL_SERVER_ERROR, err.to_string().as_str()),
        )
    }
}

pub async fn fallback() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Res::<()>::from_msg(StatusCode::NOT_FOUND, "404"),
    )
}
