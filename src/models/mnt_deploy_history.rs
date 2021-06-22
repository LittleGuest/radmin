use anyhow::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::models::RBatisModel;
use crate::RB;

/// 部署历史管理
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct MntDeployHistory {
    /// ID
    pub id: Option<String>,
    /// 应用名称
    pub app_name: Option<String>,
    /// 部署日期
    pub deploy_date: Option<chrono::NaiveDateTime>,
    /// 部署用户
    pub deploy_user: Option<String>,
    /// 服务器IP
    pub ip: Option<String>,
    /// 部署编号
    pub deploy_id: Option<i64>,
}
