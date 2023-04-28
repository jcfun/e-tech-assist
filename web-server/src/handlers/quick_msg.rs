use crate::{
    common::{errors::MyError, res::Res},
    config::init::APP_CONTEXT,
    dbaccess::{quick_msg, user},
    models::dto::quick_msg::CreateQuickMsgDTO,
    utils::{email, fields::fill_fields, jwt::Claims, validate},
};
use axum::Json;
use hyper::StatusCode;

/// 发送快捷消息
pub async fn send_quick_msg(
    claims: Claims,
    Json(mut payload): Json<CreateQuickMsgDTO>,
) -> Result<Res<()>, MyError> {
    fill_fields(&mut payload.base_dto, &claims, true);
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
    let user_vo = user::query_user_by_id(&mut tx, &payload.recipient_id.as_ref().unwrap())
        .await?
        .unwrap();
    payload.send_type = Some("1".into());
    // 如果是发送邮件
    if payload.send_type.as_ref().unwrap() == "1" {
        let to = user_vo.email.unwrap_or("".into());
        if to.is_empty() {
            return Ok(Res::from_fail(
                StatusCode::BAD_REQUEST,
                "发送失败，该用户未设置邮箱",
            ));
        }
        // 发送邮件
        let email_res = email::send_email_single(
            to,
            payload.title.clone().unwrap_or("".into()),
            payload.content.clone().unwrap_or("消息内容为空".into()),
        )
        .await;
        // 根据结果创建发送日志
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
}
