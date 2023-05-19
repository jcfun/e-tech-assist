use crate::{
    common::{
        errors::MyError,
        res::{PageRes, Res},
    },
    config::init::{get_cfg, get_ctx},
    dbaccess::{quick_msg, user},
    models::{
        dto::{
            base::BaseDTO,
            quick_msg::{CreateQuickMsgDTO, QueryQuickMsgDTO, UpdateReadFlagDTO},
        },
        vo::quick_msg::QueryQuickMsgVO,
    },
    utils::{
        email,
        fields::{self, fill_fields},
        jwt::Claims,
        validate,
    },
};
use axum::{extract::Path, Json};
use hyper::StatusCode;

/// 发送快捷消息
pub async fn send_quick_msg(
    claims: Claims,
    Json(mut payload): Json<CreateQuickMsgDTO>,
) -> Result<Res<()>, MyError> {
    fill_fields(&mut payload.base_dto, &claims, true);
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
    // 查询发送人信息
    let sender = user::query_user_by_identity(&mut tx, &claims.id.unwrap())
        .await?
        .unwrap();
    // 查询收件人信息
    let recipient =
        user::query_user_by_identity(&mut tx, &payload.recipient_identity.as_ref().unwrap())
            .await?;
    if let Some(recipient) = recipient {
        payload.recipient_identity = recipient.id;
        payload.send_method = Some("1".into());
        // 如果是发送邮件
        if payload.send_method.as_ref().unwrap() == "1" {
            let to = recipient.email.unwrap_or("".into());
            if to.is_empty() {
                return Ok(Res::from_fail(
                    StatusCode::BAD_REQUEST,
                    "发送失败，该用户未设置邮箱",
                ));
            }
            // 发送邮件
            let reply = sender.email.unwrap_or(get_cfg().email.email_addr.clone());
            payload.content =
                Some("来自 ".to_string() + &reply + " 的邮件：\n\n" + &payload.content.unwrap());
            let email_res = email::send_email_single(
                reply,
                to,
                payload.title.clone().unwrap_or("".into()),
                payload.content.clone().unwrap_or("消息内容为空".into()),
            )
            .await;
            // 根据结果创建发送日志
            payload.read_flag = Some("0".into());
            match email_res {
                Ok(msg) => {
                    payload.success_flag = Some("1".into());
                    payload.description = Some(msg.clone());
                    quick_msg::create_quick_msg(&mut tx, &payload).await?;
                    tx.commit().await.unwrap();
                    Ok(Res::from_success_msg(&msg))
                }
                Err(e) => {
                    payload.success_flag = Some("0".into());
                    payload.description = Some(e.error_msg());
                    quick_msg::create_quick_msg(&mut tx, &payload).await?;
                    tx.commit().await.unwrap();
                    Ok(Res::from_fail(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        &format!("邮件发送失败，{}", e.error_msg()),
                    ))
                }
            }
        } else {
            Ok(Res::from_success_msg("发送成功"))
        }
    } else {
        return Ok(Res::from_fail(
            StatusCode::BAD_REQUEST,
            "发送失败，该用户不存在",
        ));
    }
}

/// 查询快捷消息
pub async fn query_quick_msg_log(
    claims: Claims,
    Path((page_no, page_size)): Path<(usize, usize)>,
) -> Result<Res<PageRes<QueryQuickMsgVO>>, MyError> {
    let db = &get_ctx().db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    let offset = PageRes::get_offset(page_no, page_size);
    let mut logs =
        quick_msg::query_quick_msg(&mut tx, claims.id.as_ref().unwrap(), page_size, offset).await?;
    for log in logs.iter_mut() {
        log.children = quick_msg::query_by_reply_id(&mut tx, log.id.as_ref().unwrap(), "0").await?;
    }
    let count = quick_msg::query_quick_msg_count(&mut tx, claims.id.as_ref().unwrap()).await?;
    let page_res = PageRes::new(
        logs,
        count,
        PageRes::get_total_page(count, page_size),
        page_no,
    );
    tx.commit().await.unwrap();
    Ok(Res::from_success("查询成功", page_res))
}

/// 通过回复消息id查询快捷消息
pub async fn query_by_reply_id(
    Path(id): Path<String>,
) -> Result<Res<PageRes<QueryQuickMsgVO>>, MyError> {
    let db = &get_ctx().db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    let reply_quick_msgs = quick_msg::query_by_reply_id(&mut tx, &id, "0")
        .await?
        .unwrap();
    let total = reply_quick_msgs.len() as usize;
    tx.commit().await.unwrap();
    Ok(Res::from_success(
        "查询成功",
        PageRes::default().data(reply_quick_msgs).total(total),
    ))
}

/// 修改快捷消息已读状态
pub async fn update_read_flag(
    claims: Claims,
    Json(mut payload): Json<UpdateReadFlagDTO>,
) -> Result<Res<usize>, MyError> {
    fill_fields(&mut payload.base_dto, &claims, false);
    let db = &get_ctx().db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    if !payload.ids.is_none() && payload.ids.as_ref().unwrap().len() > 0 {
        let count = quick_msg::update_read_flag(&mut tx, &payload)
            .await?
            .rows_affected as usize;
        tx.commit().await.unwrap();
        Ok(Res::from_success("修改成功", count))
    } else {
        Ok(Res::from_fail(
            StatusCode::BAD_REQUEST,
            "修改失败，参数错误",
        ))
    }
}

/// 修改快捷消息禁用状态
pub async fn update_disable_flag(
    claims: Claims,
    Path((id, disable_flag)): Path<(String, String)>,
) -> Result<Res<usize>, MyError> {
    let db = &get_ctx().db;
    let mut dto = BaseDTO::default();
    fields::fill_fields(&mut dto, &claims, false);
    dto.id = Some(id);
    let count = quick_msg::update_disable_flag(db, &dto, &disable_flag)
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

/// 多条件模糊查询快捷消息
pub async fn query_quick_msgs_fq(
    Json(mut payload): Json<QueryQuickMsgDTO>,
) -> Result<Res<PageRes<QueryQuickMsgVO>>, MyError> {
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
    // msg_type默认为0
    if payload.msg_type.is_none() || payload.msg_type.as_ref().unwrap().is_empty() {
        payload.msg_type = Some("0".into());
    }
    let mut vos = quick_msg::query_quick_msgs_fq(&mut tx, &payload, &page_size, &offset).await?;
    let count = quick_msg::query_quick_msgs_fq_count(&mut tx, &payload).await?;
    if payload.msg_type.as_ref().unwrap() == "0" {
        for vo in vos.iter_mut() {
            vo.children =
                quick_msg::query_by_reply_id(&mut tx, vo.id.as_ref().unwrap(), "").await?;
        }
    }
    // 构建返回值
    let page_res = PageRes::new(
        vos,
        count,
        PageRes::get_total_page(count, page_size),
        page_no,
    );
    tx.commit().await.unwrap();
    return Ok(Res::from_success("查询成功", page_res));
}
