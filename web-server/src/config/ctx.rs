use std::sync::Arc;

use crate::{dbaccess::get_db_conn, utils::ip::get_reader};
use log::info;
use maxminddb::Reader;
use rbatis::Rbatis;

#[derive(Debug)]
pub struct AppState {
    pub db: Rbatis,
    pub reader: Arc<Reader<Vec<u8>>>,
}

impl AppState {
    // 初始化应用上下文
    pub fn new() -> Self {
        // rbatis数据库连接池
        let db = get_db_conn();
        // ip归属地，maxminddb数据库reader
        let reader = get_reader();

        info!("应用上下文初始化完毕!");
        AppState { db, reader }
    }
}
