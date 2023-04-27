use crate::{
    common::{
        errors::MyError,
        res::{PageRes, Res},
    },
    config::init::APP_CONTEXT,
    dbaccess::role,
    models::{
        dto::{
            base::BaseDTO,
            role::{CreateRoleDTO, QueryRoleDTO, UpdateRoleDTO, UpdateRolePermDTO},
        },
        vo::role::QueryRoleVO,
    },
    utils::{fields, jwt::Claims, validate},
};
use axum::{extract::Path, Json};
use hyper::StatusCode;

// 创建角色
pub async fn create_role(
    claims: Claims,
    Json(mut payload): Json<CreateRoleDTO>,
) -> Result<Res<()>, MyError> {
    // 填充公共属性
    fields::fill_fields(&mut payload.base_dto, &claims, true);
    validate::param_validate(&payload)?;
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
    // 添加角色
    role::create_role(&mut tx, &payload).await?;
    // 添加角色权限关联信息
    if !payload.perm_ids.is_none() && payload.perm_ids.as_ref().unwrap().len() > 0 {
        let count = role::create_role_perm(&mut tx, &payload)
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

/// 删除角色
pub async fn delete_role(claims: Claims, Path(id): Path<String>) -> Result<Res<()>, MyError> {
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
    fields::fill_fields(&mut dto, &claims, false);
    dto.id = Some(id);
    let count = role::delete_role(&mut tx, &dto).await?.rows_affected;
    if count == 0 {
        return Ok(Res::from_fail(StatusCode::BAD_REQUEST, "删除失败"));
    }
    tx.commit().await.unwrap();
    Ok(Res::from_success_msg("删除成功"))
}

/// 修改角色信息
pub async fn update_role(
    claims: Claims,
    Json(mut payload): Json<UpdateRoleDTO>,
) -> Result<Res<()>, MyError> {
    fields::fill_fields(&mut payload.base_dto, &claims, false);
    validate::param_validate(&payload)?;
    let db = &APP_CONTEXT.db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    // 更新角色信息
    let count = role::update_role(&mut tx, &payload).await?.rows_affected;
    if count == 0 {
        return Ok(Res::from_fail(StatusCode::BAD_REQUEST, "修改失败"));
    } else {
        tx.commit().await.unwrap();
        Ok(Res::from_success_msg("修改成功"))
    }
}

/// 修改角色权限信息
pub async fn update_role_perm(
    claims: Claims,
    Json(mut payload): Json<UpdateRolePermDTO>,
) -> Result<Res<()>, MyError> {
    fields::fill_fields(&mut payload.base_dto, &claims, false);
    validate::param_validate(&payload)?;
    let db = &APP_CONTEXT.db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    // 更新角色权限关联信息
    let mut base_dto = BaseDTO::default();
    fields::copy_fields(&payload, &mut base_dto, false, false)?;
    // 删除角色下已关联的权限
    role::delete_role_perm(&mut tx, &base_dto).await?;
    // 添加角色下关联的权限
    let mut create_role_dto = CreateRoleDTO::default();
    fields::fill_fields(&mut create_role_dto.base_dto, &claims, true);
    fields::copy_fields(&payload, &mut create_role_dto, false, false)?;
    if !create_role_dto.perm_ids.is_none() && create_role_dto.perm_ids.as_ref().unwrap().len() > 0 {
        let count = role::create_role_perm(&mut tx, &create_role_dto)
            .await?
            .rows_affected;
        if count == 0 {
            return Ok(Res::from_fail(
                StatusCode::INTERNAL_SERVER_ERROR,
                "修改失败",
            ));
        }
    }
    tx.commit().await.unwrap();
    Ok(Res::from_success_msg("修改成功"))
}

/// 多条件查询角色信息
pub async fn query_roles_fq(
    Json(payload): Json<QueryRoleDTO>,
) -> Result<Res<PageRes<QueryRoleVO>>, MyError> {
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
    let page_size = payload.page_size.map(|v| v).unwrap_or(10);
    let offset = PageRes::offset(page_no, page_size);
    let res = role::query_roles_fq(&mut tx, &payload, &page_size, &offset).await?;
    let count = role::query_roles_fq_count(&mut tx, &payload).await?;
    if let Some(mut vos) = res {
        // 获取角色下关联的权限
        for vo in vos.iter_mut() {
            vo.perms = role::query_perms_by_role_id(&mut tx, &vo.id.as_ref().unwrap())
                .await
                .unwrap_or(Some(vec![]));
        }
        let page_res = PageRes::new(vos, count, PageRes::total_page(count, page_size), page_no);
        tx.commit().await.unwrap();
        return Ok(Res::from_success("查询成功", page_res));
    } else {
        Ok(Res::from_vec_not_found(PageRes::default()))
    }
}

/// 更新角色状态(是否禁用)
pub async fn update_disable_flag(
    claims: Claims,
    Path((id, disable_flag)): Path<(String, String)>,
) -> Result<Res<u64>, MyError> {
    let db = &APP_CONTEXT.db;
    let mut dto = BaseDTO::default();
    fields::fill_fields(&mut dto, &claims, false);
    dto.id = Some(id);
    let count = role::update_disable_flag(db, &dto, &disable_flag)
        .await?
        .rows_affected;
    if count == 0 {
        return Ok(Res::from_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            "修改失败",
        ));
    }
    Ok(Res::from_success("修改成功", count))
}

/// 全量查询角色信息
pub async fn query_roles() -> Result<Res<PageRes<QueryRoleVO>>, MyError> {
    let db = &APP_CONTEXT.db;
    let res = role::query_roles(&db).await?;
    if let Some(vos) = res {
        let total = vos.len() as u64;
        Ok(Res::from_success(
            "查询成功",
            PageRes::new(vos, total, 0, 0),
        ))
    } else {
        Ok(Res::from_vec_not_found(PageRes::default()))
    }
}
