use anyhow::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::dtos::app::{AppExportDto, AppQuery};
use crate::models::RBatisModel;
use crate::RB;

/// 应用管理
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct MntApp {
    /// ID
    pub id: Option<i64>,
    /// 应用名称
    pub name: Option<String>,
    /// 上传目录
    pub upload_path: Option<String>,
    /// 部署路径
    pub deploy_path: Option<String>,
    /// 备份路径
    pub backup_path: Option<String>,
    /// 应用端口
    pub port: Option<i32>,
    /// 启动脚本
    pub start_script: Option<String>,
    /// 部署脚本
    pub deploy_script: Option<String>,
    /// 创建者
    pub create_by: Option<String>,
    /// 更新者
    pub update_by: Option<String>,
    /// 创建日期
    pub create_time: Option<chrono::NaiveDateTime>,
    /// 更新时间
    pub update_time: Option<chrono::NaiveDateTime>,
}

impl MntApp {
    pub async fn page(query: AppQuery) -> Result<Page<Self>> {
        let mut wr = RB.new_wrapper();
        if query.name.is_some() {
            wr.eq("name", query.name);
        }
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

    pub async fn export_list() -> Result<Vec<AppExportDto>> {
        let sql = r#"
        SELECT
            ma.name ,
            ma.port ,
            ma.upload_path ,
            ma.deploy_path ,
            ma.backup_path ,
            ma.start_script ,
            ma.deploy_script ,
            ma.create_time
        FROM
            mnt_app ma
        "#;

        let export_list: Vec<AppExportDto> = RB.py_fetch("", sql, &json!({})).await?;
        Ok(export_list)
    }
}
