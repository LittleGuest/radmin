#[derive(Serialize, Deserialize)]
pub struct ToolEmailConfigForm {
    // ID
    pub config_id: Option<i64>,
    // 收件人
    pub from_user: Option<String>,
    // 邮件服务器SMTP地址
    pub host: Option<String>,
    // 密码
    pub pass: Option<String>,
    // 端口
    pub port: Option<String>,
    // 发件者用户名
    pub user: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EmailSendForm {
    // 邮件标题
    pub subject: String,
    // 收件邮箱
    pub to: Vec<String>,
    // 内容
    pub content: String,
}
