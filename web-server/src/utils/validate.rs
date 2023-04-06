use crate::common::errors::MyError;
use validator::Validate;

/// 参数校验
pub fn param_validate<T>(payload: &T) -> Result<(), MyError>
where
    T: Validate + Clone,
{
    match payload.validate() {
        Ok(_) => Ok(()),
        Err(error) => Err(MyError::InvalidInput(error.to_string())),
    }
}
