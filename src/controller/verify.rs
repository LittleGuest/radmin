use actix_web::{Responder, web};
use captcha::{Captcha, Geometry};
use captcha::filters::{Cow, Noise, Wave};
use redis::Commands;

use crate::commons::{EMAIL_RESET_EMAIL_CODE, RespBuilder};
use crate::dtos::verify::VerifyQuery;
use crate::models::ToolEmailConfig;
use crate::utils::email_util::{EmailInfo, send_email};
use crate::utils::redis_util;

pub struct VerifyController;

impl VerifyController {
    pub async fn reset_email(query: web::Query<VerifyQuery>) -> impl Responder {
        let email = query.0.email.unwrap_or_default();
        let email_config = ToolEmailConfig::get_by_id().await.unwrap_or_default();
        if email_config.is_none() {
            return RespBuilder::<&str>::ok().with_msg("请先配置邮箱").build();
        }
        let email_config = email_config.unwrap();

        let mut captcha = Captcha::new();
        captcha
            .add_chars(6)
            .apply_filter(Noise::new(0.2))
            .apply_filter(Wave::new(2.0, 20.0))
            .view(220, 120)
            .apply_filter(
                Cow::new()
                    .min_radius(40)
                    .max_radius(50)
                    .circles(1)
                    .area(Geometry::new(40, 150, 50, 70)),
            );

        let key = format!("{}{}", EMAIL_RESET_EMAIL_CODE, email);
        let code = captcha.chars_as_string();

        redis_util::set(&key, &code, Some(60 * 5)).unwrap();

        let mut ctx = tera::Context::new();
        ctx.insert("code", &code);
        let mut t = tera::Tera::default();
        let subject = t.render_str(RESET_EMAIL_TEMPLATE, &ctx).unwrap_or_default();

        let email_info = EmailInfo {
            host: email_config.host.unwrap(),
            port: email_config.port.unwrap(),
            account: email_config.from_user.unwrap(),
            password: email_config.pass.unwrap(),
            subject,
            content: "".to_string(),
            to: vec![email],
        };

        send_email(email_info);

        RespBuilder::<&str>::ok().with_msg("发送成功").build()
    }
}

const RESET_EMAIL_TEMPLATE: &str = r#"
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Strict//EN"  "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd">
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <meta http-equiv="Content-Type" content="text/html; charset=UTF-8"/>
    <style>
        @page {
            margin: 0;
        }
    </style>
</head>
<body style="margin: 0px;
            padding: 0px;
			font: 100% SimSun, Microsoft YaHei, Times New Roman, Verdana, Arial, Helvetica, sans-serif;
            color: #000;">
<div style="height: auto;
			width: 820px;
			min-width: 820px;
			margin: 0 auto;
			margin-top: 20px;
            border: 1px solid #eee;">
    <div style="padding: 10px;padding-bottom: 0px;">
        <p style="margin-bottom: 10px;padding-bottom: 0px;">尊敬的用户，您好：</p>
        <p style="text-indent: 2em; margin-bottom: 10px;">您正在申请邮箱验证，您的验证码为：</p>
        <p style="text-align: center;
			font-family: Times New Roman;
			font-size: 22px;
			color: #C60024;
			padding: 20px 0px;
			margin-bottom: 10px;
			font-weight: bold;
			background: #ebebeb;">{{ code }}</p>
        <div class="foot-hr hr" style="margin: 0 auto;
			z-index: 111;
			width: 800px;
			margin-top: 30px;
			border-top: 1px solid #DA251D;">
        </div>
        <div style="text-align: center;
			font-size: 12px;
			padding: 20px 0px;
			font-family: Microsoft YaHei;">
            Copyright &copy;${.now?string("yyyy")} <a hover="color: #DA251D;" style="color: #999;" href="" target="_blank">RADMIN</a> 后台管理系统 All Rights Reserved.
        </div>

    </div>
</div>
</body>
</html>
"#;
