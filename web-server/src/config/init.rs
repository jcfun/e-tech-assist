use once_cell::sync::Lazy;

use super::{context::AppState, cfg::Config};


pub static APP_CFG: Lazy<Config> = Lazy::new(Config::new);
pub static APP_CONTEXT: Lazy<AppState> = Lazy::new(AppState::new);

pub fn app_init() {
    let _app_fcg = &APP_CFG.server;
    let _app_context = &APP_CONTEXT.db;
}