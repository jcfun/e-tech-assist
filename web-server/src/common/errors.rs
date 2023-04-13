use std::net::AddrParseError;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use log::info;
use maxminddb::MaxMindDBError;
use serde::{Deserialize, Serialize};

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
}

impl MyError {
    pub fn _error_msg(&self) -> String {
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
                info!("404: {:?}", msg);
                format!("404: {:?}", msg)
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
                info!("404: {:?}", msg);
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
        };
        (code, msg).into_response()
    }
}

impl From<rbatis::rbdc::Error> for MyError {
    fn from(value: rbatis::rbdc::Error) -> Self {
        MyError::DBError(value.to_string())
    }
}

impl From<redis::RedisError> for MyError {
    fn from(value: redis::RedisError) -> Self {
        MyError::RedisError(value.to_string())
    }
}

impl From<MaxMindDBError> for MyError {
    fn from(value: MaxMindDBError) -> Self {
        MyError::RedisError(value.to_string())
    }
}

impl From<AddrParseError> for MyError {
    fn from(value: AddrParseError) -> Self {
        MyError::RedisError(value.to_string())
    }
}
impl From<Box<dyn std::error::Error>> for MyError {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        MyError::RedisError(value.to_string())
    }
}
