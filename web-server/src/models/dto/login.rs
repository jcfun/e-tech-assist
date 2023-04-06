use serde::{Deserialize, Serialize};
use validator::Validate;

use super::base::BaseDTO;

/// 用户登录dto
#[derive(Debug, Deserialize, Validate, Clone)]
pub struct LoginDTO {
    #[validate(
        required(message = "账号不可为空"),
        length(min = 6, max = 18, message = "账号格式错误")
    )]
    pub account: Option<String>,
    #[validate(
        required(message = "密码不可为空"),
        length(min = 6, max = 18, message = "密码格式错误")
    )]
    pub password: Option<String>,
    #[validate(
        required(message = "验证码不可为空"),
        length(min = 4, max = 6, message = "验证码格式错误")
    )]
    pub captcha: Option<String>,

    #[validate(
        required(message = "uuid不可为空"),
        length(equal = 36, message = "uuid格式错误")
    )]
    pub uuid: Option<String>,
}

/// 用户注册dto
#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
#[serde(rename_all = "snake_case")]
pub struct RegisterDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    #[validate(
        required(message = "账号不可为空"),
        length(min = 6, max = 18, message = "账号格式错误")
    )]
    pub account: Option<String>,

    #[validate(
        required(message = "密码不可为空"),
        length(min = 6, max = 18, message = "密码格式错误")
    )]
    pub password: Option<String>,

    #[validate(required(message = "电话号不可为空"))]
    pub phone: Option<String>,

    #[validate(
        required(message = "验证码不可为空"),
        length(min = 4, max = 6, message = "验证码格式错误")
    )]
    pub captcha: Option<String>,

    #[validate(
        required(message = "uuid不可为空"),
        length(equal = 36, message = "uuid格式错误")
    )]
    pub uuid: Option<String>,

    #[serde(skip_deserializing)]
    #[validate(length(min = 1, max = 18, message = "昵称格式错误"))]
    pub nickname: Option<String>,

    #[serde(skip_deserializing)]
    #[validate(length(equal = 18, message = "用户详情id格式错误"))]
    pub detail_id: Option<String>,
}

/// 用户登录日志dto
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct LoginLogDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    pub account: Option<String>,

    pub status: Option<String>,

    pub description: Option<String>,

    pub user_agent: Option<String>,

    pub ip: Option<String>,

    pub ip_addr: Option<String>,

    pub mac: Option<String>,
}

/// 用户重置密码dto
#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ResetPwdDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    /// 用户标识(账号、手机号、邮箱)
    #[validate(
        required(message = "用户标识不可为空"),
        length(min = 6, max = 18, message = "用户标识格式错误")
    )]
    pub user_id: Option<String>,

    #[validate(
        required(message = "新密码不可为空"),
        length(min = 6, max = 18, message = "新密码格式错误")
    )]
    pub new_password: Option<String>,

    #[validate(required(message = "电话号不可为空"))]
    pub phone: Option<String>,

    #[validate(
        required(message = "验证码不可为空"),
        length(min = 4, max = 6, message = "验证码格式错误")
    )]
    pub captcha: Option<String>,

    #[validate(
        required(message = "uuid不可为空"),
        length(equal = 36, message = "uuid格式错误")
    )]
    pub uuid: Option<String>,
}
