use std::time::{SystemTime, UNIX_EPOCH};

/// 获取当前时间戳
pub fn get_epoch() -> usize {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_secs() as usize
}
