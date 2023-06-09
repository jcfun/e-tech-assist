use crate::{
    common::{
        errors::MyError,
        res::{PageRes, Res},
    },
    config::init::get_ctx,
    dbaccess::{perm, role},
    models::{
        dto::{
            base::BaseDTO,
            perm::{CreatePermDTO, QueryPermDTO, UpdatePermDTO},
            role::CreateRoleDTO,
        },
        vo::perm::QueryPermVO,
    },
    utils::{fields, jwt::Claims, validate},
};
use axum::{extract::Path, Json};
use hyper::StatusCode;

// 创建权限
pub async fn create_perm(
    claims: Claims,
    Json(mut payload): Json<CreatePermDTO>,
) -> Result<Res<()>, MyError> {
    // 填充公共属性
    fields::fill_fields(&mut payload.base_dto, &claims, true);
    validate::param_validate(&payload)?;
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
    // 添加权限信息
    let count = perm::create_perm(&mut tx, &payload).await?.rows_affected;
    let id = payload.base_dto.id.unwrap();
    let parent_id = payload.parent_id.unwrap();
    // 查询角色与权限关联表中是否有其父权限的关联信息
    let res = perm::query_role_perms_by_perm_id(&mut tx, &parent_id).await?;
    // 如果有则为新添加的权限添加其父权限关联角色的关联信息
    for item in res.iter() {
        if item.perm_id.as_ref().unwrap().eq(&parent_id) {
            let mut dto = CreateRoleDTO::default();
            fields::fill_fields(&mut dto.base_dto, &claims, true);
            dto.base_dto.id = Some(item.role_id.clone().unwrap());
            dto.perm_ids = Some(vec![id.clone()]);
            let count = role::create_role_perm(&mut tx, &dto)
                .await
                .unwrap()
                .rows_affected;
            if count == 0 {
                return Ok(Res::from_fail(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "添加失败",
                ));
            }
        }
    }
    if count == 0 {
        return Ok(Res::from_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            "添加失败",
        ));
    }
    tx.commit().await.unwrap();
    Ok(Res::from_success_msg("添加成功"))
}

/// 删除权限
pub async fn delete_perm(claims: Claims, Path(id): Path<String>) -> Result<Res<()>, MyError> {
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
    let mut dto = BaseDTO::default();
    fields::fill_fields(&mut dto, &claims, false);
    dto.id = Some(id);
    let count = perm::delete_perm(&mut tx, &dto).await?.rows_affected;
    if count == 0 {
        return Ok(Res::from_fail(StatusCode::BAD_REQUEST, "删除失败"));
    }
    tx.commit().await.unwrap();
    Ok(Res::from_success_msg("删除成功"))
}

/// 修改权限信息
pub async fn update_perm(
    claims: Claims,
    Json(mut payload): Json<UpdatePermDTO>,
) -> Result<Res<()>, MyError> {
    fields::fill_fields(&mut payload.base_dto, &claims, false);
    validate::param_validate(&payload)?;
    let db = &get_ctx().db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    // 更新权限信息
    let count = perm::update_perm(&mut tx, &payload).await?.rows_affected;
    if count == 0 {
        return Ok(Res::from_fail(StatusCode::BAD_REQUEST, "修改失败"));
    } else {
        tx.commit().await.unwrap();
        Ok(Res::from_success_msg("修改成功"))
    }
}

/// 多条件查询权限信息
pub async fn query_perms_fq(
    Json(payload): Json<QueryPermDTO>,
) -> Result<Res<PageRes<QueryPermVO>>, MyError> {
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
    let res = perm::query_perms_fq(&mut tx, &payload).await?;
    if let Some(vos) = res {
        // 构建树形结构
        let mut parents = vos
            .iter()
            .filter(|vo| {
                vo.parent_id.is_none()
                    || vo.parent_id.as_ref().unwrap() == ""
                    || !vos
                        .iter()
                        .map(|vo1| vo1.id.as_ref().unwrap())
                        .collect::<Vec<&String>>()
                        .contains(&vo.parent_id.as_ref().unwrap())
            })
            .map(|vo| vo.clone())
            .collect();
        get_children(&mut parents, &vos);
        // 获取切片索引
        let start = (page_no * page_size - page_size) as usize;
        let end = if parents.len() < page_size {
            parents.len()
        } else {
            page_no * page_size
        } as usize;
        // 切片
        let res = (&parents[start..end]).to_vec();
        // 构建返回值
        let page_res = PageRes::new(
            res,
            parents.len() as usize,
            PageRes::get_total_page(parents.len(), page_size),
            page_no,
        );
        tx.commit().await.unwrap();
        return Ok(Res::from_success("查询成功", page_res));
    } else {
        Ok(Res::from_vec_not_found(
            PageRes::default().current_page(page_no),
        ))
    }
}

/// 更新权限状态(是否禁用)
pub async fn update_disable_flag(
    claims: Claims,
    Path((id, disable_flag)): Path<(String, String)>,
) -> Result<Res<usize>, MyError> {
    let db = &get_ctx().db;
    let mut dto = BaseDTO::default();
    fields::fill_fields(&mut dto, &claims, false);
    dto.id = Some(id);
    let count = perm::update_disable_flag(db, &dto, &disable_flag)
        .await?
        .rows_affected as usize;
    if count == 0 {
        return Ok(Res::from_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            "修改失败",
        ));
    }
    Ok(Res::from_success("修改成功", count))
}

/// 构建权限信息树状结构
pub fn get_children(parents: &mut Vec<QueryPermVO>, vos: &Vec<QueryPermVO>) {
    for parent in parents {
        let children: Vec<QueryPermVO> = vos
            .iter()
            .filter(|vo| parent.id == vo.parent_id)
            .map(|vo| vo.clone())
            .collect();
        parent.children = Some(children);
        get_children(parent.children.as_mut().unwrap(), vos);
    }
}

/// 全量查询权限信息
pub async fn query_perms() -> Result<Res<PageRes<QueryPermVO>>, MyError> {
    let db = &get_ctx().db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    let res = perm::query_perms(&mut tx).await?;
    if let Some(vos) = res {
        // 构建树形结构
        let mut parents = vos
            .iter()
            .filter(|vo| vo.parent_id.is_none() || vo.parent_id.as_ref().unwrap() == "")
            .map(|vo| vo.clone())
            .collect();
        get_children(&mut parents, &vos);
        let total = vos.len() as usize;
        tx.commit().await.unwrap();
        Ok(Res::from_success(
            "查询成功",
            PageRes::new(parents, total, 0, 0),
        ))
    } else {
        Ok(Res::from_vec_not_found(PageRes::default()))
    }
}
