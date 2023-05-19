use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq)]
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
    pub route: Option<String>,
    pub route_name: Option<String>,
    pub code: Option<String>,
    pub description: Option<String>,
    pub resource: Option<String>,
    pub hidden_flag: Option<String>,
    pub parent_route: Option<String>,
    pub children: Option<Vec<QueryPermVO>>,
}

impl std::cmp::PartialEq for QueryPermVO {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::hash::Hash for QueryPermVO {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
