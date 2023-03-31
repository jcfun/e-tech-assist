use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "snake_case")]
pub struct BaseDTO {
    #[validate(
        required(message = "id不可为空"),
        length(equal = 18, message = "id格式错误")
    )]
    pub id: Option<String>,
    #[validate(required(message = "操作时间不可为空"))]
    pub operate_time: Option<DateTime>,
    #[validate(required(message = "操作者不可为空"))]
    pub operator: Option<String>,
    #[validate(required(message = "操作者id不可为空"))]
    pub operator_id: Option<String>,
    pub create_time: Option<DateTime>,
    pub creator: Option<String>,
    pub creator_id: Option<String>,
    pub delete_flag: Option<String>,
}
