use rbatis::{
    executor::RBatisTxExecutorGuard,
    py_sql,
    rbdc::{db::ExecResult, Error},
    Rbatis,
};
use rbs::to_value;

use crate::models::{
    dto::login::{LoginLogDTO, RegisterDTO, ResetPwdDTO},
    entity::user_detail::TUserDetail,
    vo::login::UserInfoVO,
};

/// 根据用户账号密码查询用户信息
#[py_sql(
    r#"select a.id, a.operate_time, a.operator, a.operator_id, a.create_time, a.creator, a.creator_id, a.delete_flag, a.account, a.disable_flag, a.detail_id, a.description, a.openid, b.phone_number, b.email, b.nickname, b.avatar_url, b.last_login_time, b.last_login_ip, b.gender, b.language, b.country, b.province, b.city from t_user a join t_user_detail b on a.detail_id = b.id where (a.account = #{identity} or b.email = #{identity} or b.phone_number = #{identity} ) and a.password = #{password} and a.delete_flag = '0' and a.disable_flag = '0'"#,
)]
pub async fn get_user_info(
    tx: &mut RBatisTxExecutorGuard,
    identity: Option<String>,
    password: Option<String>,
) -> Result<Option<UserInfoVO>, Error> {
    impled!();
}

/// 新增用户信息
#[py_sql(
    r#"`insert into t_user(id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, account, password, detail_id, openid) `
    values
    (
        #{register_dto.id}, #{register_dto.operate_time}, #{register_dto.operator}, #{register_dto.operator_id}, #{register_dto.create_time}, #{register_dto.creator}, #{register_dto.creator_id}, #{register_dto.delete_flag}, #{register_dto.account}, #{register_dto.password}, #{register_dto.detail_id}, #{register_dto.openid}
    )"#
)]
pub async fn create_user_info(
    tx: &mut RBatisTxExecutorGuard,
    register_dto: &RegisterDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 新增用户详情信息
#[py_sql(
    r#"`insert into t_user_detail(id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, phone_number, nickname) `
    values
        (
            #{register_dto.id}, #{register_dto.operate_time}, #{register_dto.operator}, #{register_dto.operator_id}, #{register_dto.create_time}, #{register_dto.creator}, #{register_dto.creator_id}, #{register_dto.delete_flag}, #{register_dto.phone_number}, #{register_dto.nickname}
        ) returning id"#
)]
pub async fn create_user_detail(
    tx: &mut RBatisTxExecutorGuard,
    register_dto: &RegisterDTO,
) -> Result<String, Error> {
    impled!();
}

/// 新增登录日志
#[py_sql(
    r#"`insert into t_login_log(id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, identity, status, description, user_agent, ip, ip_addr, mac, method ) `
    `values`
        (
            `#{login_log_dto.id}, #{login_log_dto.operate_time}, #{login_log_dto.operator}, #{login_log_dto.operator_id}, #{login_log_dto.create_time}, #{login_log_dto.creator}, #{login_log_dto.creator_id}, #{login_log_dto.delete_flag}, #{login_log_dto.identity}, #{login_log_dto.status}, #{login_log_dto.description}, #{login_log_dto.user_agent}, #{login_log_dto.ip}, #{login_log_dto.ip_addr}, #{login_log_dto.mac}, #{login_log_dto.method}`
        )"#
)]
pub async fn create_login_log(
    rb: &Rbatis,
    login_log_dto: &LoginLogDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 重置用户密码
#[py_sql(
    r#"`update t_user a`
    ` set`
        ` operate_time = #{dto.operate_time},`
        ` operator = #{dto.operator},`
        ` operator_id = #{dto.operator_id},`
        ` password = #{dto.new_password}`
    ` from`
        ` t_user_detail b`
    ` where`
        ` a.delete_flag = '0'`
        ` and a.disable_flag = '0'`
        ` and a.detail_id = b.id`
        ` and (a.account = #{dto.identity}`
        ` or b.phone_number = #{dto.identity}`
        ` or b.email = #{dto.identity})`
        ` and b.phone_number = #{dto.phone_number}`"#
)]
pub async fn reset_pwd(rb: &Rbatis, dto: &ResetPwdDTO) -> Result<ExecResult, Error> {
    impled!();
}

// 通过open_id查询用户信息(微信授权登录)
pub async fn get_user_info_by_openid(
    rb: &Rbatis,
    openid: &String,
) -> Result<Option<UserInfoVO>, Error> {
    rb.query_decode(
        r#"select a.id, a.operate_time, a.operator, a.operator_id, a.create_time, a.creator, a.creator_id, a.delete_flag, a.account, a.disable_flag, a.detail_id, a.description, a.openid, b.phone_number, b.email, b.nickname, b.avatar_url, b.last_login_time, b.last_login_ip, b.gender, b.language, b.country, b.province, b.city from t_user a join t_user_detail b on a.detail_id = b.id where a.delete_flag = '0' and a.disable_flag = '0' and a.openid = ?"#,
        vec![to_value!(openid)],
    )
    .await
}

// 通过手机号/用户账号查询用户数量
#[py_sql(
    r#"select count(*) from t_user a join t_user_detail b on a.detail_id = b.id where a.delete_flag = '0' and a.disable_flag = '0' and (a.account = #{identity} or b.phone_number = #{identity})"#
)]
pub async fn get_user_count(rb: &Rbatis, identity: &String) -> Result<usize, Error> {
    impled!();
}

/// 新增微信注册用户详情信息
#[py_sql(
    r#"`insert into t_user_detail(id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, phone_number, nickname, avatar_url, gender, language, country, province, city) `
    values
        (
            #{detail.id}, #{detail.operate_time}, #{detail.operator}, #{detail.operator_id}, #{detail.create_time}, #{detail.creator}, #{detail.creator_id}, #{detail.delete_flag}, #{detail.phone_number}, #{detail.nickname}, #{detail.avatar_url}, #{detail.gender}, #{detail.language}, #{detail.country}, #{detail.province}, #{detail.city}
        ) returning id"#
)]
pub async fn create_user_detail_wxapp(
    tx: &mut RBatisTxExecutorGuard,
    detail: &TUserDetail,
) -> Result<String, Error> {
    impled!();
}

// 通过手机号更新用户信息第一步(微信小程序授权时登录更新用户信息)
#[py_sql(
    r#"update t_user a set openid = #{info.openid} from t_user_detail b where a.delete_flag = '0' and a.detail_id = b.id and b.phone_number = #{detail.phone_number}"#
)]
pub async fn update_user_by_phone1(
    tx: &mut RBatisTxExecutorGuard,
    detail: &TUserDetail,
    info: &RegisterDTO,
) -> Result<ExecResult, Error> {
    impled!();
}
// 通过手机号更新用户信息第二步(微信小程序授权时登录更新用户信息)
#[py_sql(
    r#"update t_user_detail set avatar_url = #{detail.avatar_url}, gender = #{detail.gender}, language = #{detail.language}  where a.delete_flag = '0' and phone_number = #{detail.phone_number}"#
)]
pub async fn update_user_by_phone2(
    tx: &mut RBatisTxExecutorGuard,
    detail: &TUserDetail,
) -> Result<ExecResult, Error> {
    impled!();
}
