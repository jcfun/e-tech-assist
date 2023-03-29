use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct LoginDTO {
    #[validate(regex(path = "ACCOUNT", message = "账号格式错误"))]
    pub account: String,
    pub password: String,
}

lazy_static! {
    static ref ACCOUNT: Regex = Regex::new(r"^[a-z]").unwrap();
}
