use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

use crate::utils::jwt::Token;

use super::perm::QueryPermVO;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct UserInfoVO {
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
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub last_login_time: Option<DateTime>,
    pub last_login_ip: Option<String>,
    pub gender: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    pub openid: Option<String>,
    pub session_key: Option<String>,
    pub perms: Option<Vec<QueryPermVO>>,
    pub perm_codes: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct LoginVO {
    pub user_info: UserInfoVO,
    pub token: Token,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct QueryLoginLogVO {
    pub id: Option<String>,
    pub operate_time: Option<DateTime>,
    pub operator: Option<String>,
    pub operator_id: Option<String>,
    pub create_time: Option<DateTime>,
    pub creator: Option<String>,
    pub creator_id: Option<String>,
    pub delete_flag: Option<String>,
    pub identity: Option<String>,
    pub account: Option<String>,
    pub success_flag: Option<String>,
    pub description: Option<String>,
    pub user_agent: Option<String>,
    pub ip: Option<String>,
    pub location: Option<String>,
    pub mac: Option<String>,
    pub method: Option<String>,
}
