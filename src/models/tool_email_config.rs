use anyhow::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::models::RBatisModel;
use crate::RB;

/// 邮箱配置
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug, Eq, PartialEq)]
pub struct ToolEmailConfig {
    /// ID
    pub id: Option<i64>,
    /// 收件人
    pub from_user: Option<String>,
    /// 邮件服务器SMTP地址
    pub host: Option<String>,
    /// 密码
    pub pass: Option<String>,
    /// 端口
    pub port: Option<String>,
    /// 发件者用户名
    pub user: Option<String>,
}

impl ToolEmailConfig {
    pub async fn get_by_id() -> Result<Option<Self>> {
        let mut w = RB.new_wrapper();
        w.eq("config_id", 1);
        w.check()?;
        let res: Self = RB.fetch_by_wrapper("", &w).await?;
        Ok(Option::from(res))
    }

    pub async fn update_by_id(form: Self) -> Result<()> {
        RB.update_by_id("", &form).await?;
        Ok(())
    }
}
