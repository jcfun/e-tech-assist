use rbatis::{rbdc::Error, Rbatis};
use rbs::to_value;

use crate::{models::vo::user::UserVO};

/// 根据用户id查询用户信息
pub async fn select_user_by_id(rb: &Rbatis, id: String) -> Result<Option<UserVO>, Error> {
    rb.query_decode("select * from t_user where id = ?", vec![to_value!(id)])
        .await
}

/// 根据用户账号查询用户数量
pub async fn select_user_count_by_account(rb: &Rbatis, account: &String) -> Result<u64, Error> {
    rb.query_decode("select count(*) from t_user where account = ?", vec![to_value!(account)])
        .await
}