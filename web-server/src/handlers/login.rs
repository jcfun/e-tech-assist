use std::collections::HashSet;

use crate::{
    common::{
        errors::MyError,
        res::{PageRes, Res},
    },
    config::init::{get_cfg, get_ctx},
    dbaccess::{
        login::{
            self, create_user_detail_wxapp, get_user_count, get_user_info_by_openid,
            update_user_by_phone1, update_user_by_phone2,
        },
        role, user,
    },
    models::{
        dto::login::{
            AuthDTO, AuthRegisterDTO, LoginDTO, LoginLogDTO, QueryLoginLogDTO, RegisterDTO,
            ResetPwdDTO,
        },
        entity::user_detail::TUserDetail,
        vo::login::{LoginVO, QueryLoginLogVO, UserInfoVO},
    },
    utils::{
        self,
        captcha::Captcha,
        epc::encrypt_sha256,
        fields::{fill_fields_system, fill_fields_system_entity},
        ip::get_location,
        jwt::{encode_jwt, Token},
        redis::{del_string, get_string, set_string_ex},
        time::get_epoch,
        validate::param_validate,
        wxapp::{self, resolve_data},
    },
};
use axum::{http::StatusCode, Json};
use axum_client_ip::SecureClientIp;
use headers::HeaderMap;
use rbatis::rbdc::db::ExecResult;

use super::perm::get_children;

/// 用户登录
pub async fn login(
    headers: HeaderMap,
    secure_ip: SecureClientIp,
    Json(mut payload): Json<LoginDTO>,
) -> Result<Res<LoginVO>, MyError> {
    param_validate(&payload)?;
    // 验证码校验
    let uuid = &payload.uuid.clone().unwrap();
    let res = get_string(uuid).await;
    // 删除验证码
    let _del_res = del_string(uuid).await;
    payload.method = Some("2".into());
    if let Ok(value) = res {
        if value != payload.captcha.clone().unwrap().to_lowercase() {
            login_log(&headers, secure_ip, &payload, "0", "验证码错误").await?;
            return Ok(Res::from_fail(StatusCode::BAD_REQUEST, "验证码错误"));
        }
    } else {
        login_log(&headers, secure_ip, &payload, "0", "验证码已失效").await?;
        return Ok(Res::from_fail(StatusCode::BAD_REQUEST, "验证码已失效"));
    }
    let db = &get_ctx().db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    // 账号密码校验
    payload.password = Some(encrypt_sha256(&payload.password.unwrap()));
    let result =
        login::get_user_info(&mut tx, payload.identity.clone(), payload.password.clone()).await;
    match result? {
        Some(mut user_info) => {
            // 获取用户下关联的角色
            let roles = user::query_roles_by_user_id(&mut tx, &user_info.id.as_ref().unwrap())
                .await
                .unwrap_or(Some(vec![]));

            // 获取角色下关联的权限
            let roles = roles.as_ref().unwrap();
            let mut perms = vec![];
            for role in roles.iter() {
                let role_perms = role::query_perms_by_role_id(&mut tx, &role.id.as_ref().unwrap())
                    .await
                    .unwrap_or(Some(vec![]));
                perms.append(&mut role_perms.unwrap());
            }
            // 去重
            let unique_perms = {
                let mut perms_set = HashSet::new();
                let mut result = vec![];
                for perm in perms {
                    if perms_set.insert(perm.clone()) {
                        result.push(perm);
                    }
                }
                result
            };
            // 构建树形结构
            let mut parents = unique_perms
                .iter()
                .filter(|vo| {
                    vo.parent_id.is_none()
                        || vo.parent_id.as_ref().unwrap() == ""
                        || !unique_perms
                            .iter()
                            .map(|vo1| vo1.id.as_ref().unwrap())
                            .collect::<Vec<&String>>()
                            .contains(&vo.parent_id.as_ref().unwrap())
                })
                .map(|vo| vo.clone())
                .collect();
            get_children(&mut parents, &unique_perms);
            user_info.perms = Some(parents);
            let token = encode_jwt(&user_info).await;
            login_log(&headers, secure_ip, &payload, "1", "登录成功").await?;
            tx.commit().await.unwrap();
            Ok(Res::from_success("登录成功", LoginVO { user_info, token }))
        }
        None => {
            login_log(&headers, secure_ip, &payload, "登录失败", "账号或密码错误").await?;
            Ok(Res::from_fail(
                StatusCode::UNAUTHORIZED,
                "登录失败，账号或密码错误",
            ))
        }
    }
}

/// 用户注册
pub async fn register(Json(mut payload): Json<RegisterDTO>) -> Result<Res<usize>, MyError> {
    // 参数校验
    param_validate(&payload)?;
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
    let db = &get_ctx().db;
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
        Ok(Res::from_success("注册成功", value.rows_affected as usize))
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
    set_string_ex(&captcha.uuid, &captcha.captcha, get_cfg().captcha.exp).await?;
    Ok(Res::from_success("获取成功", captcha))
}

/// 登录日志
pub async fn login_log(
    headers: &HeaderMap,
    secure_ip: SecureClientIp,
    login_dto: &LoginDTO,
    success_flag: &str,
    description: &str,
) -> Result<ExecResult, MyError> {
    let db = &get_ctx().db;
    let mut login_log = LoginLogDTO::default();
    let user_agent = headers
        .get("User-Agent")
        .map(|v1| v1.to_str().map(|v2| v2).unwrap_or("未知"))
        .unwrap_or("未知");
    let ip = headers
        .get("X-Forwarded-For")
        .map(|v1| {
            v1.to_str()
                .map(|v2| v2.split(',').next().unwrap().trim())
                .unwrap_or("unknown")
        })
        .unwrap_or_else(|| {
            headers
                .get("X-Real-IP")
                .map(|v1| v1.to_str().map(|v2| v2).unwrap_or("unknown"))
                .unwrap_or("unknown")
        });
    let ip = if ip == "unknown" {
        if secure_ip.0.to_string().starts_with("::ffff:") {
            secure_ip
                .0
                .to_string()
                .split(":")
                .collect::<Vec<_>>()
                .last()
                .unwrap_or(&"unknown")
                .to_string()
        } else {
            secure_ip.0.to_string()
        }
    } else {
        ip.to_string()
    };
    // 获取ip归属地
    let location = get_location(&ip)
        .await
        .map(|v| v)
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
    login_log.location = Some(location);
    login_log.success_flag = Some(success_flag.into());
    login_log.description = Some(description.into());
    login_log.method = login_dto.method.clone();
    Ok(login::create_login_log(db, &login_log).await?)
}

/// 重置密码
pub async fn reset_pwd(Json(mut payload): Json<ResetPwdDTO>) -> Result<Res<usize>, MyError> {
    // 参数校验
    param_validate(&payload)?;
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
    // 密码sha256加密
    payload.new_password = Some(encrypt_sha256(payload.new_password.as_ref().unwrap()));
    let db = &get_ctx().db;
    let res = login::reset_pwd(db, &payload).await?;
    Ok(Res::from_success("重置成功", res.rows_affected as usize))
}

/// 微信小程序授权登录
pub async fn login_wxapp(
    headers: HeaderMap,
    secure_ip: SecureClientIp,
    Json(payload): Json<AuthDTO>,
) -> Result<Res<LoginVO>, MyError> {
    // 参数校验
    param_validate(&payload)?;
    // 获取openid和session_key
    let res = wxapp::get_session_key(
        &get_cfg().wxapp.appid,
        &get_cfg().wxapp.secret,
        &payload.code.unwrap(),
    )
    .await?;
    // 构建登录日志所需数据
    let mut login_dto = LoginDTO::default();
    login_dto.identity = Some("微信预授权登录用户".into());
    login_dto.method = Some("0".into());
    if res.as_object().is_none() {
        login_log(&headers, secure_ip, &login_dto, "0", "用户授权失败").await?;
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
    let res = get_user_info_by_openid(&get_ctx().db, &String::from(openid)).await?;
    // 判断是否已经授权过微信登录
    if res.is_none() {
        let mut user_info = UserInfoVO::default();
        user_info.openid = Some(openid.into());
        user_info.session_key = Some(session_key.into());
        login_dto.identity = Some(openid.into());
        login_log(&headers, secure_ip, &login_dto, "0", "未进行过微信授权").await?;
        return Ok(Res::from(
            StatusCode::NOT_FOUND,
            "您未进行过授权",
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
        login_log(&headers, secure_ip, &login_dto, "1", "登录成功").await?;
        Ok(Res::from_success("登录成功", LoginVO { user_info, token }))
    }
}

/// 微信小程序授权注册并登录
pub async fn wxapp_register(
    headers: HeaderMap,
    secure_ip: SecureClientIp,
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
        user_detail.gender = Some("1".into())
    } else if res.gender.unwrap() == 2 {
        user_detail.gender = Some("2".into())
    } else {
        user_detail.gender = Some("0".into())
    }
    user_detail.language = res.language;
    user_detail.country = res.country;
    user_detail.province = res.province;
    user_detail.city = res.city;

    // 如果是系统用户，但未授权过微信登录
    // 构建用户信息所需的数据
    let mut register_dto = RegisterDTO::default();
    register_dto.openid = payload.openid.clone();
    let db = &get_ctx().db;
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
            let res = get_user_info_by_openid(&get_ctx().db, &payload.openid.unwrap()).await?;
            let mut user_info = res.unwrap();
            user_info.openid = Some(register_dto.openid.unwrap());

            // 构建登录日志所需的数据
            let mut login_dto = LoginDTO::default();
            login_dto.identity = Some(user_info.openid.clone().unwrap());
            login_dto.method = Some("0".into());
            login_log(&headers, secure_ip, &login_dto, "登录成功", "登录成功").await?;

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
    let res = get_user_info_by_openid(&get_ctx().db, &payload.openid.unwrap()).await?;
    let mut user_info = res.unwrap();
    user_info.openid = Some(register_dto.openid.unwrap());

    // 构建登录日志所需的数据
    let mut login_dto = LoginDTO::default();
    login_dto.identity = Some(user_info.openid.clone().unwrap());
    login_dto.method = Some("0".into());
    login_log(&headers, secure_ip, &login_dto, "登录成功", "登录成功").await?;
    let token = encode_jwt(&user_info).await;
    Ok(Res::from_success("登录成功", LoginVO { user_info, token }))
}

/// 多条件分页查询登录日志
pub async fn query_login_log_fq(
    Json(mut payload): Json<QueryLoginLogDTO>,
) -> Result<Res<PageRes<QueryLoginLogVO>>, MyError> {
    let db = &get_ctx().db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    let page_no = payload.page_no.map(|v| v).unwrap_or(1);
    let page_size = payload.page_size.map(|v| v).unwrap_or(10);
    let offset = PageRes::get_offset(page_no, page_size);
    // 如果有用户标识，查出用户信息
    if let Some(identity) = payload.identity.clone() {
        if !identity.is_empty() {
            let res = user::query_user_by_identity_fq(&mut tx, &identity).await?;
            if let Some(user) = res {
                payload.account = user.account;
                payload.phone_number = user.phone_number;
                payload.email = user.email;
                payload.openid = user.openid;
            } else {
                return Ok(Res::from_vec_not_found(
                    PageRes::default()
                        .total(0)
                        .total_page(0)
                        .current_page(payload.page_no),
                ));
            }
        }
    }
    let mut res = login::query_login_log_fq(&mut tx, &payload, &page_size, &offset).await?;
    // 查出每个登录日志的账号信息
    if !res.is_empty() {
        for item in res.iter_mut() {
            let account = user::query_user_by_identity_fq(
                &mut tx,
                item.identity.as_ref().unwrap_or(&"未知".to_string()),
            )
            .await?
            .unwrap_or_default()
            .account;
            item.account = account;
        }
    }
    let count = login::query_login_log_fq_count(&mut tx, &payload).await? as usize;
    let page_res = PageRes::default()
        .data(res)
        .total(count)
        .total_page(PageRes::get_total_page(count, page_size))
        .current_page(payload.page_no);
    tx.commit().await.unwrap();
    Ok(Res::from_success("查询成功", page_res))
}
