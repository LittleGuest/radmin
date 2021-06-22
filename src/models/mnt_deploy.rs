use anyhow::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::models::RBatisModel;
use crate::RB;

/// 部署管理
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct MntDeploy {
    /// ID
    pub id: Option<i64>,
    /// 应用编号
    pub app_id: Option<i64>,
    /// 创建者
    pub create_by: Option<String>,
    /// 更新者
    pub update_by: Option<String>,
    /// 创建时间
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 更新时间
    pub update_time: Option<chrono::NaiveDateTime>,
}
