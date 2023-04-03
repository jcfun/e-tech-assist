use crate::common::errors::MyError;
use maxminddb::{geoip2::City, Reader};
use once_cell::sync::Lazy;
use std::{fs, net::IpAddr};
use tracing::info;

pub static READER: Lazy<Reader<Vec<u8>>> = Lazy::new(get_reader);

pub fn get_reader() -> Reader<Vec<u8>> {
    let buf = fs::read("GeoLite2-City.mmdb").unwrap();
    maxminddb::Reader::from_source(buf).unwrap()
}

pub async fn get_ip_addr(ip: &str) -> Result<String, MyError> {
    let addr: IpAddr = ip.parse()?;
    let res: City = READER.lookup(addr)?;
    info!("Test result =========> {:?}", res);
    let (continent, country, city) = (
        res.continent
            .map(|v1| {
                v1.names
                    .map(|v2| {
                        v2.get("zh-CN")
                            .map(|v3| v3.to_string())
                            .unwrap_or("".to_string())
                    })
                    .unwrap_or("".to_string())
            })
            .unwrap_or("".to_string()),
        res.country
            .map(|v1| {
                v1.names
                    .map(|v2| {
                        v2.get("zh-CN")
                            .map(|v3| v3.to_string())
                            .unwrap_or("".to_string())
                    })
                    .unwrap_or("".to_string())
            })
            .unwrap_or("".to_string()),
        res.city
            .map(|v1| {
                v1.names
                    .map(|v2| {
                        v2.get("zh-CN")
                            .map(|v3| v3.to_string())
                            .unwrap_or("".to_string())
                    })
                    .unwrap_or("".to_string())
            })
            .unwrap_or("".to_string()),
    );
    let addr = continent + &country + &city;
    Ok(addr)
}
