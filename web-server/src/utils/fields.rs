use super::{epc::get_snowflake, jwt::Claims};
use crate::models::{dto::base::BaseDTO, entity::base::BaseEntity};
use heck::ToLowerCamelCase;
use rbatis::rbdc::datetime::DateTime;
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, fmt::Debug};
// use tracing::info;

/// 填充创建和更新数据时的公共字段
pub fn fill_fields(model: &mut BaseDTO, claims: &Claims, create_flag: bool) {
    let datetime = DateTime::now();
    let claims = claims.clone();
    if create_flag {
        model.id = Some(get_snowflake());
        model.create_time = Some(datetime.clone());
        (model.creator, model.creator_id) = (claims.nickname.clone(), claims.id.clone());
        model.delete_flag = Some("0".to_string());
    } else {
        (
            model.create_time,
            model.creator,
            model.creator_id,
            model.delete_flag,
        ) = (None, None, None, None);
    }
    model.operate_time = Some(datetime);
    (model.operator, model.operator_id) = (claims.nickname, claims.id);
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

/// ## 属性拷贝
/// + source:  拷贝源
/// + target:  拷贝目标
/// + only_cover_null:  是否只覆盖target中的空值字段，为true时cover_by_null不生效
/// + cover_by_null:  是否允许source中的空值覆盖target中的对应字段(即覆盖target中的所有字段)
pub fn copy_fields<T, U>(
    source: &T,
    target: &mut U,
    only_cover_null: bool,
    cover_by_null: bool,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize + DeserializeOwned + Debug,
    U: Serialize + DeserializeOwned + Debug,
{
    let source_map: HashMap<String, serde_json::Value> =
        serde_json::from_str(&serde_json::to_string(source)?)?;
    let temp_map: &mut HashMap<String, serde_json::Value> =
        &mut serde_json::from_str(&serde_json::to_string(target)?)?;
    // info!("source_map: {:#?}", source_map);
    // info!("temp_map: {:#?}", temp_map);
    let mut target_map: HashMap<String, serde_json::Value> = HashMap::new();
    for (key, source_value) in source_map {
        if let Some(target_value) = temp_map.get(&key) {
            // 是否只覆盖target中的空值字段(互补)
            if only_cover_null {
                if target_value.is_null() {
                    target_map.insert(key.to_lower_camel_case(), source_value);
                } else {
                    target_map.insert(key.to_lower_camel_case(), target_value.clone());
                }
            } else {
                // 是否允许source中的空值覆盖target中的对应字段(即覆盖target中的所有字段)
                if cover_by_null {
                    target_map.insert(key.to_lower_camel_case(), source_value);
                    continue;
                }
                // 如果source中对应字段为空值，则使用target中原本的值，否则使用source中的值
                if source_value.is_null() {
                    target_map.insert(key.to_lower_camel_case(), target_value.clone());
                } else {
                    target_map.insert(key.to_lower_camel_case(), source_value);
                }
            }
        }
    }
    // info!("target_map: {:#?}", target_map);
    *target = serde_json::from_value(serde_json::to_value(target_map)?)?;
    Ok(())
}
