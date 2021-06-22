use anyhow::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::models::RBatisModel;
use crate::RB;

/// 数据库管理
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct MntDatabase {
    /// ID
    pub id: Option<String>,
    /// 名称
    pub name: Option<String>,
    /// jdbc连接
    pub jdbc_url: Option<String>,
    /// 账号
    pub user_name: Option<String>,
    /// 密码
    pub pwd: Option<String>,
    /// 创建者
    pub create_by: Option<String>,
    /// 更新者
    pub update_by: Option<String>,
    /// 创建时间
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 更新时间
    pub update_time: Option<chrono::NaiveDateTime>,
}
