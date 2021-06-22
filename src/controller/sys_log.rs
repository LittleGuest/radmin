use actix_web::{HttpResponse, Responder, web};
use chrono::Local;
use excel::*;

use crate::commons::RespBuilder;
use crate::dtos::log::LogQuery;
use crate::models::SysLog;

pub struct LogController;

impl LogController {
    pub async fn page(query: web::Query<LogQuery>) -> impl Responder {
        let data = SysLog::page(query.0).await.unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn page_error(mut query: web::Query<LogQuery>) -> impl Responder {
        query.0.log_type = Some("ERROR".to_string());
        let data = SysLog::page(query.0).await.unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn user_log(mut query: web::Query<LogQuery>) -> impl Responder {
        // TODO 用户ID
        let user_id = 0;
        query.0.log_type = Some("INFO".to_string());
        query.0.username = Some(user_id);
        let data = SysLog::page(query.0).await.unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn delete_error() -> impl Responder {
        SysLog::delete_by_type(Some("ERROR".to_string())).await;
        RespBuilder::<&str>::ok().build()
    }

    pub async fn delete_info() -> impl Responder {
        SysLog::delete_by_type(Some("INFO".to_string())).await;
        RespBuilder::<&str>::ok().build()
    }

    pub async fn error_detail(id: web::Path<i64>) -> impl Responder {
        let data = SysLog::error_detail(id.0).await.unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn export_info() -> HttpResponse {
        let data = Self::download(Some("INFO".to_string())).await;

        HttpResponse::Ok()
            .header("Content-Type", "application/octet-stream")
            .header(
                "Content-Disposition",
                format!(
                    "attachment;filename={}日志数据.xlsx",
                    Local::now().format("%Y-%m-%d_%H:%M:%S"),
                ),
            )
            .body(data)
    }

    pub async fn export_error() -> HttpResponse {
        let data = Self::download(Some("ERROR".to_string())).await;
        HttpResponse::Ok()
            .header("Content-Type", "application/octet-stream")
            .header(
                "Content-Disposition",
                format!(
                    "attachment;filename={}异常日志数据.xlsx",
                    Local::now().format("%Y-%m-%d_%H:%M:%S"),
                ),
            )
            .body(data)
    }

    pub async fn download(log_type: Option<String>) -> Vec<u8> {
        let export_list = SysLog::export_list(log_type).await.unwrap_or_default();

        let mut wb = Workbook::create_in_memory();
        let mut sheet = wb.create_sheet("日志");

        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;
            sw.append_row(row![
                "用户名",
                "IP",
                "IP来源",
                "描述",
                "浏览器",
                "请求耗时/毫秒",
                "异常详情",
                "创建日期"
            ])?;
            export_list.into_iter().for_each(|el| {
                let username = el.username.unwrap_or_default();
                let request_ip = el.request_ip.unwrap_or_default();
                let address = el.address.unwrap_or_default();
                let description = el.description.unwrap_or_default();
                let browser = el.browser.unwrap_or_default();
                let time = el.time.unwrap_or_default().to_string();
                let exception_detail = el.exception_detail.unwrap_or_default();
                let create_time = el.create_time.unwrap_or_default();
                let _ = sw.append_row(row![
                    username,
                    request_ip,
                    address,
                    description,
                    browser,
                    time,
                    exception_detail,
                    create_time
                ]);
            });
            Ok(())
        })
            .expect("write excel error!");
        let data = wb.close().expect("close excel error!");
        data.unwrap_or_default()
    }
}
