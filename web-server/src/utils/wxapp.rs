use crate::common::errors::MyError;
use crate::config::init::APP_CFG;
use bytes::Bytes;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer, symmetriccipher};
use reqwest::{header, Client as HttpClient};
use serde::{Deserialize, Serialize};
use std::str;
use std::time::Duration;
use tracing::info;

pub fn resolve_data(
    session_key: String,
    iv: String,
    encrypted_data: String,
) -> Result<WxUserInfo, MyError> {
    let key = base64::decode(session_key).unwrap();
    let iv = base64::decode(iv).unwrap();
    let aes = AesCrypt::new(key, iv);
    let data = aes.decrypt(encrypted_data);
    let info: WxUserInfo = serde_json::from_str(&data).unwrap();
    Ok(info)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WxUserInfo {
    pub openid: Option<String>,
    pub nick_name: Option<String>,
    pub gender: Option<u8>,
    pub language: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub avatar_url: Option<String>,
    pub pure_phone_number: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WxappSessionKey {
    pub session_key: String,
    pub expires_in: i64,
    pub openid: String,
    pub unionid: Option<String>,
}

pub async fn get_session_key(
    appid: &str,
    secret: &str,
    code: &str,
) -> Result<serde_json::Value, MyError> {
    let url = format!(
        "{api}/sns/jscode2session?appid={appid}&secret={secret}&js_code={code}&grant_type=authorization_code",
        api = &APP_CFG.get().unwrap().wxapp.api_domain,
        appid = appid,
        code = code,
        secret = secret
    );
    let api = Client::new();
    let res = api.get(&url).await?;
    match json_decode(&res) {
        Ok(data) => {
            info!("{:?}", data);
            if data.get("errcode").is_some() {
                Err(MyError::AxumError(format!(
                    "auth error: {}",
                    data["errmsg"].as_str().unwrap_or("")
                )))
            } else {
                Ok(data)
            }
        }
        Err(err) => {
            return Err(err);
        }
    }
}

pub fn json_decode(data: &str) -> Result<serde_json::Value, MyError> {
    let obj: serde_json::Value = match serde_json::from_str(data) {
        Ok(decoded) => decoded,
        Err(ref e) => {
            return Err(MyError::AxumError(format!("Json decode error: {}", e)));
        }
    };
    let dic = obj.as_object().unwrap();
    let code = if dic.contains_key("errcode") {
        "errcode"
    } else {
        "code"
    };

    let code = match obj[code].as_i64() {
        Some(v) => v,
        None => 0,
    };
    if code != 0 {
        let msg: String = if dic.contains_key("msg") {
            obj["msg"].to_string()
        } else {
            obj["errmsg"].to_string()
        };
        return Err(MyError::AxumError(msg));
    }
    info!("obj====={:?}", obj);
    Ok(obj)
}

///aes 加解密
pub struct AesCrypt {
    key: Vec<u8>,
    iv: Vec<u8>,
    key_size: aes::KeySize,
}
impl AesCrypt {
    /// 针对key的长度，keysize动态变化
    pub fn new(_key: Vec<u8>, iv: Vec<u8>) -> AesCrypt {
        let l = _key.len();
        let mut key = _key.clone();
        println!("l={}", l);
        let mut key_size = aes::KeySize::KeySize128;
        if l <= 16 {
            key = vec_pad(_key, 16);
        } else if l > 16 && l < 24 {
            key = _key[0..16].to_vec();
        } else if l > 24 && l < 32 {
            key_size = aes::KeySize::KeySize192;
            key = _key[0..24].to_vec();
        } else if l >= 32 {
            key_size = aes::KeySize::KeySize256;
            key = _key[0..32].to_vec();
        }
        AesCrypt { key, iv, key_size }
    }

    /// 针对字符串进行加密
    pub fn _encrypt(&self, text: String) -> String {
        // aes 加密
        let encrypted_data = _aes_cbc_encrypt(self.key_size, text.as_bytes(), &self.key, &self.iv)
            .ok()
            .unwrap();
        // 编码成base64
        let mut base64_encode = String::new();
        base64::encode_config_buf(&encrypted_data, base64::STANDARD, &mut base64_encode);
        base64_encode
    }

    /// 针对byte进行加密
    /// param1: 需要进行加密的文本信息
    pub fn _encrypt_byte(&self, text: Vec<u8>) -> String {
        // aes 加密
        let encrypted_data = _aes_cbc_encrypt(self.key_size, &text, &self.key, &self.iv)
            .ok()
            .unwrap();
        // 编码成base64
        let mut base64_encode = String::new();
        base64::encode_config_buf(&encrypted_data, base64::STANDARD, &mut base64_encode);

        base64_encode
    }

    /// aes解密
    /// param1: 待解密数据
    pub fn decrypt(&self, text: String) -> String {
        let mut base64_decode = Vec::<u8>::new();

        // 如是不正确的base64则返回空
        match base64::decode_config_buf(&text, base64::STANDARD, &mut base64_decode) {
            Ok(_) => {}
            Err(_e) => {
                return "".to_owned();
            }
        };
        // println!("=== msg de text===={:?}", base64_decode);
        // aes 解码
        let decrypted_data =
            match aes_cbc_decrypt(self.key_size, &base64_decode[..], &self.key, &self.iv) {
                Ok(data) => data,
                Err(_e) => {
                    println!("base64_decode={:?}", _e);
                    return "".to_owned();
                }
            };

        // 转换成string
        let the_string = str::from_utf8(&decrypted_data).expect("not UTF-8");

        the_string.to_owned()
    }
}

/// Encrypt a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
fn _aes_cbc_encrypt(
    key_size: aes::KeySize,
    data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut encryptor = aes::cbc_encryptor(key_size, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = (encryptor.encrypt(&mut read_buffer, &mut write_buffer, true))?;

        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }
    Ok(final_result)
}

/// Decrypts a buffer with the given key and iv using AES-256/CBC/Pkcs encryption.
/// @param1: key大小
/// @param2: 加密数据
fn aes_cbc_decrypt(
    key_size: aes::KeySize,
    encrypted_data: &[u8],
    key: &[u8],
    iv: &[u8],
) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(key_size, key, iv, blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
    // println!("=== decrypt key {:?} ", key);
    loop {
        let result = (decryptor.decrypt(&mut read_buffer, &mut write_buffer, true))?;
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    // println!("==== aes === result {:?}", final_result);

    Ok(final_result)
}

/// 位数不够时用0补齐
fn vec_pad(txt: Vec<u8>, length: usize) -> Vec<u8> {
    if txt.len() < length {
        let s = length - txt.len();
        let mut xs = txt;
        for _i in 0..s {
            xs.push(0u8);
        }
        return xs;
    }
    txt
}

pub struct Client {
    pub(crate) client: HttpClient,
}
impl Client {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(&APP_CFG.get().unwrap().wxapp.default_user_agent),
        );

        Client {
            client: HttpClient::builder()
                .timeout(Duration::from_secs(300))
                .connect_timeout(Duration::from_secs(300))
                .default_headers(headers)
                .build()
                .unwrap(),
        }
    }
    pub async fn _post<T: Serialize + ?Sized>(
        &self,
        url: &str,
        params: &T,
    ) -> Result<String, MyError> {
        match self.client.post(url).json(params).send().await {
            Ok(res) => {
                if res.status() == 200 {
                    match res.text().await {
                        Ok(txt) => {
                            // println!("--- {} ----", txt);
                            Ok(txt)
                        }
                        Err(e) => Err(MyError::AxumError(e.to_string())),
                    }
                } else {
                    Err(MyError::AxumError(format!("status={}", res.status())))
                }
            }
            Err(e) => Err(MyError::AxumError(format!("Send request error: {}", e))),
        }
    }

    pub async fn _post_betyes(&self, url: &str, body: Bytes) -> Result<String, MyError> {
        match self.client.post(url).body(body).send().await {
            Ok(res) => {
                if res.status() == 200 {
                    match res.text().await {
                        Ok(txt) => {
                            // println!("--- {} ----", txt);
                            Ok(txt)
                        }
                        Err(e) => Err(MyError::AxumError(e.to_string())),
                    }
                } else {
                    Err(MyError::AxumError(format!("status={}", res.status())))
                }
            }
            Err(e) => Err(MyError::AxumError(format!("Send request error: {}", e))),
        }
    }
    #[allow(dead_code)]
    pub async fn get(&self, url: &str) -> Result<String, MyError> {
        match self.client.get(url).send().await {
            Ok(res) => {
                if res.status() == 200 {
                    match res.text().await {
                        Ok(txt) => {
                            // println!("--- {} ----", txt);
                            Ok(txt)
                        }
                        Err(e) => Err(MyError::AxumError(e.to_string())),
                    }
                } else {
                    Err(MyError::AxumError(format!("status={}", res.status())))
                }
            }
            Err(e) => Err(MyError::AxumError(format!("Send request error: {}", e))),
        }
    }
}
