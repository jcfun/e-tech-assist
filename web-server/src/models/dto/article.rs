use super::base::BaseDTO;
use crate::{utils::validate::id_vector, models::enums::article::ArticleStatus};
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct CreateArticleDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    #[validate(
        required(message = "标题不可为空"),
        length(max = 100, message = "标题格式错误")
    )]
    pub title: Option<String>,

    #[validate(
        required(message = "内容不可为空"),
        length(min = 100, max = 10000, message = "内容格式错误")
    )]
    pub content: Option<String>,

    pub cover: Option<String>,
    pub view_count: Option<usize>,
    pub like_count: Option<usize>,
    pub comment_count: Option<usize>,
    pub collect_count: Option<usize>,
    pub forward_count: Option<usize>,
    pub category_id: Option<String>,
    pub tag_ids: Option<String>,
    pub status: Option<String>,
}

impl Default for CreateArticleDTO {
    fn default() -> Self {
        CreateArticleDTO {
            base_dto: BaseDTO::default(),
            title: None,
            content: None,
            cover: None,
            view_count: Some(0),
            like_count: Some(0),
            comment_count: Some(0),
            collect_count: Some(0),
            forward_count: Some(0),
            category_id: None,
            tag_ids: None,
            status: Some(ArticleStatus::Audit.get_code()),
        }
    }
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
