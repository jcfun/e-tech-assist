use super::{epc::get_snowflake, jwt::Claims};
use crate::models::{dto::base::BaseDTO, entity::base::BaseEntity};
use rbatis::rbdc::datetime::DateTime;

/// 填充创建和更新数据时的公共字段
pub fn _fill_fields(model: &mut BaseDTO, claim: &Claims, create_flag: bool) {
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


/// 填充非登录状态下操作时的公共字段(注册、登录日志、修改密码等)
pub fn fill_fields_system(model: &mut BaseDTO) {
    let datetime = DateTime::now();
    model.id = Some(get_snowflake());
    model.create_time = Some(datetime.clone());
    (model.creator, model.creator_id) = (Some("system".into()), Some("system".into()));
    model.delete_flag = Some("0".to_string());
    model.operate_time = Some(datetime);
    (model.operator, model.operator_id) = (Some("system".into()), Some("system".into()));
}


/// 填充非登录状态下操作时的公共字段(注册、登录日志、修改密码等)
pub fn fill_fields_system_entity(model: &mut BaseEntity) {
    let datetime = DateTime::now();
    model.id = Some(get_snowflake());
    model.create_time = Some(datetime.clone());
    (model.creator, model.creator_id) = (Some("system".into()), Some("system".into()));
    model.delete_flag = Some("0".to_string());
    model.operate_time = Some(datetime);
    (model.operator, model.operator_id) = (Some("system".into()), Some("system".into()));
}