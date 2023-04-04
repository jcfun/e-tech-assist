use crate::{common::errors::MyError, config::init::APP_CONTEXT};
use maxminddb::{geoip2::City, Reader};
use std::{collections::BTreeMap, fs, net::IpAddr, sync::Mutex};
use tracing::info;

pub fn get_reader() -> Mutex<Reader<Vec<u8>>> {
    let buf = fs::read("GeoLite2-City.mmdb").expect("读取数据文件失败");
    Mutex::new(maxminddb::Reader::from_source(buf).expect("数据流创建失败"))
}

fn get_name(names: Option<BTreeMap<&str, &str>>) -> String {
    names
        .and_then(|m| m.get("zh-CN").map(|v| v.to_string()))
        .unwrap_or_default()
}

pub async fn get_ip_addr(ip: &str) -> Result<String, MyError> {
    let reader = APP_CONTEXT.reader.lock().unwrap();
    let addr: IpAddr = ip.parse()?;
    let res: City = reader.lookup(addr)?;
    info!("Test result =========> {:?}", res);

    let continent = res
        .continent
        .map(|v| get_name(v.names))
        .unwrap_or("".into());

    let country = res.country.map(|v| get_name(v.names)).unwrap_or("".into());

    let city = res.city.map(|v| get_name(v.names)).unwrap_or("".into());

    Ok(format!("{}{}{}", continent, country, city))
}
