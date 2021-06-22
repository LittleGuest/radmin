use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

pub struct EmailInfo {
    pub host: String,
    pub port: String,
    pub account: String,
    pub password: String,

    pub subject: String,
    pub content: String,
    pub to: Vec<String>,
}

pub fn send_email(email_info: EmailInfo) {
    email_info.to.iter().for_each(|to| {
        let messages = Message::builder()
            .from(
                format!(" <{}>", email_info.account.clone())
                    .parse()
                    .unwrap(),
            )
            .to(format!(" <{}>", to).parse().unwrap())
            .subject(email_info.subject.clone())
            .body(email_info.content.clone())
            .unwrap();

        let credentials = Credentials::new(email_info.account.clone(), email_info.password.clone());

        let st = SmtpTransport::relay(email_info.host.as_str())
            .unwrap()
            .credentials(credentials)
            .build();

        let _ = st.send(&messages);
    });
}
