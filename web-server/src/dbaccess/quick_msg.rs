use crate::models::dto::quick_msg::CreateQuickMsgDTO;
use rbatis::{
    executor::RBatisTxExecutorGuard,
    py_sql,
    rbdc::{db::ExecResult, Error},
};

/// 新增快捷消息日志
#[py_sql(
    r#"`insert into t_quick_msg_log`
    ` (id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, sender_id, recipient_id, title, content, fail_flag, send_type, description)`
    ` values`
    ` (#{dto.id}, #{dto.operate_time}, #{dto.operator}, #{dto.operator_id}, #{dto.create_time}, #{dto.creator}, #{dto.creator_id}, #{dto.delete_flag}, #{dto.sender_id}, #{dto.recipient_id}, #{dto.title}, #{dto.content}, #{dto.fail_flag}, #{dto.send_type}, #{dto.description})`
    "#
)]
pub async fn create_quick_msg_log(
    tx: &mut RBatisTxExecutorGuard,
    dto: &CreateQuickMsgDTO,
) -> Result<ExecResult, Error> {
    impled!();
}
