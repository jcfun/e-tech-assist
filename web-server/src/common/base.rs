/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-23 19:24:59
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-25 15:08:49
 * @FilePath: /e-tech-assist/web-server/src/common/base.rs
 * @Description:
 */

use axum::{body::Body, http::StatusCode, response::Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::errors::MyError;

/// 公共返回结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BaseResult<T> {
    pub code: Option<u16>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> BaseResult<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn _from_result(arg: &Result<T, MyError>) -> Self {
        if arg.is_ok() {
            Self {
                code: Some(StatusCode::OK.as_u16()),
                msg: None,
                data: arg.clone().ok(),
            }
        } else {
            Self {
                code: Some(StatusCode::INTERNAL_SERVER_ERROR.as_u16()),
                msg: Some(arg.clone().err().unwrap().error_msg()),
                data: None,
            }
        }
    }

    pub fn from_success(data: &T, msg: &str) -> Self {
        Self {
            code: Some(StatusCode::OK.as_u16()),
            msg: Some(msg.into()),
            data: Some(data.clone()),
        }
    }

    pub fn from_msg(msg: &str) -> Self {
        Self {
            code: Some(StatusCode::NOT_FOUND.as_u16()),
            msg: Some(msg.into()),
            data: None,
        }
    }

    pub fn _from_error_msg(err: &MyError) -> Self {
        Self {
            code: Some(StatusCode::INTERNAL_SERVER_ERROR.as_u16()),
            msg: Some(err.error_msg()),
            data: None,
        }
    }

    pub fn from_error_info(code: u16, err: &MyError) -> Self {
        Self {
            code: Some(code),
            msg: Some(err.error_msg()),
            data: None,
        }
    }

    pub fn resp_json(&self) -> Response<Body> {
        Response::builder()
            .extension(|| {})
            .header("Access-Control-Allow-Origin", "*")
            .header("Cache-Control", "no-cache")
            .header("Content-Type", "text/json;charset=UTF-8")
            .body(Body::from(self.to_string()))
            .unwrap()
    }
}

impl<T> ToString for BaseResult<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
