use once_cell::sync::OnceCell;

use super::{cfg::Config, context::AppState};

pub static APP_CFG: OnceCell<Config> = OnceCell::new();
pub static APP_CONTEXT: OnceCell<AppState> = OnceCell::new();

pub fn app_init() {
    APP_CFG.set(Config::new()).unwrap();
    APP_CONTEXT.set(AppState::new()).unwrap();
    // let _app_fcg = &APP_CFG.server;
    // let _app_context = &APP_CONTEXT.get().unwrap().db;
}
