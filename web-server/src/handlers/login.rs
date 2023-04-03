use crate::{
    common::{errors::MyError, res::Res, state::AppState},
    dbaccess::{
        login::{create_login_log, create_user_detail, create_user_info, get_user_info},
        user::select_user_count_by_account,
    },
    models::dto::login::{LoginDTO, LoginLogDTO, RegisterDTO},
    utils::{
        self,
        captcha::Captcha,
        epc::encrypt_sha256,
        fill::{fill_create_fields, fill_fields},
        ip::get_ip_addr,
        jwt::{encode_jwt, get_epoch, Claims, Token},
        redis::{del_string, get_string, set_string_ex},
        validate::param_validate,
    },
};
use axum::{extract::State, http::StatusCode, Json};
use headers::HeaderMap;
use log::info;
use rbatis::rbdc::db::ExecResult;

/// 用户登录
pub async fn login(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(mut payload): Json<LoginDTO>,
) -> Result<Res<Token>, MyError> {
    param_validate(&payload)?;
    // 验证码校验
    let uuid = &payload.uuid.clone().unwrap().to_lowercase();
    let res = get_string(uuid).await;
    // 删除验证码
    let _del_res = del_string(uuid).await;
    if let Ok(value) = res {
        if value != payload.captcha.clone().unwrap() {
            login_log(&state, &headers, &payload, "登录失败", "验证码错误").await?;
            return Ok(Res::from_msg(StatusCode::BAD_REQUEST, "验证码错误"));
        }
    } else {
        login_log(&state, &headers, &payload, "登录失败", "验证码已失效").await?;
        return Ok(Res::from_msg(StatusCode::BAD_REQUEST, "验证码已失效"));
    }
    // 账号密码校验
    payload.password = Some(encrypt_sha256(&payload.password.unwrap()));
    let result = get_user_info(&state.db, payload.account.clone(), payload.password.clone()).await;
    match result? {
        Some(user_info) => {
            let token = encode_jwt(user_info).await;
            login_log(&state, &headers, &payload, "登录成功", "登录成功").await?;
            Ok(Res::from_success_msg("登录成功", token))
        }
        None => {
            login_log(&state, &headers, &payload, "登录失败", "账号或密码错误").await?;
            Ok(Res::from_msg(
                StatusCode::UNAUTHORIZED,
                "登录失败，账号或密码错误",
            ))
        }
    }
}

/// 用户注册
pub async fn register(
    State(state): State<AppState>,
    Json(mut payload): Json<RegisterDTO>,
) -> Result<Res<u64>, MyError> {
    // 验证码校验
    let uuid = &payload.uuid.clone().unwrap().to_lowercase();
    let res = get_string(uuid).await;
    // 删除验证码
    let _del_res = del_string(uuid).await;
    if let Ok(value) = res {
        if value != payload.captcha.clone().unwrap() {
            return Ok(Res::from_msg(StatusCode::BAD_REQUEST, "验证码错误"));
        }
    } else {
        return Ok(Res::from_msg(StatusCode::BAD_REQUEST, "验证码已失效"));
    }
    if let Some(account) = &payload.account {
        let count = select_user_count_by_account(&state.db, account).await?;
        if count > 0 {
            return Ok(Res::from_msg(
                StatusCode::BAD_REQUEST,
                "注册失败，账号已存在",
            ));
        }
    }
    let claim = Claims::new(
        Some("system".into()),
        Some("system".into()),
        Some("system".into()),
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
    // 异步回滚回调
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

/// 验证码
pub async fn captcha() -> Result<Res<Captcha>, MyError> {
    let captcha = utils::captcha::get_captcha();
    set_string_ex(&captcha.uuid, &captcha.captcha, 300).await?;
    Ok(Res::from_success_msg("获取成功", captcha))
}

/// 登录日志
pub async fn login_log(
    state: &AppState,
    headers: &HeaderMap,
    login_dto: &LoginDTO,
    status: &str,
    description: &str,
) -> Result<ExecResult, MyError> {
    let mut login_log = LoginLogDTO::default();
    let user_agent = headers
        .get("User-Agent")
        .map(|v1| v1.to_str().map(|v2| v2).unwrap_or(""))
        .unwrap_or("");
    let ip = headers
        .get("X-Forwarded-For")
        .map(|v1| {
            v1.to_str()
                .map(|v2| v2.split(',').next().unwrap().trim())
                .unwrap_or("error")
        })
        .unwrap_or_else(|| {
            headers
                .get("X-Real-IP")
                .map(|v1| v1.to_str().map(|v2| v2).unwrap_or("error"))
                .unwrap_or("unknown")
        });
    let ip_addr = get_ip_addr(ip)
        .await
        .map(|v1| v1)
        .unwrap_or("unknown".into());
    let account = login_dto
        .account
        .clone()
        .map(|v| v)
        .unwrap_or("unknown".into());
    // 填充公共字段
    fill_fields(&mut login_log.base_dto);
    login_log.account = Some(account);
    login_log.user_agent = Some(user_agent.into());
    login_log.ip = Some(ip.into());
    login_log.ip_addr = Some(ip_addr);
    login_log.status = Some(status.into());
    login_log.description = Some(description.into());
    Ok(create_login_log(&state.db, &login_log).await?)
}
