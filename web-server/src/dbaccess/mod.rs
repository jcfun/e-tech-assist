use std::env;

use rbatis::Rbatis;
use rbdc_pg::driver::PgDriver;
use crate::common::state::AppState;

/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-23 15:50:10
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-24 14:43:25
 * @FilePath: /e-tech-assist/web-server/src/dbaccess/mod.rs
 * @Description:
 */
pub mod user;
pub mod login;

pub fn get_db_conn() -> AppState{
    // 获取数据库链接
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not found");
    // 初始化数据库连接池
    let rb = Rbatis::new();
    rb.init(PgDriver {}, database_url.as_str()).unwrap();
    // 设置共享状态
    AppState { db: rb }
}
