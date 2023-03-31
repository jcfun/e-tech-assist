use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::base::BaseDTO;

#[derive(Debug, Deserialize, Validate, Clone)]
pub struct LoginDTO {
    #[validate(required(message  = "账号不可为空"), length(min = 6, max = 18, message = "账号格式错误"))]
    pub account: Option<String>,
    #[validate(required(message  = "密码不可为空"), length(min = 6, max = 18, message = "密码格式错误"))]
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
#[serde(rename_all = "snake_case")]
pub struct RegisterDTO {
    #[validate(required(message  = "账号不可为空"), length(min = 6, max = 18, message = "账号格式错误"))]
    pub account: Option<String>,
    #[validate(required(message  = "密码不可为空"), length(min = 6, max = 18, message = "密码格式错误"))]
    pub password: Option<String>,
    #[serde(flatten)]
    pub base_dto: BaseDTO,
}

lazy_static! {
    static ref ACCOUNT: Regex = Regex::new(r"^[a-z]").unwrap();
}
