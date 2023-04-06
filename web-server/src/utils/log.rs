use std::env;

use color_eyre;
use time::UtcOffset;
use tracing_appender::{non_blocking, rolling};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    fmt::{self, time::OffsetTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Registry,
};

pub fn log_init() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("DEBUG"));
    let timer = OffsetTime::new(
        UtcOffset::current_local_offset().expect("本地时间偏移量获取失败"),
        time::format_description::parse(
            "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]",
        )
        .expect("时间格式化失败"),
    );
    // 输出到控制台中
    let console_layer = fmt::layer()
        .pretty()
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_timer(timer.clone())
        .with_writer(std::io::stderr);

    // 输出到文件中
    // 获取环境变量
    let log_dir = env::var("LOG_DIR").expect("日志输出文件夹路径获取失败");
    let log_file = env::var("LOG_FILE").expect("日志输出文件夹路径获取失败");
    let file_appender = rolling::daily(log_dir, log_file);
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_ansi(false)
        .with_timer(timer)
        .with_writer(non_blocking_appender);

    // 注册
    Registry::default()
        .with(env_filter)
        // ErrorLayer 可以让 color-eyre 获取到 span 的信息
        .with(ErrorLayer::default())
        .with(console_layer)
        .with(file_layer)
        .init();

    // 安裝 color-eyre 的 panic 处理句柄
    color_eyre::install().unwrap();
}
