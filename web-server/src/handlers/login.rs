use crate::{
    common::{errors::MyError, res::Res, state::AppState},
    dbaccess::{
        login::{create_user_detail, create_user_info, get_user_info},
        user::select_user_count_by_id,
    },
    models::dto::login::{LoginDTO, RegisterDTO},
    utils::{
        epc::encrypt_sha256,
        fill::fill_create_fields,
        jwt::{encode_jwt, Claims, Token, get_epoch},
        validate::param_validate,
    },
};
use axum::{extract::State, http::StatusCode, Json};
use log::info;

/// 用户登录
pub async fn login(
    State(state): State<AppState>,
    Json(mut payload): Json<LoginDTO>,
) -> Result<Res<Token>, MyError> {
    param_validate(&payload)?;
    if let Some(raw) = payload.password {
        payload.password = Some(encrypt_sha256(&raw));
    }
    let result = get_user_info(&state.db, payload.account, payload.password).await;
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
    // 填充公共属性
    fill_create_fields(&mut payload.base_dto, &claim, true);
    // 参数校验
    param_validate(&payload)?;
    // 密码sha256加密
    if let Some(raw) = &payload.password {
        payload.password = Some(encrypt_sha256(raw));
    }
    // 开启事务
    let tx = state.db.acquire_begin().await.unwrap();
    // 异步回滚
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            info!("An error occurred, rollback!");
        }
    });
    payload.nickname = Some(format!("{}{}", "用户", get_epoch().to_string()));
    // 添加用户详情
    let detail_res = create_user_detail(&mut tx, &payload).await;
    payload.detail_id = Some(detail_res?);
    // 添加用户信息
    let info_res = create_user_info(&mut tx, &payload).await;
    if let Ok(value) = info_res {
        tx.commit().await.unwrap();
        Ok(Res::from_success_msg("注册成功", value.rows_affected))
    } else {
        Ok(Res::from_msg(StatusCode::INTERNAL_SERVER_ERROR, "注册失败"))
    }
}
