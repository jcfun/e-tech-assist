use rbatis::{crud};
use serde::{Serialize, Deserialize};

use crate::models::entity::base_entity::BaseEntity;

/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-26 13:43:58
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-26 15:15:09
 * @FilePath: /e-tech-assist/web-server/src/models/vo/user.rs
 * @Description: 
 */
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserVo {
    pub id: Option<String>,
    pub account: Option<String>,
    pub disable_flag: Option<String>,
    #[serde(flatten)]
    pub base_entity: Option<BaseEntity>,
    pub detail_id: Option<String>,
}
crud!(UserVo{});