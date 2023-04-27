use super::base::BaseDTO;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CreatePermDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    #[validate(
        required(message = "权限名称不可为空"),
        length(min = 2, max = 10, message = "权限名称格式错误")
    )]
    pub name: Option<String>,

    #[validate(length(min = 5, max = 20, message = "父权限id格式错误"))]
    pub parent_id: Option<String>,

    #[validate(
        required(message = "权限类型不可为空"),
        length(equal = 1, message = "权限类型格式错误")
    )]
    pub perm_type: Option<String>,

    #[validate(
        required(message = "是否禁用不可为空"),
        length(equal = 1, message = "是否禁用格式错误")
    )]
    pub disable_flag: Option<String>,

    #[validate(
        required(message = "接口路径不可为空"),
        length(max = 100, message = "接口路径格式错误")
    )]
    pub api_path: Option<String>,

    #[validate(
        required(message = "前端路由不可为空"),
        length(max = 100, message = "前端路由格式错误")
    )]
    pub fe_route: Option<String>,

    #[validate(
        required(message = "前端路由名称不可为空"),
        length(max = 100, message = "前端路由名称格式错误")
    )]
    pub fe_name: Option<String>,

    #[validate(
        required(message = "前端权限编号不可为空"),
        length(min = 5, max = 20, message = "前端权限编号格式错误")
    )]
    pub fe_code: Option<String>,

    #[validate(length(max = 100, message = "父路由路由格式错误"))]
    pub parent_route: Option<String>,

    #[validate(length(min = 1, max = 20, message = "权限资源(菜单icon)格式错误"))]
    pub resource: Option<String>,

    #[validate(length(equal = 1, message = "是否隐藏格式错误"))]
    pub hidden_flag: Option<String>,

    #[validate(length(max = 100, message = "权限描述格式错误"))]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UpdatePermDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    #[validate(length(min = 2, max = 10, message = "权限名称格式错误"))]
    pub name: Option<String>,

    #[validate(length(min = 5, max = 20, message = "父权限id格式错误"))]
    pub parent_id: Option<String>,

    #[validate(length(min = 1, max = 10, message = "权限类型格式错误"))]
    pub perm_type: Option<String>,

    #[validate(length(equal = 1, message = "是否禁用格式错误"))]
    pub disable_flag: Option<String>,

    #[validate(length(max = 100, message = "接口路径格式错误"))]
    pub api_path: Option<String>,

    #[validate(length(max = 100, message = "前端路由格式错误"))]
    pub fe_route: Option<String>,

    #[validate(length(max = 100, message = "前端路由名称格式错误"))]
    pub fe_name: Option<String>,

    #[validate(length(min = 5, max = 20, message = "前端权限编号格式错误"))]
    pub fe_code: Option<String>,

    #[validate(length(max = 100, message = "父路由路由格式错误"))]
    pub parent_route: Option<String>,

    #[validate(length(min = 1, max = 20, message = "权限资源(菜单icon)格式错误"))]
    pub resource: Option<String>,

    #[validate(length(min = 1, max = 10, message = "是否隐藏格式错误"))]
    pub hidden_flag: Option<String>,

    #[validate(length(max = 100, message = "权限描述格式错误"))]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct QueryPermDTO {
    pub create_time_start: Option<DateTime>,

    pub create_time_end: Option<DateTime>,

    #[validate(length(min = 2, max = 10, message = "权限名称格式错误"))]
    pub name: Option<String>,

    #[validate(length(min = 2, max = 10, message = "权限类型格式错误"))]
    pub perm_type: Option<String>,

    #[validate(length(equal = 1, message = "是否禁用格式错误"))]
    pub disable_flag: Option<String>,

    #[validate(length(equal = 1, message = "是否隐藏格式错误"))]
    pub hidden_flag: Option<String>,

    #[validate(range(min = 1, message = "页码最小为1"))]
    pub page_no: Option<u64>,

    #[validate(range(min = 1, message = "分页最小为1"))]
    pub page_size: Option<u64>,
}
