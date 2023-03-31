use rbatis::{
    executor::RBatisTxExecutor,
    py_sql,
    rbdc::{db::ExecResult, Error},
    Rbatis,
};
use rbs::to_value;

use crate::models::{dto::login::RegisterDTO, vo::login::LoginVO};

pub async fn user_login(
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

#[py_sql(
    "insert into t_user(id, operate_time, operator, operator_id, create_time, creator, creator_id, delete_flag, account, password ) values(#{register_dto.id}, #{register_dto.operate_time}, #{register_dto.operator}, #{register_dto.operator_id}, #{register_dto.create_time}, #{register_dto.creator}, #{register_dto.creator_id}, #{register_dto.delete_flag}, #{register_dto.account}, #{register_dto.password})"
)]
pub async fn user_register(
    tx: &mut RBatisTxExecutor,
    register_dto: RegisterDTO,
) -> Result<ExecResult, Error> {
    impled!();
}
