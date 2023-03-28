use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    common::{errors::MyError, res::Res, state::AppState},
    dbaccess::login::user_login,
    models::dto::login::LoginDTO,
    utils::{epc::encrypt_sha256, jwt::encode_jwt},
};

pub async fn login(
    State(mut state): State<AppState>,
    Json(payload): Json<LoginDTO>,
) -> impl IntoResponse {
    let s = encrypt_sha256(payload.password, "salt".into());
    let login_data = user_login(
        &mut state.db,
        payload.account,
        s,
    )
    .await;
    match login_data {
        Ok(op) => match op {
            Some(login_vo) => {
                let token = encode_jwt(login_vo.id, login_vo.account).await;
                (StatusCode::OK, Res::from_success_msg(token, "登录成功"))
            }
            None => (
                StatusCode::OK,
                Res::from_msg(StatusCode::UNAUTHORIZED, "登录失败"),
            ),
        },
        Err(err) => {
            println!("error = {}", err);
            (
                StatusCode::OK,
                Res::from_error_msg(MyError::AxumError("服务器内部错误".into())),
            )
        }
    }
}
