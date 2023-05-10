use axum::response::IntoResponse;
use axum_client_ip::{InsecureClientIp, SecureClientIp};
use log::info;

#[allow(dead_code)]
pub async fn test(insecure_ip: InsecureClientIp, secure_ip: SecureClientIp) -> impl IntoResponse {
    if secure_ip.0.is_ipv6() {
        info!("{:?}", secure_ip.0.to_string().split(":").collect::<Vec<_>>().last().unwrap());
    }
    info!("{} {secure_ip:#?}", insecure_ip.0.is_ipv6());
    "ok"
}
