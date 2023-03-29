use crate::common::{errors::MyError, res::Res};
use axum::{
    body::{self, Bytes, Full, HttpBody},
    http::{header, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
// use serde_json::{json, Error};

/// 全局统一错误处理中间件
pub async fn mid_handler<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse
where
    B: std::fmt::Debug,
{
    let resp = next.run(req).await;
    let status = resp.status();
    let body_string = body_into_string(resp.into_body()).await;
    if !status.is_success() {
        return (
            StatusCode::OK,
            Res::<()>::from_error_msg(MyError::AxumError(body_string.clone())).into_response(),
        );
    }
    (
        StatusCode::OK,
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(header::CONTENT_TYPE, "application/json")
            .body(body::boxed(Full::from(body_string.to_string())))
            .unwrap(),
    )
}

async fn body_into_string<B>(body: B) -> String
where
    B: HttpBody<Data = Bytes> + Unpin,
    B::Error: std::fmt::Debug,
{
    let bytes = hyper::body::to_bytes(body).await.unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}
