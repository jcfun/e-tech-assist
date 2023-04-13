use crate::{
    common::{
        errors::MyError,
        res::{PageRes, Res},
    },
    config::init::APP_CONTEXT,
    dbaccess::{login::get_user_count, user},
    models::{
        dto::{
            base::BaseDTO,
            user::{CreateUserDTO, QueryUserDTO, UpdateUserDTO, UpdateUserWxDTO},
        },
        vo::user::QueryUserVO,
    },
    utils::{
        epc::encrypt_sha256,
        fields::{self, fill_fields},
        jwt::Claims,
        time::get_epoch,
        validate::param_validate,
    },
};
use axum::{extract::Path, Json};
use hyper::StatusCode;

/// 微信小程序修改用户信息
pub async fn update_user_wx(
    claims: Claims,
    Json(mut payload): Json<UpdateUserWxDTO>,
) -> Result<Res<u64>, MyError> {
    fill_fields(&mut payload.base_dto, &claims, false);
    payload.base_dto.id = claims.id;
    param_validate(&payload)?;
    let db = &APP_CONTEXT.db;
    let tx = db.acquire_begin().await.unwrap();
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    let res = user::update_user_detail(&mut tx, &payload).await?;
    if res.rows_affected != 0 {
        tx.commit().await.unwrap();
        return Ok(Res::from_success("保存成功", res.rows_affected));
    } else {
        Ok(Res::from_fail_msg("保存失败"))
    }
}

// 创建用户
pub async fn create_user(
    claims: Claims,
    Json(mut payload): Json<CreateUserDTO>,
) -> Result<Res<()>, MyError> {
    // 填充公共属性
    fill_fields(&mut payload.base_dto, &claims, true);
    param_validate(&payload)?;
    let db = &APP_CONTEXT.db;
    let count = get_user_count(&db, &payload.phone_number.clone().unwrap()).await?;
    if count != 0 {
        return Ok(Res::from_fail(
            StatusCode::BAD_REQUEST,
            "添加失败，手机号已存在",
        ));
    }
    let count = get_user_count(&db, &payload.account.clone().unwrap()).await?;
    if count != 0 {
        return Ok(Res::from_fail(
            StatusCode::BAD_REQUEST,
            "添加失败，账号已存在",
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
    let detail_id = user::create_user_detail(&mut tx, &payload).await;
    payload.detail_id = Some(detail_id?);
    // 填充公共属性
    fill_fields(&mut payload.base_dto, &claims, true);
    // 添加用户信息
    user::create_user(&mut tx, &payload).await?;
    if !payload.role_ids.is_none() && payload.role_ids.as_ref().unwrap().len() > 0 {
        let count = user::create_user_role(&mut tx, &payload)
            .await?
            .rows_affected;
        if count == 0 {
            return Ok(Res::from_fail(
                StatusCode::INTERNAL_SERVER_ERROR,
                "添加失败",
            ));
        }
    }
    tx.commit().await.unwrap();
    Ok(Res::from_success_msg("添加成功"))
}

/// 删除用户
pub async fn delete_user(claims: Claims, Path(id): Path<String>) -> Result<Res<()>, MyError> {
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
    let mut dto = BaseDTO::default();
    fill_fields(&mut dto, &claims, false);
    dto.id = Some(id);
    let count1 = user::delete_user_detail(&mut tx, &dto).await?.rows_affected;
    if count1 == 0 {
        return Ok(Res::from_fail(StatusCode::BAD_REQUEST, "删除失败"));
    }
    let count2 = user::delete_user(&mut tx, &dto).await?.rows_affected;
    if count2 == 0 {
        return Ok(Res::from_fail(StatusCode::BAD_REQUEST, "删除失败"));
    }
    tx.commit().await.unwrap();
    Ok(Res::from_success_msg("删除成功"))
}

/// 修改用户信息
pub async fn update_user(
    claims: Claims,
    Json(mut payload): Json<UpdateUserDTO>,
) -> Result<Res<()>, MyError> {
    fill_fields(&mut payload.base_dto, &claims, false);
    param_validate(&payload)?;
    let db = &APP_CONTEXT.db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    // 更新用户详情信息
    let mut update_user_wx_dto = UpdateUserWxDTO::default();
    fields::copy_fields(&payload, &mut update_user_wx_dto, false, false)?;
    let count1 = user::update_user_detail(&mut tx, &update_user_wx_dto)
        .await?
        .rows_affected;
    if count1 == 0 {
        return Ok(Res::from_fail(
            StatusCode::BAD_REQUEST,
            "修改失败, 该用户不存在",
        ));
    }

    // 更新用户角色关联信息
    let mut base_dto = BaseDTO::default();
    fields::copy_fields(&payload, &mut base_dto, false, false)?;
    // 删除用户下已关联的角色
    user::delete_user_role(&mut tx, &base_dto).await?;
    // 添加用户下关联的角色
    let mut create_user_dto = CreateUserDTO::default();
    fill_fields(&mut create_user_dto.base_dto, &claims, true);
    fields::copy_fields(&payload, &mut create_user_dto, false, false)?;
    if !create_user_dto.role_ids.is_none() && create_user_dto.role_ids.as_ref().unwrap().len() > 0 {
        let count = user::create_user_role(&mut tx, &create_user_dto)
            .await?
            .rows_affected;
        if count == 0 {
            return Ok(Res::from_fail(
                StatusCode::INTERNAL_SERVER_ERROR,
                "添加失败",
            ));
        }
    }

    // 更新用户信息
    // 密码sha256加密
    payload.password = Some(encrypt_sha256(payload.password.as_ref().unwrap()));
    let count2 = user::update_user(&mut tx, &payload).await?.rows_affected;
    if count2 == 0 {
        return Ok(Res::from_fail(StatusCode::BAD_REQUEST, "修改失败"));
    } else {
        tx.commit().await.unwrap();
        Ok(Res::from_success_msg("修改成功"))
    }
}

/// 根据id查询用户信息
pub async fn query_user(
    Json(payload): Json<QueryUserDTO>,
) -> Result<Res<PageRes<QueryUserVO>>, MyError> {
    let db = &APP_CONTEXT.db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    let page_no = payload.page_no.map(|v| v).unwrap_or(1);
    let page_size = payload.page_size.map(|v| v - 1).unwrap_or(10);
    let offset = (page_no - 1) * page_size;
    let res = user::query_user(&mut tx, &payload, &page_size, &offset).await?;
    let count = user::query_user_count(&mut tx, &payload).await?;
    if let Some(vo) = res {
        let page_res = PageRes {
            data: Some(vo),
            total_page: Some(if count == 0 { 0 } else { count / page_size + 1 }),
            total: Some(count),
            current_page: Some(page_no),
        };
        tx.commit().await.unwrap();
        return Ok(Res::from_success("查询成功", page_res));
    } else {
        Ok(Res::from_vec_not_found(PageRes::default()))
    }
}
