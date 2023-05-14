use crate::common::res::Res;
use axum::{
    body::{Body, Bytes, HttpBody},
    http::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use tracing::info;

/// 过滤器
pub async fn filter(req: Request<Body>, next: Next<Body>) -> Result<Response, Res<()>> {
    // 处理请求
    let (parts, req_body) = req.into_parts();
    let (body_bytes, body_string) = match get_body_data(req_body).await {
        Ok((body_bytes, body_string)) => (body_bytes, body_string),
        Err(res) => return Err(res),
    };
    info!("请求头 ===========> {:?}", parts.headers);
    info!("请求体 ===========> {:?}", body_string);
    let req = Request::from_parts(parts, Body::from(body_bytes));

    // 获取响应
    let resp = next.run(req).await;
    // 如果为成功响应，则直接返回
    let status = resp.status();
    if status.is_success() {
        Ok(resp)
    } else {
        // 如果为错误响应，则构造Res返回
        let mut body_string = get_body_string(resp.into_body()).await;
        if body_string.is_empty() {
            body_string = "服务器内部错误".to_string()
        }
        let resp = Res::<()>::from_fail(status, &body_string);
        Err(resp)
    }
}

async fn get_body_string<B>(body: B) -> String
where
    B: HttpBody<Data = Bytes> + Unpin,
    B::Error: std::fmt::Debug,
{
    let bytes = hyper::body::to_bytes(body).await.unwrap();
    String::from_utf8(bytes.to_vec()).unwrap()
}

async fn get_body_data<B>(body: B) -> Result<(Bytes, String), Res<()>>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err(Res::from_fail(
                StatusCode::OK,
                &format!("failed to read body: {}", err),
            ));
        }
    };

    match std::str::from_utf8(&bytes) {
        Ok(x) => {
            let res_data = x.to_string();
            Ok((bytes, res_data))
        }
        Err(_) => Ok((bytes, "该数据无法转输出, 可能为blob, binary".to_string())),
    }
}
