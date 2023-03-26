use rbatis::rbdc::datetime::DateTime;
use serde::{Serialize, Deserialize};

/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-26 15:11:04
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-26 15:13:36
 * @FilePath: /e-tech-assist/web-server/src/models/entity/base_entity.rs
 * @Description: 
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEntity {
    pub operate_time: Option<DateTime>,
    pub operator: Option<String>,
    pub operator_id: Option<String>,
    pub create_time: Option<DateTime>,
    pub creator: Option<String>,
    pub creator_id: Option<String>,
    pub delete_flag: Option<String>,
}