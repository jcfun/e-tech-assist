use std::{fmt::Debug, net::AddrParseError};

use axum::{
    extract::multipart::MultipartError,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maxminddb::MaxMindDBError;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::config::init::get_cfg;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MyError {
    DBError(String),
    AxumError(String),
    NotFound(String),
    InvalidInput(String),
    BadRequest(String),
    Forbidden(String),
    HandlersError(String),
    RedisError(String),
    AddrParseError(String),
    MaxMindDBError(String),
    UnwrapError(String),
    EmailSendError(String),
    MultipartError(String),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

// impl std::error::Error for MyError {}

impl MyError {
    pub fn error_msg(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                warn!("数据库错误: {msg}");
                format!("数据库错误: {:?}", msg)
            }
            MyError::AxumError(msg) => {
                warn!("服务器内部错误: {msg}");
                format!("服务器内部错误: {:?}", msg)
            }
            MyError::NotFound(msg) => {
                warn!("路径不存在: {msg}");
                format!("路径不存在: {:?}", msg)
            }
            MyError::InvalidInput(msg) => {
                warn!("非法输入: {msg}");
                format!("非法输入: {:?}", msg)
            }
            MyError::BadRequest(msg) => {
                warn!("非法请求: {msg}");
                format!("非法请求: {:?}", msg)
            }
            MyError::Forbidden(msg) => {
                warn!("没有权限: {msg}");
                format!("没有权限: {:?}", msg)
            }
            MyError::HandlersError(msg) => {
                warn!("程序处理错误: {msg}");
                format!("程序处理错误: {:?}", msg)
            }
            MyError::RedisError(msg) => {
                warn!("Redis错误: {msg}");
                format!("Redis错误: {:?}", msg)
            }
            MyError::AddrParseError(msg) => {
                warn!("ip地址格式错误: {msg}");
                format!("ip地址格式错误: {:?}", msg)
            }
            MyError::MaxMindDBError(msg) => {
                warn!("ip数据库解析错误: {msg}");
                format!("ip数据库解析错误: {:?}", msg)
            }
            MyError::UnwrapError(msg) => {
                warn!("拆箱错误: {msg}");
                format!("拆箱错误: {:?}", msg)
            }
            MyError::EmailSendError(msg) => {
                warn!("邮件发送错误: {msg}");
                format!("邮件发送错误: {:?}", msg)
            }
            MyError::MultipartError(msg) => {
                warn!("文件过大错误: {msg}");
                format!("文件过大错误: {:?}", msg)
            }
        }
    }
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            MyError::DBError(msg) => {
                warn!("数据库错误: {msg}");
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            MyError::AxumError(msg) => {
                warn!("服务器内部错误: {msg}");
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            MyError::NotFound(msg) => {
                warn!("路径不存在: {msg}");
                (StatusCode::NOT_FOUND, msg)
            }
            MyError::InvalidInput(msg) => {
                warn!("非法输入: {msg}");
                (StatusCode::BAD_REQUEST, msg)
            }
            MyError::BadRequest(msg) => {
                warn!("非法请求: {msg}");
                (StatusCode::BAD_REQUEST, msg)
            }
            MyError::Forbidden(msg) => {
                warn!("没有权限: {msg}");
                (StatusCode::FORBIDDEN, msg)
            }
            MyError::HandlersError(msg) => {
                warn!("程序处理错误: {msg}");
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            MyError::RedisError(msg) => {
                warn!("Redis错误: {msg}");
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            MyError::AddrParseError(msg) => {
                warn!("ip地址格式错误: {msg}");
                (StatusCode::BAD_REQUEST, msg)
            }
            MyError::MaxMindDBError(msg) => {
                warn!("ip数据库解析错误: {msg}");
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            MyError::UnwrapError(msg) => {
                warn!("拆箱错误: {msg}");
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            MyError::EmailSendError(msg) => {
                warn!("邮件发送错误: {msg}");
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            MyError::MultipartError(msg) => {
                warn!("文件过大错误: {msg}");
                (
                    StatusCode::BAD_REQUEST,
                    format!(
                        "文件过大, 最大只支持 {} MB",
                        get_cfg().mime.image.size / 1024 / 1024
                    ),
                )
            }
        };
        (code, msg).into_response()
    }
}

impl From<rbatis::rbdc::Error> for MyError {
    fn from(error: rbatis::rbdc::Error) -> Self {
        MyError::DBError(error.to_string())
    }
}

impl From<redis::RedisError> for MyError {
    fn from(error: redis::RedisError) -> Self {
        MyError::RedisError(error.to_string())
    }
}

impl From<MaxMindDBError> for MyError {
    fn from(error: MaxMindDBError) -> Self {
        MyError::MaxMindDBError(error.to_string())
    }
}

impl From<AddrParseError> for MyError {
    fn from(error: AddrParseError) -> Self {
        MyError::AddrParseError(error.to_string())
    }
}

impl From<MultipartError> for MyError {
    fn from(error: MultipartError) -> Self {
        MyError::MultipartError(error.to_string())
    }
}

impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        MyError::AxumError(error.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for MyError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        MyError::AxumError(error.to_string())
    }
}

// impl<T> From<T> for MyError
// where
//     T: std::error::Error,
// {
//     fn from(error: T) -> Self {
//         MyError::AxumError(error.to_string())
//     }
// }
