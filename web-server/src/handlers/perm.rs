use crate::{
    common::{
        errors::MyError,
        res::{PageRes, Res},
    },
    config::init::APP_CONTEXT,
    dbaccess::perm,
    models::{
        dto::{
            base::BaseDTO,
            perm::{CreatePermDTO, QueryPermDTO, UpdatePermDTO},
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
    // 添加权限信息
    let count = perm::create_perm(&mut tx, &payload).await?.rows_affected;
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
    let db = &APP_CONTEXT.db;
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
    let res = perm::query_perms_fq(&mut tx, &payload, &page_size, &offset).await?;
    let count = perm::query_perms_fq_count(&mut tx, &payload).await?;
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
        // 构建返回值
        let page_res = PageRes::new(
            parents,
            count,
            PageRes::total_page(count, page_size),
            page_no,
        );
        tx.commit().await.unwrap();
        return Ok(Res::from_success("查询成功", page_res));
    } else {
        Ok(Res::from_vec_not_found(PageRes::default()))
    }
}

/// 更新权限状态(是否禁用)
pub async fn update_disable_flag(
    claims: Claims,
    Path((id, disable_flag)): Path<(String, String)>,
) -> Result<Res<u64>, MyError> {
    let db = &APP_CONTEXT.db;
    let mut dto = BaseDTO::default();
    fields::fill_fields(&mut dto, &claims, false);
    dto.id = Some(id);
    let count = perm::update_disable_flag(db, &dto, &disable_flag)
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
    let db = &APP_CONTEXT.db;
    let res = perm::query_perms(&db).await?;
    if let Some(vos) = res {
        // 构建树形结构
        let mut parents = vos
            .iter()
            .filter(|vo| vo.parent_id.is_none() || vo.parent_id.as_ref().unwrap() == "")
            .map(|vo| vo.clone())
            .collect();
        get_children(&mut parents, &vos);
        let total = vos.len() as u64;
        Ok(Res::from_success(
            "查询成功",
            PageRes::new(parents, total, 0, 0),
        ))
    } else {
        Ok(Res::from_vec_not_found(PageRes::default()))
    }
}
