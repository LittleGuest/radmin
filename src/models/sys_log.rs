use std::collections::HashMap;

use anyhow::Result;
use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::dtos::log::{LogExportDto, LogQuery};
use crate::models::RBatisModel;
use crate::RB;

/// 系统日志
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct SysLog {
    /// ID
    pub id: Option<i64>,
    /// 描述
    pub description: Option<String>,
    /// 日志类型
    pub log_type: Option<String>,
    /// 方法
    pub method: Option<String>,
    /// 参数
    pub params: Option<String>,
    /// 请求IP地址
    pub request_ip: Option<String>,
    /// 耗时
    pub time: Option<i64>,
    /// 用户名
    pub username: Option<String>,
    /// 地址
    pub address: Option<String>,
    /// 浏览器
    pub browser: Option<String>,
    /// 异常信息
    pub exception_detail: Option<String>,
    /// 创建时间
    pub create_time: Option<chrono::NaiveDateTime>,
}

impl SysLog {
    pub async fn page(query: LogQuery) -> Result<Page<Self>> {
        let mut wr = RB.new_wrapper();
        if query.blurry.is_some() {
            wr.eq("description", query.blurry);
        }
        if query.log_type.is_some() {
            wr.eq("log_type", query.log_type);
        }
        if query.username.is_some() {
            wr.eq("username", query.username);
        }
        wr.check()?;

        let logs: Page<Self> = RB
            .fetch_page_by_wrapper(
                "",
                &wr,
                &PageRequest::new(query.current.unwrap_or(1), query.size.unwrap_or(20)),
            )
            .await?;
        Ok(logs)
    }

    pub async fn delete_by_type(log_type: Option<String>) -> Result<()> {
        if log_type.is_none() {
            return Ok(());
        }
        let mut wr = RB.new_wrapper();
        wr.eq("log_type", log_type);
        wr.check()?;
        RB.remove_by_wrapper::<Self>("", &wr).await?;
        Ok(())
    }

    pub async fn error_detail(log_id: i64) -> Result<HashMap<String, String>> {
        let mut wr = RB.new_wrapper();
        wr.eq("log_id", log_id);
        wr.check()?;
        let log_detail: Self = RB.fetch_by_wrapper("", &wr).await?;
        let mut map: HashMap<String, String> = HashMap::with_capacity(1);
        map.insert(
            "exception".to_string(),
            log_detail.exception_detail.unwrap_or_default(),
        );
        Ok(map)
    }

    pub async fn export_list(log_type: Option<String>) -> Result<Vec<LogExportDto>> {
        let sql = r#"
        SELECT
            sl.username ,
            sl.request_ip ,
            sl.address ,
            sl.description ,
            sl.browser ,
            sl.`time` ,
            sl.exception_detail ,
            sl.create_time
        FROM
            sys_log sl
        where
            1 = 1
            if log_type != null:
              and sl.log_type = #{log_type}
        "#;
        let export_list: Vec<LogExportDto> = RB
            .py_fetch("", &sql, &json!({ "log_type": log_type }))
            .await?;
        Ok(export_list)
    }
}
