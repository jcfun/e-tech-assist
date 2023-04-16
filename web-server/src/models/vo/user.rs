use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

use super::role::QueryRoleVO;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct QueryUserVO {
    pub id: Option<String>,
    pub operate_time: Option<DateTime>,
    pub operator: Option<String>,
    pub operator_id: Option<String>,
    pub create_time: Option<DateTime>,
    pub creator: Option<String>,
    pub creator_id: Option<String>,
    pub delete_flag: Option<String>,
    pub account: Option<String>,
    pub disable_flag: Option<String>,
    pub detail_id: Option<String>,
    pub description: Option<String>,
    pub openid: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub last_login_time: Option<DateTime>,
    pub last_login_ip: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub roles: Option<Vec<QueryRoleVO>>,
}
