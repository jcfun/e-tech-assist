use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
#[serde(rename_all = "snake_case")]
pub struct BaseDTO {
    /// 主键id
    #[validate(
        required(message = "id不可为空"),
        length(equal = 18, message = "id格式错误")
    )]
    #[serde(skip_deserializing)]
    pub id: Option<String>,

    /// 操作时间
    #[validate(required(message = "操作时间不可为空"))]
    #[serde(skip_deserializing)]
    pub operate_time: Option<DateTime>,

    /// 操作者
    #[validate(required(message = "操作者不可为空"))]
    #[serde(skip_deserializing)]
    pub operator: Option<String>,

    /// 操作者id
    #[validate(required(message = "操作者id不可为空"))]
    #[serde(skip_deserializing)]
    pub operator_id: Option<String>,

    /// 创建时间
    #[serde(skip_deserializing)]
    pub create_time: Option<DateTime>,

    /// 创建者
    #[serde(skip_deserializing)]
    pub creator: Option<String>,

    /// 创建者id
    #[serde(skip_deserializing)]
    pub creator_id: Option<String>,

    /// 删除标志
    #[serde(skip_deserializing)]
    pub delete_flag: Option<String>,
}
