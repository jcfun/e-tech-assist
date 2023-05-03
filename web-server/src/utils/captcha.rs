use captcha_rs::CaptchaBuilder;
use serde::Serialize;
use uuid::Uuid;

use crate::config::init::APP_CFG;

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
        .length(APP_CFG.get().unwrap().captcha.length)
        .width(APP_CFG.get().unwrap().captcha.width)
        .height(APP_CFG.get().unwrap().captcha.height)
        .dark_mode(APP_CFG.get().unwrap().captcha.dark_mode)
        .complexity(APP_CFG.get().unwrap().captcha.complexity) // min: 1, max: 10
        .compression(APP_CFG.get().unwrap().captcha.compression) // min: 1, max: 99
        .build();
    let img = captcha.to_base64();
    let uuid = Uuid::new_v4().to_string();
    let captcha = captcha.text.to_lowercase();
    Captcha { captcha, uuid, img }
}
