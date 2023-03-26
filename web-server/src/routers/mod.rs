use axum::Router;

use crate::{common::state::AppState, dbaccess::get_db_conn};
use self::user::user_routes;
use self::login::login_routes;

/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-23 15:29:00
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-25 18:48:35
 * @FilePath: /e-tech-assist/web-server/src/routers/mod.rs
 * @Description:
 */
pub mod login;
pub mod user;


pub fn get_routers() -> Router {
    let rb = get_db_conn();
    Router::<AppState>::new().nest("/user", user_routes()).merge(login_routes()).with_state(rb)
}
