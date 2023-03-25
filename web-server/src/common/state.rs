use rbatis::Rbatis;
// use sqlx::PgPool;

/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-24 19:44:31
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-24 21:26:25
 * @FilePath: /e-tech-assist/web-server/src/common/state.rs
 * @Description: 
 */
#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Rbatis,
}