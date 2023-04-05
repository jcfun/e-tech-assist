use crate::{common::errors::MyError, config::init::APP_CONTEXT};
use maxminddb::{geoip2::City, Reader};
use std::{collections::BTreeMap, fs, net::IpAddr, sync::Arc};
use tracing::info;

pub fn get_reader() -> Arc<Reader<Vec<u8>>> {
    let buf = fs::read("GeoLite2-City.mmdb").expect("读取数据文件失败");
    Arc::new(maxminddb::Reader::from_source(buf).expect("数据流创建失败"))
}

fn get_name(names: Option<BTreeMap<&str, &str>>) -> String {
    names
        .and_then(|m| m.get("zh-CN").map(|v| v.to_string()))
        .unwrap_or_default()
}

pub async fn get_ip_addr(ip: &str) -> Result<String, MyError> {
    let ip_addr = if ip.starts_with("192") || ip.starts_with("172") || ip.starts_with("10") {
        "局域网".into()
    } else if ip.starts_with("127") || ip.starts_with("0") || ip.starts_with("::") {
        "本机".into()
    } else {
        let reader = &APP_CONTEXT.reader;
        let addr: IpAddr = ip.parse()?;
        let res: City = reader.lookup(addr)?;
        info!("geoip2 =========> {:?}", res);
        let continent = res
            .continent
            .map(|v| get_name(v.names))
            .unwrap_or("".into());
        let country = res.country.map(|v| get_name(v.names)).unwrap_or("".into());
        let subdivisions = res
            .subdivisions
            .map(|v| {
                v.get(0)
                    .and_then(move |v| Some(get_name(v.to_owned().names)))
                    .unwrap_or("".to_string())
            })
            .unwrap_or("".into());
        let city = res.city.map(|v| get_name(v.names)).unwrap_or("".into());

        format!("{} {} {} {}", continent, country, subdivisions, city)
    };
    info!("ip_addr =========> {:?}", ip_addr);
    Ok(ip_addr)
}
