use rbatis::{
    executor::RBatisTxExecutorGuard,
    py_sql,
    rbdc::{db::ExecResult, Error},
    Rbatis,
};

use crate::{
    models::{
        dto::{
            base::BaseDTO,
            user::{CreateUserDTO, QueryUserDTO, UpdateUserDTO, UpdateUserWxDTO},
        },
        vo::user::QueryUserVO,
    },
    utils::epc,
};

/// 更新用户详情信息
#[py_sql(
    r#"`update t_user_detail a`
    ` set`
    trim ',':
        if dto.operate_time != '':
            ` operate_time = #{dto.operate_time}`
        if dto.operator != '':
            `, operator = #{dto.operator}`
        if dto.operator_id != '':
            `, operator_id = #{dto.operator_id}`
        if dto.nickname != '':
            `, nickname = #{dto.nickname}`
        if dto.email != '':
            `, email = #{dto.email}`
        if dto.phone_number != '':
            `, phone_number = #{dto.phone_number}`
        if dto.gender != '':
            `, gender = #{dto.gender}`
        if dto.country != '':
            `, country = #{dto.country}`
        if dto.province != '':
            `, province = #{dto.province}`
        if dto.city != '':
            `, city = #{dto.city}`
        ` from t_user b`
        ` where a.delete_flag = '0'` 
        ` and b.delete_flag = '0'`
        ` and b.disable_flag = '0'`
        ` and b.detail_id = a.id`
        ` and b.id = #{dto.id}`"#
)]
pub async fn update_user_detail(
    tx: &mut RBatisTxExecutorGuard,
    dto: &UpdateUserWxDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 新增用户详情信息
#[py_sql(
    r#"`insert into t_user_detail`
    ` (id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, phone_number, nickname)`
    ` values`
    ` (#{dto.id}, #{dto.operate_time}, #{dto.operator}, #{dto.operator_id}, #{dto.create_time}, #{dto.creator}, #{dto.creator_id}, #{dto.delete_flag}, #{dto.phone_number}, #{dto.nickname}) returning id`"#
)]
pub async fn create_user_detail(
    tx: &mut RBatisTxExecutorGuard,
    dto: &CreateUserDTO,
) -> Result<String, Error> {
    impled!();
}

/// 新增用户信息
#[py_sql(
    r#"`insert into t_user`
    ` (id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, account, password, disable_flag, detail_id, description)`
    ` values`
    ` (#{dto.id}, #{dto.operate_time}, #{dto.operator}, #{dto.operator_id}, #{dto.create_time}, #{dto.creator}, #{dto.creator_id}, #{dto.delete_flag}, #{dto.account}, #{dto.password}, #{dto.disable_flag}, #{dto.detail_id}, #{dto.description})`"#
)]
pub async fn create_user(
    tx: &mut RBatisTxExecutorGuard,
    dto: &CreateUserDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 新增用户角色关联信息
#[py_sql(
    r#"`insert into t_user_role`
    ` (id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, user_id, role_id)`
    ` values`
    trim ',': for role_id in dto.role_ids: 
        `, (`
            #{epc::get_snowflake()}, #{dto.operate_time}, #{dto.operator}, #{dto.operator_id}, #{dto.create_time}, #{dto.creator}, #{dto.creator_id}, #{dto.delete_flag}, #{dto.id}, #{role_id}
        ` )`"#
)]
pub async fn create_user_role(
    tx: &mut RBatisTxExecutorGuard,
    dto: &CreateUserDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 删除用户详情信息
#[py_sql(
    r#"`update t_user_detail a`
    ` set`
    ` operate_time = #{dto.operate_time}`
    `, operator = #{dto.operator}`
    `, operator_id = #{dto.operator_id}`
    `, delete_flag = '1'`
    ` from t_user b`
    ` where b.detail_id = a.id`
    ` and b.id = #{dto.id}`"#
)]
pub async fn delete_user_detail(
    tx: &mut RBatisTxExecutorGuard,
    dto: &BaseDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 删除用户信息
#[py_sql(
    r#"`update t_user`
    ` set`
    ` operate_time = #{dto.operate_time}`
    `, operator = #{dto.operator}`
    `, operator_id = #{dto.operator_id}`
    `, delete_flag = '1'`
    ` where id = #{dto.id}`"#
)]
pub async fn delete_user(
    tx: &mut RBatisTxExecutorGuard,
    dto: &BaseDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 更新用户信息
#[py_sql(
    r#"`update t_user`
    ` set`
    if dto.operate_time != '':
        ` operate_time = #{dto.operate_time}`
    if dto.operator != '':
        `, operator = #{dto.operator}`
    if dto.operator_id != '':
        `, operator_id = #{dto.operator_id}`
    if dto.password != '':
        `, password = #{dto.password}`
    ` where delete_flag = '0'` 
    ` and disable_flag = '0'`
    ` and id = #{dto.id}`"#
)]
pub async fn update_user(
    tx: &mut RBatisTxExecutorGuard,
    dto: &UpdateUserDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 删除用户角色关联信息
#[py_sql(
    r#"`update t_user_role`
    ` set`
    ` operate_time = #{dto.operate_time}`
    `, operator = #{dto.operator}`
    `, operator_id = #{dto.operator_id}`
    `, delete_flag = '1'`
    ` where user_id = #{dto.id}`"#
)]
pub async fn delete_user_role(
    tx: &mut RBatisTxExecutorGuard,
    dto: &BaseDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 分页查询用户信息
#[py_sql(
    r#"`select a.id, a.operate_time, a.operator, a.operator_id, a.create_time, a.creator, a.creator_id, a.delete_flag, a.account, a.disable_flag, a.detail_id, a.description, a.openid, b.phone_number, b.email, b.nickname, b.avatar_url, b.last_login_time, b.last_login_ip, b.language, b.country, b.province, b.city`
    ` from t_user a join t_user_detail b on a.detail_id = b.id`
    ` where a.delete_flag = '0'` 
    ` and b.delete_flag = '0'`
    if dto.create_time_start != '':
        ` and a.create_time >= #{dto.create_time_start}`
    if dto.create_time_end != '':
        ` and a.create_time <= #{dto.create_time_end}`
    if dto.nickname != '':
        ` and b.nickname like '%${dto.nickname}%'`
    if dto.email != '':
        ` and b.email like '%${dto.email}%'`
    if dto.phone_number != '':
        ` and b.phone_number like '%${dto.phone_number}%'`
    if dto.gender != '':
        ` and b.gender = #{dto.gender}`
    if dto.id != '':
        ` and a.id = #{dto.id}`
    ` limit ${page_size}`
    ` offset ${offset}`"#
)]
pub async fn query_user(
    tx: &mut RBatisTxExecutorGuard,
    dto: &QueryUserDTO,
    page_size: &u64,
    offset: &u64,
) -> Result<Option<Vec<QueryUserVO>>, Error> {
    impled!();
}

/// 分页查询用户信息
#[py_sql(
    r#"`select count(*)`
    ` from t_user a join t_user_detail b on a.detail_id = b.id`
    ` where a.delete_flag = '0'` 
    ` and b.delete_flag = '0'`
    if dto.create_time_start != '':
        ` and a.create_time >= #{dto.create_time_start}`
    if dto.create_time_end != '':
        ` and a.create_time <= #{dto.create_time_end}`
    if dto.nickname != '':
        ` and b.nickname like '%${dto.nickname}%'`
    if dto.email != '':
        ` and b.email like '%${dto.email}%'`
    if dto.phone_number != '':
        ` and b.phone_number like '%${dto.phone_number}%'`
    if dto.gender != '':
        ` and b.gender = #{dto.gender}`
    if dto.id != '':
        ` and a.id = #{dto.id}`"#
)]
pub async fn query_user_count(
    tx: &mut RBatisTxExecutorGuard,
    dto: &QueryUserDTO,
) -> Result<u64, Error> {
    impled!();
}

/// 更新用户状态(是否禁用)
#[py_sql(
    r#"`update t_user`
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
    ` and id = #{dto.id}`"#
)]
pub async fn update_disable_flag(
    db: &Rbatis,
    dto: &BaseDTO,
    disable_flag: &String,
) -> Result<ExecResult, Error> {
    impled!();
}
