use crate::{
    common::{errors::MyError, res::Res, state::AppState},
    dbaccess::user::select_user_by_id,
    models::{dto::user::CreateUserDTO, vo::user::UserVO},
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use log::info;
use serde_json::json;

/// 根据id查询用户信息
pub async fn get_user_by_id(
    State(mut state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Res<UserVO>, MyError> {
    let result = select_user_by_id(&mut state.db, id).await;
    info!("select_by_id = {}", json!(result));
    result?
        .map(|user| Ok(Res::from_success_msg("查询成功", user)))
        .unwrap_or_else(|| Ok(Res::from_not_found()))
}
pub async fn create_user(Json(_data): Json<CreateUserDTO>) -> impl IntoResponse {
    // let body = BaseResult {
    //     code: Some(StatusCode::OK.as_u16()),
    //     msg: Some("不行".into()),
    //     data: Some(data),
    // };
    // let error_data = MyError::NotFound("not found".into());
    // (
    //     StatusCode::NOT_FOUND,
    //     Res::<()>::from_error_info(StatusCode::NOT_FOUND.as_u16(), &error_data).resp_json(),
    // )
    // BaseResult::resp_json(&error_data.error_msg())
}

pub async fn update_user() -> &'static str {
    "update"
}

pub async fn delete_user() -> &'static str {
    "delete"
}
