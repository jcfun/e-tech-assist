use derive_builder::Builder;
use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct QueryArticleVO {
    pub id: Option<String>,
    pub operate_time: Option<DateTime>,
    pub operator: Option<String>,
    pub operator_id: Option<String>,
    pub create_time: Option<DateTime>,
    pub creator: Option<String>,
    pub creator_id: Option<String>,
    pub delete_flag: Option<String>,
    pub title: Option<String>,
    pub cover: Option<String>,
    pub content: Option<String>,
    pub view_count: Option<usize>,
    pub like_count: Option<usize>,
    pub comment_count: Option<usize>,
    pub category_id: Option<String>,
    pub tag_ids: Option<String>,
    pub status: Option<String>,
    pub collect_count: Option<usize>,
    pub forward_count: Option<usize>,
    pub top_flag: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Builder, Default)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct QueryArticleInfoVO {
    total_article_count: Option<usize>,
    total_view_count: Option<usize>,
    total_like_count: Option<usize>,
    total_comment_count: Option<usize>,
    total_collect_count: Option<usize>,
    total_forward_count: Option<usize>,
}
