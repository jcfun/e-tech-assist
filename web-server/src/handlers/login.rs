use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse, Json,
};

use crate::{
    common::{errors::MyError, jwt::encode_jwt, res::Res, state::AppState},
    dbaccess::login::user_login,
    models::dto::login::LoginDTO,
};

pub async fn login(
    State(mut state): State<AppState>,
    Json(payload): Json<LoginDTO>,
) -> impl IntoResponse {
    let login_data = user_login(&mut state.db, payload.account, payload.password).await;
    login_data
        .map(|op| {
            op.map(|login_vo| {
                let token = encode_jwt(login_vo.id, login_vo.account);
                (StatusCode::OK, Res::from_success_msg(token, "登录成功"))
            })
            .unwrap_or_else(|| {
                (
                    StatusCode::OK,
                    Res::from_msg(StatusCode::UNAUTHORIZED, "登录失败"),
                )
            })
        })
        .unwrap_or_else(|err| {
            println!("error = {}", err);
            (
                StatusCode::OK,
                Res::from_error_msg(MyError::AxumError("服务器内部错误".into())),
            )
        })
}
