use axum::{http::{Request, StatusCode, Response, Method, Uri}, middleware::Next, response::IntoResponse, BoxError};

use crate::common::res::Res;

/// 全局错误捕获中间件

// pub async fn error_handle<B>(mut resp: Response<B>) -> impl IntoResponse {
//     if !resp.status().is_success() {
//         let res_status = resp.status_mut();
//         let mut status_code = StatusCode::OK.;
//         res_status = &status_code;
//         (StatusCode::OK, Res::<()>::from_msg(resp.status(), "发生错误"))
//     } else {
//         (StatusCode::OK, Res::from_msg(resp.status(), "发生错误"))
//     }
// }

pub async fn error_handle(
    err: BoxError,
) -> (StatusCode, String) {
    (
        StatusCode::OK,
        "ssss".into(),
    )
}