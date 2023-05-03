use crate::{
    common::{
        errors::MyError,
        res::{PageRes, Res},
    },
    config::init::{APP_CFG, APP_CONTEXT},
    dbaccess::{quick_msg, user},
    models::{
        dto::quick_msg::{CreateQuickMsgDTO, UpdateReadFlagDTO},
        vo::quick_msg::QueryQuickMsgVO,
    },
    utils::{email, fields::fill_fields, jwt::Claims, validate},
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
    let db = &APP_CONTEXT.get().unwrap().db;
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
        payload.send_type = Some("1".into());
        // 如果是发送邮件
        if payload.send_type.as_ref().unwrap() == "1" {
            let to = recipient.email.unwrap_or("".into());
            if to.is_empty() {
                return Ok(Res::from_fail(
                    StatusCode::BAD_REQUEST,
                    "发送失败，该用户未设置邮箱",
                ));
            }
            // 发送邮件
            let reply = sender
                .email
                .unwrap_or(APP_CFG.get().unwrap().email.email_addr.clone());
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
                    payload.fail_flag = Some("0".into());
                    payload.description = Some(msg.clone());
                    quick_msg::create_quick_msg_log(&mut tx, &payload).await?;
                    tx.commit().await.unwrap();
                    Ok(Res::from_success_msg(&msg))
                }
                Err(e) => {
                    payload.fail_flag = Some("1".into());
                    payload.description = Some(e.error_msg());
                    quick_msg::create_quick_msg_log(&mut tx, &payload).await?;
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
    Path((page_no, page_size)): Path<(u64, u64)>,
) -> Result<Res<PageRes<QueryQuickMsgVO>>, MyError> {
    let db = &APP_CONTEXT.get().unwrap().db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    let offset = PageRes::offset(page_no, page_size);
    let mut logs =
        quick_msg::query_quick_msg_log(&mut tx, claims.id.as_ref().unwrap(), page_size, offset)
            .await?;
    for log in logs.iter_mut() {
        log.reply_quick_msg =
            quick_msg::query_by_reply_id(&mut tx, log.id.as_ref().unwrap()).await?;
    }
    let count = quick_msg::query_quick_msg_log_count(&mut tx, claims.id.as_ref().unwrap()).await?;
    let page_res = PageRes::new(logs, count, PageRes::total_page(count, page_size), page_no);
    tx.commit().await.unwrap();
    Ok(Res::from_success("查询成功", page_res))
}

/// 通过回复消息id查询快捷消息
pub async fn query_by_reply_id(
    Path(id): Path<String>,
) -> Result<Res<PageRes<QueryQuickMsgVO>>, MyError> {
    let db = &APP_CONTEXT.get().unwrap().db;
    let tx = db.acquire_begin().await.unwrap();
    // 异步回滚回调
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            tracing::error!("An error occurred, rollback!");
        }
    });
    let reply_quick_msgs = quick_msg::query_by_reply_id(&mut tx, &id).await?.unwrap();
    let total = reply_quick_msgs.len() as u64;
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
) -> Result<Res<u64>, MyError> {
    fill_fields(&mut payload.base_dto, &claims, false);
    let db = &APP_CONTEXT.get().unwrap().db;
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
            .rows_affected;
        tx.commit().await.unwrap();
        Ok(Res::from_success("修改成功", count))
    } else {
        Ok(Res::from_fail(
            StatusCode::BAD_REQUEST,
            "修改失败，参数错误",
        ))
    }
}
