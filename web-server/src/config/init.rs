use once_cell::sync::OnceCell;

use super::{cfg::Config, ctx::AppState};

pub static APP_CFG: OnceCell<Config> = OnceCell::new();
pub static APP_CONTEXT: OnceCell<AppState> = OnceCell::new();

pub fn app_init() {
    APP_CFG.set(Config::new()).unwrap();
    APP_CONTEXT.set(AppState::new()).unwrap();
    // let _app_fcg = &APP_CFG.server;
    // let _app_context = &get_ctx().db;
}

pub fn get_ctx() -> &'static AppState {
    APP_CONTEXT.get().unwrap()
}

pub fn get_cfg() -> &'static Config {
    APP_CFG.get().unwrap()
}
