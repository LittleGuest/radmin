use actix_web::{HttpResponse, Responder, web};
use chrono::Local;
use excel::*;

use crate::commons::RespBuilder;
use crate::controller::Controller;
use crate::dtos::dept::{DeptForm, DeptQuery};
use crate::models::SysDept;

pub struct DeptController;

impl Controller for DeptController {
    type M = SysDept;
}

impl DeptController {
    pub async fn page(query: web::Query<DeptQuery>) -> impl Responder {
        let data = SysDept::page(query.0).await.unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn superior(ids: web::Json<Vec<i64>>) -> impl Responder {
        let ids = ids.0;
        let data = SysDept::superior(ids).await.unwrap_or_default();
        RespBuilder::ok().with_data(data).build()
    }

    pub async fn export() -> impl Responder {
        let export_list = SysDept::export_list().await.unwrap_or_default();

        let mut wb = Workbook::create_in_memory();
        let mut sheet = wb.create_sheet("部门");

        // set column width
        sheet.add_column(Column { width: 20.0 });
        sheet.add_column(Column { width: 20.0 });
        sheet.add_column(Column { width: 20.0 });

        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;
            sw.append_row(row!["部门名称", "部门状态", "创建日期"])?;
            export_list.into_iter().for_each(|el| {
                let name = el.name.unwrap_or_default();
                let enabled = el.enabled.unwrap_or_default();
                let create_time = el.create_time.unwrap_or_default();
                let _ = sw.append_row(row![name, enabled, create_time]);
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
                    "attachment;filename={}部门数据.xlsx",
                    Local::now().format("%Y-%m-%d_%H:%M:%S")
                ),
            )
            .body(data.unwrap())
    }
}
