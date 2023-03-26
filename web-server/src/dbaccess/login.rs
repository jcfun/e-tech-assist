use rbatis::{rbdc::Error, Rbatis};
use rbs::to_value;

use crate::models::vo::login::LoginVO;


pub async fn user_login(rb: &mut Rbatis, account: String, password: String) -> Result<Option<LoginVO>, Error> {
    rb.query_decode("select * from t_user where account = ? and password = ?", vec![to_value!(account), to_value!(password)])
        .await
}