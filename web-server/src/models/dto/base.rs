use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct BaseDTO {
    /// 主键id
    #[validate(
        // required(message = "id不可为空"),
        length(equal = 18, message = "id格式错误")
    )]
    pub id: Option<String>,

    /// 操作时间
    // #[validate(required(message = "操作时间不可为空"))]
    pub operate_time: Option<DateTime>,

    /// 操作者
    // #[validate(required(message = "操作者不可为空"))]
    pub operator: Option<String>,

    /// 操作者id
    // #[validate(required(message = "操作者id不可为空"))]
    pub operator_id: Option<String>,

    /// 创建时间
    pub create_time: Option<DateTime>,

    /// 创建者
    pub creator: Option<String>,

    /// 创建者id
    pub creator_id: Option<String>,

    /// 删除标志
    pub delete_flag: Option<String>,
}
