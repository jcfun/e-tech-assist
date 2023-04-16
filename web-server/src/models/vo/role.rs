use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

use super::perm::QueryPermVO;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct QueryRoleVO {
    pub id: Option<String>,
    pub operate_time: Option<DateTime>,
    pub operator: Option<String>,
    pub operator_id: Option<String>,
    pub create_time: Option<DateTime>,
    pub creator: Option<String>,
    pub creator_id: Option<String>,
    pub delete_flag: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub disable_flag: Option<String>,
    pub code: Option<String>,
    pub perms: Option<Vec<QueryPermVO>>,
}
