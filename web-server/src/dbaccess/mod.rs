use rbatis::Rbatis;
use rbdc_pg::driver::PgDriver;
use std::env;

pub mod login;
pub mod sql;
pub mod user;

pub fn get_db_conn() -> Rbatis {
    // 获取数据库链接
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not found");
    // 初始化数据库连接池
    let rb = Rbatis::new();
    rb.init(PgDriver {}, database_url.as_str()).unwrap();
    rb
}