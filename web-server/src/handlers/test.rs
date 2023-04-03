use log::info;
// use maxminddb::geoip2::City;

use crate::{common::{res::Res, errors::MyError}, utils::ip::get_ip_addr};

pub async fn test<'a>() -> Result<Res<String>, MyError> {
    let res = get_ip_addr("43.142.19.144").await?;
    info!("Test result =========> {:#?}", res);
    Ok(Res::from_success_msg("Test success", res))
}
