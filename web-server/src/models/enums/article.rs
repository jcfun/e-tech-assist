pub enum ArticleStatus {
    Draft,
    Audit,
    Published,
}

#[allow(dead_code)]
impl ArticleStatus {
    pub fn get_code(&self) -> String {
        match *self {
            ArticleStatus::Draft => "0".into(),
            ArticleStatus::Audit => "1".into(),
            ArticleStatus::Published => "2".into(),
        }
    }

    pub fn from_code(code: &str) -> Option<ArticleStatus> {
        match code {
            "0" => Some(ArticleStatus::Draft),
            "1" => Some(ArticleStatus::Audit),
            "2" => Some(ArticleStatus::Published),
            _ => None,
        }
    }

    pub fn get_desc(&self) -> String {
        match *self {
            ArticleStatus::Draft => "草稿".into(),
            ArticleStatus::Audit => "审核中".into(),
            ArticleStatus::Published => "已发布".into(),
        }
    }

    pub fn from_desc(desc: &str) -> Option<ArticleStatus> {
        match desc {
            "草稿" => Some(ArticleStatus::Draft),
            "审核中" => Some(ArticleStatus::Audit),
            "已发布" => Some(ArticleStatus::Published),
            _ => None,
        }
    }

    pub fn get_sort(&self) -> usize {
        match *self {
            ArticleStatus::Draft => 0,
            ArticleStatus::Audit => 1,
            ArticleStatus::Published => 2,
        }
    }

    pub fn from_sort(sort: usize) -> Option<ArticleStatus> {
        match sort {
            0 => Some(ArticleStatus::Draft),
            1 => Some(ArticleStatus::Audit),
            2 => Some(ArticleStatus::Published),
            _ => None,
        }
    }
}

pub enum ArticleTopFlag {
    True,
    False,
}

#[allow(dead_code)]
impl ArticleTopFlag {
    pub fn get_code(&self) -> String {
        match *self {
            ArticleTopFlag::False => "0".into(),
            ArticleTopFlag::True => "1".into(),
        }
    }

    pub fn from_code(code: &str) -> Option<ArticleTopFlag> {
        match code {
            "0" => Some(ArticleTopFlag::False),
            "1" => Some(ArticleTopFlag::True),
            _ => None,
        }
    }

    pub fn get_desc(&self) -> String {
        match *self {
            ArticleTopFlag::False => "不置顶".into(),
            ArticleTopFlag::True => "置顶".into(),
        }
    }

    pub fn from_desc(desc: &str) -> Option<ArticleTopFlag> {
        match desc {
            "不置顶" => Some(ArticleTopFlag::False),
            "置顶" => Some(ArticleTopFlag::True),
            _ => None,
        }
    }

    pub fn get_sort(&self) -> usize {
        match *self {
            ArticleTopFlag::False => 0,
            ArticleTopFlag::True => 1,
        }
    }

    pub fn from_sort(sort: usize) -> Option<ArticleTopFlag> {
        match sort {
            0 => Some(ArticleTopFlag::False),
            1 => Some(ArticleTopFlag::True),
            _ => None,
        }
    }
}
