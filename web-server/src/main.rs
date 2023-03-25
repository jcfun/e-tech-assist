/*
 * @Author: jcfun jcfunstar@gmail.com
 * @Date: 2023-03-22 20:41:36
 * @LastEditors: jcfun jcfunstar@gmail.com
 * @LastEditTime: 2023-03-25 20:03:57
 * @FilePath: /e-tech-assist/web-server/src/main.rs
 * @Description:
 */

use std::{
    env,
    net::{IpAddr, Ipv6Addr, SocketAddr},
};

use crate::common::state::AppState;
use dotenv::dotenv;
use rbatis::Rbatis;
use rbdc_pg::driver::PgDriver;
use routers::user::user_routes;
use colored::Colorize;

#[path = "./models/mod.rs"]
mod models;

#[path = "./routers/mod.rs"]
mod routers;

#[path = "./handlers/mod.rs"]
mod handlers;

#[path = "./dbaccess/mod.rs"]
mod dbaccess;

#[path = "./common/mod.rs"]
mod common;

#[tokio::main]
async fn main() {
    // 设置环境变量
    dotenv().ok();
    // 获取环境变量
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not found");
    // 启用sql日志打印
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    // 初始化数据库连接池
    let rb = Rbatis::new();
    rb.init(PgDriver {}, database_url.as_str()).unwrap();

    // let db_pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(&database_url)
    //     .await
    //     .unwrap();
    // 设置共享状态
    let shared_state = AppState { db: rb };
    // 设置监听地址
    let socket = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), 3000);
    let banner = r#"
                  __            __                          _      __ 
      ___        / /____  _____/ /_        ____ ___________(_)____/ /_
     / _ \______/ __/ _ \/ ___/ __ \______/ __ `/ ___/ ___/ / ___/ __/
    /  __/_____/ /_/  __/ /__/ / / /_____/ /_/ (__  |__  ) (__  ) /_  
    \___/      \__/\___/\___/_/ /_/      \__,_/____/____/_/____/\__/   
    "#;
    println!("{}", banner.bright_cyan());
    // 打印服务连接信息
    println!("listening on {} ...\n", socket);
    // 获取路由
    let routers = user_routes().with_state(shared_state);
    // 启动服务
    axum::Server::bind(&socket)
        .serve(routers.into_make_service())
        .await
        .unwrap();
}
