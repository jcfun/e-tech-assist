use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserVO {
    pub account: Option<String>,
    pub disable_flag: Option<String>,
    pub detail_id: Option<String>,
}
