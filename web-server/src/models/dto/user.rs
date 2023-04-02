use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

use super::base::BaseDTO;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct CreateUserDTO {
    #[serde(flatten)]
    pub base_entity: Option<BaseDTO>,
    pub account: String,
    pub password: String,
    pub disable_flag: Option<String>,
    pub detail_id: Option<String>,
    pub role_id: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateUserDTO {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub disable_flag: Option<String>,
    pub operate_time: Option<DateTime>,
    pub operator: Option<String>,
    pub operator_id: Option<String>,
    pub create_time: Option<DateTime>,
    pub creator: Option<String>,
    pub creator_id: Option<String>,
    pub delete_flag: Option<String>,
    pub detail_id: Option<String>,
}
