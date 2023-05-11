use std::borrow::Cow;

use crate::common::errors::MyError;
use tracing::warn;
use validator::{Validate, ValidationError};

/// 参数校验
pub fn param_validate<T>(payload: &T) -> Result<(), MyError>
where
    T: Validate + Clone,
{
    match payload.validate() {
        Ok(_) => Ok(()),
        Err(errors) => {
            let mut error_msg = String::new();
            for (field, errors) in errors.field_errors() {
                let mut message = errors[0].message.as_ref().unwrap_or(&Cow::default()).to_string();
                warn!("{}: {:?}", field, message);
                message.push_str(" ");
                error_msg.push_str(&message);
            }
            Err(MyError::InvalidInput(error_msg))
        },
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
