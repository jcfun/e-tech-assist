pub enum PermitFlag {
    Permit,
    NotPermit,
}

#[allow(dead_code)]
impl PermitFlag {
    pub fn get_code(&self) -> String {
        match *self {
            PermitFlag::NotPermit => "0".into(),
            PermitFlag::Permit => "1".into(),
        }
    }

    pub fn from_code(code: &str) -> Option<PermitFlag> {
        match code {
            "0" => Some(PermitFlag::NotPermit),
            "1" => Some(PermitFlag::Permit),
            _ => None,
        }
    }

    pub fn get_desc(&self) -> String {
        match *self {
            PermitFlag::NotPermit => "允许".into(),
            PermitFlag::Permit => "不允许".into(),
        }
    }

    pub fn from_desc(desc: &str) -> Option<PermitFlag> {
        match desc {
            "不允许" => Some(PermitFlag::NotPermit),
            "允许" => Some(PermitFlag::Permit),
            _ => None,
        }
    }

    pub fn get_sort(&self) -> usize {
        match *self {
            PermitFlag::NotPermit => 0,
            PermitFlag::Permit => 1,
        }
    }

    pub fn from_sort(sort: usize) -> Option<PermitFlag> {
        match sort {
            0 => Some(PermitFlag::NotPermit),
            1 => Some(PermitFlag::Permit),
            _ => None,
        }
    }
}