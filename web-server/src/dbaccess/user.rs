use rbatis::{rbdc::Error, Rbatis};
use rbs::to_value;

use crate::{models::vo::user::UserVO};

/// 根据用户id查询用户信息
pub async fn select_user_by_id(rb: &Rbatis, id: String) -> Result<Option<UserVO>, Error> {
    rb.query_decode("select * from t_user where id = ?", vec![to_value!(id)])
        .await
}