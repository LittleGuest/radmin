use actix_web::{HttpResponse, Responder, web};
use chrono::Local;
use excel::*;

use crate::commons::RespBuilder;
use crate::controller::Controller;
use crate::dtos::app::AppQuery;
use crate::models::MntApp;

pub struct AppController;

impl Controller for AppController {
    type M = MntApp;
}

impl AppController {
    pub async fn page(query: web::Query<AppQuery>) -> impl Responder {
        let data = MntApp::page(query.0).await.unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn export() -> impl Responder {
        let export_list = MntApp::export_list().await.unwrap_or_default();

        let mut wb = Workbook::create_in_memory();
        let mut sheet = wb.create_sheet("应用");

        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;
            sw.append_row(row![
                "应用名称",
                "端口",
                "上传目录",
                "部署目录",
                "备份目录",
                "启动脚本",
                "部署脚本",
                "创建日期"
            ])?;
            export_list.into_iter().for_each(|el| {
                let name = el.name.unwrap_or_default();
                let port = el.port.unwrap_or_default().to_string();
                let upload_path = el.upload_path.unwrap_or_default();
                let deploy_path = el.deploy_path.unwrap_or_default();
                let backup_path = el.backup_path.unwrap_or_default();
                let start_script = el.start_script.unwrap_or_default();
                let deploy_script = el.deploy_script.unwrap_or_default();
                let create_time = el.create_time.unwrap_or_default();
                let _ = sw.append_row(row![
                    name,
                    port,
                    upload_path,
                    deploy_path,
                    backup_path,
                    start_script,
                    deploy_script,
                    create_time
                ]);
            });
            Ok(())
        })
            .expect("write excel error!");
        let data = wb.close().expect("close excel error!");

        if data.is_none() {
            return HttpResponse::InternalServerError().finish();
        }
        HttpResponse::Ok()
            .header("Content-Type", "application/octet-stream")
            .header(
                "Content-Disposition",
                format!(
                    "attachment;filename={}应用数据.xlsx",
                    Local::now().format("%Y-%m-%d_%H:%M:%S")
                ),
            )
            .body(data.unwrap())
    }
}
