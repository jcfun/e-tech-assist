use crate::{
    common::{errors::MyError, res::Res},
    config::init::get_ctx,
    dbaccess::article,
    models::dto::article::CreateArticleDTO,
    utils::{fields, jwt::Claims, validate},
};
use axum::{http::StatusCode, Json};

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
    let count = article::create_quick_msg(&mut tx, &payload)
        .await?
        .rows_affected;
    if count == 0 {
        return Ok(Res::from_fail(
            StatusCode::INTERNAL_SERVER_ERROR,
            "添加失败",
        ));
    }
    tx.commit().await.unwrap();
    Ok(Res::from_success("添加成功", count as usize))
}
