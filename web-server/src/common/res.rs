/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-23 19:24:59
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-26 15:58:51
 * @FilePath: /e-tech-assist/web-server/src/common/base.rs
 * @Description:
 */

use axum::{
    body::{self, Full},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::errors::MyError;

/// 公共返回结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Res<T> {
    pub code: Option<u16>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

// 分页查询返回结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageRes<T> {
    pub data: Option<Vec<T>>,
    pub total: Option<u64>,
    pub total_page: Option<u64>,
}

#[derive(Debug)]
pub struct ResJsonString(pub String);

impl<T> IntoResponse for Res<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
{
    fn into_response(self) -> Response {
        let data = Self {
            code: self.code,
            msg: self.msg,
            data: self.data,
        };
        let json_string = match serde_json::to_string(&data) {
            Ok(v) => v,
            Err(e) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(body::boxed(Full::from(e.to_string())))
                    .unwrap();
            }
        };
        let res_json_string = ResJsonString(json_string.clone());
        let mut response = json_string.into_response();
        response.extensions_mut().insert(res_json_string);
        response
    }
}

impl<T> Res<T>
where
    T: Serialize + DeserializeOwned + Clone,
{

    pub fn from_msg(code: StatusCode, msg: &str) -> Self {
        Self {
            code: Some(code.as_u16()),
            msg: Some(msg.into()),
            data: None,
        }
    }

    pub fn from_success_msg(data: T, msg: &str) -> Self {
        Self {
            code: Some(StatusCode::OK.as_u16()),
            msg: Some(msg.into()),
            data: Some(data),
        }
    }

    pub fn from_error_msg(err: MyError) -> Self {
        Self {
            code: Some(StatusCode::INTERNAL_SERVER_ERROR.as_u16()),
            msg: Some(err.error_msg()),
            data: None,
        }
    }

    pub fn from_not_found() -> Self {
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
    //     Response::builder()
    //         .extension(|| {})
    //         .header("Access-Control-Allow-Origin", "*")
    //         .header("Cache-Control", "no-cache")
    //         .header("Content-Type", "text/json;charset=UTF-8")
    //         .body(Body::from(self.to_string()))
    //         .unwrap()
    // }
}

impl<T> ToString for Res<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
