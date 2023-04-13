use std::fmt::Debug;

use super::errors::MyError;
use axum::{
    body,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use http_body::Full;
use hyper::header;
use serde::{Deserialize, Serialize};
use tracing::info;

// 分页查询返回结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageRes<T> {
    pub data: Option<Vec<T>>,
    pub total: Option<u64>,
    pub total_page: Option<u64>,
    pub current_page: Option<u64>,
}

impl<T> Default for PageRes<T> {
    fn default() -> PageRes<T> {
        PageRes {
            data: Some(Vec::new()),
            total: None,
            total_page: None,
            current_page: None,
        }
    }
}

/// 公共返回结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Res<T> {
    pub code: Option<u16>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> ToString for Res<T>
where
    T: Serialize + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
            .map(|value| value)
            .unwrap_or_else(|err| err.to_string())
    }
}

impl<T> IntoResponse for Res<T>
where
    T: Serialize + Clone + Debug,
{
    fn into_response(self) -> Response {
        // let resp = Response::builder()
        //     // .extension(|| {})
        //     // .header("Access-Control-Allow-Origin", "*")
        //     // .header("Cache-Control", "no-cache")
        //     .status(StatusCode::OK)
        //     .header(header::CONTENT_TYPE, "application/json")
        //     .body(Body::from(self.to_string()))
        //     .unwrap();
        info!("响应体 ==============> {:?}", self);

        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .body(body::boxed(Full::from(self.to_string())))
            .unwrap()
    }
}

impl<T> Res<T>
where
    T: Serialize + Clone,
{
    pub fn from(code: StatusCode, msg: &str, data: T) -> Self {
        Self {
            code: Some(code.as_u16()),
            msg: Some(msg.into()),
            data: Some(data),
        }
    }

    pub fn from_success(msg: &str, data: T) -> Self {
        Self {
            code: Some(StatusCode::OK.as_u16()),
            msg: Some(msg.into()),
            data: Some(data),
        }
    }

    pub fn from_success_msg(msg: &str) -> Self {
        Self {
            code: Some(StatusCode::OK.as_u16()),
            msg: Some(msg.into()),
            data: None,
        }
    }

    pub fn from_fail(code: StatusCode, msg: &str) -> Self {
        Self {
            code: Some(code.as_u16()),
            msg: Some(msg.into()),
            data: None,
        }
    }

    pub fn from_fail_msg(msg: &str) -> Self {
        Self {
            code: Some(StatusCode::INTERNAL_SERVER_ERROR.as_u16()),
            msg: Some(msg.into()),
            data: None,
        }
    }

    pub fn _from_error_msg(err: MyError) -> Self {
        Self {
            code: Some(StatusCode::INTERNAL_SERVER_ERROR.as_u16()),
            msg: Some(err._error_msg()),
            data: None,
        }
    }

    pub fn _from_not_found() -> Self {
        Self {
            code: Some(StatusCode::NOT_FOUND.as_u16()),
            msg: Some("没有符合条件的结果".into()),
            data: None,
        }
    }

    pub fn from_vec_not_found(data: T) -> Self {
        Self {
            code: Some(StatusCode::NOT_FOUND.as_u16()),
            msg: Some("没有符合条件的结果".into()),
            data: Some(data),
        }
    }

    // pub fn resp_json(&self) -> Response<Body> {
    //     let resp = Response::builder()
    //         // .extension(|| {})
    //         // .header("Access-Control-Allow-Origin", "*")
    //         // .header("Cache-Control", "no-cache")
    //         .header(header::CONTENT_TYPE, "application/json")
    //         .body(Body::from(self.to_string()))
    //         .unwrap();

    //     info!("响应体 ==============> {:#?}", resp);
    //     resp
    // }
}
