use rbatis::Rbatis;
use rbdc_pg::driver::PgDriver;

use crate::config::init::get_cfg;

pub mod login;
pub mod perm;
pub mod quick_msg;
pub mod role;
pub mod sql;
pub mod user;

pub fn get_db_conn() -> Rbatis {
    // 获取数据库链接
    let database_url = &get_cfg().database.pg_url;
    // 初始化数据库连接池
    let rb = Rbatis::new();
    rb.init(PgDriver {}, database_url)
        .expect("数据库初始化失败");
    rb
}
