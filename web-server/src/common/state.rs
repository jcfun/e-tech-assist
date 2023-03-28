use rbatis::Rbatis;
use crate::dbaccess::get_db_conn;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Rbatis,
}

pub fn get_state() -> AppState {
    AppState {
        db: get_db_conn(),
    }
}