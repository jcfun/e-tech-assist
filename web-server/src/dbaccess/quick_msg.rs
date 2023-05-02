use crate::models::{
    dto::quick_msg::{CreateQuickMsgDTO, UpdateReadFlagDTO},
    vo::quick_msg::QueryQuickMsgVO,
};
use rbatis::{
    executor::RBatisTxExecutorGuard,
    py_sql,
    rbdc::{db::ExecResult, Error},
};

/// 新增快捷消息日志
#[py_sql(
    r#"`insert into t_quick_msg_log`
    ` (id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, sender_id, recipient_id, title, content, fail_flag, send_type, description, msg_type, reply_id, read_flag)`
    ` values`
    ` (#{dto.id}, #{dto.operate_time}, #{dto.operator}, #{dto.operator_id}, #{dto.create_time}, #{dto.creator}, #{dto.creator_id}, #{dto.delete_flag}, #{dto.sender_id}, #{dto.recipient_identity}, #{dto.title}, #{dto.content}, #{dto.fail_flag}, #{dto.send_type}, #{dto.description}, #{dto.msg_type}, #{dto.reply_id}, #{dto.read_flag})`
    "#
)]
pub async fn create_quick_msg_log(
    tx: &mut RBatisTxExecutorGuard,
    dto: &CreateQuickMsgDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 查询快捷消息日志
#[py_sql(
    r#"`select a.id, to_char(a.operate_time, 'YYYY-MM-DD HH24:MI:SS') as operate_time, a.operator, a.operator_id, to_char(a.create_time, 'YYYY-MM-DD HH24:MI:SS') as create_time, a.creator, a.creator_id, a.delete_flag, a.sender_id, a.recipient_id, a.title, a.content, a.fail_flag, a.send_type, a.description, a.msg_type, a.reply_id, a.read_flag, c.email as sender_email, e.email as recipient_email, c.avatar_url as sender_avatar, e.avatar_url as recipient_avatar`
    ` from t_quick_msg_log a`
    ` join t_user b`
    ` on a.sender_id = b.id`
    ` join t_user_detail c`
    ` on b.detail_id = c.id`
    ` join t_user d`
    ` on a.recipient_id = d.id`
    ` join t_user_detail e`
    ` on d.detail_id = e.id`
    ` where a.delete_flag = '0'`
    ` and b.delete_flag = '0'`
    ` and c.delete_flag = '0'`
    ` and d.delete_flag = '0'`
    ` and e.delete_flag = '0'`
    ` and a.msg_type = '0'`
    ` and (a.sender_id = #{id} or a.recipient_id = #{id})`
    ` order by a.create_time desc`
    ` limit #{page_size}`
    ` offset #{offset}`
    "#
)]
pub async fn query_quick_msg_log(
    tx: &mut RBatisTxExecutorGuard,
    id: &String,
    page_size: u64,
    offset: u64,
) -> Result<Vec<QueryQuickMsgVO>, Error> {
    impled!();
}

/// 查询快捷消息日志数量
#[py_sql(
    r#"`select count(*)`
    ` from t_quick_msg_log a`
    ` join t_user b`
    ` on a.sender_id = b.id`
    ` join t_user_detail c`
    ` on b.detail_id = c.id`
    ` join t_user d`
    ` on a.recipient_id = d.id`
    ` join t_user_detail e`
    ` on d.detail_id = e.id`
    ` where a.delete_flag = '0'`
    ` and b.delete_flag = '0'`
    ` and c.delete_flag = '0'`
    ` and d.delete_flag = '0'`
    ` and e.delete_flag = '0'`
    ` and a.msg_type = '0'`
    ` and (a.sender_id = #{id} or a.recipient_id = #{id})`
    "#
)]
pub async fn query_quick_msg_log_count(
    tx: &mut RBatisTxExecutorGuard,
    id: &String,
) -> Result<u64, Error> {
    impled!();
}

/// 根据回复ID查询快捷消息日志
#[py_sql(
    r#"`select a.id, to_char(a.operate_time, 'YYYY-MM-DD HH24:MI:SS') as operate_time, a.operator, a.operator_id, to_char(a.create_time, 'YYYY-MM-DD HH24:MI:SS') as create_time, a.creator, a.creator_id, a.delete_flag, a.sender_id, a.recipient_id, a.title, a.content, a.fail_flag, a.send_type, a.description, a.msg_type, a.reply_id, a.read_flag, c.email as sender_email, e.email as recipient_email, c.avatar_url as sender_avatar, e.avatar_url as recipient_avatar`
    ` from t_quick_msg_log a`
    ` join t_user b`
    ` on a.sender_id = b.id`
    ` join t_user_detail c`
    ` on b.detail_id = c.id`
    ` join t_user d`
    ` on a.recipient_id = d.id`
    ` join t_user_detail e`
    ` on d.detail_id = e.id`
    ` where a.delete_flag = '0'`
    ` and b.delete_flag = '0'`
    ` and c.delete_flag = '0'`
    ` and d.delete_flag = '0'`
    ` and e.delete_flag = '0'`
    ` and a.msg_type = '1'`
    ` and a.reply_id = #{id}`
    ` order by a.create_time desc`
    "#
)]
pub async fn query_by_reply_id(
    tx: &mut RBatisTxExecutorGuard,
    id: &String,
) -> Result<Option<Vec<QueryQuickMsgVO>>, Error> {
    impled!();
}

/// 修改快捷消息日志已读状态
#[py_sql(
    r#"`update t_quick_msg_log`
    ` set`
    if dto.operate_time != '':
        ` operate_time = #{dto.operate_time}`
    if dto.operator != '':
        `, operator = #{dto.operator}`
    if dto.operator_id != '':
        `, operator_id = #{dto.operator_id}`
    `, read_flag = #{dto.read_flag}`
    ` where delete_flag = '0'` 
    ` and( 1 = 2`
    for id in dto.ids:
        ` or id = #{id}`
    `)`
    "#
)]
pub async fn update_read_flag(
    tx: &mut RBatisTxExecutorGuard,
    dto: &UpdateReadFlagDTO,
) -> Result<ExecResult, Error> {
    impled!();
}
