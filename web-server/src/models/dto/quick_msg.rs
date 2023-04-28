use super::base::BaseDTO;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CreateQuickMsgDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    #[validate(
        required(message = "发送者id不可为空"),
        length(equal = 18, message = "发送者id格式错误")
    )]
    pub sender_id: Option<String>,

    #[validate(
        required(message = "接收者id不可为空"),
        length(equal = 18, message = "接收者id格式错误")
    )]
    pub recipient_id: Option<String>,

    #[validate(length(max = 100, message = "标题格式错误"))]
    pub title: Option<String>,

    #[validate(
        required(message = "消息内容不可为空"),
        length(max = 2000, message = "消息内容格式错误")
    )]
    pub content: Option<String>,

    // 发送类型：0-全部, 1-邮箱(默认)，2-短信，3-公众号
    pub send_type: Option<String>,

    #[serde(skip_deserializing)]
    pub fail_flag: Option<String>,

    #[serde(skip_deserializing)]
    pub description: Option<String>,
}
