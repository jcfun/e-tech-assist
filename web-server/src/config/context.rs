use std::sync::Arc;

use crate::{dbaccess::get_db_conn, utils::ip::get_reader};
use log::info;
use maxminddb::Reader;
use once_cell::sync::Lazy;
use rbatis::Rbatis;

#[derive(Debug)]
pub struct AppState {
    pub db: Lazy<Rbatis>,
    pub reader: Lazy<Arc<Reader<Vec<u8>>>>,
}

impl AppState {
    // 初始化应用上下文
    pub fn new() -> Self {
        // rbatis数据库连接池
        let db = Lazy::<Rbatis>::new(get_db_conn);
        // ip归属地，maxminddb数据库reader
        let reader = Lazy::<Arc<Reader<Vec<u8>>>>::new(get_reader);

        info!("应用上下文初始化完毕");
        AppState { db, reader }
    }
}
