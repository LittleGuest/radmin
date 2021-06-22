use actix_web::{Responder, web};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;

use crate::commons::RespBuilder;
use crate::dtos::email_config::EmailSendForm;
use crate::models::ToolEmailConfig;

pub struct EmailController;

impl EmailController {
    pub async fn get_by_id() -> impl Responder {
        let data = ToolEmailConfig::get_by_id().await.unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn update_by_id(form: web::Json<ToolEmailConfig>) -> impl Responder {
        ToolEmailConfig::update_by_id(form.0).await;
        RespBuilder::<&str>::ok().build()
    }

    pub async fn send_email(send_form: web::Json<EmailSendForm>) -> impl Responder {
        let email_config = ToolEmailConfig::get_by_id().await.unwrap_or_default();
        if email_config.is_none() {
            return RespBuilder::<&str>::ok().with_msg("请先配置邮箱").build();
        }
        let send_form = send_form.0;

        if send_form.to.is_empty() {
            return RespBuilder::<&str>::ok().with_msg("收件人为空").build();
        }

        let email_config = email_config.unwrap();

        send_form.to.iter().for_each(|to| {
            let messages = Message::builder()
                .from(
                    format!(" <{}>", email_config.from_user.clone().unwrap())
                        .parse()
                        .unwrap(),
                )
                .to(format!(" <{}>", to).parse().unwrap())
                .subject(send_form.subject.to_string())
                .body(send_form.content.to_string())
                .unwrap();

            let credentials = Credentials::new(
                email_config.user.clone().unwrap(),
                email_config.pass.clone().unwrap(),
            );

            let st = SmtpTransport::relay("smtp.qq.com")
                .unwrap()
                .credentials(credentials)
                .build();

            let _ = st.send(&messages);
        });

        RespBuilder::ok().with_msg("发送成功").build()
    }
}
