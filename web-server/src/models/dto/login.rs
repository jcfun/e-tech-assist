use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDTO {
    pub account: String,
    pub password: String,
}