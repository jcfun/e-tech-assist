use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginVO {
    pub id: Option<String>,
    pub account: Option<String>,
    pub nickname: Option<String>,
}