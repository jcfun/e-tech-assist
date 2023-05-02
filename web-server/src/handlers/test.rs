use axum::response::IntoResponse;
use log::info;

use crate::utils;

#[allow(dead_code)]
pub async fn test() -> impl IntoResponse {
    utils::qrcode::generate_qrcode("Test").await;
    info!("ok");
    "ok"
}
