#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Database,
    pub jwt: Jwt,
    pub server: Server,
    pub api: Api,
    pub captcha: Captcha,
}
use log::info;
use serde::Deserialize;
use std::{env, fs::File, io::Read};

/// 数据库配置
#[derive(Debug, Deserialize)]
pub struct Database {
    pub pg_url: String,
    pub redis_url: String,
}

/// jwt 配置
#[derive(Debug, Deserialize)]
pub struct Jwt {
    // 密钥
    pub secret: String,
    // 过期时间
    pub exp: usize,
}

/// server
#[derive(Debug, Deserialize)]
pub struct Server {
    // ip
    pub ip: String,
    // 端口
    pub port: String,
}

/// api
#[derive(Debug, Deserialize)]
pub struct Api {
    // ip
    pub prefix: String,
    // 端口
    pub version: String,
}

/// 验证码
#[derive(Debug, Deserialize)]
pub struct Captcha {
    // 长度
    pub length: usize,
    // 宽度
    pub width: u32,
    // 高度
    pub height: u32,
    // 深色模式
    pub dark_mode: bool,
    // 复杂度
    pub complexity: u32,
    // 压缩
    pub compression: u8,
    // 过期时间
    pub exp: usize,
}

impl Config {
    pub fn new() -> Self {
        let cfg_path = env::var("CFG_PATH").expect("配置文件路径读取失败");
        let mut file = File::open(&cfg_path).map(|f| f).unwrap_or_else(|e| {
            panic!(
                "配置文件路径错误，无法在该路径下找到对应配置文件: {}, 错误: {}",
                cfg_path, e
            )
        });
        let mut cfg_string = String::new();
        file.read_to_string(&mut cfg_string)
            .map(|cs| cs)
            .unwrap_or_else(|err| panic!("配置文件信息读取失败: {}", err));

        let config = serde_yaml::from_str(&cfg_string)
            .map(|data| data)
            .unwrap_or_else(|err| panic!("配置文件解析失败: {}", err));
        info!("配置文件初始化完毕");
        config
    }
}
