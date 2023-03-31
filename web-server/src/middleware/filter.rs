use crate::common::res::Res;
use axum::{
    body::{Bytes, HttpBody},
    http::Request,
    middleware::Next,
    response::Response,
};
use log::info;

/// 全局统一错误处理中间件
pub async fn mid_handler<B>(req: Request<B>, next: Next<B>) -> Result<Response, Res<()>>
where
    B: std::fmt::Debug,
{
    // 获取响应
    let resp = next.run(req).await;
    // 如果为成功响应，则直接返回
    let status = resp.status();
    if status.is_success() {
        info!("响应 ===========> {:?}", resp);
        return Ok(resp);
    }
    // 如果为错误响应，则构造Res返回
    let mut body_string = body_into_string(resp.into_body()).await;
    info!("响应 ===========> {:?}", body_string);
    if body_string == "" {
        body_string = "请求错误".to_string()
    }
    Err(Res::<()>::from_msg(status, &body_string))
}

async fn body_into_string<B>(body: B) -> String
where
    B: HttpBody<Data = Bytes> + Unpin,
    B::Error: std::fmt::Debug,
{
    let bytes = hyper::body::to_bytes(body).await.unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}
