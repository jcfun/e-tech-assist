use super::jwt::Claims;
use crate::models::enums::common::PermitFlag;

pub fn get_permit(claims: &Claims, perm_code: &str) -> String {
    if claims
        .perm_codes
        .as_ref()
        .unwrap_or(&vec![])
        .contains(&perm_code.to_string())
    {
        PermitFlag::Permit.get_code()
    } else {
        PermitFlag::NotPermit.get_code()
    }
}
