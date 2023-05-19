use crate::common::errors::MyError;
use std::{borrow::Cow, collections::HashMap};
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
                let message = errors[0]
                    .message
                    .as_ref()
                    .unwrap_or(&Cow::default())
                    .to_string();
                warn!("{}: {:?}", field, message);
                error_msg.push_str(&format!("{message} "));
            }
            Err(MyError::InvalidInput(error_msg.trim_end().into()))
        }
    }
}

pub fn id_vector(values: &Vec<String>) -> Result<(), ValidationError> {
    for s in values {
        if s.len() != 18 {
            let error = ValidationError {
                code: Cow::from("id_vector"),
                message: Some(Cow::from("id格式错误")),
                params: HashMap::new(),
            };
            return Err(error);
        }
    }
    Ok(())
}
