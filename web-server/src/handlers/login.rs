use crate::{
    common::{errors::MyError, res::Res, state::AppState},
    dbaccess::{login::{user_login, user_register}, user::select_user_count_by_id},
    models::dto::login::{LoginDTO, RegisterDTO},
    utils::{
        epc::encrypt_sha256,
        fill::fill_create_fields,
        jwt::{encode_jwt, Claims, Token},
        validate::param_validate,
    },
};
use axum::{extract::State, http::StatusCode, Json};

/// 用户登录
pub async fn login(
    State(state): State<AppState>,
    Json(mut payload): Json<LoginDTO>,
) -> Result<Res<Token>, MyError> {
    param_validate(&payload)?;
    if let Some(raw) = payload.password {
        payload.password = Some(encrypt_sha256(&raw));
    }
    let result = user_login(&state.db, payload.account, payload.password).await;
    match result? {
        Some(user_info) => {
            let token = encode_jwt(user_info).await;
            Ok(Res::from_success_msg("登录成功", token))
        }
        None => Ok(Res::from_msg(
            StatusCode::UNAUTHORIZED,
            "登录失败，账号或密码错误",
        )),
    }
}

/// 用户注册
pub async fn register(
    State(state): State<AppState>,
    Json(mut payload): Json<RegisterDTO>,
) -> Result<Res<u64>, MyError> {
    if let Some(account) = &payload.account {
        let count = select_user_count_by_id(&state.db, account).await?;
        if count > 0 {
            return Ok(Res::from_msg(
                StatusCode::BAD_REQUEST,
                "注册失败，账号已存在",
            ));
        }
    }
    let claim = Claims::new(
        Some("admin".into()),
        Some("admin".into()),
        Some("admin".into()),
    );
    fill_create_fields(&mut payload.base_dto, &claim, true);
    param_validate(&payload)?;
    if let Some(raw) = &payload.password {
        payload.password = Some(encrypt_sha256(raw));
    }
    let tx = &mut state.db.acquire_begin().await.unwrap();
    let result = user_register(tx, payload).await;
    if let Ok(value) = result {
        tx.commit().await.unwrap();
        Ok(Res::from_success_msg("注册成功", value.rows_affected))
    } else {
        tx.rollback().await.unwrap();
        Ok(Res::from_msg(StatusCode::INTERNAL_SERVER_ERROR, "注册失败"))
    }
}
