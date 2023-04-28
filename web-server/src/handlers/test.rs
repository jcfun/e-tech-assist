use log::info;
// use maxminddb::geoip2::City;

use crate::utils::wxapp::resolve_data;

#[allow(dead_code)]
pub async fn test<'a>() {
    // let res = get_session_key(&APP_CFG.wxapp.appid, &APP_CFG.wxapp.secret, "0f3KMv0008xZKP1Kni300Y9J8j2KMv0m").await;
    let res = resolve_data("6xFWjLMFJTxzJdO3Jqml8g==".into(), "RRfBnNVCSMdcWVPPhp+1Ew==".into(), "brgzQkt8+t8YPAja/aNU9Dc+pELbMTJK9osGGJ+NRxOB9soOHcv79XKZnxgb2WAr7z8zmPXAnFv0HxEJ/tvItaqDBDl1tcc/xeszFzRSd/UyzI73xZpBXup3OP/g/XsArNc8UrjH9tBOqOASk+7CkCNElYiptHzjx69ANeIq2RuagF5XAjTe3RwtbAAb4djQidIW1qCjX088ILcQUx5JesGm3ItQDNs+utYfCFHKDHsmz/LSHptFtAIgqCLjRKTTCgyp/1c5ncpFRQIcEn8PiDCKQx0uSvqGGwz50opaximIHV/Saa2RSDsu2MXOfWpPI5cXzHQxWd6sI96K4W3c9cKw17J923DXIkHh8G4yWtEzuQPFmwNZFd8aDY+of0FtmjRWY7NrYUJ+9GQRY0ucS50grL2lyd1jY7Sz2dPJkV8=".into());
    info!("Test result =========> {:#?}", res);
    // Ok(Res::from_success_msg("Test success", res))
}
