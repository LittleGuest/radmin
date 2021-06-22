use anyhow::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::models::RBatisModel;
use crate::RB;

/// 应用与服务器关联
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct MntDeployServer {
    /// 部署ID
    pub deploy_id: Option<i64>,
    /// 服务ID
    pub server_id: Option<i64>,
}
