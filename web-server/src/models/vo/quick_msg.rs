use rbatis::rbdc::datetime::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct QueryQuickMsgVO {
    pub id: Option<String>,
    pub operate_time: Option<DateTime>,
    pub operator: Option<String>,
    pub operator_id: Option<String>,
    pub create_time: Option<DateTime>,
    pub creator: Option<String>,
    pub creator_id: Option<String>,
    pub delete_flag: Option<String>,
    pub sender_id: Option<String>,
    pub recipient_id: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub success_flag: Option<String>,
    pub send_method: Option<String>,
    pub description: Option<String>,
    pub msg_type: Option<String>,
    pub reply_id: Option<String>,
    pub read_flag: Option<String>,
    pub sender_email: Option<String>,
    pub recipient_email: Option<String>,
    pub sender_avatar: Option<String>,
    pub recipient_avatar: Option<String>,
    pub disable_flag: Option<String>,
    pub children: Option<Vec<QueryQuickMsgVO>>,
}
