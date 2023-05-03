use captcha_rs::CaptchaBuilder;
use serde::Serialize;
use uuid::Uuid;

use crate::config::init::get_cfg;

#[derive(Debug, Serialize, Clone)]
pub struct Captcha {
    #[serde(skip)]
    pub captcha: String,
    pub uuid: String,
    pub img: String,
}

/// 获取验证码
pub fn get_captcha() -> Captcha {
    let captcha = CaptchaBuilder::new()
        .length(get_cfg().captcha.length)
        .width(get_cfg().captcha.width)
        .height(get_cfg().captcha.height)
        .dark_mode(get_cfg().captcha.dark_mode)
        .complexity(get_cfg().captcha.complexity) // min: 1, max: 10
        .compression(get_cfg().captcha.compression) // min: 1, max: 99
        .build();
    let img = captcha.to_base64();
    let uuid = Uuid::new_v4().to_string();
    let captcha = captcha.text.to_lowercase();
    Captcha { captcha, uuid, img }
}
