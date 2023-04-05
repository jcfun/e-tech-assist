use rbatis::{
    executor::RBatisTxExecutorGuard,
    py_sql,
    rbdc::{db::ExecResult, Error},
    Rbatis,
};
use rbs::to_value;

use crate::models::{
    dto::login::{LoginLogDTO, RegisterDTO, ResetPwdDTO},
    vo::login::LoginVO,
};

/// 根据用户账号密码查询用户信息
pub async fn get_user_info(
    rb: &Rbatis,
    account: Option<String>,
    password: Option<String>,
) -> Result<Option<LoginVO>, Error> {
    rb.query_decode(
        "select * from t_user where account = ? and password = ?",
        vec![to_value!(account), to_value!(password)],
    )
    .await
}

/// 新增用户信息
#[py_sql(
    "`insert into t_user(id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, account, password, detail_id) `
    values
    (
        #{register_dto.id}, #{register_dto.operate_time}, #{register_dto.operator}, #{register_dto.operator_id}, #{register_dto.create_time}, #{register_dto.creator}, #{register_dto.creator_id}, #{register_dto.delete_flag}, #{register_dto.account}, #{register_dto.password}, #{register_dto.detail_id}
    )"
)]
pub async fn create_user_info(
    tx: &mut RBatisTxExecutorGuard,
    register_dto: &RegisterDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 新增用户详情信息
#[py_sql(
    "`insert into t_user_detail(id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, phone, nickname) `
    values
        (
            #{register_dto.id}, #{register_dto.operate_time}, #{register_dto.operator}, #{register_dto.operator_id}, #{register_dto.create_time}, #{register_dto.creator}, #{register_dto.creator_id}, #{register_dto.delete_flag}, #{register_dto.phone}, #{register_dto.nickname}
        ) returning id"
)]
pub async fn create_user_detail(
    tx: &mut RBatisTxExecutorGuard,
    register_dto: &RegisterDTO,
) -> Result<String, Error> {
    impled!();
}

/// 新增登录日志
#[py_sql(
    "`insert into t_login_log(id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, account, status, description, user_agent, ip, ip_addr, mac ) `
    `values`
        (
            `#{login_log_dto.id}, #{login_log_dto.operate_time}, #{login_log_dto.operator}, #{login_log_dto.operator_id}, #{login_log_dto.create_time}, #{login_log_dto.creator}, #{login_log_dto.creator_id}, #{login_log_dto.delete_flag}, #{login_log_dto.account}, #{login_log_dto.status}, #{login_log_dto.description}, #{login_log_dto.user_agent}, #{login_log_dto.ip}, #{login_log_dto.ip_addr}, #{login_log_dto.mac}`
        )"
)]
pub async fn create_login_log(
    rb: &Rbatis,
    login_log_dto: &LoginLogDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 重置用户密码
#[py_sql(
    "`update t_user a `
    `set `
        `operate_time = #{dto.operate_time}, `
        `operator = #{dto.operator}, `
        `operator_id = #{dto.operator_id}, `
        `password = #{dto.new_password} `
    `from `
        `t_user_detail b `
    `where `
        `a.detail_id = b.id `
        `and (a.account = #{dto.user_id} `
        `or b.phone = #{dto.user_id} `
        `or b.email = #{dto.user_id}) `
        `and b.phone = #{dto.user_id}`"
)]
pub async fn reset_pwd(rb: &Rbatis, dto: &ResetPwdDTO) -> Result<ExecResult, Error> {
    impled!();
}
