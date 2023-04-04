use std::sync::Mutex;

use log::info;
use maxminddb::Reader;
use once_cell::sync::Lazy;
use rbatis::Rbatis;
use crate::{dbaccess::get_db_conn, utils::ip::get_reader};

#[derive(Debug)]
pub struct AppState {
    pub db: Lazy<Rbatis>,
    pub reader: Lazy<Mutex<Reader<Vec<u8>>>>,
}

impl AppState {
    // 初始化应用上下文
    pub fn new() -> Self {
        // rbatis数据库连接池
        let db = Lazy::<Rbatis>::new(get_db_conn);
        // ip归属地，maxminddb数据库reader
        let reader = Lazy::<Mutex<Reader<Vec<u8>>>>::new(get_reader);

        info!("应用上下文创建完毕");
        AppState { db, reader }
    }
}
