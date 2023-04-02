use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use log::info;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MyError {
    DBError(String),
    AxumError(String),
    NotFound(String),
    InvalidInput(String),
    HandlersError(String),
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
        };
        (code, msg).into_response()
    }
}

impl From<rbatis::rbdc::Error> for MyError {
    fn from(value: rbatis::rbdc::Error) -> Self {
        MyError::DBError(value.to_string())
    }
}
