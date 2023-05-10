use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::base::BaseDTO;

/// 用户登录dto
#[derive(Debug, Deserialize, Validate, Clone, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct LoginDTO {
    #[validate(
        required(message = "用户标识不可为空"),
        length(min = 1, max = 50, message = "用户标识格式错误")
    )]
    pub identity: Option<String>,
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

    // 登录方式(前台、后台、小程序)
    #[serde(skip_deserializing)]
    pub method: Option<String>,
}

/// 用户注册dto
#[derive(Debug, Deserialize, Serialize, Validate, Clone, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct RegisterDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    #[validate(
        required(message = "用户账号不可为空"),
        length(min = 1, max = 50, message = "用户账号格式错误")
    )]
    pub account: Option<String>,

    #[validate(
        required(message = "密码不可为空"),
        length(min = 6, max = 18, message = "密码格式错误")
    )]
    pub password: Option<String>,

    #[validate(required(message = "电话号不可为空"))]
    pub phone_number: Option<String>,

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
    #[validate(length(min = 1, max = 50, message = "昵称格式错误"))]
    pub nickname: Option<String>,

    #[serde(skip_deserializing)]
    #[validate(length(equal = 18, message = "用户详情id格式错误"))]
    pub detail_id: Option<String>,

    #[serde(skip_deserializing)]
    pub openid: Option<String>,
}

/// 用户登录日志dto
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct LoginLogDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    pub identity: Option<String>,

    pub success_flag: Option<String>,

    pub description: Option<String>,

    pub user_agent: Option<String>,

    pub ip: Option<String>,

    pub location: Option<String>,

    pub mac: Option<String>,
    pub method: Option<String>,
}

/// 用户重置密码dto
#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ResetPwdDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    /// 用户标识(账号、手机号、邮箱)
    #[validate(
        required(message = "用户标识不可为空"),
        length(min = 1, max = 50, message = "用户标识格式错误")
    )]
    pub identity: Option<String>,

    #[validate(
        required(message = "新密码不可为空"),
        length(min = 6, max = 18, message = "新密码格式错误")
    )]
    pub new_password: Option<String>,

    #[validate(
        required(message = "电话号不可为空"),
        length(equal = 11, message = "电话号格式错误")
    )]
    pub phone_number: Option<String>,

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

/// 微信授权登录dto
#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AuthDTO {
    // 微信授权登录code
    #[validate(required(message = "code不可为空"))]
    pub code: Option<String>,
}

/// 微信授权注册dto
#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct AuthRegisterDTO {
    #[validate(required(message = "session_key不可为空"))]
    pub session_key: Option<String>,
    #[validate(required(message = "openid不可为空"))]
    pub openid: Option<String>,
    #[validate(required(message = "iv不可为空"))]
    pub iv: Option<String>,
    #[validate(required(message = "encrypted_data不可为空"))]
    pub encrypted_data: Option<String>,
    #[validate(required(message = "电话号不可为空"))]
    pub phone_number: Option<String>,
}

/// 多条件查询登录日志dto
#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct QueryLoginLogDTO {
    pub create_time_start: Option<DateTime>,

    pub create_time_end: Option<DateTime>,

    #[validate(length(min = 1, max = 50, message = "登录用户格式错误"))]
    pub identity: Option<String>,

    #[validate(length(equal = 1, message = "是否成功格式错误"))]
    pub success_flag: Option<String>,

    #[validate(length(equal = 1, message = "登录方式格式错误"))]
    pub method: Option<String>,

    #[validate(range(min = 1, message = "页码最小为1"))]
    pub page_no: Option<usize>,

    #[validate(range(min = 1, message = "分页最小为1"))]
    pub page_size: Option<usize>,

    #[serde(skip_deserializing)]
    pub account: Option<String>,

    #[serde(skip_deserializing)]
    pub phone_number: Option<String>,

    #[serde(skip_deserializing)]
    pub email: Option<String>,

    #[serde(skip_deserializing)]
    pub openid: Option<String>,
}
