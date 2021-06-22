use anyhow::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::models::RBatisModel;
use crate::RB;

/// 七牛云配置
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct ToolQiniuConfig {
    /// ID
    pub id: Option<i64>,
    /// accessKey
    pub access_key: Option<String>,
    /// Bucket 识别符
    pub bucket: Option<String>,
    /// 外链域名
    pub host: Option<String>,
    /// secretKey
    pub secret_key: Option<String>,
    /// 空间类型
    pub r#type: Option<String>,
    /// 机房
    pub zone: Option<String>,
}
