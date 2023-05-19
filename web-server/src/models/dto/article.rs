use std::{borrow::Cow, collections::HashMap};

use super::base::BaseDTO;
use crate::models::enums::article::{ArticleStatus, ArticleTopFlag};
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

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
        length(min = 100, max = 65535, message = "内容格式错误")
    )]
    pub content: Option<String>,

    #[validate(
        required(message = "封面不可为空"),
        length(min = 1, max = 200, message = "封面格式错误")
    )]
    pub cover: Option<String>,

    pub view_count: Option<usize>,

    pub like_count: Option<usize>,

    pub comment_count: Option<usize>,

    pub collect_count: Option<usize>,

    pub forward_count: Option<usize>,

    pub category_id: Option<String>,

    pub tag_ids: Option<String>,

    #[validate(required(message = "status不可为空"), custom = "status_validator")]
    pub status: Option<String>,

    pub top_flag: Option<String>,
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
            top_flag: Some(ArticleTopFlag::False.get_code()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate, Default)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UpdateArticleDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    #[validate(
        required(message = "标题不可为空"),
        length(max = 100, message = "标题格式错误")
    )]
    pub title: Option<String>,

    #[validate(
        required(message = "内容不可为空"),
        length(min = 100, max = 65535, message = "内容格式错误")
    )]
    pub content: Option<String>,

    #[validate(
        required(message = "封面不可为空"),
        length(min = 1, max = 200, message = "封面格式错误")
    )]
    pub cover: Option<String>,

    #[validate(required(message = "status不可为空"), custom = "status_validator")]
    pub status: Option<String>,
}
pub fn status_validator(value: &str) -> Result<(), ValidationError> {
    if value.is_empty() || !value.eq("0") && !value.eq("1") {
        let error = ValidationError {
            code: Cow::from("status_validator"),
            message: Some(Cow::from("参数错误")),
            params: HashMap::new(),
        };
        return Err(error);
    }
    Ok(())
}

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct QueryArticleDTO {
    #[serde(flatten)]
    pub base_dto: BaseDTO,

    pub create_time_start: Option<DateTime>,

    pub create_time_end: Option<DateTime>,

    #[validate(length(min = 1, max = 100, message = "标题格式错误"))]
    pub title: Option<String>,

    #[validate(length(equal = 18, message = "分类id格式错误"))]
    pub category_id: Option<String>,

    #[validate(length(equal = 18, message = "标签id格式错误"))]
    pub tag_id: Option<String>,

    #[validate(length(equal = 1, message = "文章状态格式错误"))]
    pub status: Option<String>,

    #[validate(length(equal = 1, message = "是否通过用户id查询格式错误"))]
    pub by_user_id_flag: Option<String>,

    #[validate(length(equal = 1, message = "是否置顶格式错误"))]
    pub top_flag: Option<String>,

    #[validate(range(min = 1, message = "页码最小为1"))]
    pub page_no: Option<usize>,

    #[validate(range(min = 1, message = "分页最小为1"))]
    pub page_size: Option<usize>,
}
impl Default for QueryArticleDTO {
    fn default() -> Self {
        QueryArticleDTO {
            base_dto: BaseDTO::default(),
            create_time_start: None,
            create_time_end: None,
            title: None,
            category_id: None,
            tag_id: None,
            status: Some(ArticleStatus::Published.get_code()),
            by_user_id_flag: None,
            top_flag: None,
            page_no: Some(1),
            page_size: Some(10),
        }
    }
}
