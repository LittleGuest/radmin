use anyhow::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::models::RBatisModel;
use crate::RB;

/// 定时任务日志
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct SysQuartzLog {
    /// ID
    pub id: Option<i64>,
    /// Bean名称
    pub bean_name: Option<String>,
    /// 创建时间
    pub create_time: Option<chrono::NaiveDateTime>,
    /// cron表达式
    pub cron_expression: Option<String>,
    /// 异常信息
    pub exception_detail: Option<String>,
    /// 成功
    pub is_success: Option<u8>,
    /// 任务名称
    pub job_name: Option<String>,
    /// 方法名
    pub method_name: Option<String>,
    /// 参数
    pub params: Option<String>,
    /// 耗时
    pub time: Option<i64>,
}
