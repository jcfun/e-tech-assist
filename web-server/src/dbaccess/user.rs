use rbatis::{rbdc::Error, Rbatis};
use rbs::to_value;

use crate::{models::vo::user::UserVo};

pub async fn select_user_by_id(rb: &mut Rbatis, id: String) -> Result<Option<UserVo>, Error> {
    rb.query_decode("select * from t_user where id = ?", vec![to_value!(id)])
        .await
}