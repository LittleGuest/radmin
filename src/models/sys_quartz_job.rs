use anyhow::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::models::RBatisModel;
use crate::RB;

/// 定时任务
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct SysQuartzJob {
    /// ID
    pub id: Option<i64>,
    /// Spring Bean名称
    pub bean_name: Option<String>,
    /// cron 表达式
    pub cron_expression: Option<String>,
    /// 状态：1暂停、0启用
    pub is_pause: Option<u8>,
    /// 任务名称
    pub job_name: Option<String>,
    /// 方法名称
    pub method_name: Option<String>,
    /// 参数
    pub params: Option<String>,
    /// 备注
    pub description: Option<String>,
    /// 负责人
    pub person_in_charge: Option<String>,
    /// 报警邮箱
    pub email: Option<String>,
    /// 子任务ID
    pub sub_task: Option<String>,
    /// 任务失败后是否暂停
    pub pause_after_failure: Option<u8>,
    /// 创建者
    pub create_by: Option<String>,
    /// 更新者
    pub update_by: Option<String>,
    /// 创建日期
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 更新时间
    pub update_time: Option<chrono::NaiveDateTime>,
}
