use super::jwt::Claims;
use crate::{common::errors::MyError, models::enums::common::PermitFlag};

pub fn get_permit(claims: &Claims, perm_code: &str) -> Result<String, MyError> {
    if claims
        .perm_codes.as_ref()
        .ok_or(MyError::Forbidden("没有权限".into()))?
        .contains(&perm_code.to_string())
    {
        Ok(PermitFlag::Permit.get_code())
    } else {
        Ok(PermitFlag::NotPermit.get_code())
    }
}
