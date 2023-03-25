/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-23 17:31:41
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-24 23:40:50
 * @FilePath: /e-tech-assist/web-server/src/common/errors.rs
 * @Description:
 */

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
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
    pub fn error_msg(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                format!("Database error occurred: {:?}", msg)
            }
            MyError::AxumError(msg) => {
                println!("Server error occurred: {:?}", msg);
                format!("Server error occurred: {:?}", msg)
            }
            MyError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                format!("Not found error occurred: {:?}", msg)
            }
            MyError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                format!("Invalid parameters received: {:?}", msg)
            }
            MyError::HandlersError(msg) => {
                println!("Handler error occurred: {:?}", msg);
                format!("Handler error occurred: {:?}", msg)
            }
        }
    }
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            MyError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            MyError::AxumError(msg) => {
                println!("Server error occurred: {:?}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            MyError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                (StatusCode::NOT_FOUND, msg)
            }
            MyError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                (StatusCode::BAD_REQUEST, msg)
            }
            MyError::HandlersError(msg) => {
                println!("Handler error occurred: {:?}", msg);
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