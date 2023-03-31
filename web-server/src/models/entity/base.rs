use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct BaseEntity {
    pub id: String,
    pub operate_time: DateTime,
    pub operator: String,
    pub operator_id: String,
    pub create_time: DateTime,
    pub creator: String,
    pub creator_id: String,
    pub delete_flag: String,
}
