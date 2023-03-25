use crate::{
    common::{base::BaseResult, errors::MyError, state::AppState},
    models::user::*,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-23 15:43:48
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-25 19:01:52
 * @FilePath: /e-tech-assist/web-server/src/handlers/user.rs
 * @Description:
 */
/// 根据id查询用户信息
pub async fn get_user(State(mut state): State<AppState>) -> impl IntoResponse {
    let data = TUser::select_by_id(&mut state.db, 2 as i64).await;
    println!("select_by_id = {}", json!(data));


    data.map(|value| {
        value.map(|user| {
            (
                StatusCode::OK,
                BaseResult::from_success(&user, "成功").resp_json(),
            )
        }).unwrap_or_else(|| {
            (
                StatusCode::OK,
                BaseResult::<()>::from_msg("没有符合的数据").resp_json(),
            )
        })
    })
    .unwrap_or_else(|err| {
        println!("handler error occurred: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            BaseResult::<()>::from_error_info(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                &MyError::DBError("错误".into()),
            )
            .resp_json(),
        )
    })
}
pub async fn create_user(Json(_data): Json<CreateUser>) -> impl IntoResponse {
    // let body = BaseResult {
    //     code: Some(StatusCode::OK.as_u16()),
    //     msg: Some("不行".into()),
    //     data: Some(data),
    // };
    let error_data = MyError::NotFound("not found".into());
    (
        StatusCode::NOT_FOUND,
        BaseResult::<()>::from_error_info(StatusCode::NOT_FOUND.as_u16(), &error_data).resp_json(),
    )
    // BaseResult::resp_json(&error_data.error_msg())
}

pub async fn update_user() -> &'static str {
    "update"
}

pub async fn delete_user() -> &'static str {
    "delete"
}
