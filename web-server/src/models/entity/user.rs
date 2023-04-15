use serde::{Deserialize, Serialize};

use super::base::BaseEntity;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TUser {
    #[serde(flatten)]
    pub base_entity: Option<BaseEntity>,
    pub account: String,
    pub password: String,
    pub disable_flag: String,
    pub detail_id: String,
    pub description: String,
}
