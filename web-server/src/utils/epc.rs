// encrypt and encode

use rbatis::snowflake;
use ring::digest;

/// sha256加密
pub fn encrypt_sha256(raw: String, salt: String) -> String {
    let raw_bytes = raw.as_bytes();
    let salt_bytes = salt.as_bytes();
    let salted = vec![raw_bytes, salt_bytes].concat();
    let mut digest_ctx = digest::Context::new(&digest::SHA256);
    digest_ctx.update(&salted);
    let result = digest_ctx.finish();
    hex::encode(result)
}

/// 雪花id
pub fn get_snowflake() -> String {
    snowflake::new_snowflake_id().to_string()
}