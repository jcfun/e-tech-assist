use captcha_rs::CaptchaBuilder;
use serde::Serialize;
use uuid::Uuid;

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
        .length(5)
        .width(130)
        .height(40)
        .dark_mode(false)
        .complexity(1) // min: 1, max: 10
        .compression(40) // min: 1, max: 99
        .build();
    let img = captcha.to_base64();
    let uuid = Uuid::new_v4().to_string();
    let captcha = captcha.text.to_lowercase();
    Captcha { captcha, uuid, img }
}
