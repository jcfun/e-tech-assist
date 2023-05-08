use super::base::BaseDTO;
use crate::utils::validate::id_vector;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CreateRoleDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    #[validate(
        required(message = "角色名称不可为空"),
        length(min = 2, max = 10, message = "角色名称格式错误")
    )]
    pub name: Option<String>,

    #[validate(
        required(message = "是否禁用不可为空"),
        length(equal = 1, message = "是否禁用格式错误")
    )]
    pub disable_flag: Option<String>,

    #[validate(length(max = 100, message = "角色描述格式错误"))]
    pub description: Option<String>,

    #[validate(
        required(message = "角色编号不可为空"),
        length(min = 5, max = 20, message = "角色编号格式错误")
    )]
    pub code: Option<String>,

    #[validate(custom(function = "id_vector", message = "权限id格式错误"))]
    pub perm_ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UpdateRoleDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    #[validate(length(min = 2, max = 10, message = "角色名称格式错误"))]
    pub name: Option<String>,

    #[validate(length(equal = 1, message = "是否禁用格式错误"))]
    pub disable_flag: Option<String>,

    #[validate(length(max = 100, message = "角色描述格式错误"))]
    pub description: Option<String>,

    #[validate(length(min = 5, max = 20, message = "角色编号格式错误"))]
    pub code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UpdateRolePermDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    #[validate(custom(function = "id_vector", message = "权限id格式错误"))]
    pub perm_ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct QueryRoleDTO {
    pub create_time_start: Option<DateTime>,

    pub create_time_end: Option<DateTime>,

    #[validate(length(min = 2, max = 10, message = "角色名称格式错误"))]
    pub name: Option<String>,

    #[validate(length(equal = 1, message = "是否禁用格式错误"))]
    pub disable_flag: Option<String>,

    #[validate(range(min = 1, message = "页码最小为1"))]
    pub page_no: Option<usize>,

    #[validate(range(min = 1, message = "分页最小为1"))]
    pub page_size: Option<usize>,
}
