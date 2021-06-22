use anyhow::Result;
use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::dtos::job::{JobExportDto, JobQuery};
use crate::models::RBatisModel;
use crate::RB;

/// 岗位
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct SysJob {
    /// ID
    pub id: Option<i64>,
    /// 岗位名称
    pub name: Option<String>,
    /// 岗位状态
    pub enabled: Option<u8>,
    /// 排序
    pub job_sort: Option<i32>,
    /// 创建者
    pub create_by: Option<String>,
    /// 更新者
    pub update_by: Option<String>,
    /// 创建日期
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 更新时间
    pub update_time: Option<chrono::NaiveDateTime>,
}

impl SysJob {
    pub async fn page(query: JobQuery) -> Result<Page<Self>> {
        let mut wr = RB.new_wrapper();
        if query.name.is_some() {
            wr.eq("name", query.name);
        }
        if query.enabled.is_some() {
            wr.eq("enabled", query.enabled);
        }
        wr.order_by(true, &["job_sort"]);
        wr.check()?;

        let pages: Page<Self> = RB
            .fetch_page_by_wrapper(
                "",
                &wr,
                &PageRequest::new(query.current.unwrap_or(1), query.size.unwrap_or(20)),
            )
            .await?;
        Ok(pages)
    }

    pub async fn export_list() -> Result<Vec<JobExportDto>> {
        let sql = r#"
        SELECT
            sj.name ,
            case
                when sj.enabled = 1 then '启用'
                else '停用'
            end as enabled,
            sj.create_time
        FROM
            sys_job sj
        order by
            sj.job_sort
        "#;
        let export_list: Vec<JobExportDto> = RB.py_fetch("", sql, &json!({})).await?;
        Ok(export_list)
    }
}
