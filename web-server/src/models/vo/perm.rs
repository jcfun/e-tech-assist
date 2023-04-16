use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct QueryPermVO {
    pub id: Option<String>,
    pub operate_time: Option<DateTime>,
    pub operator: Option<String>,
    pub operator_id: Option<String>,
    pub create_time: Option<DateTime>,
    pub creator: Option<String>,
    pub creator_id: Option<String>,
    pub delete_flag: Option<String>,
    pub name: Option<String>,
    pub parent_id: Option<String>,
    pub perm_type: Option<String>,
    pub disable_flag: Option<String>,
    pub api_path: Option<String>,
    pub fe_route: Option<String>,
    pub fe_code: Option<String>,
    pub description: Option<String>,
    pub resource: Option<String>,
}
