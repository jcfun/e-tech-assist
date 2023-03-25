/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-23 13:22:50
 * @LastEditors: jc-fun urainstar@gmail.com
 * @LastEditTime: 2023-03-25 21:20:25
 * @FilePath: /e-tech-assist/web-server/src/models/user.rs
 * @Description:
 */

use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use rbatis::{crud, impl_update, impl_delete, impl_select_page, impl_select};
use rbatis::rbdc::datetime::DateTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TUser {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub delete_flag: Option<String>,
    pub disable_flag: Option<String>,
    pub update_time: Option<DateTime>,
    pub operator: Option<String>,
    pub operator_id: Option<String>,
    pub create_time: Option<DateTime>,
    pub creator: Option<String>,
    pub creator_id: Option<String>,
}
crud!(TUser{});

impl_select!(TUser{select_all_by_id(id:i64,name:&str) => "`where id = #{id} and name = #{name}`"});
impl_select!(TUser{select_by_id(id:i64) -> Option => "`where id = #{id} limit 1`"});
impl_update!(TUser{update_by_name(name:&str) => "`where id = 1`"});
impl_delete!(TUser {delete_by_name(name:&str) => "`where name= '2'`"});
//limiting condition：maybe pg/mssql not support sql `limit 0,10` you should add arg  `limit_sql:&str` of set limit_sql = " limit 0 offset 10"
//`impl_select_page!(BizActivity{select_page(name:&str,limit_sql:&str) => "`where name != #{name}`"});`
impl_select_page!(TUser{select_page(name:&str) => "`where name != #{name}`"});

impl IntoResponse for CreateUser {
    fn into_response(self) -> axum::response::Response {
        ().into_response()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateUser {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub delete_flag: Option<String>,
    pub disable_flag: Option<String>,
    pub update_time: Option<DateTime>,
    pub operator: Option<String>,
    pub operator_id: Option<String>,
    pub create_time: Option<DateTime>,
    pub creator: Option<String>,
    pub creator_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateUser {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub delete_flag: Option<String>,
    pub disable_flag: Option<String>,
    pub update_time: Option<DateTime>,
    pub operator: Option<String>,
    pub operator_id: Option<String>,
    pub create_time: Option<DateTime>,
    pub creator: Option<String>,
    pub creator_id: Option<String>,
}
