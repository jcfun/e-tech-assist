use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

use super::base::BaseEntity;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct TUserDetail {
    #[serde(flatten)]
    pub base_entity: BaseEntity,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub last_login_time: Option<DateTime>,
    pub last_login_ip: Option<String>,
    pub gender: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
}