/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-23 13:22:50
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-26 14:15:06
 * @FilePath: /e-tech-assist/web-server/src/models/dto/user.rs
 * @Description:
 */

use serde::{Deserialize, Serialize};
use rbatis::rbdc::datetime::DateTime;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateUserDTO {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub disable_flag: Option<String>,
    pub operate_time: Option<DateTime>,
    pub operator: Option<String>,
    pub operator_id: Option<String>,
    pub create_time: Option<DateTime>,
    pub creator: Option<String>,
    pub creator_id: Option<String>,
    pub delete_flag: Option<String>,
    pub detail_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateUserDTO {
    pub id: Option<String>,
    pub account: Option<String>,
    pub password: Option<String>,
    pub disable_flag: Option<String>,
    pub operate_time: Option<DateTime>,
    pub operator: Option<String>,
    pub operator_id: Option<String>,
    pub create_time: Option<DateTime>,
    pub creator: Option<String>,
    pub creator_id: Option<String>,
    pub delete_flag: Option<String>,
    pub detail_id: Option<String>,
}
