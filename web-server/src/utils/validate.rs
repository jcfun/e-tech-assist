use crate::common::res::Res;
use axum::http::StatusCode;
use serde::Serialize;
use validator::Validate;

/// 参数校验
pub fn param_validate<T, U>(payload: T) -> Result<String, (StatusCode, Res<U>)>
where
    T: Validate + Clone,
    U: Serialize + Clone,
{
    match payload.validate() {
        Ok(_) => Ok("OK".into()),
        Err(error) => Err((
            StatusCode::OK,
            Res::from_msg(StatusCode::UNAUTHORIZED, error.to_string().as_str()),
        )),
    }
}
