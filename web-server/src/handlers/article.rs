use crate::{
    common::{
        errors::MyError,
        res::{PageRes, Res},
    },
    config::init::get_ctx,
    dbaccess::{article, user},
    models::{
        dto::{
            article::{CreateArticleDTO, QueryArticleDTO, UpdateArticleDTO},
            base::BaseDTO,
        },
        enums::article::{ArticleTopFlag, ArticleStatus},
        vo::{
            article::{
                QueryArticleInfoVO, QueryArticleVO, QueryUserArticleVO, QueryUserArticleVOBuilder,
            },
            user::QueryUserVO,
        },
    },
    utils::{
        db::get_tx,
        fields,
        jwt::{self, Claims},
        permission::get_permit,
        validate,
    },
};
use axum::{extract::Path, http::StatusCode, Json};
use headers::HeaderMap;

/// 创建文章
pub async fn create_article(
    claims: Claims,
    Json(mut payload): Json<CreateArticleDTO>,
) -> Result<Res<usize>, MyError> {
    validate::param_validate(&payload)?;
    fields::fill_fields(&mut payload.base_dto, &claims, true);
    let default_dto = CreateArticleDTO::default();
    fields::copy_fields(&default_dto, &mut payload, true, false)?;
    let db = &get_ctx().db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    let count = article::create_article(&mut tx, &payload)
        .await?
        .rows_affected;
    if count == 0 {
        return Ok(Res::from_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            "操作失败",
        ));
    }
    tx.commit().await.unwrap();
    let msg = if payload.status.as_ref().unwrap().eq("0") {
        "保存成功"
    } else {
        "发布成功"
    };
    Ok(Res::from_success(msg, count as usize))
}

/// 多条件分页查询文章
pub async fn query_articles_fq(
    headers: HeaderMap,
    Json(mut payload): Json<QueryArticleDTO>,
) -> Result<Res<PageRes<QueryArticleVO>>, MyError> {
    let db = &get_ctx().db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    if payload.by_user_id_flag.is_some()
        && !payload.by_user_id_flag.as_ref().unwrap().is_empty()
        && payload.by_user_id_flag.as_ref().unwrap().eq("1")
    {
        let claims = headers
            .get("Authorization")
            .ok_or(MyError::AxumError("未登录".into()))?;
        let claims = jwt::decode_jwt(claims.to_str().unwrap().to_string().replace("Bearer ", ""))
            .await
            .unwrap();
        payload.base_dto.id = Some(claims.id.unwrap());
    } else {
        payload.status = Some("2".into());
    }
    let page_no = payload.page_no.map(|v| v).unwrap_or(1);
    let page_size = payload.page_size.map(|v| v).unwrap_or(10);
    let offset = PageRes::get_offset(page_no, page_size);
    let res = article::query_articles_fq(&mut tx, &payload, &page_size, &offset).await?;
    let count = article::query_articles_fq_count(&mut tx, &payload).await?;
    let res = PageRes::default()
        .data(res)
        .total(count)
        .total_page(PageRes::get_total_page(count, page_size))
        .current_page(page_no);
    tx.commit().await.unwrap();
    Ok(Res::from_success("查询成功", res))
}

/// 获取置顶文章
pub async fn query_top_articles() -> Result<Res<Vec<QueryArticleVO>>, MyError> {
    let db = &get_ctx().db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    let res = article::query_top_articles(&mut tx).await?;
    tx.commit().await.unwrap();
    Ok(Res::from_success("查询成功", res))
}

/// 获取热门文章
pub async fn query_hot_articles() -> Result<Res<Vec<QueryArticleVO>>, MyError> {
    let db = &get_ctx().db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    let res = article::query_hot_articles(&mut tx).await?;
    tx.commit().await.unwrap();
    Ok(Res::from_success("查询成功", res))
}

/// 根据用户id查询对应的文章数量和用户头像
pub async fn query_user_article_count_and_avatar(
    Path(id): Path<String>,
) -> anyhow::Result<Res<QueryUserArticleVO>, MyError> {
    let db = &get_ctx().db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    let count = article::query_article_count_by_user_id(&mut tx, &id).await?;
    let user = user::query_user_by_identity(&mut tx, &id)
        .await?
        .unwrap_or(QueryUserVO::default());
    let res = QueryUserArticleVOBuilder::default()
        .avatar(user.avatar_url)
        .total_article_count(Some(count))
        .build()
        .unwrap_or(QueryUserArticleVO::default());
    tx.commit().await.unwrap();
    Ok(Res::from_success("查询成功", res))
}

/// 根据用户id查询文章投稿数据
pub async fn query_article_info_by_user_id(
    claims: Claims,
) -> anyhow::Result<Res<QueryArticleInfoVO>, MyError> {
    let db = &get_ctx().db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    let res = article::query_article_info_by_user_id(
        &mut tx,
        &claims.id.ok_or(MyError::BadRequest("非法请求".into()))?,
    )
    .await?;
    tx.commit().await.unwrap();
    Ok(Res::from_success("查询成功", res))
}

/// 更新文章
pub async fn update_article(
    claims: Claims,
    Json(mut payload): Json<UpdateArticleDTO>,
) -> Result<Res<usize>, MyError> {
    validate::param_validate(&payload)?;
    fields::fill_fields(&mut payload.base_dto, &claims, false);
    let db = &get_ctx().db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    let count = article::update_article(&mut tx, &payload)
        .await?
        .rows_affected;
    if count == 0 {
        return Ok(Res::from_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            "操作失败",
        ));
    }
    tx.commit().await.unwrap();
    let msg = if payload.status.as_ref().unwrap().eq("0") {
        "保存成功"
    } else {
        "发布成功"
    };
    Ok(Res::from_success(msg, count as usize))
}

/// 删除文章
pub async fn delete_article(claims: Claims, Path(id): Path<String>) -> Result<Res<usize>, MyError> {
    let mut tx = get_tx().await;
    let mut dto = BaseDTO::default();
    fields::fill_fields(&mut dto, &claims, false);
    dto.id = Some(id);
    let permit_flag = get_permit(&claims, "PERM_ARTICLE")?;
    let count = article::delete_article(&mut tx, &permit_flag, &dto)
        .await?
        .rows_affected;
    if count == 0 {
        return Ok(Res::from_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            "操作失败",
        ));
    }
    tx.commit().await.unwrap();
    Ok(Res::from_success("删除成功", count as usize))
}

/// 修改文章状态(草稿、审核、发布)
pub async fn update_article_status(
    claims: Claims,
    Path((id, status)): Path<(String, String)>,
) -> Result<Res<usize>, MyError> {
    let status = ArticleStatus::from_code(&status)
        .ok_or(MyError::BadRequest("非法请求".into()))?
        .get_code();
    let mut tx = get_tx().await;
    let mut dto = BaseDTO::default();
    fields::fill_fields(&mut dto, &claims, false);
    dto.id = Some(id);
    let permit_flag = get_permit(&claims, "PERM_ARTICLE")?;
    let count = article::update_article_status(&mut tx, &dto, &status, &permit_flag)
        .await?
        .rows_affected;
    if count == 0 {
        return Ok(Res::from_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            "操作失败",
        ));
    }
    tx.commit().await.unwrap();
    Ok(Res::from_success("操作成功", count as usize))
}

/// 修改文章置顶状态
pub async fn update_article_top_flag(
    claims: Claims,
    Path((id, top_flag)): Path<(String, String)>,
) -> Result<Res<usize>, MyError> {
    let top_flag = ArticleTopFlag::from_code(&top_flag)
        .ok_or(MyError::BadRequest("非法请求".into()))?
        .get_code();
    let mut tx = get_tx().await;
    let mut dto = BaseDTO::default();
    fields::fill_fields(&mut dto, &claims, false);
    dto.id = Some(id);
    let permit_flag = get_permit(&claims, "PERM_ARTICLE")?;
    let count = article::update_article_top_flag(&mut tx, &dto, &top_flag, &permit_flag)
        .await?
        .rows_affected;
    if count == 0 {
        return Ok(Res::from_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            "操作失败",
        ));
    }
    tx.commit().await.unwrap();
    Ok(Res::from_success("操作成功", count as usize))
}
