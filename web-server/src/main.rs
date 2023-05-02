#![feature(async_closure)]
#![feature(unboxed_closures)]
#![feature(drain_filter)]
use std::{net::SocketAddr, str::FromStr};

use axum::Router;
use dotenv::dotenv;
use routers::get_sys_routers;
use tracing::info;

use crate::{
    common::banner,
    config::init::{self, APP_CFG},
    utils::log,
};
// #[macro_use]
// extern crate lazy_static;
extern crate maxminddb;

#[path = "./config/mod.rs"]
mod config;

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

// #[tokio::main]
fn main() {
    // 设置环境变量
    dotenv().ok();
    // 初始化日志配置（多线程暂时无法获取本地时间，因此要在tokio初始化之前完成初始化）
    log::log_init();
    // banner
    banner::print_banner();
    let rt = tokio::runtime::Runtime::new().expect("异步运行时初始化失败");
    rt.block_on(async {
        // 初始化配置
        init::app_init();
        // 设置socket地址
        let socket =
            SocketAddr::from_str(&format!("{}:{}", &APP_CFG.server.ip, &APP_CFG.server.port))
                .expect("socket地址绑定失败");
        // 打印服务连接信息
        info!("listening on {} ...", socket);
        // 获取路由
        let routers = Router::new().nest(
            &format!("/{}/{}", &APP_CFG.api.prefix, &APP_CFG.api.version),
            get_sys_routers(),
        );
        // 启动服务
        axum::Server::bind(&socket)
            .serve(routers.into_make_service())
            .await
            .unwrap();
    });
}
