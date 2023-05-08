use super::base::BaseDTO;
use crate::utils::validate::id_vector;
use rbatis::rbdc::datetime::DateTime;
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
        required(message = "接收者标识不可为空"),
        length(min = 1, max = 50, message = "接收者标识格式错误")
    )]
    pub recipient_identity: Option<String>,

    #[validate(length(max = 100, message = "标题格式错误"))]
    pub title: Option<String>,

    #[validate(
        required(message = "消息内容不可为空"),
        length(min = 1, max = 2000, message = "消息内容格式错误")
    )]
    pub content: Option<String>,

    // 发送方式：0-全部, 1-邮件(默认)，2-短信，3-公众号
    pub send_method: Option<String>,

    #[serde(skip_deserializing)]
    pub success_flag: Option<String>,

    #[serde(skip_deserializing)]
    pub description: Option<String>,

    // 消息类型：0-发送，1-回复
    #[validate(
        required(message = "消息类型不可为空"),
        length(equal = 1, message = "消息类型格式错误")
    )]
    pub msg_type: Option<String>,

    #[validate(length(equal = 18, message = "回复消息id格式错误"))]
    pub reply_id: Option<String>,

    #[validate(length(equal = 1, message = "是否已读格式错误"))]
    pub read_flag: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UpdateReadFlagDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    #[validate(
        required(message = "是否已读不可为空"),
        length(equal = 1, message = "是否已读格式错误")
    )]
    pub read_flag: Option<String>,

    #[validate(custom(function = "id_vector", message = "id格式错误"))]
    pub ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct QueryQuickMsgDTO {
    pub create_time_start: Option<DateTime>,

    pub create_time_end: Option<DateTime>,

    #[validate(length(min = 1, max = 50, message = "发送者格式错误"))]
    pub sender: Option<String>,

    #[validate(length(min = 1, max = 50, message = "接收者格式错误"))]
    pub recipient: Option<String>,

    #[validate(length(min = 1, max = 100, message = "标题格式错误"))]
    pub title: Option<String>,

    #[validate(length(equal = 1, message = "发送方式格式错误"))]
    pub send_method: Option<String>,

    #[validate(length(equal = 1, message = "消息类型格式错误"))]
    pub msg_type: Option<String>,

    #[validate(length(equal = 1, message = "是否已读格式错误"))]
    pub read_flag: Option<String>,

    #[validate(length(equal = 1, message = "是否成功格式错误"))]
    pub success_flag: Option<String>,

    #[validate(length(equal = 1, message = "是否禁用格式错误"))]
    pub disable_flag: Option<String>,

    #[validate(range(min = 1, message = "页码最小为1"))]
    pub page_no: Option<usize>,

    #[validate(range(min = 1, message = "分页最小为1"))]
    pub page_size: Option<usize>,
}