use rbatis::{
    executor::RBatisTxExecutorGuard,
    py_sql,
    rbdc::{db::ExecResult, Error},
    Rbatis,
};
use rbs::to_value;

use crate::models::{dto::login::RegisterDTO, vo::login::LoginVO};

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
    "insert into t_user(id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, account, password, detail_id ) values(#{register_dto.id}, #{register_dto.operate_time}, #{register_dto.operator}, #{register_dto.operator_id}, #{register_dto.create_time}, #{register_dto.creator}, #{register_dto.creator_id}, #{register_dto.delete_flag}, #{register_dto.account}, #{register_dto.password}, #{register_dto.detail_id})"
)]
pub async fn create_user_info(
    tx: &mut RBatisTxExecutorGuard,
    register_dto: &RegisterDTO,
) -> Result<ExecResult, Error> {
    impled!();
}

/// 新增用户详情信息
#[py_sql(
    "insert into t_user_detail(id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, phone, nickname ) values(#{register_dto.id}, #{register_dto.operate_time}, #{register_dto.operator}, #{register_dto.operator_id}, #{register_dto.create_time}, #{register_dto.creator}, #{register_dto.creator_id}, #{register_dto.delete_flag}, #{register_dto.phone}, #{register_dto.nickname}) returning id"
)]
pub async fn create_user_detail(
    tx: &mut RBatisTxExecutorGuard,
    register_dto: &RegisterDTO,
) -> Result<String, Error> {
    impled!();
}
