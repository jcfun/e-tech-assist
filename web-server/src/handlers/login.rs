use crate::{
    common::{errors::MyError, res::Res},
    config::init::{APP_CFG, APP_CONTEXT},
    dbaccess::login::{
        self, create_user_detail_wxapp, get_user_count, get_user_info_by_openid,
        update_user_by_phone1, update_user_by_phone2,
    },
    models::{
        dto::login::{AuthDTO, AuthRegisterDTO, LoginDTO, LoginLogDTO, RegisterDTO, ResetPwdDTO},
        entity::user_detail::TUserDetail,
        vo::login::{LoginVO, UserInfoVO},
    },
    utils::{
        self,
        captcha::Captcha,
        epc::encrypt_sha256,
        fields::{fill_fields_system, fill_fields_system_entity},
        ip::get_ip_addr,
        jwt::{encode_jwt, Token},
        redis::{del_string, get_string, set_string_ex},
        time::get_epoch,
        validate::param_validate,
        wxapp::{self, resolve_data},
    },
};
use axum::{http::StatusCode, Json};
use headers::HeaderMap;
use rbatis::rbdc::db::ExecResult;

/// 用户登录
pub async fn login(
    headers: HeaderMap,
    Json(mut payload): Json<LoginDTO>,
) -> Result<Res<LoginVO>, MyError> {
    param_validate(&payload)?;
    let db = &APP_CONTEXT.db;
    // 验证码校验
    let uuid = &payload.uuid.clone().unwrap();
    let res = get_string(uuid).await;
    // 删除验证码
    let _del_res = del_string(uuid).await;
    payload.method = Some("web登录".into());
    if let Ok(value) = res {
        if value != payload.captcha.clone().unwrap().to_lowercase() {
            login_log(&headers, &payload, "登录失败", "验证码错误").await?;
            return Ok(Res::from_fail(StatusCode::BAD_REQUEST, "验证码错误"));
        }
    } else {
        login_log(&headers, &payload, "登录失败", "验证码已失效").await?;
        return Ok(Res::from_fail(StatusCode::BAD_REQUEST, "验证码已失效"));
    }
    // 账号密码校验
    payload.password = Some(encrypt_sha256(&payload.password.unwrap()));
    let result = login::get_user_info(db, payload.identity.clone(), payload.password.clone()).await;
    match result? {
        Some(user_info) => {
            let token = encode_jwt(&user_info).await;
            login_log(&headers, &payload, "登录成功", "登录成功").await?;
            Ok(Res::from_success("登录成功", LoginVO { user_info, token }))
        }
        None => {
            login_log(&headers, &payload, "登录失败", "账号或密码错误").await?;
            Ok(Res::from_fail(
                StatusCode::UNAUTHORIZED,
                "登录失败，账号或密码错误",
            ))
        }
    }
}

/// 用户注册
pub async fn register(Json(mut payload): Json<RegisterDTO>) -> Result<Res<u64>, MyError> {
    // 验证码校验
    let uuid = &payload.uuid.clone().unwrap().to_lowercase();
    let res = get_string(uuid).await;
    // 删除验证码
    let _del_res = del_string(uuid).await;
    if let Ok(value) = res {
        if value != payload.captcha.clone().unwrap().to_lowercase() {
            return Ok(Res::from_fail(StatusCode::BAD_REQUEST, "验证码错误"));
        }
    } else {
        return Ok(Res::from_fail(StatusCode::BAD_REQUEST, "验证码已失效"));
    }
    // 填充公共属性
    fill_fields_system(&mut payload.base_dto);
    // 参数校验
    param_validate(&payload)?;
    let db = &APP_CONTEXT.db;
    let count = get_user_count(&db, &payload.phone_number.clone().unwrap()).await?;
    if count != 0 {
        return Ok(Res::from_fail(
            StatusCode::BAD_REQUEST,
            "注册失败，手机号已存在",
        ));
    }
    let count = get_user_count(&db, &payload.account.clone().unwrap()).await?;
    if count != 0 {
        return Ok(Res::from_fail(
            StatusCode::BAD_REQUEST,
            "注册失败，账号已存在",
        ));
    }
    // 密码sha256加密
    payload.password = Some(encrypt_sha256(payload.password.as_ref().unwrap()));
    // 开启事务
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    payload.nickname = Some(format!("{}{}", "用户", get_epoch().to_string()));
    // 添加用户详情
    let detail_res = login::create_user_detail(&mut tx, &payload).await;
    payload.detail_id = Some(detail_res?);
    // 填充公共属性
    fill_fields_system(&mut payload.base_dto);
    // 添加用户信息
    let info_res = login::create_user_info(&mut tx, &payload).await;
    if let Ok(value) = info_res {
        tx.commit().await.unwrap();
        Ok(Res::from_success("注册成功", value.rows_affected))
    } else {
        Ok(Res::from_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            "注册失败",
        ))
    }
}

/// 验证码
pub async fn captcha() -> Result<Res<Captcha>, MyError> {
    let captcha = utils::captcha::get_captcha();
    set_string_ex(&captcha.uuid, &captcha.captcha, APP_CFG.captcha.exp).await?;
    Ok(Res::from_success("获取成功", captcha))
}

/// 登录日志
pub async fn login_log(
    headers: &HeaderMap,
    login_dto: &LoginDTO,
    status: &str,
    description: &str,
) -> Result<ExecResult, MyError> {
    let db = &APP_CONTEXT.db;
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
    // 获取ip归属地
    let ip_addr = get_ip_addr(ip)
        .await
        .map(|v1| v1)
        .unwrap_or("unknown".into());
    let identity = login_dto
        .identity
        .clone()
        .map(|v| v)
        .unwrap_or("unknown".into());
    // 填充公共字段
    fill_fields_system(&mut login_log.base_dto);
    login_log.identity = Some(identity);
    login_log.user_agent = Some(user_agent.into());
    login_log.ip = Some(ip.into());
    login_log.ip_addr = Some(ip_addr);
    login_log.status = Some(status.into());
    login_log.description = Some(description.into());
    login_log.method = login_dto.method.clone();
    Ok(login::create_login_log(db, &login_log).await?)
}

/// 重置密码
pub async fn reset_pwd(Json(mut payload): Json<ResetPwdDTO>) -> Result<Res<u64>, MyError> {
    // 验证码校验
    let uuid = &payload.uuid.clone().unwrap().to_lowercase();
    let res = get_string(uuid).await;
    // 删除验证码
    // let _del_res = del_string(uuid).await;
    if let Ok(value) = res {
        if value != payload.captcha.clone().unwrap().to_lowercase() {
            return Ok(Res::from_fail(StatusCode::BAD_REQUEST, "验证码错误"));
        }
    } else {
        return Ok(Res::from_fail(StatusCode::BAD_REQUEST, "验证码已失效"));
    }
    // 填充公共属性
    fill_fields_system(&mut payload.base_dto);
    // 参数校验
    param_validate(&payload)?;
    // 密码sha256加密
    payload.new_password = Some(encrypt_sha256(payload.new_password.as_ref().unwrap()));
    let db = &APP_CONTEXT.db;
    let res = login::reset_pwd(db, &payload).await?;
    Ok(Res::from_success("重置成功", res.rows_affected))
}

/// 微信小程序授权登录
pub async fn login_wxapp(
    headers: HeaderMap,
    Json(payload): Json<AuthDTO>,
) -> Result<Res<LoginVO>, MyError> {
    // 参数校验
    param_validate(&payload)?;
    // 获取openid和session_key
    let res = wxapp::get_session_key(
        &APP_CFG.wxapp.appid,
        &APP_CFG.wxapp.secret,
        &payload.code.unwrap(),
    )
    .await?;
    // 构建登录日志所需数据
    let mut login_dto = LoginDTO::default();
    login_dto.identity = Some("微信授权登录用户".into());
    login_dto.method = Some("微信授权登录".into());
    if res.as_object().is_none() {
        login_log(&headers, &login_dto, "登录失败", "用户授权失败").await?;
        return Ok(Res::from_fail(StatusCode::BAD_REQUEST, "用户授权失败"));
    }
    let openid = res
        .as_object()
        .unwrap()
        .get("openid")
        .and_then(|value| value.as_str())
        .unwrap_or_default();
    let session_key = res
        .as_object()
        .unwrap()
        .get("session_key")
        .and_then(|value| value.as_str())
        .unwrap_or_default();
    // 根据openid查询用户信息
    let res = get_user_info_by_openid(&APP_CONTEXT.db, &String::from(openid)).await?;
    // 判断是否已经授权过微信登录
    if res.is_none() {
        let mut user_info = UserInfoVO::default();
        user_info.openid = Some(openid.into());
        user_info.session_key = Some(session_key.into());
        login_dto.identity = Some(openid.into());
        login_log(&headers, &login_dto, "登录失败", "未进行过微信授权").await?;
        return Ok(Res::from(
            StatusCode::NOT_FOUND,
            "您未进行过微信授权",
            LoginVO {
                user_info,
                token: Token::default(),
            },
        ));
    } else {
        let mut user_info = res.unwrap();
        user_info.session_key = Some(session_key.into());
        user_info.openid = Some(openid.into());
        login_dto.identity = Some(user_info.account.clone().unwrap());
        let token = encode_jwt(&user_info).await;
        login_log(&headers, &login_dto, "登录成功", "登录成功").await?;
        Ok(Res::from_success("登录成功", LoginVO { user_info, token }))
    }
}

/// 微信小程序授权注册并登录
pub async fn wxapp_register(
    headers: HeaderMap,
    Json(payload): Json<AuthRegisterDTO>,
) -> Result<Res<LoginVO>, MyError> {
    // 参数校验
    param_validate(&payload)?;
    // 解密用户数据
    let res = resolve_data(
        payload.session_key.unwrap(),
        payload.iv.unwrap(),
        payload.encrypted_data.unwrap(),
    )?;

    // 构建用户详情信息所需的数据
    let mut user_detail = TUserDetail::default();
    // 填充公共属性
    fill_fields_system_entity(&mut user_detail.base_entity);
    user_detail.nickname = res.nick_name;
    user_detail.phone_number = payload.phone_number.clone();
    user_detail.avatar_url = res.avatar_url;
    if res.gender.unwrap() == 1 {
        user_detail.gender = Some("男".into())
    } else if res.gender.unwrap() == 2 {
        user_detail.gender = Some("女".into())
    } else {
        user_detail.gender = Some("未知".into())
    }
    user_detail.language = res.language;
    user_detail.country = res.country;
    user_detail.province = res.province;
    user_detail.city = res.city;

    // 如果是系统用户，但未授权过微信登录
    // 构建用户信息所需的数据
    let mut register_dto = RegisterDTO::default();
    register_dto.openid = payload.openid.clone();
    let db = &APP_CONTEXT.db;
    // 开启事务
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    let count = update_user_by_phone1(&mut tx, &user_detail, &register_dto).await?;
    if count.rows_affected != 0 {
        let count = update_user_by_phone2(&mut tx, &user_detail).await?;
        tx.commit().await.unwrap();
        // 如果更新成功，说明是系统用户，直接返回登录信息
        if count.rows_affected != 0 {
            // 通过用户openid获取用户数据
            let res = get_user_info_by_openid(&APP_CONTEXT.db, &payload.openid.unwrap()).await?;
            let mut user_info = res.unwrap();
            user_info.openid = Some(register_dto.openid.unwrap());

            // 构建登录日志所需的数据
            let mut login_dto = LoginDTO::default();
            login_dto.identity = Some(user_info.openid.clone().unwrap());
            login_dto.method = Some("微信授权登录".into());
            login_log(&headers, &login_dto, "登录成功", "登录成功").await?;

            let token = encode_jwt(&user_info).await;
            return Ok(Res::from_success("登录成功", LoginVO { user_info, token }));
        }
    }

    // 如果不是系统用户
    // 开启事务
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    // 添加用户详情信息
    let detail_id = create_user_detail_wxapp(&mut tx, &user_detail).await?;

    // 补充需要的用户信息
    register_dto.account = payload.phone_number;
    register_dto.detail_id = Some(detail_id);
    // 填充公共属性
    fill_fields_system(&mut register_dto.base_dto);
    // 添加用户信息
    let info_res = login::create_user_info(&mut tx, &register_dto).await;
    if let Ok(_value) = info_res {
        tx.commit().await.unwrap();
    }

    // 通过用户openid获取用户数据
    let res = get_user_info_by_openid(&APP_CONTEXT.db, &payload.openid.unwrap()).await?;
    let mut user_info = res.unwrap();
    user_info.openid = Some(register_dto.openid.unwrap());

    // 构建登录日志所需的数据
    let mut login_dto = LoginDTO::default();
    login_dto.identity = Some(user_info.openid.clone().unwrap());
    login_dto.method = Some("微信授权登录".into());
    login_log(&headers, &login_dto, "登录成功", "登录成功").await?;

    let token = encode_jwt(&user_info).await;
    Ok(Res::from_success("登录成功", LoginVO { user_info, token }))
}
