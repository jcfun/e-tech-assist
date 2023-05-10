use crate::models::{
    dto::{
        base::BaseDTO,
        quick_msg::{CreateQuickMsgDTO, QueryQuickMsgDTO, UpdateReadFlagDTO},
    },
    vo::quick_msg::QueryQuickMsgVO,
};
use rbatis::{
    executor::RBatisTxExecutorGuard,
    py_sql,
    rbdc::{db::ExecResult, Error},
    Rbatis,
};

/// 新增快捷消息日志
#[py_sql(
    r#"`insert into t_quick_msg`
    ` (id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, sender_id, recipient_id, title, content, success_flag, send_method, description, msg_type, reply_id, read_flag, disable_flag)`
    ` values`
    ` (#{dto.id}, #{dto.operate_time}, #{dto.operator}, #{dto.operator_id}, #{dto.create_time}, #{dto.creator}, #{dto.creator_id}, #{dto.delete_flag}, #{dto.sender_id}, #{dto.recipient_identity}, #{dto.title}, #{dto.content}, #{dto.success_flag}, #{dto.send_method}, #{dto.description}, #{dto.msg_type}, #{dto.reply_id}, #{dto.read_flag}, '0')`
    "#
)]
pub async fn create_quick_msg(
    tx: &mut RBatisTxExecutorGuard,
    dto: &CreateQuickMsgDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 查询快捷消息日志
#[py_sql(
    r#"`select a.id, to_char(a.operate_time, 'YYYY-MM-DD HH24:MI:SS') as operate_time, a.operator, a.operator_id, to_char(a.create_time, 'YYYY-MM-DD HH24:MI:SS') as create_time, a.creator, a.creator_id, a.delete_flag, a.sender_id, a.recipient_id, a.title, a.content, a.success_flag, a.send_method, a.description, a.msg_type, a.reply_id, a.read_flag, a.disable_flag, c.email as sender_email, e.email as recipient_email, c.avatar_url as sender_avatar, e.avatar_url as recipient_avatar`
    ` from t_quick_msg a`
    ` join t_user b`
    ` on a.sender_id = b.id`
    ` join t_user_detail c`
    ` on b.detail_id = c.id`
    ` join t_user d`
    ` on a.recipient_id = d.id`
    ` join t_user_detail e`
    ` on d.detail_id = e.id`
    ` where a.delete_flag = '0'`
    ` and a.disable_flag = '0'`
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
pub async fn query_quick_msg(
    tx: &mut RBatisTxExecutorGuard,
    id: &String,
    page_size: usize,
    offset: usize,
) -> Result<Vec<QueryQuickMsgVO>, Error> {
    impled!();
}

/// 查询快捷消息日志数量
#[py_sql(
    r#"`select count(*)`
    ` from t_quick_msg a`
    ` join t_user b`
    ` on a.sender_id = b.id`
    ` join t_user_detail c`
    ` on b.detail_id = c.id`
    ` join t_user d`
    ` on a.recipient_id = d.id`
    ` join t_user_detail e`
    ` on d.detail_id = e.id`
    ` where a.delete_flag = '0'`
    ` and a.disable_flag = '0'`
    ` and b.delete_flag = '0'`
    ` and c.delete_flag = '0'`
    ` and d.delete_flag = '0'`
    ` and e.delete_flag = '0'`
    ` and a.msg_type = '0'`
    ` and (a.sender_id = #{id} or a.recipient_id = #{id})`
    "#
)]
pub async fn query_quick_msg_count(
    tx: &mut RBatisTxExecutorGuard,
    id: &String,
) -> Result<usize, Error> {
    impled!();
}

/// 根据回复ID查询快捷消息日志
#[py_sql(
    r#"`select a.id, to_char(a.operate_time, 'YYYY-MM-DD HH24:MI:SS') as operate_time, a.operator, a.operator_id, to_char(a.create_time, 'YYYY-MM-DD HH24:MI:SS') as create_time, a.creator, a.creator_id, a.delete_flag, a.sender_id, a.recipient_id, a.title, a.content, a.success_flag, a.send_method, a.description, a.msg_type, a.reply_id, a.read_flag, a.disable_flag, c.email as sender_email, e.email as recipient_email, c.avatar_url as sender_avatar, e.avatar_url as recipient_avatar`
    ` from t_quick_msg a`
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
    if disable_flag != '':
        ` and a.disable_flag = '0'`
    ` and a.msg_type = '1'`
    ` and a.reply_id = #{id}`
    ` order by a.create_time desc`
    "#
)]
pub async fn query_by_reply_id(
    tx: &mut RBatisTxExecutorGuard,
    id: &String,
    disable_flag: &str,
) -> Result<Option<Vec<QueryQuickMsgVO>>, Error> {
    impled!();
}

/// 修改快捷消息日志已读状态
#[py_sql(
    r#"`update t_quick_msg`
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

/// 修改快捷消息日志是否禁用
#[py_sql(
    r#"`update t_quick_msg`
    ` set`
    if dto.operate_time != '':
        ` operate_time = #{dto.operate_time}`
    if dto.operator != '':
        `, operator = #{dto.operator}`
    if dto.operator_id != '':
        `, operator_id = #{dto.operator_id}`
    if disable_flag != '':
        `, disable_flag = #{disable_flag}`
    ` where delete_flag = '0'`
    ` and id = #{dto.id}`
    "#
)]
pub async fn update_disable_flag(
    tx: &Rbatis,
    dto: &BaseDTO,
    disable_flag: &String,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 多条件模糊查询快捷消息日志
#[py_sql(
    r#"`select a.id, to_char(a.operate_time, 'YYYY-MM-DD HH24:MI:SS') as operate_time, a.operator, a.operator_id, to_char(a.create_time, 'YYYY-MM-DD HH24:MI:SS') as create_time, a.creator, a.creator_id, a.delete_flag, a.sender_id, a.recipient_id, a.title, a.content, a.success_flag, a.send_method, a.description, a.msg_type, a.reply_id, a.read_flag, a.disable_flag, c.email as sender_email, e.email as recipient_email, c.avatar_url as sender_avatar, e.avatar_url as recipient_avatar`
    ` from t_quick_msg a`
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
    if dto.create_time_start != '':
        ` and a.create_time >= #{dto.create_time_start}`
    if dto.create_time_end != '':
        ` and a.create_time <= #{dto.create_time_end}`
    if dto.sender != '':
        ` and (b.account like '%${dto.sender}%' or c.phone_number = '%${dto.sender}%' or c.email = '%${dto.sender}%')`
    if dto.recipient != '':
        ` and (d.account like '%${dto.recipient}%' or e.phone_number = '%${dto.recipient}%' or e.email = '%${dto.recipient}%')`
    if dto.title != '':
        ` and a.title like '%${dto.title}%'`
    if dto.send_method != '':
        ` and a.send_method = #{dto.send_method}`
    if dto.msg_type != '':
        ` and a.msg_type = #{dto.msg_type}`
    if dto.read_flag != '':
        ` and a.read_flag = #{dto.read_flag}`
    if dto.success_flag != '':
        ` and a.success_flag = #{dto.success_flag}`
    if dto.disable_flag != '':
        ` and a.disable_flag = #{dto.disable_flag}`
    ` order by a.create_time desc`
    ` limit #{page_size}`
    ` offset #{offset}`
    "#
)]
pub async fn query_quick_msgs_fq(
    tx: &mut RBatisTxExecutorGuard,
    dto: &QueryQuickMsgDTO,
    page_size: &usize,
    offset: &usize,
) -> Result<Vec<QueryQuickMsgVO>, Error> {
    impled!();
}

/// 多条件模糊查询快捷消息日志数量
#[py_sql(
    r#"`select count(*)`
    ` from t_quick_msg a`
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
    if dto.create_time_start != '':
        ` and a.create_time >= #{dto.create_time_start}`
    if dto.create_time_end != '':
        ` and a.create_time <= #{dto.create_time_end}`
    if dto.sender != '':
        ` and (b.account like '%${dto.sender}%' or c.phone_number = '%${dto.sender}%' or c.email = '%${dto.sender}%')`
    if dto.recipient != '':
        ` and (d.account like '%${dto.recipient}%' or e.phone_number = '%${dto.recipient}%' or e.email = '%${dto.recipient}%')`
    if dto.title != '':
        ` and a.title like '%${dto.title}%'`
    if dto.send_method != '':
        ` and a.send_method = #{dto.send_method}`
    if dto.msg_type != '':
        ` and a.msg_type = #{dto.msg_type}`
    if dto.read_flag != '':
        ` and a.read_flag = #{dto.read_flag}`
    if dto.success_flag != '':
        ` and a.success_flag = #{dto.success_flag}`
    if dto.disable_flag != '':
        ` and a.disable_flag = #{dto.disable_flag}`
    "#
)]
pub async fn query_quick_msgs_fq_count(
    tx: &mut RBatisTxExecutorGuard,
    dto: &QueryQuickMsgDTO,
) -> Result<usize, Error> {
    impled!();
}
