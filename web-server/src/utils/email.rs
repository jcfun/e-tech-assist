use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use tracing::info;

use crate::common::errors::MyError;
use crate::config::init::APP_CFG;

pub async fn send_email_single(
    to: String,
    subject: String,
    body: String,
) -> Result<String, MyError> {
    let email = Message::builder()
        .from(APP_CFG.email.email_addr.parse().unwrap())
        .reply_to(APP_CFG.email.email_addr.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .unwrap();

    let creds = Credentials::new(
        APP_CFG.email.email_addr.to_owned(),
        APP_CFG.email.code.to_owned(),
    );

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&APP_CFG.email.smtp_addr)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => {
            info!("邮件已发送至 {to}");
            Ok(format!("邮件已发送至 {to}"))
        }
        Err(e) => Err(MyError::EmailSendError(format!("邮件发送失败: {e:?}"))),
    }
}