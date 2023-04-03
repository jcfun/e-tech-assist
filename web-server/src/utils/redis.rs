use log::error;
use redis::aio::Connection;
use redis::AsyncCommands;
extern crate redis;

use crate::common::errors::MyError;

pub async fn get_redis_conn() -> Result<Connection, MyError> {
    // 获取redis链接
    // let redis_url = env::var("REDIS_URL").expect("REDIS_URL is not found");
    let client = redis::Client::open("redis://:123456@192.168.31.243:3310").unwrap();
    let conn = client.get_async_connection().await;
    if conn.is_err() {
        let err = format!("{}", conn.err().unwrap());
        let err = MyError::RedisError(err.to_string());
        error!("{:?}", err);
        return Err(err);
    }
    Ok(conn.unwrap())
}

/// 设置str
pub async fn _set_string(key: &str, value: &str) -> Result<bool, MyError> {
    let mut conn = get_redis_conn().await?;
    let res: bool = conn.set(key, value).await?;
    Ok(res)

}

/// 设置有过期时间的str
pub async fn set_string_ex(key: &str, value: &str, ex: usize) -> Result<bool, MyError> {
    let mut conn = get_redis_conn().await?;
    let res: bool = conn.set_ex(key, value, ex).await?;
    Ok(res)
}

/// 删除key
pub async fn del_string(key: &str) -> Result<bool, MyError> {
    let mut conn = get_redis_conn().await?;
    let res: bool = conn.del(key).await?;
    Ok(res)
}

/// 为已存在的key设置过期时间
pub async fn _expire(key: &str, ex: usize) -> Result<bool, MyError> {
    let mut conn = get_redis_conn().await?;
    let res: bool = conn.expire(key, ex).await?;
    Ok(res)
}

/// 获取已存在的key
pub async fn get_string(key: &str) -> Result<String, MyError> {
    let mut conn = get_redis_conn().await?;
    let res: String = conn.get(key).await?;
    Ok(res)
}

/// 获取已存在的key的ttl
pub async fn _get_ttl(key: &str) -> Result<usize, MyError> {
    let mut conn = get_redis_conn().await?;
    let res = conn.ttl(key).await?;
    Ok(res)
}