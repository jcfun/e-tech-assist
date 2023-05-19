use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct UploadVO {
    pub url: Option<String>,
    pub alt: Option<String>,
    pub href: Option<String>,
}
