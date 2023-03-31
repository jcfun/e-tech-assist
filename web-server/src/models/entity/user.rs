use serde::{Serialize, Deserialize};

use super::base::BaseEntity;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TUser {
    pub account: String,
    pub password: String,
    pub disable_flag: String,
    pub detail_id: String,
    #[serde(flatten)]
    pub base_entity: Option<BaseEntity>,
}