#![feature(async_closure)]
#![feature(unboxed_closures)]
use axum::Router;
use colored::Colorize;
use dotenv::dotenv;
use routers::get_sys_routers;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
#[macro_use]
extern crate lazy_static;

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

#[path = "middleware/mod.rs"]
mod middleware;

#[path = "utils/mod.rs"]
mod utils;

#[tokio::main]
async fn main() {
    // 设置环境变量
    dotenv().ok();
    // 启用日志打印
    tracing_subscriber::registry().with(fmt::layer()).init();
    // 设置监听地址
    let socket = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), 3000);
    // banner
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
    let routers = Router::new().nest("/api", get_sys_routers());
    // 启动服务
    axum::Server::bind(&socket)
        .serve(routers.into_make_service())
        .await
        .unwrap();
}
