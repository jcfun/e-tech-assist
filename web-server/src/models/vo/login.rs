use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginVO {
    pub id: String,
    pub account: String,
}