use std::time::SystemTime;

use axum::response::IntoResponse;
use log::info;

use crate::common::res::Res;

pub async fn test() -> impl IntoResponse {
    let now = SystemTime::now();
    info!("now =======> {:?}", now);
    Res::<SystemTime>::from_success_msg("Test success", now)
}
