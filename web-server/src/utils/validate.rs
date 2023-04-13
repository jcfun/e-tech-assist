use crate::common::errors::MyError;
use validator::{Validate, ValidationError};

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

pub fn id_vector(values: &Vec<String>) -> Result<(), ValidationError> {
    for s in values {
        if s.len() != 18 {
            return Err(ValidationError::new("id格式错误"));
        }
    }
    Ok(())
}
