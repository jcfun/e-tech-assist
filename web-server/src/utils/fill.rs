use super::{epc::get_snowflake, jwt::Claims};
use crate::models::dto::base::BaseDTO;
use rbatis::rbdc::datetime::DateTime;

/// 填充公共字段
pub fn fill_create_fields(model: &mut BaseDTO, claim: &Claims, create_flag: bool) {
    let datetime = DateTime::now();
    let claim = claim.clone();
    if create_flag {
        model.id = Some(get_snowflake());
        model.create_time = Some(datetime.clone());
        (model.creator, model.creator_id) = (claim.nickname.clone(), claim.id.clone());
        model.delete_flag = Some("0".to_string());
    }
    model.operate_time = Some(datetime);
    (model.operator, model.operator_id) = (claim.nickname, claim.id);
}

pub fn fill_fields(model: &mut BaseDTO) {
    let datetime = DateTime::now();
    model.id = Some(get_snowflake());
    model.create_time = Some(datetime.clone());
    (model.creator, model.creator_id) = (Some("system".into()), Some("system".into()));
    model.delete_flag = Some("0".to_string());
    model.operate_time = Some(datetime);
    (model.operator, model.operator_id) = (Some("system".into()), Some("system".into()));
}
