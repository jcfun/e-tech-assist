use std::{fmt::Debug, net::AddrParseError};

use axum::{
    extract::multipart::MultipartError,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use log::info;
use maxminddb::MaxMindDBError;
use serde::{Deserialize, Serialize};

use crate::config::init::get_cfg;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MyError {
    DBError(String),
    AxumError(String),
    NotFound(String),
    InvalidInput(String),
    HandlersError(String),
    RedisError(String),
    AddrParseError(String),
    MaxMindDBError(String),
    UnwrapError(String),
    EmailSendError(String),
    MultipartError(String),
}

impl MyError {
    pub fn error_msg(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                info!("数据库错误: {:?}", msg);
                format!("数据库错误: {:?}", msg)
            }
            MyError::AxumError(msg) => {
                info!("服务器内部错误: {:?}", msg);
                format!("服务器内部错误: {:?}", msg)
            }
            MyError::NotFound(msg) => {
                info!("路径不存在: {:?}", msg);
                format!("路径不存在: {:?}", msg)
            }
            MyError::InvalidInput(msg) => {
                info!("非法输入: {:?}", msg);
                format!("非法输入: {:?}", msg)
            }
            MyError::HandlersError(msg) => {
                info!("程序处理错误: {:?}", msg);
                format!("程序处理错误: {:?}", msg)
            }
            MyError::RedisError(msg) => {
                info!("Redis错误: {:?}", msg);
                format!("Redis错误: {:?}", msg)
            }
            MyError::AddrParseError(msg) => {
                info!("ip地址格式错误: {:?}", msg);
                format!("ip地址格式错误: {:?}", msg)
            }
            MyError::MaxMindDBError(msg) => {
                info!("ip数据库解析错误: {:?}", msg);
                format!("ip数据库解析错误: {:?}", msg)
            }
            MyError::UnwrapError(msg) => {
                info!("拆箱错误: {:?}", msg);
                format!("拆箱错误: {:?}", msg)
            }
            MyError::EmailSendError(msg) => {
                info!("邮件发送错误: {:?}", msg);
                format!("邮件发送错误: {:?}", msg)
            }
            MyError::MultipartError(msg) => {
                info!("文件过大错误: {:?}", msg);
                format!("文件过大错误: {:?}", msg)
            }
        }
    }
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            MyError::DBError(msg) => {
                info!("数据库错误: {:?}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            MyError::AxumError(msg) => {
                info!("服务器内部错误: {:?}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            MyError::NotFound(msg) => {
                info!("路径不存在: {:?}", msg);
                (StatusCode::NOT_FOUND, msg)
            }
            MyError::InvalidInput(msg) => {
                info!("非法输入: {:?}", msg);
                (StatusCode::BAD_REQUEST, msg)
            }
            MyError::HandlersError(msg) => {
                info!("程序处理错误: {:?}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            MyError::RedisError(msg) => {
                info!("Redis连接错误: {:?}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            MyError::AddrParseError(msg) => {
                info!("ip地址格式错误: {:?}", msg);
                (StatusCode::BAD_REQUEST, msg)
            }
            MyError::MaxMindDBError(msg) => {
                info!("ip数据库解析错误: {:?}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            MyError::UnwrapError(msg) => {
                info!("拆箱错误: {:?}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            MyError::EmailSendError(msg) => {
                info!("邮件发送错误: {:?}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            MyError::MultipartError(msg) => {
                info!("文件过大错误: {:?}", msg);
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

impl From<Box<dyn std::error::Error>> for MyError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        MyError::AxumError(error.to_string())
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

// impl<T: std::fmt::Debug + std::fmt::Display> From<T> for MyError {
//     fn from(error: T) -> Self {
//         MyError::AxumError(error.to_string())
//     }
// }