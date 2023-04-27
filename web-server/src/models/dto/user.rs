use super::base::BaseDTO;
use crate::utils::validate::id_vector;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CreateUserDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    #[validate(
        required(message = "账号不可为空"),
        length(min = 1, max = 18, message = "账号格式错误")
    )]
    pub account: Option<String>,

    #[validate(length(min = 6, max = 18, message = "密码格式错误"))]
    pub password: Option<String>,

    #[validate(
        required(message = "是否禁用不可为空"),
        length(equal = 1, message = "是否禁用格式错误")
    )]
    pub disable_flag: Option<String>,

    #[validate(length(equal = 18, message = "用户详情id格式错误"))]
    #[serde(skip_deserializing)]
    pub detail_id: Option<String>,

    #[validate(custom(function = "id_vector", message = "角色id格式错误"))]
    pub role_ids: Option<Vec<String>>,

    #[validate(length(max = 100, message = "用户描述格式错误"))]
    pub description: Option<String>,

    #[validate(
        required(message = "用户手机号不可为空"),
        length(max = 11, message = "用户手机号格式错误")
    )]
    pub phone_number: Option<String>,

    #[validate(length(min = 1, max = 18, message = "昵称格式错误"))]
    pub nickname: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UpdateUserDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    #[validate(length(min = 6, max = 18, message = "密码格式错误"))]
    pub password: Option<String>,

    #[validate(custom(function = "id_vector", message = "角色id格式错误"))]
    pub role_ids: Option<Vec<String>>,

    #[validate(length(max = 11, message = "用户手机号格式错误"))]
    pub phone_number: Option<String>,

    #[validate(email(message = "邮箱格式错误"))]
    pub email: Option<String>,

    #[validate(length(min = 1, max = 18, message = "昵称格式错误"))]
    pub nickname: Option<String>,
    pub gender: Option<String>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
    #[validate(length(min = 1, max = 50, message = "角色描述格式错误"))]
    pub description: Option<String>,
}

// 微信小程序用户信息修改结构体
#[derive(Debug, Deserialize, Serialize, Clone, Default, Validate)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UpdateUserWxDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    #[validate(length(min = 1, max = 18, message = "昵称格式错误"))]
    pub nickname: Option<String>,

    pub email: Option<String>,

    #[validate(length(equal = 11, message = "手机号格式错误"))]
    pub phone_number: Option<String>,
    pub gender: Option<String>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub city: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, Validate)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct QueryUserDTO {
    pub create_time_start: Option<DateTime>,

    pub create_time_end: Option<DateTime>,

    #[validate(length(equal = 18, message = "id格式错误"))]
    pub id: Option<String>,

    #[validate(length(min = 1, max = 18, message = "昵称格式错误"))]
    pub nickname: Option<String>,

    pub email: Option<String>,

    #[validate(length(max = 11, message = "手机号格式错误"))]
    pub phone_number: Option<String>,

    #[validate(length(equal = 1, message = "性别格式错误"))]
    pub gender: Option<String>,

    #[validate(length(equal = 1, message = "禁用标识格式错误"))]
    pub disable_flag: Option<String>,

    #[validate(range(min = 1, message = "页码最小为1"))]
    pub page_no: Option<u64>,

    #[validate(range(min = 1, message = "分页最小为1"))]
    pub page_size: Option<u64>,
}
