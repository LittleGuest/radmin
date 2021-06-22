use anyhow::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::models::RBatisModel;
use crate::RB;

/// 七牛云文件存储
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct ToolQiniuContent {
    /// ID
    pub id: Option<i64>,
    /// Bucket 识别符
    pub bucket: Option<String>,
    /// 文件名称
    pub name: Option<String>,
    /// 文件大小
    pub size: Option<String>,
    /// 文件类型：私有或公开
    pub r#type: Option<String>,
    /// 文件url
    pub url: Option<String>,
    /// 文件后缀
    pub suffix: Option<String>,
    /// 上传或同步的时间
    pub update_time: Option<chrono::NaiveDateTime>,
}
