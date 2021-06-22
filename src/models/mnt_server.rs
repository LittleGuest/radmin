use anyhow::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::models::RBatisModel;
use crate::RB;

/// 服务器管理
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct MntServer {
    /// ID
    pub id: Option<i64>,
    /// 账号
    pub account: Option<String>,
    /// IP地址
    pub ip: Option<String>,
    /// 名称
    pub name: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// 端口
    pub port: Option<i32>,
    /// 创建者
    pub create_by: Option<String>,
    /// 更新者
    pub update_by: Option<String>,
    /// 创建时间
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 更新时间
    pub update_time: Option<chrono::NaiveDateTime>,
}
